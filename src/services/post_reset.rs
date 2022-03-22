use actix_web::{post, HttpResponse, Responder};

/// Resets the context
///
/// # Preconditions
/// - None
///
/// # Can submit
/// - Admin
#[post("/reset")]
pub async fn post_reset() -> impl Responder {
    // TODO: reset the context
    // TODO: reset other servers?
    HttpResponse::Ok()
}
