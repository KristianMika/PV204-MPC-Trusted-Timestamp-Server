use crate::keygen::*;
use actix_web::web::Data;
use actix_web::{post, HttpResponse, Responder};
use frost_dalek::DistributedKeyGeneration;
use frost_dalek::Participant;
use futures::lock::Mutex;
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

    // trigger the pubkey exchange
    // TODO: check response
    log::info!("received /keygen");
    actix_rt::spawn(async {
        log::info!("async send_this_participant started");
        send_this_participant(state).await.unwrap();
        log::info!("async send_this_participant finished");
    });

    HttpResponse::Ok()
}
