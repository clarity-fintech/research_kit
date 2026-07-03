use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketRegime {
    Accumulation,
    Stable,
    Expansion,
    Adversarial,
}

impl MarketRegime {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Accumulation" => Self::Accumulation,
            "Expansion" => Self::Expansion,
            "Adversarial" => Self::Adversarial,
            _ => Self::Stable,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accumulation => "Accumulation",
            Self::Stable => "Stable",
            Self::Expansion => "Expansion",
            Self::Adversarial => "Adversarial",
        }
    }
}

pub struct RegimeManager {
    pub current: MarketRegime,
}

impl Default for RegimeManager {
    fn default() -> Self {
        Self {
            current: MarketRegime::Stable,
        }
    }
}

impl RegimeManager {
    pub fn transition(&mut self, event_regime: &str) {
        self.current = MarketRegime::from_str(event_regime);
    }

    pub fn latency_for_oq(&self, base_ms: u32, event_id: u16) -> u32 {
        match self.current {
            MarketRegime::Adversarial => base_ms.saturating_add(10 + (event_id % 40) as u32),
            MarketRegime::Expansion => base_ms.saturating_add(5),
            _ => base_ms,
        }
    }
}
