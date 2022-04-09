use frost_dalek::keygen::Coefficients;
use frost_dalek::Parameters;
use frost_dalek::Participant;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    /// a list of all server IPs includes this one
    pub servers: Vec<String>,
    /// the port of this server
    pub port: u16,
    pub t: u32,
    pub n:u32
}

/// Holds the state of the server, configuration, keys, etc.
pub struct ServerState {
    /// A unique index of the server with respect to the context
    pub server_participant: Participant,
    pub server_coef: Coefficients,
    /// Configuration of the context, namely t-n parameter
    pub parameters: Parameters,
    /// Other servers in the current context
    pub servers: Vec<String>,
    /// Other participants
    pub participants: Vec<(Participant, Coefficients)>,
}

impl ServerState {
    pub fn default() -> ServerState {
        let parameters = Parameters { t: 2, n: 3 };
        let (server_participant, server_coef) = Participant::new(&parameters, 0);
        ServerState {
            server_participant,
            server_coef,
            parameters,
            servers: vec![],
            participants: vec![],
        }
    }

    pub fn new(
        server_participant: Participant,
        server_coef: Coefficients,

        parameters: Parameters,
        servers: Vec<String>,
        participants: Vec<(Participant, Coefficients)>,
    ) -> ServerState {
        ServerState {
            server_participant,
            server_coef,
            parameters,
            servers,
            participants,
        }
    }
}
