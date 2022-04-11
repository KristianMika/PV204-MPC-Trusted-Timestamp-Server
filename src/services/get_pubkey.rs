use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use futures::lock::Mutex;
use timestamp_server::{ServerState, State};

/// Returns the server's public key in JSON format.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Anyone
#[get("/pubkey")]
pub async fn get_pubkey(state: Data<Mutex<ServerState>>) -> impl Responder {
    // TODO: check the state

    if state.lock().await.state != State::Timestamping {
        // TODO: return an error
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(state.lock().await.secret_key.as_ref().unwrap().to_public())
}
