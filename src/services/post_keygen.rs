use crate::keygen::*;
use actix_web::web::Data;
use actix_web::{post, HttpResponse, Responder};
use frost_dalek::DistributedKeyGeneration;
use frost_dalek::Participant;
use std::sync::Mutex;
use timestamp_server::ServerState;

/// Triggers the key generation phase
///
/// # Preconditions
/// - The context has been configured
///
/// # Can submit
/// - Admin
#[post("/keygen")]
pub async fn post_keygen(state: Data<Mutex<ServerState>>) -> impl Responder {
    // TODO: check the state

    let params = state.lock().unwrap().parameters.clone();
    // TODO: trigger key generation phase 1

    // TODO: check response
    let res = send_this_participant(state.clone()).await;
    let unlocked_state = state.lock().unwrap();
    let (this_participant, this_part_coefs) =
        &unlocked_state.participants[unlocked_state.this_server_index - 1];
    // TODO: store to the state
    let this_key_state = DistributedKeyGeneration::<_>::new(
        &params,
        &this_participant.index,
        &this_part_coefs,
        &mut unlocked_state.get_other_participants(),
    )
    .unwrap();

    let to_share = this_key_state.their_secret_shares().unwrap().clone();
    // TODO: check how we can serialize it

    // TODO: trigger key generation phase 2
    HttpResponse::Ok()
    // TODO: return pubkey
}
