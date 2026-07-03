# CLRTY Technical Whitepaper (Draft — S20)

**Visual framework:** [Clarity Visual Protocol](investor/clarity_visual_protocol.md) · **Depth audit:** [whitepaper_depth_layer_audit.md](audit/whitepaper_depth_layer_audit.md)

## Abstract

$CLRTY implements Autonetic Moniversion — a self-optimizing ledger where wallet classification (Sets 99→1) emerges from continuous fixed-point inference. Cross-chain supply is governed by Wormhole NTT as the primary "Global Accountant," with execution-aware settlement gated on live order book depth.

## 1. Geometric Topology — Set Manifold

Wallet addresses occupy a 100-tier binary search tree (Sets 99→1). Each transfer re-solves tier assignment via:

**S(x) = W·X + B** over telemetry features M₀–M₃:

| Feature | Symbol | Source |
|---------|--------|--------|
| Bid-ask spread thickness | M₀ | CEX/DEX order books |
| Holding half-life | M₁ | On-chain activity profile |
| Cross-chain bridge velocity | M₂ | NTT TransferSent/Received rate |
| Wallet cluster entropy | M₃ | `clustering_detector.rs` |

Set migration follows threshold crossings in the CCR orchestrator. Set 1 (singularity) receives zero-gas kernel routing.

## 2. Consensus — Proof-of-Convergence

> **CVP L2 · Figure:** λ heartbeat — [Chart 14](simulation/charts/14_sim_tick_lambda_e1_25.png) · normalized signal: [signal_normalization.md](investor/signal_normalization.md)

Validators maximize **E(x) − λR(x)** where E is execution efficiency and R is structural risk. λ adapts via the entropy sink engine.

## 3. Supply Model

> **CVP L1 · Figure:** [Chart 01](simulation/charts/01_genesis_allocation.png) genesis pie · [Chart 33](simulation/charts/33_treasury_transparency_100pct.png) 100% treasury map · [Live dashboard](frontend/investor/treasury-dashboard.html)

- **Hard cap:** 16,000,000 CLRTY (9 decimals, immutable)
- **Mint authority:** NULL post-genesis
- **Global accountant:** `supply_oracle.rs` + `supply_harmonizer.rs` across NTT + LayerZero fallback

### Distribution tiers
See `docs/tokenomics/distribution_tiers.md` — Team, Private Seed, Ecosystem, Liquidity, Treasury, Validators, Public.

## 4. Cross-Chain — CLRTY L1 Authoritative

**Launch scope:** CLRTY L1 (`clrty-1` / `uclrty`) is the sole authoritative ledger for supply and allocations.

Omnichain bridge mirrors (LayerZero OFTv2, Wormhole NTT) are **deferred Phase 10** — see [`docs/l1_launch/DEFERRED_BRIDGE.md`](l1_launch/DEFERRED_BRIDGE.md).

Tokenomics integrity: [`docs/tokenomics/TOKENOMICS_LOCKED.md`](tokenomics/TOKENOMICS_LOCKED.md).

## 5. Token Utility

| Layer | Function |
|-------|----------|
| Monetary | Native settlement (`uclrty`) |
| Execution | MVM gas, parallel execution licensing |
| Incentive | Staking tiers 1–4, LP coordination |
| Governance | Snapshot voting weighted by stake tier |
| Alignment | Fee burn + velocity defense |

## 6. Velocity Defense

Tier 4 staking (730 days, 5× weight multiplier) locks supply against early secondary distribution. High-velocity wallets incur fee deflection penalties via `activity_profile_tracker.rs`.

## 7. Bounded Epigenesis

Blue Code patch registry activates fee/opcode adjustments under elevated λ — no arbitrary bytecode mutation at launch.

## References

- **Clarity Visual Protocol:** `docs/investor/clarity_visual_protocol.md`
- **Figure index:** `docs/audit/whitepaper_depth_layer_audit.md`
- Architecture: `docs/architecture/ntt_evm_svm_architecture.md`
- Utility profiles: `docs/tokenomics/utility_profiles.md`
- Genesis: `CLRTY_SUBSTRATE/boot/genesis_entropy.json`
