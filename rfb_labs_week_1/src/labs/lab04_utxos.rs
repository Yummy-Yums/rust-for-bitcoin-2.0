//! Lab 04 — inspect UTXOs and outpoints.

use crate::model::{OutPoint, Utxo};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::LabResult;

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    // TODO: call listunspent in wallet context and decode every returned UTXO.
    // todo!("Lab 04: list unspent outputs")
    let result = client.call(Some(wallet_name), "listunspent", &[]);

    match result {
        Ok(res) => {
            let parsed = parse_cli_value(&res)?;
            let utxos = parsed
                .as_array()
                .unwrap()
                .iter()
                .map(|x| serde_json::from_value::<Utxo>(x.clone()).unwrap())
                .collect::<Vec<Utxo>>();

            Ok(utxos)
        }
        Err(err) => Err(err),
    }
}

/// Select one spendable UTXO, preferring the one with the most confirmations.
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    // TODO: filter by spendable and select deterministically.
    // todo!("Lab 04: select a spendable UTXO")
    utxos
        .into_iter()
        .filter(|utxo| utxo.spendable)
        .max_by_key(|utxo| utxo.confirmations)
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    // TODO: return the matching outpoint.
    // todo!("Lab 04: construct an outpoint")
    OutPoint {
        vout: utxo.vout,
        txid: utxo.txid.clone(),
    }
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    // TODO: ignore non-spendable entries and sum BTC amounts.
    // todo!("Lab 04: calculate spendable wallet balance")

    utxos
        .into_iter()
        .filter(|utxo| utxo.spendable)
        .fold(0.0, |acc, utxo| acc + utxo.amount)
}
