use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
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
#[get("/keygen")]
pub async fn post_keygen(state: Data<Mutex<ServerState>>) -> impl Responder {
    // TODO: check the state
    let params = state.lock().unwrap().parameters;
    state.lock().unwrap().participants =
        vec![Participant::new(&params, 1), Participant::new(&params, 2)];

    //  david.proof_of_secret_key.verify(&david.index, &david.public_key().unwrap()).expect("Not David! NOT DAVID!!!!!");

    // TODO: trigger key generation phase 1
    let mut other_participants: Vec<Participant> = vec![];
    // state.lock().unwrap().participants.into_iter().map(|(part, coef)| part.clone()).collect();
    for participant in &state.lock().unwrap().participants {
        other_participants.push(participant.0.clone());
    }

    let protocol_state = DistributedKeyGeneration::<_>::new(
        &params,
        &state.lock().unwrap().server_participant.index,
        &state.lock().unwrap().server_coef,
        // TODO: move to server_state
        &mut other_participants,
    )
    .unwrap();

    let to_share = protocol_state.their_secret_shares().unwrap().clone();
    // TODO: check how we can serialize it
    // TODO: move instantiation the server state
    let client = reqwest::Client::new();

    // TODO: The serialization issue
    // for server in state.lock().unwrap().servers.iter() {
    //     let res = client
    //         .post(String::from(server) + "/keygen_phase1")
    //         .json(&to_share)
    //         .send()
    //         .await;
    // }

    // TODO: trigger key generation phase 2
    HttpResponse::Ok()
    // TODO: return pubkey
}
