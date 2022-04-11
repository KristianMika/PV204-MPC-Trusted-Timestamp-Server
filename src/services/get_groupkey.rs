use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use futures::lock::Mutex;
use timestamp_server::{ServerState, State};

/// Returns the groupkey public key in JSON format.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Anyone
#[get("/groupkey")]
pub async fn get_groupkey(state: Data<Mutex<ServerState>>) -> impl Responder {

    if state.lock().await.state != State::Timestamping {
        // TODO: return an error
    }

    // TODO: consider returning it in hex
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(state.lock().await.group_key().unwrap())
}
