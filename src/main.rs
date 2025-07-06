pub mod utils;
use axum::Router;
use axum::routing::{get, post};
use crate::utils::eth_provider::get_zentra_tx_data;
use crate::utils::load0::upload_to_load0;
use crate::utils::methods::route_get;

use shuttle_runtime::{SecretStore, Secrets};

// #[tokio::main]
// async fn main() {
//     println!("Hello, world!");
//     let data = get_zentra_tx_data("0x469548e90a48718d2f4aacf8d4d7b4e6c66563b83fdd6bd193927321d7816355").await.unwrap();
//     let load_hash = upload_to_load0(data.clone().1, "application/json").await.unwrap();
//     println!("{:?} {:?}", data, load_hash);
// }

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
        .route("/", get(route_get));

    Ok(router.into())
}