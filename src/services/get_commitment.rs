use crate::commitments_to_generate;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};
use frost_dalek::generate_commitment_share_lists;
use futures::lock::Mutex;
use rand::rngs::OsRng;
use timestamp_server::{ServerState, State};

/// Returns the commitment at index i.
///
/// # Preconditions
/// - The key generation has finished.
///
/// # Can request
/// - Anyone
#[get("/commitment/{commitment_index}")]
pub async fn get_commitment(
    state: Data<Mutex<ServerState>>,
    path: web::Path<usize>,
) -> impl Responder {
    // TODO: check the state

    if state.lock().await.state != State::Timestamping {
        // TODO: return an error
    }

    // moved to keygen 2 endpoint
    // TODO: refactor, rn we are allowing only 100 timestamps
    // let commitments_to_generate = 100;
    // let this_server_index = state.lock().await.this_server_index.clone();
    // if !state.lock().await.public_commitment_shares.is_some() {
    //     let (public_shares, secret_shares) = generate_commitment_share_lists(
    //         &mut OsRng,
    //         this_server_index as u32,
    //         commitments_to_generate,
    //     );
    //     state.lock().await.public_commitment_shares = Some(public_shares);
    //     state.lock().await.secret_commitment_shares = Some(secret_shares);
    // }

    let commitment_index: usize = path.into_inner();
    log::info!("sending commitment index {}", commitment_index);
    if commitment_index >= commitments_to_generate as usize {
        // TODO: return error
    }
    let to_share = state
        .lock()
        .await
        .public_commitment_shares
        .as_ref()
        .unwrap()
        .commitments[commitment_index]
        .clone();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(to_share)
}
