use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEventSpec {
    pub event_id: u16,
    pub oq: String,
    pub regime: String,
    pub bucket: String,
    pub name: String,
    pub l1_action: Option<String>,
    pub market_action: Option<String>,
    pub latency_ms_inject: u32,
    pub verify: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EventsFile {
    events: Vec<SimEventSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySpec {
    pub strategy_id: u16,
    pub primitive: String,
    pub variant: u8,
    pub threshold: f64,
    pub velocity_gate: f64,
    pub churn_gate: f64,
    pub z_entry: f64,
    pub regime_mask: String,
}

#[derive(Debug, Deserialize)]
struct StrategiesFile {
    strategies: Vec<StrategySpec>,
}

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn load_events() -> Result<Vec<SimEventSpec>, String> {
    let path = data_dir().join("events_100.json");
    let data = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let file: EventsFile = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(file.events)
}

pub fn load_strategies() -> Result<Vec<StrategySpec>, String> {
    let path = data_dir().join("strategies_config_space.json");
    let data = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let file: StrategiesFile = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(file.strategies)
}
