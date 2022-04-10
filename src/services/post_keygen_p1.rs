use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use frost_dalek::Participant;
use std::sync::Mutex;
use timestamp_server::ServerState;

#[post("/keygen_phase1")]

pub async fn post_keygen_p1(
    state: Data<Mutex<ServerState>>,
    request: web::Json<String>,
) -> impl Responder {
    // TODO: check the state
    log::info!("KeygenPhase1");
    let participant: Participant = match serde_json::from_str(&request.0[..]) {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest(),
    };

    match participant
        .proof_of_secret_key
        .verify(&participant.index, &participant.public_key().unwrap())
    {
        Ok(_) => {
            log::info!(
                "Successfully veried shares from server with index {}.",
                participant.index
            )
        }
        Err(_) => return HttpResponse::Unauthorized(),
    }

    // TODO: update the state
    HttpResponse::Ok()
}
