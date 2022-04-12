use crate::commitments_to_generate;
use crate::keygen::share_groupkey;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use frost_dalek::generate_commitment_share_lists;
use frost_dalek::GroupKey;
use futures::lock::Mutex;
use rand::rngs::OsRng;

use timestamp_server::{ServerState, State};

#[post("/keygen_phase2")]
pub async fn post_keygen_p2(
    state: Data<Mutex<ServerState>>,
    request: web::Json<GroupKey>,
) -> impl Responder {
    if state.lock().await.state != State::Phase2 {
        return HttpResponse::Forbidden();
    }

    let received_groupkey: GroupKey = request.0;

    if received_groupkey != state.lock().await.group_key.unwrap() {
        log::error!("Received an invalid hroupkey");
    } else {
        log::info!("Received a valid groupkey");
    }

    if state.lock().await.group_key.is_some() {
        // TODO: check I've confirmed all publey
        state.lock().await.state = State::Timestamping;

        log::info!("Sharing the group key");

        if !state.lock().await.public_commitment_shares.is_some() {
            let this_server_index = state.lock().await.this_server_index.clone();
            let (public_shares, secret_shares) = generate_commitment_share_lists(
                &mut OsRng,
                this_server_index as u32,
                commitments_to_generate as usize,
            );
            state.lock().await.public_commitment_shares = Some(public_shares);
            state.lock().await.secret_commitment_shares = Some(secret_shares);
        }

        actix_rt::spawn(async {
            share_groupkey(state).await;
        });
    }

    // TODO: send the shares to other participants

    // TODO: update the state
    HttpResponse::Ok()
}
