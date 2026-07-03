use crate::event_loop::SimEventResult;
use crate::telemetry::SimTelemetryLog;
use serde_json;

pub trait SimStateBus: Send {
    fn publish(&mut self, result: &SimEventResult);
    fn publish_telemetry(&mut self, log: &SimTelemetryLog) {
        let _ = log;
    }
}

#[derive(Default)]
pub struct InMemoryBus {
    pub messages: Vec<String>,
    pub telemetry: Vec<String>,
}

impl SimStateBus for InMemoryBus {
    fn publish(&mut self, result: &SimEventResult) {
        if let Ok(line) = serde_json::to_string(result) {
            self.messages.push(line);
        }
    }

    fn publish_telemetry(&mut self, log: &SimTelemetryLog) {
        if let Ok(line) = serde_json::to_string(log) {
            self.telemetry.push(line);
        }
    }
}

#[cfg(feature = "redis-sync")]
mod redis_impl {
    use super::*;
    use crate::telemetry::SimTelemetryLog;

    pub struct RedisBus {
        channel: String,
        client: redis::Client,
    }

    impl RedisBus {
        pub fn from_env() -> Result<Self, String> {
            let url = std::env::var("CLRTY_SIM_REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
            let client = redis::Client::open(url.as_str()).map_err(|e| e.to_string())?;
            Ok(Self {
                channel: "CLRTY_SYNC".into(),
                client,
            })
        }
    }

    impl SimStateBus for RedisBus {
        fn publish(&mut self, result: &SimEventResult) {
            if let Ok(payload) = serde_json::to_string(result) {
                let _ = self.client.get_connection().and_then(|mut con| {
                    redis::cmd("PUBLISH")
                        .arg(&self.channel)
                        .arg(payload)
                        .query(&mut con)
                });
            }
        }

        fn publish_telemetry(&mut self, log: &SimTelemetryLog) {
            if let Ok(payload) = serde_json::to_string(log) {
                let _ = self.client.get_connection().and_then(|mut con| {
                    redis::cmd("PUBLISH")
                        .arg(&self.channel)
                        .arg(payload)
                        .query(&mut con)
                });
            }
        }
    }
}

#[cfg(feature = "redis-sync")]
pub use redis_impl::RedisBus;
