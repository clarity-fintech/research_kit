//! Tokenomics stress engine for CI (Task S17).

const SUPPLY: u64 = 16_000_000;
const NTT_OUTBOUND_CAP: u64 = 2_500_000;

fn stress_ntt_outbound(transfers: u32, clip: u64) -> (u64, u64) {
    let requested = transfers as u64 * clip;
    let executed = requested.min(NTT_OUTBOUND_CAP);
    let queued = requested.saturating_sub(NTT_OUTBOUND_CAP);
    (executed, queued)
}

fn main() {
    let (executed, queued) = stress_ntt_outbound(30, 100_000);
    assert!(executed <= NTT_OUTBOUND_CAP);
    clrty_substrate::verify_genesis().expect("genesis");
    println!("executed={executed} queued={queued} supply_cap={SUPPLY}");
}
