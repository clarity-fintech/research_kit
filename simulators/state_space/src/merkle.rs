use crate::event_loop::SimEventResult;
use clrty_substrate::token_core::merkle::{combine, leaf_hash};
use sha2::{Digest, Sha256};

pub fn event_leaf_hash(result: &SimEventResult) -> [u8; 32] {
    let payload = serde_json::to_vec(result).unwrap_or_default();
    leaf_hash(&format!("sim:{}", result.event_id), &payload)
}

pub fn batch_merkle_root(results: &[SimEventResult]) -> [u8; 32] {
    if results.is_empty() {
        return clrty_substrate::token_core::merkle::empty_root();
    }
    let mut leaves: Vec<[u8; 32]> = results.iter().map(event_leaf_hash).collect();
    while leaves.len() > 1 {
        let mut next = Vec::new();
        for chunk in leaves.chunks(2) {
            if chunk.len() == 2 {
                next.push(combine(&chunk[0], &chunk[1]));
            } else {
                next.push(combine(&chunk[0], &chunk[0]));
            }
        }
        leaves = next;
    }
    leaves[0]
}

pub fn batch_merkle_root_hex(results: &[SimEventResult]) -> String {
    hex::encode(batch_merkle_root(results))
}

pub fn hash_ndjson_line(line: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(line.as_bytes());
    h.finalize().into()
}
