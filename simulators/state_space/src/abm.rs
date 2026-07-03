//! Agent-Based Model — deterministic personas interacting with book + chain state.

use quant_stack::fma::spread_detector::OrderBookSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPersona {
    Arbitrageur,
    FomoUser,
    LongTermHolder,
}

impl AgentPersona {
    pub fn all() -> &'static [AgentPersona] {
        &[
            AgentPersona::Arbitrageur,
            AgentPersona::FomoUser,
            AgentPersona::LongTermHolder,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arbitrageur => "arbitrageur",
            Self::FomoUser => "fomo_user",
            Self::LongTermHolder => "long_term_holder",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub persona: String,
    pub action: String,
    pub delta_mid: f64,
}

#[derive(Default)]
pub struct AbmState {
    pub staked_nano: u64,
    pub prev_mid: f64,
}

fn agent_roll(seed: u64, event_id: u16, persona: AgentPersona) -> f64 {
    let mut h = DefaultHasher::new();
    seed.hash(&mut h);
    event_id.hash(&mut h);
    (persona as u8).hash(&mut h);
    (h.finish() % 10_000) as f64 / 10_000.0
}

/// Run all ABM agents deterministically before strategy evaluation.
pub fn apply_abm_agents(
    book: &mut OrderBookSnapshot,
    abm: &mut AbmState,
    seed: u64,
    event_id: u16,
    regime: &str,
) -> Vec<AgentAction> {
    let prev_mid = abm.prev_mid.max(book.mid);
    let mut actions = Vec::new();

    for persona in AgentPersona::all() {
        let roll = agent_roll(seed, event_id, *persona);
        let action = match persona {
            AgentPersona::Arbitrageur => {
                let spread = book.ask - book.bid;
                if spread > 0.02 {
                    book.mid = (book.bid + book.ask) / 2.0;
                    book.bid = book.mid - 0.005;
                    book.ask = book.mid + 0.005;
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "narrow_spread".into(),
                        delta_mid: book.mid - prev_mid,
                    }
                } else {
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "observe".into(),
                        delta_mid: 0.0,
                    }
                }
            }
            AgentPersona::FomoUser => {
                if book.mid > prev_mid * 1.001 && roll > 0.3 {
                    book.ask_depth_usdc *= 0.98;
                    book.mid *= 1.0 + 0.002 * roll;
                    book.bid = book.mid - 0.005;
                    book.ask = book.mid + 0.005;
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "momentum_buy".into(),
                        delta_mid: book.mid - prev_mid,
                    }
                } else {
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "wait".into(),
                        delta_mid: 0.0,
                    }
                }
            }
            AgentPersona::LongTermHolder => {
                if regime == "Stable" || regime == "Accumulation" {
                    abm.staked_nano = abm.staked_nano.saturating_add(10_000_000);
                    book.bid_depth_usdc *= 1.01;
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "stake_add_liquidity".into(),
                        delta_mid: 0.0,
                    }
                } else {
                    AgentAction {
                        persona: persona.as_str().into(),
                        action: "hold".into(),
                        delta_mid: 0.0,
                    }
                }
            }
        };
        actions.push(action);
    }

    abm.prev_mid = book.mid;
    actions
}
