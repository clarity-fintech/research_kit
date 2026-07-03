use crate::abm::{apply_abm_agents, AbmState};
use crate::catalog::{load_events, SimEventSpec};
use crate::invariants::{InvariantGate, MAX_DRIFT_MS};
use crate::l1_hooks::{emit_entropy, run_l1_action};
use crate::merkle::{batch_merkle_root_hex, event_leaf_hash};
use crate::regime_manager::RegimeManager;
use crate::state_bus::{InMemoryBus, SimStateBus};
use crate::strategy_generator::{SignalSide, StrategyGenerator};
use crate::telemetry::{SimStateSnapshot, SimTelemetryLog};
use clrty_substrate::kernel_core::entropy_bus::EntropyBus;
use quant_stack::execution_engine::order_logic::{OrderIntent, OrderSide};
use quant_stack::execution_engine::trade_executor::{default_book, TradeExecutor};
use quant_stack::fma::slippage_predictor::SlippageContext;
use quant_stack::fma::spread_detector::OrderBookSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const CHAIN_ID: &str = "clrty-1";
pub const TICK_SIZE: f64 = 1e-9;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimInput {
    pub book_imbalance: f64,
    pub blockchain_heartbeat_ms: f64,
    pub mid_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimDecision {
    pub strategy_id: u16,
    pub jitter_applied: f64,
    pub side: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimExecutionStatus {
    Filled,
    Rejected,
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimExecution {
    pub status: SimExecutionStatus,
    pub price: f64,
    pub latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimVerification {
    pub pnl: f64,
    pub system_drift: f64,
    pub is_deterministic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEventResult {
    pub event_id: u16,
    pub timestamp: String,
    pub regime: String,
    pub input: SimInput,
    pub decision: SimDecision,
    pub execution: SimExecution,
    pub verification: SimVerification,
    pub event_hash: String,
    pub chain_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimTick {
    pub event_id: u16,
    pub mid_price: f64,
    pub bid_depth: f64,
    pub ask_depth: f64,
    pub regime: String,
    pub lambda: f64,
}

pub struct SimState {
    pub entropy_bus: EntropyBus,
    pub strategy_weights: [f64; 101],
    pub cumulative_pnl: f64,
    pub canceled: u64,
    pub filled: u64,
    pub book: OrderBookSnapshot,
}

impl Default for SimState {
    fn default() -> Self {
        Self {
            entropy_bus: EntropyBus::new(),
            strategy_weights: [1.0; 101],
            cumulative_pnl: 0.0,
            canceled: 0,
            filled: 0,
            book: default_book("clrty-l1-sim"),
        }
    }
}

impl SimState {
    pub fn sync_node_state(&mut self, block_hint: u64) {
        let _ = block_hint;
        let payload = self.entropy_bus.root.to_vec();
        emit_entropy(&mut self.entropy_bus, 0, &payload, sim_timestamp_ns(0));
    }

    pub fn apply_risk_logic(&self, base: u64) -> u64 {
        base
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub seed: u64,
    pub events_run: usize,
    pub merkle_root: String,
    pub results: Vec<SimEventResult>,
    pub ticks: Vec<SimTick>,
    pub telemetry: Vec<SimTelemetryLog>,
}

fn snapshot(book: &OrderBookSnapshot, abm: &AbmState, entropy: &EntropyBus) -> SimStateSnapshot {
    SimStateSnapshot {
        mid_price: book.mid,
        bid_depth_usdc: book.bid_depth_usdc,
        ask_depth_usdc: book.ask_depth_usdc,
        staked_nano: abm.staked_nano,
        entropy_root: hex::encode(entropy.root),
    }
}

fn seeded_jitter(seed: u64, event_id: u16) -> f64 {
    let mut h = DefaultHasher::new();
    seed.hash(&mut h);
    event_id.hash(&mut h);
    let v = (h.finish() % 10_000) as f64;
    (v / 10_000.0 - 0.5) * 0.0002
}

fn sim_timestamp_ns(event_id: u16) -> u128 {
    1_700_000_000_000_000_000u128 + event_id as u128 * 1_000_000
}

fn iso_timestamp(event_id: u16) -> String {
    format!(
        "2026-06-15T12:{:02}:{:02}Z",
        (event_id / 60) % 60,
        event_id % 60
    )
}

fn apply_market_action(book: &mut OrderBookSnapshot, action: &str, event_id: u16) {
    match action {
        "book_inject" => {
            book.bid_depth_usdc += 100_000.0;
            book.ask_depth_usdc += 100_000.0;
        }
        "mid_absorb" => {
            book.mid += 0.001 * (event_id as f64 * 0.1);
            book.bid = book.mid - 0.005;
            book.ask = book.mid + 0.005;
        }
        "funding_vol" => {
            book.mid *= 1.0 + 0.001 * (event_id % 5) as f64;
        }
        "bid_contract" => {
            book.bid_depth_usdc *= 0.85;
        }
        "ask_sweep" => {
            book.ask_depth_usdc *= 0.7;
            book.mid += 0.01;
        }
        "latency_inject" | "entropy_inject" | "book_breath" => {
            book.bid_depth_usdc *= 0.9;
            book.ask_depth_usdc *= 0.9;
        }
        "cross_arb" => {
            book.mid += 0.002;
        }
        "regime_shift" => {
            book.mid += 0.005 * ((event_id % 3) as f64 - 1.0);
        }
        "sync_hash" => {}
        _ => {}
    }
    book.bid = book.mid - 0.005;
    book.ask = book.mid + 0.005;
}

fn rejection_probability(churn_ratio: f64) -> f64 {
    (churn_ratio / (1.0 + churn_ratio)).min(0.95)
}

pub fn simulate_event(
    state: &mut SimState,
    abm: &mut AbmState,
    spec: &SimEventSpec,
    strategy_id: u16,
    seed: u64,
    regime_mgr: &mut RegimeManager,
) -> (SimEventResult, SimTelemetryLog) {
    regime_mgr.transition(&spec.regime);
    let latency_inject = regime_mgr.latency_for_oq(spec.latency_ms_inject, spec.event_id);

    let pre = snapshot(&state.book, abm, &state.entropy_bus);

    if let Some(ref action) = spec.l1_action {
        let l1 = run_l1_action(action, spec.event_id);
        if !l1.ok {
            let result = aborted_result(
                spec,
                strategy_id,
                seed,
                &state.book,
                latency_inject,
                &l1.detail,
            );
            let post = snapshot(&state.book, abm, &state.entropy_bus);
            let log = SimTelemetryLog::from_event(pre, post, &result, vec![], "l1_abort");
            return (result, log);
        }
    }

    if let Some(ref maction) = spec.market_action {
        apply_market_action(&mut state.book, maction, spec.event_id);
    }

    let agent_actions = apply_abm_agents(&mut state.book, abm, seed, spec.event_id, &spec.regime);

    state.sync_node_state(spec.event_id as u64);

    let churn = if state.filled > 0 {
        state.canceled as f64 / state.filled as f64
    } else {
        0.1
    };
    let tick_velocity = (spec.event_id as f64) * 1.5;
    let gen = StrategyGenerator::default();
    let signal = gen.evaluate(strategy_id, &state.book, &spec.regime, tick_velocity, churn);

    let jitter = seeded_jitter(seed, spec.event_id);
    let target_price = signal.price + jitter;

    let kernel_start_ms = (seed.wrapping_add(spec.event_id as u64) % 20) as f64 * 0.1;
    let drift_ms = latency_inject as f64 * 0.1;

    let side_str = match signal.side {
        SignalSide::Buy => "BUY",
        SignalSide::Sell => "SELL",
        SignalSide::Null => "NULL",
    };

    let mut exec = TradeExecutor::new(true);
    let ctx = SlippageContext::default();
    let p_reject = rejection_probability(churn);

    let mut status = SimExecutionStatus::Rejected;
    let mut pnl_delta = 0.0;
    let reject_roll = (seed.wrapping_add(spec.event_id as u64) % 100) as f64 / 100.0;

    if signal.side != SignalSide::Null && reject_roll >= p_reject {
        let intent = OrderIntent {
            venue: state.book.venue.clone(),
            side: if signal.side == SignalSide::Buy {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            },
            amount_base_units: state.apply_risk_logic(1_000_000_000),
            min_score: 0,
        };
        if exec.execute(&intent, &state.book, &ctx).is_ok() {
            status = SimExecutionStatus::Filled;
            state.filled += 1;
            pnl_delta = signal.score * 0.0001;
            state.cumulative_pnl += pnl_delta;
        } else {
            state.canceled += 1;
        }
    } else if signal.side != SignalSide::Null {
        state.canceled += 1;
    } else {
        status = SimExecutionStatus::Filled;
        pnl_delta = 0.0;
    }

    let latency_ms = kernel_start_ms + latency_inject as f64;

    if drift_ms > MAX_DRIFT_MS && spec.regime != "Adversarial" {
        status = SimExecutionStatus::Aborted;
    }

    let imbalance = (state.book.bid_depth_usdc - state.book.ask_depth_usdc).abs()
        / (state.book.bid_depth_usdc + state.book.ask_depth_usdc).max(1.0);
    if imbalance > 0.15 && latency_ms > 1.0 {
        status = SimExecutionStatus::Aborted;
        state.canceled += 1;
    }

    if pnl_delta.abs() > 0.0001 && signal.score < 0.5 {
        let sid = strategy_id.min(100) as usize;
        state.strategy_weights[sid] = (state.strategy_weights[sid] - 0.01).max(0.1);
    }

    let mut result = SimEventResult {
        event_id: spec.event_id,
        timestamp: iso_timestamp(spec.event_id),
        regime: spec.regime.clone(),
        input: SimInput {
            book_imbalance: imbalance,
            blockchain_heartbeat_ms: latency_ms,
            mid_price: state.book.mid,
        },
        decision: SimDecision {
            strategy_id,
            jitter_applied: jitter,
            side: side_str.to_string(),
        },
        execution: SimExecution {
            status,
            price: target_price,
            latency_ms,
        },
        verification: SimVerification {
            pnl: pnl_delta,
            system_drift: drift_ms,
            is_deterministic: true,
        },
        event_hash: String::new(),
        chain_id: CHAIN_ID.to_string(),
    };

    result.event_hash = hex::encode(event_leaf_hash(&result));
    emit_entropy(
        &mut state.entropy_bus,
        spec.event_id,
        result.event_hash.as_bytes(),
        sim_timestamp_ns(spec.event_id),
    );

    if !InvariantGate::verify_result(&result) {
        result.verification.is_deterministic = false;
    }

    let post = snapshot(&state.book, abm, &state.entropy_bus);
    let event_type = if spec.l1_action.is_some() {
        "chain_market"
    } else {
        "market"
    };
    let log = SimTelemetryLog::from_event(pre, post, &result, agent_actions, event_type);

    (result, log)
}

fn aborted_result(
    spec: &SimEventSpec,
    strategy_id: u16,
    seed: u64,
    book: &OrderBookSnapshot,
    latency_ms: u32,
    _detail: &str,
) -> SimEventResult {
    let jitter = seeded_jitter(seed, spec.event_id);
    SimEventResult {
        event_id: spec.event_id,
        timestamp: iso_timestamp(spec.event_id),
        regime: spec.regime.clone(),
        input: SimInput {
            book_imbalance: 0.0,
            blockchain_heartbeat_ms: latency_ms as f64,
            mid_price: book.mid,
        },
        decision: SimDecision {
            strategy_id,
            jitter_applied: jitter,
            side: "NULL".into(),
        },
        execution: SimExecution {
            status: SimExecutionStatus::Aborted,
            price: book.mid,
            latency_ms: latency_ms as f64,
        },
        verification: SimVerification {
            pnl: 0.0,
            system_drift: latency_ms as f64,
            is_deterministic: false,
        },
        event_hash: hex::encode([0u8; 32]),
        chain_id: CHAIN_ID.to_string(),
    }
}

pub fn run_batch(seed: u64, from: u16, to: u16) -> Result<BatchResult, String> {
    let events = load_events()?;
    let mut state = SimState::default();
    let mut abm = AbmState {
        prev_mid: state.book.mid,
        ..Default::default()
    };
    let mut regime_mgr = RegimeManager::default();
    let mut bus = InMemoryBus::default();
    let mut results = Vec::new();
    let mut ticks = Vec::new();
    let mut telemetry = Vec::new();

    for spec in events
        .iter()
        .filter(|e| e.event_id >= from && e.event_id <= to)
    {
        let strategy_id = ((spec.event_id as u64 + seed) % 100 + 1) as u16;
        let (result, log) = simulate_event(
            &mut state,
            &mut abm,
            spec,
            strategy_id,
            seed,
            &mut regime_mgr,
        );
        bus.publish(&result);
        bus.publish_telemetry(&log);
        telemetry.push(log);
        ticks.push(SimTick {
            event_id: result.event_id,
            mid_price: result.input.mid_price,
            bid_depth: state.book.bid_depth_usdc,
            ask_depth: state.book.ask_depth_usdc,
            regime: result.regime.clone(),
            lambda: 0.1 + (result.event_id as f64 * 0.001),
        });
        results.push(result);
    }

    Ok(BatchResult {
        seed,
        events_run: results.len(),
        merkle_root: batch_merkle_root_hex(&results),
        results,
        ticks,
        telemetry,
    })
}
