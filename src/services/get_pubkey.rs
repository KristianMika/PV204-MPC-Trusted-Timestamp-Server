use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, Responder};

/// Returns the computed public key in JSON format.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Anyone
#[get("/pubkey")]
pub async fn get_pubkey() -> impl Responder {
    // TODO: check the state

    // TODO: return the public key
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("pubkey")
}
