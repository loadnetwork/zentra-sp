use alloy::providers::{ProviderBuilder, Provider};
use alloy::primitives::{address, Bytes, TxHash};
use anyhow::{anyhow, Error};
use std::str::FromStr;

use crate::utils::constants::BASE_SEPOLIA_RPC;


pub async fn get_transaction_data(txid: &str) -> Result<alloy::primitives::Bytes, Error> {
    let txid = TxHash::from_str(txid)?;
    let provider = ProviderBuilder::new().connect_http(BASE_SEPOLIA_RPC.parse()?);
    let tx = provider.get_transaction_by_hash(txid.into()).await?;
    let tx_bytes: Option<alloy::consensus::EthereumTxEnvelope<alloy::consensus::TxEip4844Variant>> = tx.and_then(|txb| txb.into_inner().try_into().ok());
    let tx_envelope = tx_bytes.ok_or("").map_err(|err| anyhow!(err))?;
    let tx_eip1559 = tx_envelope.as_eip1559().ok_or("").map_err(|err| anyhow!(err))?;
    let res = tx_eip1559.clone().into_parts().0.input;
    Ok(res)
}