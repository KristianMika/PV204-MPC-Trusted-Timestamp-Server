use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use std::sync::Mutex;
use timestamp_server::ServerState;

#[post("/keygen_phase2")]
pub async fn post_keygen_p2(state: Data<Mutex<ServerState>>) -> impl Responder {
    // TODO: check the state

    // TODO: store the shares

    // TODO: send the shares to other participants

    // TODO: update the state
    HttpResponse::Ok()
}
