use actix_web::{post, HttpResponse, Responder};

/// Requests a signature share for the submited hash.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Server
#[post("/partial_signature")]
pub async fn post_partial_signature() -> impl Responder {
    // TODO: check the state

    // TODO: compute the partial signature

    // TODO: return the partial signature
    HttpResponse::Ok()
}
