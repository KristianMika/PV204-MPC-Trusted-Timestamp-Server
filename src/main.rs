use actix_web::{App, HttpServer};
mod services;
use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: use an automata and track states + check transitions
    HttpServer::new(|| {
        App::new()
            .service(get_pubkey::get_pubkey)
            .service(post_create_timestamp::post_create_timestamp)
            .service(post_keygen::post_keygen)
            .service(post_partial_signature::post_partial_signature)
            .service(post_reset::post_reset)
            .service(post_setup::post_setup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
