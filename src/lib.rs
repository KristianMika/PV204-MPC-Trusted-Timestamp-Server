use frost_dalek::keygen::Coefficients;
use frost_dalek::keygen::RoundTwo;
use frost_dalek::keygen::SecretShare;
use frost_dalek::DistributedKeyGeneration;
use frost_dalek::{GroupKey, IndividualSecretKey, Parameters, Participant};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Config {
    /// a list of all server IPs includes this one
    pub servers: Vec<String>,
    /// the port of this server
    pub port: u16,
    pub t: u32,
    pub n: u32,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Event {
    /// The server has received the public key
    Init,
    KeygenPhase1,
    KeygenPhase2,
}

#[derive(PartialEq, PartialOrd, Clone)]
pub enum State {
    Reset,
    Init,
    Phase1,
    Phase2,
    Timestamping,
}

pub type ParticipantVec = Vec<Option<Participant>>;
pub type EventVec = Vec<Option<Event>>;
pub type RoundOneStruct =
    Option<frost_dalek::DistributedKeyGeneration<frost_dalek::keygen::RoundOne>>;
/// Holds the state of the server, configuration, keys, etc.
pub struct ServerState {
    /// Configuration of the context, namely t-n parameter
    pub parameters: Parameters,
    /// Addresses of other servers in the current context
    pub servers: Vec<String>,
    /// Other participants
    pub participants: ParticipantVec,
    pub this_server_index: usize,
    pub confirmations: EventVec,
    pub coefficients: Coefficients,
    pub state: State,
    // TODO: refactor
    pub pubkey_sent: bool,
    pub round_1_sent: bool,
    pub round_one_struct: RoundOneStruct,
    pub secretShares: Vec<Option<SecretShare>>,
    pub server_state: Option<DistributedKeyGeneration<RoundTwo>>,
    pub group_key: Option<GroupKey>,
    pub secret_key: Option<IndividualSecretKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretShareMessage {
    pub from: usize,
    pub val: SecretShare,
}

impl ServerState {
    pub fn default() -> ServerState {
        let parameters = Parameters { t: 2, n: 3 };
        ServerState::new(parameters, vec![], 1)
    }

    pub fn new(
        parameters: Parameters,
        servers: Vec<String>,
        this_server_index: usize,
    ) -> ServerState {
        let mut participants = vec![None; parameters.n as usize];
        let (part, coefficients) = Participant::new(&parameters, this_server_index as u32);
        participants[this_server_index] = Some(part);
        ServerState {
            parameters,
            servers,
            participants,
            this_server_index,
            confirmations: vec![None; parameters.n as usize],
            coefficients,
            state: State::Init,
            pubkey_sent: false,
            round_1_sent: false,
            round_one_struct: None,
            secretShares: vec![None; parameters.n as usize],
            server_state: None,
            group_key: None,
            secret_key: None,
        }
    }

    /// Returns a vector of other participant structs
    /// Used in the first keygen phase
    pub fn get_other_participants(&self) -> Vec<Participant> {
        self.participants
            .iter()
            .enumerate()
            .filter(|&(i, val)| i != self.this_server_index && val.is_some())
            .map(|(_, val): (usize, &Option<Participant>)| -> Participant {
                match val {
                    Some(part) => part.clone(),
                    None => panic!("Unreachable code"),
                }
            })
            .collect()
    }

    /// Returns true if the server contains pubkeys of all participants
    pub fn have_all_pubkeys(&self) -> bool {
        self.participants
            .iter()
            .fold(true, |agg, val: &Option<Participant>| -> bool {
                agg && val.is_some()
            })
    }

    /// returns a copy ot this server's participant struct
    pub fn get_this_participant(&self) -> Participant {
        match &self.participants[self.this_server_index] {
            Some(val) => val.clone(),
            None => panic!("This participant not set!"),
        }
    }

    // TODO: a duplicate
    pub fn have_all_shares(&self) -> bool {
        self.secretShares
            .iter()
            .enumerate()
            .filter(|(index, _): &(usize, &Option<SecretShare>)| *index != self.this_server_index)
            .fold(
                true,
                |agg, (_, val): (usize, &Option<SecretShare>)| -> bool { agg && val.is_some() },
            )
    }

    pub fn get_secret_shares(&self) -> Vec<SecretShare> {
        self.secretShares
            .clone()
            .into_iter()
            .filter_map(|e| e)
            .collect()
    }
}
