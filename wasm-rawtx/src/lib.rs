use rawtx_rs::bitcoin;
use rawtx_rs::bitcoin::hex::FromHex;
use rawtx_rs::tx::TxInfo;
use serde::Serialize;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Node {
    id: String,
    version: i32,
}

#[derive(Serialize)]
pub struct Link {
    source: String,
    target: String,
    intype: String,
}

#[derive(Serialize)]
pub struct GraphData {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn graphdata_from_hexblock(hex: &str) -> JsValue {
    let block = deserialize_block(hex);
    let mut nodes = Vec::with_capacity(block.txdata.len());
    let mut links = Vec::new();
    let known_txids: HashSet<bitcoin::Txid> =
        block.txdata.iter().map(|tx| tx.compute_txid()).collect();
    for tx in block.txdata.iter() {
        if tx.is_coinbase() {
            continue;
        }
        let tx_info = TxInfo::new(&tx).unwrap();
        nodes.push(Node {
            id: tx.compute_txid().to_string(),
            version: tx_info.version,
        });
        for (input, info) in tx.input.iter().zip(tx_info.input_infos) {
            if known_txids.contains(&input.previous_output.txid) {
                links.push(Link {
                    target: input.previous_output.txid.to_string(),
                    source: tx.compute_txid().to_string(),
                    intype: info.in_type.to_string(),
                });
            }
        }
    }
    serde_wasm_bindgen::to_value(&GraphData { nodes, links }).unwrap()
}

#[wasm_bindgen]
pub fn graphdata_from_hextxlist(hextxns: Vec<String>) -> JsValue {
    let mut nodes = Vec::with_capacity(hextxns.len());
    let txns: Vec<bitcoin::Transaction> = hextxns.iter().map(|hex| deserialize_tx(hex)).collect();
    let mut links = Vec::new();
    let known_txids: HashSet<bitcoin::Txid> = txns.iter().map(|tx| tx.compute_txid()).collect();
    for tx in txns.iter() {
        if tx.is_coinbase() {
            continue;
        }
        let tx_info = TxInfo::new(&tx).unwrap();
        nodes.push(Node {
            id: tx.compute_txid().to_string(),
            version: tx_info.version,
        });
        for (input, info) in tx.input.iter().zip(tx_info.input_infos) {
            if known_txids.contains(&input.previous_output.txid) {
                links.push(Link {
                    target: input.previous_output.txid.to_string(),
                    source: tx.compute_txid().to_string(),
                    intype: info.in_type.to_string(),
                });
            }
        }
    }
    serde_wasm_bindgen::to_value(&GraphData { nodes, links }).unwrap()
}

fn deserialize_tx(hex: &str) -> bitcoin::Transaction {
    let tx_bytes = Vec::from_hex(hex).unwrap();
    bitcoin::consensus::deserialize(&tx_bytes).unwrap()
}

fn deserialize_block(hex: &str) -> bitcoin::Block {
    let block_bytes = Vec::from_hex(hex).unwrap();
    bitcoin::consensus::deserialize(&block_bytes).unwrap()
}
