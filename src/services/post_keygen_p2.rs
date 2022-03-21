use crate::Mutex;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use timestamp_server::ServerState;

#[post("/keygen_phase1")]
pub async fn post_keygen_p2(state: Data<Mutex<ServerState>>) -> impl Responder {
    HttpResponse::Ok()
}
