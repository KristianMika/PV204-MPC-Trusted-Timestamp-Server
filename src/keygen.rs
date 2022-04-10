use crate::{build_address, PROTOCOL};
use actix_web::web::Data;
use futures::lock::Mutex;
use timestamp_server::{Event, ServerState};

/// Sends the server's participant struct to all other participants
///
/// # Arguments
/// * `state` - The server state
pub async fn send_this_participant(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    let mut unlocked_state = state.lock().await;

    let my_participant = &unlocked_state.participants[unlocked_state.this_server_index - 1].0;
    let my_participant = serde_json::to_string(my_participant).unwrap();

    // TODO: in paralel
    for server_index in 1..=unlocked_state.parameters.n as usize {
        if server_index == unlocked_state.this_server_index {
            // don't send my participant struct to myself
            continue;
        }

        if unlocked_state.confirmations[server_index - 1].is_some() {
            // The server already has this server's participant struct
            continue;
        }
        let server_address = unlocked_state.servers[server_index - 1].clone();
        log::info!("Sending my share to: {}", &server_address);
        let client = reqwest::Client::new();
        let res = client
            .post(build_address(PROTOCOL, &server_address, "keygen_phase1"))
            .json(&my_participant)
            .send()
            .await;
        match res {
            Ok(val) => {
                unlocked_state.confirmations[server_index - 1] = Some(Event::KeygenPhase1);
            }
            Err(err) => {
                log::error!("An error response occured: {}", err.to_string())
            }
        };
        unlocked_state.hack_val = 1;
        // TODO: check response
        // TODO: store the server has acknowledged the recipience of the share
    }
    log::info!("The participant struct has been successfully distributed");
    Ok(())
}
