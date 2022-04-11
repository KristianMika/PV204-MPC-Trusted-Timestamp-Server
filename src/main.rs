use actix_web::web::Data;
use actix_web::{App, HttpServer};
use frost_dalek::Parameters;
use futures::lock::Mutex;
use timestamp_server::{Config, ServerState};
mod services;
use services::*;
use std::env;

mod utils;
use utils::*;
pub mod keygen;

const CONFIG_PATH: &str = "config/config.toml";
const BIND_IP: &str = "0.0.0.0";
const PROTOCOL: &str = "http";

// TODO: consider anyhow
// TODO: use an automata and track states + check transitions
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server_index: String = match env::var("SERVER_INDEX") {
        Ok(val) => val,
        Err(_) => panic!("SERVER_INDEX is not set."),
    };
    let server_index: usize = match server_index.parse() {
        Ok(val) => val,
        Err(_) => panic!("Invalid SERVER_INDEX value: {}", server_index),
    };
    log::info!("Starting server with index {}.", server_index);

    let config: Config = match read_config(CONFIG_PATH) {
        Ok(val) => {
            log::info!("Successfully loaded the config file.");
            val
        }
        Err(_) => panic!(
            "Could not read/parse the config file at path {}.",
            CONFIG_PATH
        ),
    };

    let parameters = Parameters {
        t: config.t,
        n: config.n,
    };

    let server_address = format!("{ip}:{port}", ip = BIND_IP, port = &config.port.to_string());
    let server_state = Data::new(Mutex::new(ServerState::new(
        parameters,
        config.servers,
        server_index,
    )));

    log::info!("Starting the server at {}.", server_address);
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
            .service(post_init::post_init)
            .service(get_commitment::get_commitment)
    })
    .bind(server_address)?
    .run()
    .await
}
