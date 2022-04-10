use frost_dalek::keygen::Coefficients;
use frost_dalek::Parameters;
use frost_dalek::Participant;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    /// a list of all server IPs includes this one
    pub servers: Vec<String>,
    /// the port of this server
    pub port: u16,
    pub t: u32,
    pub n: u32,
}

/// Creates a vector of `n` participants
///
/// # Arguments
///
/// * `params` - threshold parameters
pub fn create_participants(params: &Parameters) -> ParticipantVec {
    let mut participants = vec![];
    for server_index in 1..=params.n {
        participants.push(Participant::new(&params, server_index));
    }
    participants
}

pub type ParticipantVec = Vec<(Participant, Coefficients)>;
/// Holds the state of the server, configuration, keys, etc.
pub struct ServerState {
    /// Configuration of the context, namely t-n parameter
    pub parameters: Parameters,
    /// Addresses of other servers in the current context
    pub servers: Vec<String>,
    /// Other participants
    pub participants: ParticipantVec,
    pub this_server_index: usize,
}

impl ServerState {
    pub fn default() -> ServerState {
        let parameters = Parameters { t: 2, n: 3 };
        ServerState {
            parameters,
            servers: vec![],
            participants: create_participants(&parameters),
            this_server_index: 1,
        }
    }

    pub fn new(
        parameters: Parameters,
        servers: Vec<String>,
        this_server_index: usize,
    ) -> ServerState {
        ServerState {
            parameters,
            servers,
            participants: create_participants(&parameters),
            this_server_index,
        }
    }

    /// Returns a vector of other participant structs
    /// Used in the first keygen phase
    pub fn get_other_participants(&self) -> Vec<Participant> {
        self.participants
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != self.this_server_index)
            .map(
                |(_, (part, _)): (usize, &(Participant, Coefficients))| -> Participant {
                    part.clone()
                },
            )
            .collect()
    }
}
