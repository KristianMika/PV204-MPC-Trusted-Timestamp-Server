use crate::keygen::compute_distributed_struct;
use crate::keygen::send_this_participant;
use crate::keygen::share_phase1;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use frost_dalek::Participant;
use futures::lock::Mutex;
use timestamp_server::ServerState;
use timestamp_server::State;

#[post("/init")]
pub async fn post_init(
    state: Data<Mutex<ServerState>>,
    request: web::Json<String>,
) -> impl Responder {
    if state.lock().await.state != State::Init {
        return HttpResponse::Forbidden();
    }

    let participant: Participant = match serde_json::from_str(&request.0[..]) {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest(),
    };

    match participant
        .proof_of_secret_key
        .verify(&participant.index, &participant.public_key().unwrap())
    {
        Ok(_) => {
            log::info!(
                "Successfully verified shares from server with index {}.",
                &participant.index
            )
        }
        Err(_) => {
            log::error!(
                "Couldn't verify shares from server with index {}.",
                &participant.index
            );
            return HttpResponse::Unauthorized();
        }
    };

    // Store the participant
    state.lock().await.participants[participant.index as usize] = Some(participant.clone());

    let this_participant = state.lock().await.get_this_participant();
    let this_phase = state.lock().await.state.clone();
    if this_phase != State::Phase1 && state.lock().await.have_all_pubkeys() {
        state.lock().await.state = State::Phase1;
        log::info!("All pubkeys have been received, going to phase 1");
    } else {
        log::info!("Index 1: {}", state.lock().await.participants[0].is_some());
        log::info!("Index 2: {}", state.lock().await.participants[1].is_some());
        log::info!("Index 3: {}", state.lock().await.participants[2].is_some());
    }
    let pubkey_sent = state.lock().await.pubkey_sent.clone();

    let state_clone = state.clone();
    if !pubkey_sent {
        log::info!("About to recursivelly share pubkey");
        // arbiter.spawn(async {
        //     send_this_participant(state).await.unwrap();
        // });
        actix_rt::spawn(async {
            log::info!("Pubkey to be shared recursivelly using async");
            send_this_participant(state).await.unwrap();
            log::info!("Pubkey shared done using async");
        });
    } else {
        log::info!("pubkey has been sent, not calling recurs");
    }
    if state_clone.lock().await.state.clone() == State::Phase1 {
        log::info!("About to compute the distributed secret");
        actix_rt::spawn(async {
            compute_distributed_struct(state_clone.clone()).await;
            if !state_clone.lock().await.round_1_sent {
                share_phase1(state_clone).await;
            }
        });
    }

    // TODO: send the shares to other participants

    // TODO: update the state
    HttpResponse::Ok()
}
