use crate::keygen::share_groupkey;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use frost_dalek::GroupKey;
use futures::lock::Mutex;

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
        log::info!("Sharing the group key");
        actix_rt::spawn(async {
            share_groupkey(state).await;
        });
    }
    // TODO: send the shares to other participants

    // TODO: update the state
    HttpResponse::Ok()
}
