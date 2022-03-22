use actix_web::web::Data;
use actix_web::{App, HttpServer};
use frost_dalek::Parameters;
use frost_dalek::Participant;
use std::sync::Mutex;
use timestamp_server::ServerState;
mod services;
use services::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: use an automata and track states + check transitions

    // TODO: error handling!
    // TODO: load from env variables / a config file
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 5 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Current tmp usage: cargo run [server_id] [server_port] [ip2:port2] [ip3:port3]",
        ));
    }

    // TODO: a temporary CLI arguments handling
    let default_params = Parameters { t: 2, n: 3 };
    let (participant, coefs) = Participant::new(&default_params, argv[1].parse::<u32>().unwrap());
    let server_address = String::from("127.0.0.1:") + &argv[2];
    let servers = vec![argv[3].clone(), argv[4].clone()];
    let server_state = Data::new(Mutex::new(ServerState::new(
        participant,
        coefs,
        default_params,
        servers,
        vec![],
    )));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&server_state))
            .service(get_pubkey::get_pubkey)
            .service(post_create_timestamp::post_create_timestamp)
            .service(post_keygen::post_keygen)
            .service(post_keygen_p1::post_keygen_p1)
            .service(post_keygen_p2::post_keygen_p2)
            .service(post_partial_signature::post_partial_signature)
            .service(post_reset::post_reset)
            .service(post_setup::post_setup)
    })
    .bind(server_address)?
    .run()
    .await
}
