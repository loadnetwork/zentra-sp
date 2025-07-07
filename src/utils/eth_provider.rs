use alloy::providers::{ProviderBuilder, Provider};
use alloy::primitives::TxHash;
use anyhow::{anyhow, Error};
use serde_json::Value;
use std::str::FromStr;

use crate::utils::constants::{BASE_SEPOLIA_RPC, SEQUENCER_ADDRESSES};

pub async fn get_transaction_data(txid: &str) -> Result<alloy::primitives::Bytes, Error> {
    let txid = TxHash::from_str(txid)?;
    let provider = ProviderBuilder::new().connect_http(BASE_SEPOLIA_RPC.parse()?);
    let tx = provider.get_transaction_by_hash(txid.into()).await?;
    let tx_bytes: Option<alloy::consensus::EthereumTxEnvelope<alloy::consensus::TxEip4844Variant>> = tx.and_then(|txb| txb.into_inner().try_into().ok());
    let tx_envelope = tx_bytes.ok_or("error accessing tx envelope").map_err(|err| anyhow!(err))?;
    let tx_eip1559 = tx_envelope.as_eip1559().ok_or("error converting envelop to eip1559").map_err(|err| anyhow!(err))?;
    let sender = tx_eip1559.recover_signer()?.to_string();
    assert!(SEQUENCER_ADDRESSES.contains(&sender.as_str()));
    let res = tx_eip1559.clone().into_parts().0.input;
    Ok(res)
}

pub async fn get_zentra_tx_data(txid: &str) -> Result<(Value, Vec<u8>), Error> {
    let input = get_transaction_data(txid).await?;
    let hex_input = String::from_utf8(input.into())?;
    let json_input: Value = serde_json::from_str(&hex_input)?;
    let raw_input = hex_input.as_bytes().to_vec();
    Ok((json_input, raw_input))
}