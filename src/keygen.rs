use crate::build_address;
use crate::ServerState;
use crate::PROTOCOL;
use actix_web::web::Data;
use std::sync::Mutex;

/// Sends the server's participant struct to all other participants
///
/// # Arguments
/// * `state` - The server state
pub async fn send_this_participant(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    let unlocked_state = state.lock().unwrap();

    let my_participant = &unlocked_state.participants[unlocked_state.this_server_index - 1].0;
    let my_participant = serde_json::to_string(my_participant).unwrap();

    for server_index in 1..=unlocked_state.parameters.n as usize {
        if server_index == unlocked_state.this_server_index {
            // don't send my participant struct to myself
            continue;
        }
        let server_address = unlocked_state.servers[server_index - 1].clone();
        let client = reqwest::Client::new();
        let res = client
            .post(build_address(PROTOCOL, &server_address, "keygen_phase1"))
            .json(&my_participant)
            .send()
            .await;
        match res {
            Ok(val) => {}
            Err(err) => {
                log::error!("An error response occured: {}", err.to_string())
            }
        };
        // TODO: check response
        // TODO: store the server has acknowledged the recipience of the share
    }
    log::info!("The participant struct has been successfully distributed");
    Ok(())
}
