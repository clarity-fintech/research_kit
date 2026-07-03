# CLRTY Full Project Manifest

**Purpose:** Comprehensive due-diligence inventory — workspace crates, boot manifests, `var/` outputs, scripts, frontend surfaces, and security stack.  
**Status date:** 2026-06-19  
**Repository:** [theangelofwill/-CLRTY](https://github.com/theangelofwill/-CLRTY)  
**Investor index:** [`INVESTOR_DATA_ROOM_INDEX.md`](INVESTOR_DATA_ROOM_INDEX.md)

---

## Summary counts

| Category | Count | Notes |
|----------|-------|-------|
| Workspace crates | **16** | Root `Cargo.toml` members |
| Investor docs | **19** | `docs/investor/*.md` |
| Boot JSON manifests | **16** | `CLRTY_SUBSTRATE/boot/` |
| Committed `var/` JSON | **11** | Compliance, launch, pretest, trading, full_test |
| Shell scripts | **197+** | `scripts/` including 165+ ATU milestone scripts |
| Frontend files | **119+** | Public hub, docs portal, investor HUD |
| Git tracked files | **~1005** | Excludes `target/`, `var/tmp/`, `.env`, runtime |
| MSA-100 security layers | **100** | Zones I–IV |
| Sovereign protocols | **600** | SP-001–600 (Zones A–Y) |
| Quantum Skills | **4** | MCA, TSR, AVR, EHL |
| Genesis supply | **16,000,000 CLRTY** | 9 decimals, immutable cap |

---

## Workspace crates (16)

Defined in root [`Cargo.toml`](../../Cargo.toml):

| # | Crate / path | Role | Primary binaries |
|---|--------------|------|------------------|
| 1 | `CLRTY_SUBSTRATE/` | L1 substrate — PoC, MVM, CCR, settlement, token_core | `clarityd`, `clrty-gatekeeper`, `l-dnet-stress` |
| 2 | `clarity-cli/` | Operator CLI entry (`clrty`) | `clrty` |
| 3 | `clrty-cli-core/` | CLI handlers, pipeline, skill registry | — |
| 4 | `clrty-cli-ui/` | TUI layouts (client / operator) | — |
| 5 | `clrty-api/` | REST :8545 + WS operator API | `clrty-api` |
| 6 | `clrty-signal-bridge/` | Signal validation proxy | — |
| 7 | `arbitrage_core/` | Producer / arb loop engine | — |
| 8 | `pretest_runner/` | 100-task systemic pretest | `pretest_runner` |
| 9 | `atu_runner/` | ATU verification phases (501–990+) | `atu_runner` |
| 10 | `fma-relayer/` | FMA bridge relayer | `fma-relayer` |
| 11 | `quant_stack/` | Off-chain quant / FMA execution | — |
| 12 | `backlog/` | Pre-launch holder capture | — |
| 13 | `simulators/tokenomics_engine/` | Tokenomics simulation | — |
| 14 | `simulators/backtest/` | Backtest engine | — |
| 15 | `simulators/state_space/` | State-space ABM (SIM100) | — |
| 16 | `simulators/state_space/` (via workspace) | Event batch simulation | — |

**Deprecated:** `CLARITY_IA/` — migrated to `CLRTY_SUBSTRATE/`

---

## CLRTY_SUBSTRATE module map

```
CLRTY_SUBSTRATE/
├── boot/                    # Genesis, listing, pretest, security manifests
├── settlement/              # gatekeeper, attestation, safe monitor, capital flight
├── token_core/              # layout_v2, blue_code, merkle, verify
├── state_manifold/          # state_root, genesis_seal, worm_audit, transaction_log
├── h2tb/                    # hardware-to-token bridge
├── l_dnet/                  # stress runner
├── poc_consensus/           # PoC engine
├── economic_engine/         # tokenomics, supply_checksum
├── treasury_sink/           # vesting escrow
├── bridge_perimeter/        # FMA, multisig, connection registry
└── launch_deployment/       # testnet manifold docker-compose
```

---

## Boot manifests (16 JSON files)

| File | Purpose |
|------|---------|
| `boot/genesis_entropy.json` | Genesis supply seal, entropy parameters |
| `boot/tokenomics_manifest.json` | Locked tokenomics parameters |
| `boot/mainnet_listing_config.json` | CEX lock-ups, vesting schedules |
| `boot/settlement_config.json` | Gatekeeper / Safe treasury config |
| `boot/pretest_manifest.json` | PT-001–PT-100 pretest task map |
| `boot/security_layers_manifest.json` | MSA-100 — 100 layers, Zones I–IV |
| `boot/sovereign_protocols_manifest.json` | Sovereign SP-001–600 |
| `boot/sovereign_protocol_map.json` | Protocol ID → module mapping |
| `boot/sovereign_zone_catalog.json` | Zone A–Y catalog |
| `boot/sovereign_zones_u_y.json` | Atomic defense zones U–Y |
| `boot/sovereign_canonical_titles.json` | Canonical protocol titles |
| `boot/quant_skills_manifest.json` | 4 Quantum Skills + gates + risk profiles |
| `boot/baseline_metrics/genesis_tier_allocations.json` | Sets matrix / tier allocations |
| `boot/baseline_metrics/foundation_sink_state.json` | Foundation sink baseline |
| `boot/baseline_metrics/initial_manifold_weights.json` | Manifold weight initialization |
| `boot/baseline_metrics/validator_singularity_set.json` | Validator singularity set |

---

## var/ outputs (committed)

Reproducible validation artifacts for dashboards and CI. **Excluded:** `var/settlement/` (runtime), `var/tmp/` (PDF scratch extracts).

| Path | Generator | Purpose |
|------|-----------|---------|
| `var/compliance/security_layers_report.json` | `verify_security_layers.sh` | MSA-100 audit — 100 layers, 100% documented |
| `var/compliance/sovereign_protocols_report.json` | `verify_sovereign_protocols.sh` | Sovereign-600 — 600 protocols, gate PASS |
| `var/compliance/listing_compliance_report.json` | `generate_listing_compliance_pack.sh` | Task 38 listing pack |
| `var/compliance/bridge_connection_audit_report.json` | `verify_bridge_connection_hashes.sh` | INF-25 bridge audit |
| `var/launch/launch_readiness_report.json` | `launch_readiness.sh` | 92.3% readiness_pct, launch_ready |
| `var/launch/mainnet_contract_gates.json` | `verify_mainnet_contract_gates.sh` | Mainnet contract GO gates |
| `var/pretest/systemic_readiness.json` | pretest pipeline | Systemic readiness snapshot |
| `var/pretest/full_pretest_report.json` | `full_pretest.sh` | 100-task pretest output |
| `var/trading/quant_skills_table.json` | skill runtime / build scripts | MCA, TSR, AVR, EHL metrics |
| `var/full_test/report.json` | `full_validation.sh` | Full validation summary |
| `var/full_test/system_check_report.json` | `system_check.sh` | System check output |

**Investor HUD sync:** `bash scripts/investor/build_treasury_data.sh` copies key JSON into `frontend/investor/data/`.

---

## Scripts index

| Directory | Count | Purpose |
|-----------|-------|---------|
| `scripts/launch/` | 2 | `launch_readiness.sh`, `verify_mainnet_contract_gates.sh` |
| `scripts/audit/` | 8 | MSA-100, Sovereign-600, listing, bridge, immutability, Slither, Mythril |
| `scripts/atu/` | 165+ | ATU milestone scripts (101–990, 2001–2200, run_all_milestones) |
| `scripts/investor/` | 3 | `build_treasury_data.sh`, `generate_nano_details.py`, `generate_sovereign_protocols.py` |
| `scripts/test/` | 3 | `full_validation.sh`, `full_pretest.sh`, `system_check.sh` |
| `scripts/stress/` | 3 | `break_it_suite.sh`, `l1_concurrency.sh`, `fork_swap_stress.sh` |
| `scripts/fma/` | 3 | deploy, relayer, supply cap verify |
| `scripts/integration/` | 2 | NTT gate E2E, sandbox dry run |
| `scripts/predeploy/` | 2 | full stack + L1 launch simulation |
| `scripts/sim/` | 1 | `run_100_events.sh` |
| `scripts/multisig/` | 1 | custody deploy |
| Root | 5 | `verify_launch.sh`, `bootstrap_testnet.sh`, `code_freeze.sh`, `export_abis.sh`, `sync_builder_root.sh` |

---

## Frontend surfaces

**Guide:** [`website_and_investor_portal.md`](website_and_investor_portal.md)

| Path | Audience | Key pages |
|------|----------|-----------|
| `frontend/index.html` | Public | Products hub, live chart |
| `frontend/docs/` | Public | Documentation portal (119+ content files) |
| `frontend/network/` | Public | Network hub, validators, RPC, status |
| `frontend/products/` | Public | clarity-skills, x402, agents-ai, developer-platform |
| `frontend/tools/` | Public | clarity-pay, commerce-kit, token-extensions |
| `frontend/community/` | Public | events, news, podcasts, breakpoint |
| `frontend/portal/` | Mixed | gate, swap, staking, settings |
| `frontend/investor/` | Private | treasury-dashboard, clrty-graph, nexus, investor-hub |
| `frontend/b2b-panel/` | Private | B2B institutional panel |
| `frontend/web3-ui/` | Public | Web3 UI scaffold |

**Investor data JSON** (`frontend/investor/data/`):  
`genesis_entropy.json`, `mainnet_listing_config.json`, `security_layers_report.json`, `sovereign_protocols_report.json`, `sovereign_protocols_manifest.json`, `launch_readiness_report.json`, `mainnet_contract_gates.json`, `listing_compliance_report.json`, `full_pretest_report.json`, `pretest_manifest.json`, `systemic_readiness.json`, `quant_skills_manifest.json`, `quant_skills_table.json`, `nano_details_100.json`

---

## Security stack summary

| Layer | IDs | Zones | Manifest | Latest gate |
|-------|-----|-------|----------|-------------|
| MSA-100 | PT-001–100 | I–IV (layers 1–100) | `security_layers_manifest.json` | 100% documented, PASS |
| Sovereign Perimeter | SP-001–500 | A–T | `sovereign_protocols_manifest.json` | 80% perimeter readiness |
| Atomic Defense | SP-501–600 | U–Y | same manifest, band `atomic` | 100% atomic readiness |
| Blue Code structural | Tasks 201–500 | — | `docs/token/structural_tasks_201_400.md` | CLI `token structural` |
| ATU hard-kernel | 501–600 / p59–p62 | — | `atu_runner/src/phases/` | 165+ ATU scripts |
| VIS identity | N01–N12, N21–N25 | — | `VIS_CLRITY_PROTOCOL_MAP.md` | ZK + gatekeeper ops |

**Audit scripts:**

```bash
bash scripts/audit/verify_security_layers.sh
bash scripts/audit/verify_sovereign_protocols.sh
bash scripts/launch/launch_readiness.sh --continue --skip-foundry
bash scripts/audit/generate_listing_compliance_pack.sh
bash scripts/audit/verify_bridge_connection_hashes.sh
```

---

## Quantum Skills module map

| Component | Path |
|-----------|------|
| Manifest | `CLRTY_SUBSTRATE/boot/quant_skills_manifest.json` |
| Registry + dual-lock | `clrty-cli-core/src/skills/mod.rs` |
| MCA | `clrty-cli-core/src/skills/mca.rs` |
| TSR | `clrty-cli-core/src/skills/tsr.rs` |
| AVR | `clrty-cli-core/src/skills/avr.rs` |
| EHL | `clrty-cli-core/src/skills/ehl.rs` |
| Pipeline | `clrty-cli-core/src/skills/pipeline.rs` |
| Handlers | `clrty-cli-core/src/handlers/skill.rs`, `strategy.rs` |
| CLI entry | `clarity-cli/src/main.rs` — `skill`, `strategy` subcommands |
| Runtime table | `var/trading/quant_skills_table.json` |
| Investor doc | `docs/investor/quantum_skills_trading_suite.md` |

---

## Documentation layers

| Layer | Path | Count |
|-------|------|-------|
| Investor data room | `docs/investor/` | 19 + index + manifest |
| Security | `docs/security/` | MSA-100, Sovereign-600 |
| Compliance | `docs/compliance/` | VIS map, data room, phase 2 |
| Tokenomics | `docs/tokenomics/` | locked params, compliance |
| Launch | `docs/launch/` | GO sequence, traceability |
| Infrastructure | `docs/infrastructure/` | listing, bridge, nodes |
| Protocol | `docs/protocol/` | x402, clarity-skills, agent-protocol |
| Audit | `docs/audit/` | investor summary, completion gates |
| Simulation | `docs/simulation/` | CLRTY Live Market Notion |

**Master indexes:** [`INVESTOR_DATA_ROOM_INDEX.md`](INVESTOR_DATA_ROOM_INDEX.md) · [`DOCUMENTATION_INDEX.md`](../DOCUMENTATION_INDEX.md) · [`REPO_MAP.md`](../architecture/REPO_MAP.md)

---

## Intentionally excluded from git

| Path / pattern | Reason |
|----------------|--------|
| `.env`, `.env.*` | Secrets (except `.env.example`) |
| `var/tmp/` | PDF scratch extracts — not source of truth |
| `var/settlement/` | Runtime settlement state |
| `CLRTY_SUBSTRATE/var_runtime/` | Local runtime artifacts |
| `target/`, `**/target/` | Rust build output |
| `**/node_modules/` | Node dependencies |
| `.cursor/` | Editor local config |

---

## Verification one-liner

```bash
cargo build --workspace && \
bash scripts/audit/verify_sovereign_protocols.sh && \
bash scripts/investor/build_treasury_data.sh && \
cargo test -p clrty-cli-core --quiet
```
