pub mod utils;
use crate::utils::eth_provider::get_zentra_tx_data;
use crate::utils::load0::upload_to_load0;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let data = get_zentra_tx_data("0x469548e90a48718d2f4aacf8d4d7b4e6c66563b83fdd6bd193927321d7816355").await.unwrap();
    let load_hash = upload_to_load0(data.to_string().as_bytes().to_vec(), "application/json").await.unwrap();
    println!("{:?} {:?}", data, load_hash);
}
