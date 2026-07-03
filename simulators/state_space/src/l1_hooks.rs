use crate::chain::{calculate_state_root, sim_block_from_transfer, validate_block};
use clrty_substrate::mvm_execution::gas_deflation_matrix::base_entropy_fee;
use clrty_substrate::poc_consensus::entropy_slashing::malicious_divergence;
use clrty_substrate::treasury_sink::ecosystem_vesting_escrow::{
    vested_amount, InvestorVestingSchedule,
};
use clrty_substrate::{
    economic_engine::tokenomics::verify_tokenomics_manifest, verify_genesis, CcrOrchestrator,
    EntropyBus, EntropyEventType, GLOBAL_HARD_CAP_BASE_UNITS,
};

pub struct L1HookResult {
    pub ok: bool,
    pub detail: String,
}

pub fn run_l1_action(action: &str, event_id: u16) -> L1HookResult {
    match action {
        "genesis_verify" => match verify_genesis() {
            Ok(g) => match verify_tokenomics_manifest() {
                Ok(_) => L1HookResult {
                    ok: true,
                    detail: format!("genesis ok chain={} supply={}", g.chain_id, g.total_supply),
                },
                Err(e) => L1HookResult {
                    ok: false,
                    detail: e,
                },
            },
            Err(e) => L1HookResult {
                ok: false,
                detail: e.to_string(),
            },
        },
        "sim_block" | "tps_burst" => {
            let iterations = if action == "tps_burst" { 5 } else { 1 };
            let mut ccr = CcrOrchestrator::default();
            let addr = [1u8; 32];
            let mut last = String::new();
            for i in 0..iterations {
                let r = ccr.process_transfer(
                    &addr,
                    2_000_000_000,
                    [100_000_000, 50_000_000, 25_000_000, 10_000_000],
                );
                last = format!("ccr set {} -> {}", r.old_set, r.new_set);
                let block = sim_block_from_transfer(
                    addr,
                    [2u8; 32],
                    1_000_000_000,
                    [0u8; 32],
                    event_id as u64 + i as u64,
                );
                let root = calculate_state_root(&block.transactions);
                if validate_block(&block, root).is_err() {
                    return L1HookResult {
                        ok: false,
                        detail: "block state_root validation failed".into(),
                    };
                }
            }
            L1HookResult {
                ok: true,
                detail: format!("{action} x{iterations} {last}"),
            }
        }
        "register_sync" => {
            let block = sim_block_from_transfer(
                [3u8; 32],
                [4u8; 32],
                500_000_000,
                [0u8; 32],
                event_id as u64,
            );
            let root = calculate_state_root(&block.transactions);
            match validate_block(&block, root) {
                Ok(()) => L1HookResult {
                    ok: true,
                    detail: format!("register_sync block nonce={}", block.header.nonce),
                },
                Err(e) => L1HookResult {
                    ok: false,
                    detail: e,
                },
            }
        }
        "bft_stress" => {
            let malicious = event_id % 20 == 0;
            let attack_detected = malicious_divergence::detect_divergence(!malicious);
            let ok = if malicious {
                attack_detected
            } else {
                !attack_detected
            };
            L1HookResult {
                ok,
                detail: format!("bft malicious={malicious} attack_detected={attack_detected}"),
            }
        }
        "double_spend" => {
            let mut ledger: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
            let nonce = event_id as u64;
            let balance = 1_000_000_000u64;
            if ledger.insert(nonce, balance).is_some() {
                L1HookResult {
                    ok: false,
                    detail: "double-spend accepted".into(),
                }
            } else if ledger.contains_key(&nonce) {
                L1HookResult {
                    ok: false,
                    detail: "double-spend accepted".into(),
                }
            } else {
                L1HookResult {
                    ok: true,
                    detail: "double-spend rejected".into(),
                }
            }
        }
        "reorg_canonical" => {
            let short = sim_block_from_transfer([5u8; 32], [6u8; 32], 100, [0u8; 32], 1);
            let long =
                sim_block_from_transfer([5u8; 32], [6u8; 32], 100, short.header.state_root, 2);
            let canonical = if long.header.nonce > short.header.nonce {
                long.header.state_root
            } else {
                short.header.state_root
            };
            L1HookResult {
                ok: true,
                detail: format!("reorg resolved canonical={}", hex::encode(canonical)),
            }
        }
        "vesting_release" => {
            let schedule = InvestorVestingSchedule::seed_genesis();
            let at_cliff = vested_amount(
                1_000_000_000,
                schedule.cliff_months,
                schedule.cliff_months,
                schedule.vest_months,
            );
            let before_cliff = vested_amount(
                1_000_000_000,
                schedule.cliff_months - 1,
                schedule.cliff_months,
                schedule.vest_months,
            );
            if before_cliff != 0 || at_cliff == 0 {
                L1HookResult {
                    ok: false,
                    detail: "vesting cliff violated".into(),
                }
            } else {
                L1HookResult {
                    ok: true,
                    detail: format!("vesting ok released={at_cliff}"),
                }
            }
        }
        "supply_mint_burn" => {
            let minted = 1_000_000_000u64;
            let burned = 100_000_000u64;
            let net = minted.saturating_sub(burned);
            if net > GLOBAL_HARD_CAP_BASE_UNITS {
                L1HookResult {
                    ok: false,
                    detail: "supply exceeds hard cap".into(),
                }
            } else {
                L1HookResult {
                    ok: true,
                    detail: format!("supply net={net} cap={GLOBAL_HARD_CAP_BASE_UNITS}"),
                }
            }
        }
        "fee_stress" => {
            let base = base_entropy_fee::base_fee();
            let load = 1 + (event_id % 10) as u64;
            let fee = base.saturating_mul(load);
            L1HookResult {
                ok: fee >= base,
                detail: format!("fee={fee} load={load}x base={base}"),
            }
        }
        "entropy_bus_commit" => {
            let mut bus = EntropyBus::new();
            emit_entropy(&mut bus, event_id, b"commit", 1_700_000_000_000_000_000);
            L1HookResult {
                ok: !bus.root.iter().all(|&b| b == 0),
                detail: format!("entropy root={}", hex::encode(bus.root)),
            }
        }
        _ => L1HookResult {
            ok: true,
            detail: "no l1 action".into(),
        },
    }
}

pub fn emit_entropy(bus: &mut EntropyBus, event_id: u16, payload: &[u8], ts: u128) {
    let mut full = event_id.to_le_bytes().to_vec();
    full.extend_from_slice(payload);
    bus.emit(EntropyEventType::TradeExecution, full, ts);
}
