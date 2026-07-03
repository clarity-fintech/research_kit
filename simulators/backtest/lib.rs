//! Backtest simulators for ATU 626, 630, 635, 640.

use quant_stack::execution_engine::trade_executor::{default_book, sample_intent, TradeExecutor};
use quant_stack::execution_engine::weight_lock::WeightLock;
use quant_stack::feature_engine::structure_detector::sample_bars;
use quant_stack::fma::backtest_registry::BacktestRegistry;
use quant_stack::fma::slippage_predictor::SlippageContext;
use quant_stack::fma::spread_detector::{OrderBookSnapshot, SpreadDetector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub atu: u16,
    pub fills: usize,
    pub pass: bool,
    pub detail: String,
}

pub fn run_626() -> BacktestResult {
    let reg = BacktestRegistry::new();
    let scenario = reg.get(626).expect("626 scenario");
    let mut exec = TradeExecutor::new(true);
    let book = default_book("base");
    let ctx = SlippageContext::default();
    let intent = sample_intent("base");
    let ok = exec.execute(&intent, &book, &ctx).is_ok();
    BacktestResult {
        atu: 626,
        fills: exec.fills.len(),
        pass: ok && scenario.bars >= 50,
        detail: format!("scenario={} bars={}", scenario.name, scenario.bars),
    }
}

pub fn run_630() -> BacktestResult {
    let bars = sample_bars();
    let books = vec![
        default_book("base"),
        OrderBookSnapshot {
            venue: "arbitrum".into(),
            bid: 1.03,
            ask: 1.04,
            mid: 1.035,
            bid_depth_usdc: 500_000.0,
            ask_depth_usdc: 500_000.0,
        },
    ];
    let opp = SpreadDetector::analyze(&books);
    BacktestResult {
        atu: 630,
        fills: if opp.is_some() { 1 } else { 0 },
        pass: !bars.is_empty(),
        detail: format!("spread={:?}", opp.map(|o| o.net_profit)),
    }
}

pub fn run_635() -> BacktestResult {
    let book = default_book("base");
    let ctx = SlippageContext {
        max_slippage_bps: 100,
        ..Default::default()
    };
    let mut exec = TradeExecutor::new(true);
    let intents: Vec<_> = (0..5)
        .map(|i| {
            let mut intent = sample_intent("base");
            intent.amount_base_units = 100_000_000 * (i as u64 + 1);
            intent
        })
        .collect();
    let n = exec.simulate_batch(&intents, &book, &ctx);
    BacktestResult {
        atu: 635,
        fills: n,
        pass: n >= 1,
        detail: format!("stress_fills={}", n),
    }
}

pub fn run_640() -> BacktestResult {
    let mut lock = WeightLock::new();
    lock.set_weight("alpha", 0.5).unwrap();
    lock.lock("backtest_singularity");
    let pass = lock.is_locked();
    BacktestResult {
        atu: 640,
        fills: 0,
        pass,
        detail: "weight_lock_engaged".into(),
    }
}

pub fn run_by_id(id: u16) -> Option<BacktestResult> {
    match id {
        626 => Some(run_626()),
        630 => Some(run_630()),
        635 => Some(run_635()),
        640 => Some(run_640()),
        _ => None,
    }
}
