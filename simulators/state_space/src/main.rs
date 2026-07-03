use clap::Parser;
use clrty_state_space_sim::{run_batch, SimEventResult};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "clrty-state-space-sim")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run event batch (default events 1-100)
    Batch {
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, default_value = "1-100")]
        events: String,
        #[arg(long)]
        ndjson: Option<PathBuf>,
        #[arg(long)]
        write_fixture: bool,
    },
}

fn parse_range(s: &str) -> Result<(u16, u16), String> {
    if let Some((a, b)) = s.split_once('-') {
        let from: u16 = a
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let to: u16 = b
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok((from, to))
    } else {
        let n: u16 = s
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok((n, n))
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Batch {
            seed,
            events,
            ndjson,
            write_fixture,
        } => {
            let (from, to) = parse_range(&events)?;
            let batch = run_batch(seed, from, to)?;
            println!(
                "batch seed={} events={} merkle={}",
                batch.seed, batch.events_run, batch.merkle_root
            );
            if batch.events_run != (to - from + 1) as usize {
                return Err(format!(
                    "expected {} events, ran {}",
                    to - from + 1,
                    batch.events_run
                ));
            }
            let ndjson_path = ndjson.unwrap_or_else(|| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/batch_100.ndjson")
            });
            if let Some(parent) = ndjson_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut f = std::fs::File::create(&ndjson_path).map_err(|e| e.to_string())?;
            for r in &batch.results {
                let line = serde_json::to_string(r).map_err(|e| e.to_string())?;
                writeln!(f, "{line}").map_err(|e| e.to_string())?;
            }
            let ticks_path = ndjson_path.with_file_name("batch_100_ticks.json");
            std::fs::write(
                &ticks_path,
                serde_json::to_string_pretty(&batch.ticks).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            let telemetry_path = ndjson_path.with_file_name("batch_100_telemetry.ndjson");
            let mut tf = std::fs::File::create(&telemetry_path).map_err(|e| e.to_string())?;
            for log in &batch.telemetry {
                let line = serde_json::to_string(log).map_err(|e| e.to_string())?;
                writeln!(tf, "{line}").map_err(|e| e.to_string())?;
            }
            if write_fixture || from == 1 && to == 100 {
                let fixture =
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/batch_100_merkle.txt");
                std::fs::write(&fixture, &batch.merkle_root).map_err(|e| e.to_string())?;
            }
            // determinism self-check
            let batch2 = run_batch(seed, from, to)?;
            if batch2.merkle_root != batch.merkle_root {
                return Err(format!(
                    "merkle mismatch {} vs {}",
                    batch.merkle_root, batch2.merkle_root
                ));
            }
            Ok(())
        }
    }
}

// silence unused import warning in binary
#[allow(dead_code)]
fn _type_check(_: SimEventResult) {}
