use crate::build_address;
use crate::PROTOCOL;
use actix_web::web::Data;
use actix_web::Responder;
use actix_web::{post, web};
use chrono::{DateTime, Utc};
use curve25519_dalek::ristretto::RistrettoPoint;
use frost_dalek::compute_message_hash;
use frost_dalek::signature::PartialThresholdSignature;
use frost_dalek::signature::Signer;
use frost_dalek::IndividualPublicKey;
use frost_dalek::Parameters;
use frost_dalek::SignatureAggregator;
use futures::lock::Mutex;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use timestamp_server::PartialSignatureRequest;
use timestamp_server::{ServerState, State};

/// The struct sent by the client in the body as JSON
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct TimestampStruct {
    /// The algorithm to be used for hashing
    /// TODO: change to enum
    hashAlgorithm: String,
    /// The hash of the data
    hashedMessage: String,
}

/// THe response struct sent to clients in JSON
/// TODO: read RFC 3161!!!
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct TimeStampResp {
    status: String, // for now, later PKIStatusInfo
    /// Can be casted to [u8;64], see partial_sig or utils, function to_array
    timeStampToken: Vec<u8>, // TODO: read the RFC 3161
    final_hash: Vec<u8>,
}

/// Requests a signed timestamp of the provided data using the specified hash algorithm
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Anyone
#[post("/timestamp")]
pub async fn post_create_timestamp(
    state: Data<Mutex<ServerState>>,
    request: web::Json<TimestampStruct>,
) -> impl Responder {
    // TODO: check the state

    let commitment_index = state.lock().await.get_and_increment_comiitment_index();

    log::info!("Using com index {}", commitment_index);
    if state.lock().await.state != State::Timestamping {
        // return HttpResponse::Forbidden();
        // TODO: return an error
    }

    const CONTEXT: &[u8] = b"diks-tits";

    //let file_hash = b"HASH-OF-THE-FILE---RECEIVED-BY-THE-USER--PRELIM-TESTED-BY-US";
    let file_hash = match hex::decode(&request.hashedMessage) {
        Ok(val) => val,
        Err(_) => {
            let err_response = TimeStampResp {
                status: String::from("fail"),
                timeStampToken: vec![],
                final_hash: vec![],
            };
            return web::Json(err_response);
            // return web::Json(err_response);
        }
    };

    let timenow = SystemTime::now();
    let datetime: DateTime<Utc> = timenow.into();
    let timestr = datetime.format("%Y%m%d%H%M%SZ").to_string();
    println!("RFC 3161 compliant timestamp: {}", timestr);
    let timestr = timestr.as_bytes(); // Time Stamp in UTC to avoid timezone issues. Format compliant with RFC 3161

    let mut hasher = Sha256::new();
    hasher.update(file_hash);
    hasher.update(timestr);
    let fin_hash = hasher.finalize(); // Final hash of the timestamp and the file hash

    let message_hash = compute_message_hash(&CONTEXT[..], &fin_hash[..]);

    let parameters = state.lock().await.parameters.clone();
    let group_key = state.lock().await.group_key.unwrap().clone();
    let mut aggregator =
        SignatureAggregator::new(parameters, group_key, &CONTEXT[..], &fin_hash[..]);

    let signers_to_sign = get_random_signers(&state.lock().await.parameters);

    let this_server_index = state.lock().await.this_server_index.clone();
    for signer_index in signers_to_sign.clone() {
        let target_ip = state.lock().await.servers[signer_index].clone(); // TODO: get_nth_ip(n)

        let commitment = match signer_index == this_server_index {
            true => state
                .lock()
                .await
                .public_commitment_shares
                .as_ref()
                .unwrap()
                .commitments[commitment_index]
                .clone(),
            false => get_commitment(&target_ip, commitment_index as u32).await,
        };

        let public_key = match signer_index == this_server_index {
            true => state.lock().await.secret_key.as_ref().unwrap().to_public(),
            false => get_public_key(&target_ip).await,
        };

        aggregator.include_signer(signer_index as u32, commitment, public_key);
    }

    for signer_index in signers_to_sign {
        let target_ip = state.lock().await.servers[signer_index].clone(); // TODO: get_nth_ip(n)
        let partial_sig = get_partial_signature(
            &target_ip,
            &message_hash,
            aggregator.get_signers(),
            commitment_index as usize,
        )
        .await;
        aggregator.include_partial_signature(partial_sig);
    }

    let aggregator = match aggregator.finalize() {
        Ok(v) => v,
        Err(e) => panic!("Aggregator pooped!\n{:?}", e),
    };

    let threshold_sign = match aggregator.aggregate() {
        Ok(v) => v,
        Err(e) => panic!(
            "Bad signing. Likely corrupted signees or signatures!\n{:?}",
            e
        ),
    };
    threshold_sign.verify(&group_key, &message_hash).unwrap();

    let response: TimeStampResp = TimeStampResp {
        status: String::from("Ok?"),
        timeStampToken: threshold_sign.to_bytes().to_vec(),
        final_hash: message_hash.to_vec(),
    };

    web::Json(response)
}

// TODO: implement
/// Returns a vector (a subset) of random signers
///
/// # Arguments
/// - `n` - the number of participants
pub fn get_random_signers(parameters: &Parameters) -> Vec<usize> {
    vec![0, 1] // rn a trully randomly subset chosen by me is being returned
}

pub async fn get_commitment(
    server_address: &str,
    commitment_index: u32,
) -> (RistrettoPoint, RistrettoPoint) {
    let client = reqwest::Client::new();
    // TODO: return a result!!!!!
    let res = reqwest::get(build_address(
        PROTOCOL,
        server_address,
        &format!("commitment/{}", commitment_index.to_string()),
    ))
    .await
    .unwrap()
    .json::<(RistrettoPoint, RistrettoPoint)>()
    .await;
    res.unwrap()
}

// TODO: use caching, try to cache address in the state, maybe pre-cache them
pub async fn get_public_key(server_address: &str) -> IndividualPublicKey {
    // TODO: return a result!!!!!

    let addr = build_address(PROTOCOL, server_address, "pubkey");
    log::info!(
        "Requesting a pubkey from {} address({})",
        server_address,
        addr
    );

    // TODO: check response codes everywhere!!!
    let res = reqwest::get(addr)
        .await
        .unwrap()
        .json::<IndividualPublicKey>()
        .await;
    res.unwrap()
}

pub async fn get_partial_signature(
    server_address: &str,
    message_hash: &[u8; 64],
    signers: &Vec<Signer>,
    commitment_index: usize,
) -> PartialThresholdSignature {
    let client = reqwest::Client::new();
    // TODO: return a result!!!!!
    let res = client
        .post(build_address(PROTOCOL, server_address, "partial_signature"))
        .json(&PartialSignatureRequest {
            message_hash: message_hash.to_vec(),
            signers: signers.clone(),
            commitment_index,
        })
        .send()
        .await
        .unwrap()
        .json::<PartialThresholdSignature>()
        .await;
    res.unwrap()
}
