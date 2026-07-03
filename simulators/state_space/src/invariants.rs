use crate::event_loop::{SimEventResult, SimExecutionStatus};

pub const CLRTY_TICK: f64 = 1e-9;
pub const MAX_KERNEL_LATENCY_MS: f64 = 2.0;
pub const MAX_DRIFT_MS: f64 = 1.0;
pub const MAX_IMBALANCE_PCT: f64 = 0.15;
pub const MAX_PNL_DEVIATION: f64 = 0.0001;

pub struct InvariantGate;

impl InvariantGate {
    pub fn tick_aligned(price: f64) -> bool {
        let ticks = (price / CLRTY_TICK).round();
        (ticks * CLRTY_TICK - price).abs() < CLRTY_TICK * 0.5
    }

    pub fn kernel_latency_ok(latency_ms: f64) -> bool {
        latency_ms <= MAX_KERNEL_LATENCY_MS
    }

    pub fn drift_ok(drift_ms: f64) -> bool {
        drift_ms <= MAX_DRIFT_MS
    }

    pub fn verify_result(result: &SimEventResult) -> bool {
        let exec = &result.execution;
        if exec.latency_ms > MAX_KERNEL_LATENCY_MS && result.regime != "Adversarial" {
            return false;
        }
        if !Self::tick_aligned(exec.price) && exec.price > 0.0 {
            return false;
        }
        if result.verification.system_drift > MAX_DRIFT_MS {
            return false;
        }
        exec.status != SimExecutionStatus::Aborted
    }
}
