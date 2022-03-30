use rust_fsm::StateMachine;
use frost_dalek::keygen::Coefficients;
use frost_dalek::Parameters;
use frost_dalek::Participant;
use rust_fsm::*;

// The automata representing state transitions
state_machine! {
    derive(Debug)
    StateAutomata(Init)
    Init => {
        Setup => Setup,
        Reset => Init
    },
    Setup => {
        StoreShareRound1 => KeyGenPhase1,
        Reset => Init
    },
    KeyGenPhase1 => {
        Phase1Done => KeyGenPhase2,
        Reset => Init
    },
    KeyGenPhase2 => {
        Phase2Done => Timestamp,
        Reset => Init
    }
}


/// Holds the state of the server, configuration, keys, etc.
pub(crate) struct ServerState {
    /// A unique index of the server with respect to the context
    pub server_participant: Participant,
    pub server_coef: Coefficients,
    /// Configuration of the context, namely t-n parameter
    pub parameters: Parameters,
    /// Other servers in the current context
    pub servers: Vec<String>,
    /// Other participants
    pub participants: Vec<(Participant, Coefficients)>,
    /// State automata
    pub state: StateMachine<StateAutomata>
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
            state: StateMachine::new()
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
            state:StateMachine::new()
        }
    }
}

/// Uniquelly identifies another server
pub struct ServerAddress {
    /// Server IP
    ip: String,
    /// Server port
    port: u16,
}
