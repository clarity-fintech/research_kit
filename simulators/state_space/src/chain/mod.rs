//! Deterministic block structure for clrty-1 simulation (Moniversion state sync).

use clrty_substrate::kernel_core::entropy_bus::hash_state;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount_nano: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub nonce: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub merkle_root: Hash,
    pub timestamp: u64,
}

pub fn calculate_state_root(transactions: &[Transaction]) -> Hash {
    let mut h = Sha256::new();
    for tx in transactions {
        h.update(tx.from);
        h.update(tx.to);
        h.update(tx.amount_nano.to_le_bytes());
        h.update(tx.nonce.to_le_bytes());
    }
    let out = h.finalize();
    let mut root = [0u8; 32];
    root.copy_from_slice(&out);
    root
}

pub fn block_merkle_root(transactions: &[Transaction]) -> Hash {
    if transactions.is_empty() {
        return [0u8; 32];
    }
    let mut leaves: Vec<Hash> = transactions
        .iter()
        .map(|tx| {
            let payload = serde_json::to_vec(tx).unwrap_or_default();
            hash_state([0u8; 32], &payload)
        })
        .collect();
    while leaves.len() > 1 {
        let mut next = Vec::new();
        for chunk in leaves.chunks(2) {
            let combined = if chunk.len() == 2 {
                hash_state(chunk[0], chunk[1].as_ref())
            } else {
                hash_state(chunk[0], chunk[0].as_ref())
            };
            next.push(combined);
        }
        leaves = next;
    }
    leaves[0]
}

/// Enforces absolute state determinism — raises on computational entropy.
pub fn validate_block(block: &Block, expected_state_root: Hash) -> Result<(), String> {
    let actual = calculate_state_root(&block.transactions);
    if actual != expected_state_root {
        return Err(format!(
            "Computational Entropy Detected: state_root mismatch expected={} actual={}",
            hex::encode(expected_state_root),
            hex::encode(actual)
        ));
    }
    if block.header.state_root != actual {
        return Err("Block header state_root does not match transaction set".into());
    }
    let merkle = block_merkle_root(&block.transactions);
    if merkle != block.merkle_root {
        return Err("Block merkle_root does not match transactions".into());
    }
    Ok(())
}

pub fn sim_block_from_transfer(
    from: [u8; 32],
    to: [u8; 32],
    amount: u64,
    parent: Hash,
    nonce: u64,
) -> Block {
    let tx = Transaction {
        from,
        to,
        amount_nano: amount,
        nonce,
    };
    let txs = vec![tx];
    let state_root = calculate_state_root(&txs);
    let merkle_root = block_merkle_root(&txs);
    Block {
        header: BlockHeader {
            parent_hash: parent,
            state_root,
            nonce,
            timestamp: 1_700_000_000 + nonce,
        },
        transactions: txs,
        merkle_root,
        timestamp: 1_700_000_000 + nonce,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_block_passes_deterministic_state() {
        let block = sim_block_from_transfer([1u8; 32], [2u8; 32], 1_000_000_000, [0u8; 32], 1);
        let expected = calculate_state_root(&block.transactions);
        assert!(validate_block(&block, expected).is_ok());
    }
}
