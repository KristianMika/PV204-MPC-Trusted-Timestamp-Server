use actix_web::web::Data;
use actix_web::{App, HttpServer};
use frost_dalek::Parameters;
use futures::lock::Mutex;
use timestamp_server::{Config, ServerState};
mod services;
use services::*;
use std::env;

use rustls::server::{ServerConfig};
use rustls::server::AllowAnyAnonymousOrAuthenticatedClient;
use rustls_pemfile::{certs, read_all, pkcs8_private_keys};

use std::io::BufReader;
use std::fs::File;


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
        Ok(val) => val,
        Err(_) => panic!(
            "Could not read/parse the config file at path {}.",
            CONFIG_PATH
        ),
    };

    log::info!("Successfully loaded the config file.");

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


    // Load key files
    let cert_file = &mut BufReader::new(
    File::open("certificates/fullchain.pem").unwrap());
    let key_file = &mut BufReader::new(
    File::open("certificates/privkey.pem").unwrap());

    // Parse the certificate and set it in the configuration
    // may show error of self declared cerificate or something like that
    let cert_chain = read_all(cert_file).unwrap();
    //this is in case of der fromat which should probably be used
    let der_certs = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();

    // empty for now
    let mut roots = rustls::RootCertStore::empty();

    let mut config = ServerConfig::builder()
                    .with_safe_defaults()
                    //how the fuck do I create RootCertStore
                    .with_client_cert_verifier(AllowAnyAnonymousOrAuthenticatedClient::new(roots))
                    .with_single_cert(der_certs, key).unwrap();


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
    })
    .bind_rustls(server_address, config)?
    .run()
    .await
}
