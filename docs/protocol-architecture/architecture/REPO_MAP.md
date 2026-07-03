# CLRTY Repository Map

Canonical index of workspace layout, security architecture, surface routing, and investor data room.

**Nexus architecture:** [`NEXUS_REPOSITORY.md`](NEXUS_REPOSITORY.md) · **Launch stages:** [`../launch/LAUNCH_STAGES.md`](../launch/LAUNCH_STAGES.md) · **100 steps:** [`../launch/NANO_ORGANIZATION_100.md`](../launch/NANO_ORGANIZATION_100.md)

**Orchestration:** `make help` from repo root · Manifest index: [`manifests/MANIFEST_INDEX.json`](../../manifests/MANIFEST_INDEX.json)

**Latest commit reference:** `94143e65` (2026-06-19) — investor data room index + manifest enrichment  
**Investor master index:** [`docs/investor/INVESTOR_DATA_ROOM_INDEX.md`](../investor/INVESTOR_DATA_ROOM_INDEX.md)  
**Full manifest:** [`docs/investor/FULL_PROJECT_MANIFEST.md`](../investor/FULL_PROJECT_MANIFEST.md)

## Workspace crates

| Crate | Role | Binaries |
|-------|------|----------|
| `CLRTY_SUBSTRATE` | L1 substrate (PoC, MVM, CCR, settlement, token_core) | `clarityd`, `clrty-gatekeeper`, `l-dnet-stress` |
| `clarity-cli` | Operator CLI entry | `clrty` |
| `clrty-cli-core` | CLI handlers, pipeline, registry | — |
| `clrty-cli-ui` | TUI layouts (client / operator) | — |
| `clrty-api` | REST + WS operator API | `clrty-api` (:8545) |
| `clrty-signal-bridge` | Signal validation proxy | — |
| `arbitrage_core` | Producer / arb loop engine | — |
| `pretest_runner` | 100-task systemic pretest | `pretest_runner` |
| `atu_runner` | ATU verification phases | `atu_runner` |
| `fma-relayer` | FMA bridge relayer | `fma-relayer` |
| `quant_stack` | Off-chain quant / FMA execution | — |
| `helix_engine` | HELIX L0.5 hidden exchange (HELIX-01..10) | `helixd` |
| `cortexpay_engine` | CortexPay commerce orchestration (CORTEX-01..08) | — |
| `neuro_templates_engine` | NeuroTemplates adaptive scaffolds (NT-01..08) | — |
| `backlog` | Pre-launch holder capture | — |
| `simulators/*` | Tokenomics, state_space ABM, clrty_mirror | various |

## CLRTY_SUBSTRATE modules

```
boot/              genesis, listing, pretest, security_layers, sovereign manifests
settlement/        gatekeeper, attestation, safe monitor, capital flight
token_core/        layout_v2, blue_code, merkle, verify
state_manifold/    state_root, genesis_seal, worm_audit, transaction_log
h2tb/              hardware-to-token bridge
l_dnet/            stress runner
poc_consensus/     PoC engine
economic_engine/   tokenomics, supply_checksum
treasury_sink/     vesting escrow
bridge_perimeter/  FMA, multisig, connection registry
```

## Security architecture

| Layer | IDs | Manifest |
|-------|-----|----------|
| MSA-100 | PT-001–100, Layers 1–100 | `boot/security_layers_manifest.json` |
| Sovereign Perimeter | SP-001–500, Zones A–T | `boot/sovereign_protocols_manifest.json` |
| Atomic Defense | SP-501–600, Zones U–Y | same manifest, band `atomic` |
| Blue Code structural | Tasks 201–500 | `docs/token/structural_tasks_201_400.md` |
| ATU hard-kernel | 501–600 / p59–p62 | `atu_runner/src/phases/` |

Audit scripts:

- `scripts/audit/verify_security_layers.sh` → `var/compliance/security_layers_report.json`
- `scripts/audit/verify_sovereign_protocols.sh` → `var/compliance/sovereign_protocols_report.json`
- `scripts/audit/verify_helix_components.sh` → `var/compliance/helix_components_report.json`
- `scripts/launch/verify_mainnet_contract_gates.sh` → `var/launch/mainnet_contract_gates.json`
- `scripts/launch/launch_readiness.sh` → all-in-one battery

## Frontend surfaces

| Path | Audience | Notes |
|------|----------|-------|
| `frontend/index.html` | Public | Legacy product grid — **not** suite hub (deferred) |
| `frontend/products/`, `frontend/skills/` | Integration preview | Built; `noindex` until [`public_website_integration.json`](../../CLRTY_SUBSTRATE/boot/public_website_integration.json) gate |
| `frontend/docs/` | Public | Documentation portal |
| `frontend/portal/gate.html` | Private | Investor access gate |
| `frontend/investor/` | Private | Treasury, graph HUD, Sovereign 600 panel |
| `frontend/network/` | Public | Network hub |
| `frontend/products/`, `tools/` | Public | Product scaffolds |

Sync investor JSON: `bash scripts/investor/build_treasury_data.sh`

## Scripts index

| Directory | Purpose |
|-----------|---------|
| `scripts/launch/` | Launch readiness, mainnet gates |
| `scripts/audit/` | MSA, sovereign, listing compliance |
| `scripts/atu/` | ATU 701–990 verification |
| `scripts/investor/` | Treasury data, nano details, sovereign generator |
| `scripts/test/` | full_pretest, full_validation |
| `scripts/stress/` | Concurrency, break-it |

## var/ outputs (committed)

Reproducible validation artifacts for dashboards and CI:

- `var/compliance/` — security_layers, sovereign_protocols, listing_compliance, helix_components
- `var/launch/` — launch_readiness, mainnet_contract_gates
- `var/trading/` — quant_skills_table, helix_runtime_table, nsd_confidence_index
- `var/pretest/` — systemic_readiness, full_pretest_report
- `var/ldnet/` — stress audit binaries

See [DOCUMENTATION_INDEX.md](../DOCUMENTATION_INDEX.md) and [protocol/PLATFORM_SURFACE_MAP.md](../protocol/PLATFORM_SURFACE_MAP.md).

---

## Investor data room layer

**18 documents** in `docs/investor/` — indexed in [`INVESTOR_DATA_ROOM_INDEX.md`](../investor/INVESTOR_DATA_ROOM_INDEX.md).

| Tier | Documents |
|------|-----------|
| Core DD (6) | `technical_due_diligence.md`, `tokenomics_model.md`, `security_audit_report.md`, `regulatory_opinion_memo.md`, `roadmap_milestone_tracker.md`, `quantum_skills_trading_suite.md` |
| Genesis / settlement | `genesis_participation_protocol.md`, `settlement_gatekeeper.md` |
| Treasury / HUD | `treasury_transparency_dashboard.md`, `hyperion_hud.md` |
| Positioning pack | `moniverse_economic_engine.md`, `abstraction_logic_curriculum.md`, `cognitive_edge_training.md`, `register_binding_visualization.md` |
| Reporting | `chain_shift_reporting_template.md`, `clarity_visual_protocol.md`, `comms_doctrine.md`, `signal_normalization.md` |

**Machine-readable sync:** `bash scripts/investor/build_treasury_data.sh` → `frontend/investor/data/*.json`

**Latest audit snapshot (2026-06-19):**

| Report | Path | Gate |
|------|------|------|
| MSA-100 | `var/compliance/security_layers_report.json` | 100% documented, PASS |
| Sovereign-600 | `var/compliance/sovereign_protocols_report.json` | 600 protocols, PASS |
| Launch readiness | `var/launch/launch_readiness_report.json` | 92.3% readiness, `launch_ready: true` |

---

## Quantum Skills paths

| Component | Path |
|-----------|------|
| Manifest | `CLRTY_SUBSTRATE/boot/quant_skills_manifest.json` |
| Runtime table | `var/trading/quant_skills_table.json` |
| HELIX runtime table | `var/trading/helix_runtime_table.json` |
| Investor doc | `docs/investor/quantum_skills_trading_suite.md` |
| HELIX investor doc | `docs/investor/helix_engine.md` |
| Registry | `clrty-cli-core/src/skills/mod.rs` |
| Skill modules | `clrty-cli-core/src/skills/{mca,tsr,avr,ehl}.rs` |
| Pipeline | `clrty-cli-core/src/skills/pipeline.rs` |
| CLI | `clarity-cli/src/main.rs` — `skill run|status|halt`, `strategy run` |
| HUD copy | `frontend/investor/data/quant_skills_{manifest,table}.json` |

```bash
cargo run -p clarity-cli -- skill run metric-collapse-arbitrage --account=0x... --capital=1000000
cargo run -p clarity-cli -- strategy run --steps=mca,tsr,avr,ehl --capital=1000000
cargo test -p clrty-cli-core
```
