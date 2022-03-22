use actix_web::{post, HttpResponse, Responder};

/// Configures the context
///
/// # Preconditions
/// - Keys have not been generated or
/// - The server has been reset
///
/// # Can submit
/// - Admin
#[post("/setup")]
pub async fn post_setup() -> impl Responder {
    // TODO: check the state

    // TODO: modify the configuration

    // TODO: update the state
    HttpResponse::Ok()
}
