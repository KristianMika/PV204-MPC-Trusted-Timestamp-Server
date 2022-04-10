use actix_web::web::Data;
use actix_web::{post, HttpResponse, Responder};
use futures::lock::Mutex;
use timestamp_server::{ServerState, State};

/// Requests a signature share for the submited hash.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Server
#[post("/partial_signature")]
pub async fn post_partial_signature(state: Data<Mutex<ServerState>>) -> impl Responder {
    if state.lock().await.state != State::Timestamping {
        return HttpResponse::Forbidden();
    }

    // TODO: compute the partial signature

    // TODO: return the partial signature
    HttpResponse::Ok()
}
