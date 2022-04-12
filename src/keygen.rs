use crate::{build_address, PROTOCOL};
use actix_web::web::Data;
use frost_dalek::DistributedKeyGeneration;
use futures::lock::Mutex;
use timestamp_server::SecretShareMessage;
use timestamp_server::{Event, ServerState, State};

/// Sends the server's participant struct to all other participants
///
/// # Arguments
/// * `state` - The server state
pub async fn send_this_participant(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    log::info!("in send_this_participant");
    let this_server_index = state.lock().await.this_server_index.clone();
    let n = state.lock().await.parameters.n.clone();

    let this_participant = state.lock().await.get_this_participant();
    let this_participant = serde_json::to_string(&this_participant).unwrap();

    // TODO: in paralel
    log::info!("Entering loop, n:{}", n);

    for server_index in 0..n as usize {
        log::info!("Server index: {}", server_index);
        if server_index == this_server_index {
            // don't send my participant struct to myself
            log::info!("Not sending the share to myself");
            continue;
        }

        if state.lock().await.confirmations[server_index].is_some() {
            log::info!("The server already has mu pubkey");
            // The server already has this server's participant struct
            continue;
        }
        state.lock().await.confirmations[server_index] = Some(Event::Init);
        let server_address = state.lock().await.servers[server_index].clone();
        log::info!("Sending my share to: {}", &server_address);
        let client = reqwest::Client::new();
        let res = client
            .post(build_address(PROTOCOL, &server_address, "init"))
            .json(&this_participant)
            .send()
            .await;
        match res {
            Ok(val) => {
                log::info!("Received response: {}", val.status());
            }
            Err(err) => {
                log::error!("An error response occured: {}", err.to_string());
                state.lock().await.confirmations[server_index] = None;
            }
        };

        // TODO: check response
    }
    state.lock().await.pubkey_sent = true;
    log::info!("The participant struct has been successfully distributed");
    Ok(())
}

pub async fn compute_distributed_struct(state: Data<Mutex<ServerState>>) {
    // if state.lock().await.state == State::Phase1 {
    let params = state.lock().await.parameters.clone();
    let coefficients = state.lock().await.coefficients.clone();
    let this_participant = state.lock().await.get_this_participant();
    let this_key_state = DistributedKeyGeneration::<_>::new(
        &params,
        &this_participant.index,
        &coefficients,
        &mut state.lock().await.get_other_participants(),
    )
    .unwrap();

    state.lock().await.round_one_struct = Some(this_key_state);

    // TODO: share to others

    // to phase 1
    state.lock().await.state = State::Phase1;
    // TODO: check how we can serialize it
    log::info!("Keygen phase 1 done");
    // TODO: trigger key generation phase 2
}

/// TODO: refactor
pub async fn share_phase1(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    log::info!("in share_phase_1");
    let this_server_index = state.lock().await.this_server_index.clone();

    // TODO: in paralel
    let my_struct = state.lock().await.round_one_struct.clone().unwrap();

    let shares = my_struct.their_secret_shares().unwrap();
    for share in shares {
        if Some(Event::KeygenPhase1)
            == state.lock().await.confirmations[share.index as usize].clone()
        {
            log::info!("The server already has mu dist struct");
            // The server already has this server's participant struct
            continue;
        }

        state.lock().await.confirmations[share.index as usize] = Some(Event::KeygenPhase1);
        let server_address = state.lock().await.servers[share.index as usize].clone();
        log::info!("Sending my share to: {}", &server_address);

        let client = reqwest::Client::new();
        // TODO
        // if !state.lock().await.round_one_struct.is_some() {
        //     return Ok(());
        // }

        let attemps = 5;
        for attempt in 1..=attemps {
            let res = client
                .post(build_address(PROTOCOL, &server_address, "keygen_phase1"))
                .json(&SecretShareMessage {
                    from: this_server_index,
                    val: share.clone(),
                })
                .send()
                .await;
            match res {
                Ok(val) => {
                    log::info!("Received response: {}", val.status());
                    if val.status() == 200 {
                        break;
                    }
                    // TODO: better synchronization
                    use std::{thread, time};

                    let ten_millis = time::Duration::from_millis(100);

                    thread::sleep(ten_millis);
                }
                Err(err) => {
                    log::error!("An error response occured: {}", err.to_string());
                    state.lock().await.confirmations[share.index as usize] = Some(Event::Init);
                }
            };

            if attempt == attemps {
                state.lock().await.confirmations[share.index as usize] = Some(Event::Init);
            }
        }
    }

    // TODO: check response

    // TODO: what if all attempts were wasted
    state.lock().await.round_1_sent = true;

    log::info!("The participant struct has been successfully distributed");
    Ok(())
}

/// TODO: refactor
pub async fn to_phase_2(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    let round_1_struct = state.lock().await.round_one_struct.clone().unwrap();
    let secret_shares = state.lock().await.get_secret_shares();

    let round_2_struct = round_1_struct.to_round_two(secret_shares).unwrap();
    state.lock().await.server_state = Some(round_2_struct.clone());
    let this_participant = state.lock().await.get_this_participant();

    let (group_key, secret_key) = round_2_struct
        .finish(this_participant.public_key().expect("Key access error"))
        .expect("Suyash pooped deriving his group and secret keys");

    state.lock().await.group_key = Some(group_key);
    state.lock().await.secret_key = Some(secret_key);

    actix_rt::spawn(async {
        share_groupkey(state).await;
    });

    Ok(())
}

pub async fn share_groupkey(state: Data<Mutex<ServerState>>) -> Result<(), ()> {
    let n = state.lock().await.parameters.n.clone();
    let this_server_index = state.lock().await.this_server_index.clone();
    let group_key = state.lock().await.group_key.clone();
    // TODO: multiple attempts
    for server_index in 0..n as usize {
        if server_index == this_server_index {
            // don't send my participant struct to myself
            log::info!("Not sending the share to myself");
            continue;
        }

        if state.lock().await.confirmations[server_index] == Some(Event::KeygenPhase2) {
            log::info!("The server already has mu pubkey");
            // The server already has this server's participant struct
            continue;
        }
        state.lock().await.confirmations[server_index] = Some(Event::KeygenPhase2);

        let server_address = state.lock().await.servers[server_index].clone();
        log::info!("Sending the groupkey to: {}", &server_address);
        let attemps = 5;
        for attempt in 1..=attemps {
            let client = reqwest::Client::new();
            let res = client
                .post(build_address(PROTOCOL, &server_address, "keygen_phase2"))
                .json(&group_key)
                .send()
                .await;
            match res {
                Ok(val) => {
                    if val.status() == 200 {
                        break;
                    }
                    // TODO: better synchronization
                    use std::{thread, time};

                    let ten_millis = time::Duration::from_millis(100);

                    thread::sleep(ten_millis);
                }
                Err(err) => {
                    log::error!("An error response occured: {}", err.to_string());
                    state.lock().await.confirmations[server_index] = Some(Event::KeygenPhase1);
                }
            };
        }
    }
    Ok(())
}
