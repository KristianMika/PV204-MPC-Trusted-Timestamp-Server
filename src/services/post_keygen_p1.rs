use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use std::sync::Mutex;
use timestamp_server::ServerState;

#[post("/keygen_phase1")]

pub async fn post_keygen_p1(state: Data<Mutex<ServerState>>) -> impl Responder {
    HttpResponse::Ok()
}
