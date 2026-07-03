pub mod abm;
pub mod catalog;
pub mod chain;
pub mod event_loop;
pub mod invariants;
pub mod l1_hooks;
pub mod merkle;
pub mod regime_manager;
pub mod state_bus;
pub mod strategy_generator;
pub mod telemetry;

pub use abm::{AbmState, AgentAction, AgentPersona};
pub use catalog::{load_events, load_strategies, SimEventSpec};
pub use chain::{calculate_state_root, validate_block, Block, BlockHeader, Transaction};
pub use event_loop::{run_batch, BatchResult, SimEventResult, SimState};
pub use merkle::batch_merkle_root_hex;
pub use telemetry::{SimStateSnapshot, SimTelemetryLog};
