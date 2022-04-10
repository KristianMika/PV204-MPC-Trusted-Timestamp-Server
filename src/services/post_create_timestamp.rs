use actix_web::error::ParseError::Status;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use futures::lock::Mutex;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use timestamp_server::{ServerState, State};

/// The struct sent by the client in the body as JSON
#[derive(Deserialize)]
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
    status: String,         // for now, later PKIStatusInfo
    timeStampToken: String, // TODO: read the RFC 3161
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

    if state.lock().await.state != State::Timestamping {
        // return HttpResponse::Forbidden();
        // TODO: return an error
    }

    // TODO: compute the hash(hash(data) || timestamp)
    let now = SystemTime::now();

    let mut hasher = Sha256::new();

    let request_hash = match hex::decode(&request.hashedMessage) {
        Ok(val) => val,
        Err(_) => {
            let err_response = TimeStampResp {
                status: String::from("fail"),
                timeStampToken: String::from(""),
            };
            return web::Json(err_response);
        }
    };
    // now.hash(& mut hasher); TODO
    hasher.update(request_hash);
    let result = hasher.finalize();

    // TODO: request partial signatures

    // TODO: compute the composite signature

    // TODO: return the composite signature
    let response: TimeStampResp = TimeStampResp {
        status: String::from("Ok?"),
        timeStampToken: format!("{:X}", result),
    };

    web::Json(response)
}
