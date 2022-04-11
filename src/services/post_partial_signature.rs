use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder};
use futures::lock::Mutex;
use timestamp_server::PartialSignatureRequest;
use timestamp_server::{ServerState, State};

/// Requests a signature share for the submited hash.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Server
#[post("/partial_signature")]
pub async fn post_partial_signature(
    state: Data<Mutex<ServerState>>,
    request: web::Json<PartialSignatureRequest>,
) -> impl Responder {
    if state.lock().await.state != State::Timestamping {
        // TODO: Return error
        //return HttpResponse::Forbidden();
    }

    let secret_key = state.lock().await.secret_key.as_ref().unwrap().clone();
    let group_key = state.lock().await.group_key.unwrap().clone();
    let mut unlocked_state = state.lock().await;
    let mut secret_shares = unlocked_state.secret_commitment_shares.as_mut().unwrap();
    let partial_sig = match secret_key.sign(
        &to_array(&request.message_hash),
        &group_key,
        &mut secret_shares,
        request.commitment_index,
        &request.signers,
    ) {
        Ok(v) => v,
        Err(e) => panic!("Kristian is corrupt!!!\n{}", e),
    };
    drop(unlocked_state);

    web::Json(partial_sig)
}

// TODO: move to utils
fn to_array(v: &Vec<u8>) -> [u8; 64] {
    let slice = v.as_slice();
    let array: [u8; 64] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 64, v.len()),
    };
    array
}
