use crate::catalog::{load_strategies, StrategySpec};
use quant_stack::feature_engine::structure_detector::sample_bars;
use quant_stack::fma::spread_detector::OrderBookSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalSide {
    Buy,
    Sell,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySignal {
    pub strategy_id: u16,
    pub side: SignalSide,
    pub price: f64,
    pub score: f64,
    pub active: bool,
}

pub struct StrategyGenerator {
    strategies: Vec<StrategySpec>,
}

impl StrategyGenerator {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            strategies: load_strategies()?,
        })
    }

    pub fn get(&self, id: u16) -> Option<&StrategySpec> {
        self.strategies.iter().find(|s| s.strategy_id == id)
    }

    pub fn evaluate(
        &self,
        strategy_id: u16,
        book: &OrderBookSnapshot,
        regime: &str,
        tick_velocity: f64,
        churn_ratio: f64,
    ) -> StrategySignal {
        let spec = self
            .get(strategy_id)
            .or_else(|| self.get((strategy_id % 100).max(1)));
        let Some(spec) = spec else {
            return StrategySignal {
                strategy_id,
                side: SignalSide::Null,
                price: book.mid,
                score: 0.0,
                active: false,
            };
        };
        let active = spec.regime_mask == regime || regime == "Stable";
        if !active {
            return StrategySignal {
                strategy_id: spec.strategy_id,
                side: SignalSide::Null,
                price: book.mid,
                score: 0.0,
                active: false,
            };
        }
        let score = match spec.primitive.as_str() {
            "liquidity_sniping" => {
                let depth = book.bid_depth_usdc + book.ask_depth_usdc;
                if depth > spec.threshold * 1_000_000.0 {
                    1.0
                } else {
                    0.0
                }
            }
            "mean_reversion" => {
                let bars = sample_bars();
                let mean: f64 = bars.iter().map(|b| b.close).sum::<f64>() / bars.len() as f64;
                let z = (book.mid - mean).abs() / 0.01_f64.max(mean * 0.01);
                if z > spec.z_entry {
                    1.0
                } else {
                    0.0
                }
            }
            "flow_momentum" => {
                if tick_velocity > spec.velocity_gate {
                    1.0
                } else {
                    0.0
                }
            }
            "adversarial_churn" => {
                if churn_ratio > spec.churn_gate {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };
        let side = if score >= spec.threshold {
            if book.mid > 1.0 {
                SignalSide::Buy
            } else {
                SignalSide::Sell
            }
        } else {
            SignalSide::Null
        };
        StrategySignal {
            strategy_id: spec.strategy_id,
            side,
            price: book.mid,
            score,
            active: true,
        }
    }
}

impl Default for StrategyGenerator {
    fn default() -> Self {
        Self::new().expect("strategies config")
    }
}
