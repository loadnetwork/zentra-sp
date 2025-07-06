pub mod utils;
use axum::Router;
use axum::routing::{get, post};
use crate::utils::methods::{route_get, upload};

use shuttle_runtime::{SecretStore, Secrets};

#[shuttle_runtime::main]
async fn main(
    #[Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    unsafe {
    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });
    }

    let router = Router::new()
        .route("/", get(route_get))
        .route("/process/{txid}", post(upload));

    Ok(router.into())
}