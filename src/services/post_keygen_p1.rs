use crate::keygen::to_phase_2;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{post, web};
use futures::lock::Mutex;
use timestamp_server::SecretShareMessage;
use timestamp_server::{ServerState, State};

#[post("/keygen_phase1")]
pub async fn post_keygen_p1(
    state: Data<Mutex<ServerState>>,
    request: web::Json<SecretShareMessage>,
) -> impl Responder {
    if state.lock().await.state != State::Phase1 {
        return HttpResponse::Forbidden();
    }

    // TODO: check the key has not been combined already
    let share: SecretShareMessage = request.0;
    log::info!("received: {:?}", share);
    state.lock().await.secretShares[share.from as usize] = Some(share.val);

    if state.lock().await.have_all_shares() {
        state.lock().await.state = State::Phase2;
        log::info!("Into phase 2!!!!!!!!");
        actix_rt::spawn(async {
            to_phase_2(state).await;
        });
    }

    // TODO: update the state
    HttpResponse::Ok()
}
