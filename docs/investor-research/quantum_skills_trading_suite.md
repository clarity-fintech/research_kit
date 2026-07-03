# Quantum Skills Trading Suite

The **4 Quantum Skills** are CLRTY's institutional trading augmentation layer — deterministic, sequentially composable units wired into the CLI, quantitative tables, and investor dashboards.

**Core principle:** Each skill does exactly one thing, deterministically. One skill at a time per account; IP concurrency is capped.

Related: [`protocol/clarity-skills.md`](../protocol/clarity-skills.md) · [`investor/technical_due_diligence.md`](technical_due_diligence.md) §5 · **Full overview:** [`clarity_skills_overview.md`](clarity_skills_overview.md) · [`../arbitrage/arbitrage_program_guide.md`](../arbitrage/arbitrage_program_guide.md) (MCA ↔ `arbitrage_core`)

---

## The 4 Quantum Skills

### 1. Metric-Collapse Arbitrage (MCA) — `metric-collapse-arbitrage`

- Calculates distance from **99-1 optimization state**; filters noisy trades
- Executes only when deterministic score predicts **net edge after fees/latency**
- Links: `arbitrage_core/`, CCR orchestrator, MIRRA dark pool concepts — see [Arbitrage Program Guide](../arbitrage/arbitrage_program_guide.md)

### 2. Topological State-Rebalancing (TSR) — `topological-state-rebalance`

- Manages **99→1 set mechanics**; wallet tier migration based on entropy, holding duration, velocity
- **Fee-priority / Set 1 privilege buff**
- Links: `token_core/user_profile`, CCR, `blue_code/lane_enforcer`

### 3. Attestation-Verified Routing (AVR) — `attestation-verify`

- **Compliance-as-Code** via Attestation Blob; FATF/Travel Rule at kernel
- Links: `settlement/attestation_blob.rs`, gatekeeper

### 4. Entropy-Heartbeat Liquidation (EHL) — `entropy-heartbeat-check`

- **Black swan circuit breaker**; locks account lanes during extreme entropy
- Links: `entropy_bus`, `panic_stabilizer`, `capital_flight_guard`

---

## CLI commands

Install: [`cli/install.md`](../cli/install.md)

```bash
# Single skill (dual-lock: account + IP)
clarity skill run metric-collapse-arbitrage --account=0xINST --capital=5000000 --risk-mode=hard-kernel-strict

# Account lock status + quant table snapshot
clarity skill status --account=0xINST

# Halt active execution
clarity skill halt --account=0xINST

# Sequential pipeline (no parallel skill execution)
clarity strategy run --steps="attestation-verify,metric-collapse-arbitrage,entropy-heartbeat-check" \
  --capital=10000000 --risk-profile=institutional-max

# IP concurrency guard
clarity network ip-status
```

Alias: `clrty` binary is equivalent to `clarity`.

---

## Execution gate (dual-lock)

Implemented in `clrty-cli-core/src/skills/mod.rs`:

| Gate | Rule |
|------|------|
| Account lock | One active skill per account ID |
| IP guard | Max concurrent skills per IP (default 3, from manifest) |
| Halt | Releases lock; blocks re-acquire until cleared |
| Pipeline | Sequential only — output of step N feeds step N+1 |

---

## Machine-readable configs

| File | Purpose |
|------|---------|
| `CLRTY_SUBSTRATE/boot/quant_skills_manifest.json` | Skill definitions, gates, risk profiles |
| `var/trading/quant_skills_table.json` | Runtime metrics (MCA edge, TSR tier, AVR pass/fail, EHL heartbeat) |

Sync to investor dashboards:

```bash
bash scripts/investor/build_treasury_data.sh
```

---

## Rust module map

| Path | Role |
|------|------|
| `clrty-cli-core/src/skills/mod.rs` | Registry, dual-lock gate, table persistence |
| `clrty-cli-core/src/skills/mca.rs` | Metric-Collapse Arbitrage |
| `clrty-cli-core/src/skills/tsr.rs` | Topological State-Rebalancing |
| `clrty-cli-core/src/skills/avr.rs` | Attestation-Verified Routing |
| `clrty-cli-core/src/skills/ehl.rs` | Entropy-Heartbeat Liquidation |
| `clrty-cli-core/src/skills/pipeline.rs` | Sequential strategy runner |
| `clrty-cli-core/src/handlers/skill.rs` | `skill run|status|halt` |
| `clrty-cli-core/src/handlers/strategy.rs` | `strategy run` |

---

## Scaffold vs wired

Skills call real substrate modules where available. Partial wiring returns structured JSON with `"scaffold": true` and an `evidence` block documenting what ran vs what was modeled.

| Skill | Wired | Scaffold areas |
|-------|-------|----------------|
| MCA | `arbitrage_core::spread_scan`, FeedHub | CCR/MIRRA depth |
| TSR | Deterministic tier model | Live user_profile migration |
| AVR | AttestationLedger, ComplianceSigner probe | Full gatekeeper routing |
| EHL | EntropyBus emit, PanicStabilizer | capital_flight_guard lane lock |

---

## Tests

```bash
cargo test -p clrty-cli-core
cargo build --workspace
```

Unit tests cover skill registry resolution, account dual-lock, IP concurrency limit, halt release, and pipeline step parsing.
