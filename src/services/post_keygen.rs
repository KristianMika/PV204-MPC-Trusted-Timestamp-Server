use actix_web::{get, HttpResponse, Responder};

/// Triggers the key generation phase
///
/// # Preconditions
/// - The context has been configured
///
/// # Can submit
/// - Admin
#[get("/keygen")]
pub async fn post_keygen() -> impl Responder {
    // TODO: check the state

    // TODO: trigger key generation
    HttpResponse::Ok()
    // TODO: return pubkey
}
