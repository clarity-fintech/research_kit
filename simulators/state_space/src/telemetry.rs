//! Unified simulation telemetry — pre/post state JSON for every event.

use crate::abm::AgentAction;
use crate::event_loop::{SimEventResult, SimExecutionStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimStateSnapshot {
    pub mid_price: f64,
    pub bid_depth_usdc: f64,
    pub ask_depth_usdc: f64,
    pub staked_nano: u64,
    pub entropy_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimTelemetryLog {
    pub timestamp: String,
    pub event_type: String,
    pub event_id: u16,
    pub pre_state: SimStateSnapshot,
    pub post_state: SimStateSnapshot,
    pub status: String,
    pub event_hash: String,
    pub agent_actions: Vec<AgentAction>,
    pub chain_id: String,
}

impl SimTelemetryLog {
    pub fn from_event(
        pre: SimStateSnapshot,
        post: SimStateSnapshot,
        result: &SimEventResult,
        agents: Vec<AgentAction>,
        event_type: &str,
    ) -> Self {
        let status = match result.execution.status {
            SimExecutionStatus::Filled => "filled",
            SimExecutionStatus::Rejected => "rejected",
            SimExecutionStatus::Aborted => "aborted",
        };
        Self {
            timestamp: result.timestamp.clone(),
            event_type: event_type.to_string(),
            event_id: result.event_id,
            pre_state: pre,
            post_state: post,
            status: status.to_string(),
            event_hash: result.event_hash.clone(),
            agent_actions: agents,
            chain_id: result.chain_id.clone(),
        }
    }
}
