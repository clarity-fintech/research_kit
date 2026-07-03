# Investor Data Room — Master Index

**Repository:** [theangelofwill/-CLRTY](https://github.com/theangelofwill/-CLRTY)  
**Status date:** 2026-06-19  
**Latest commit (at index build):** `94143e65eb5adb59f56e5154989b8fb88e1a171e`  
**Full project manifest:** [`FULL_PROJECT_MANIFEST.md`](FULL_PROJECT_MANIFEST.md)

Central navigation for all **20** investor-grade documents in `docs/investor/`, linked machine-readable artifacts, verification commands, and Notion workspace pages.

---

## Six core due-diligence documents

Start here for institutional technical, economic, security, regulatory, and operational review.

| # | Document | Purpose | Key metrics (from repo) |
|---|----------|---------|-------------------------|
| 1 | [`technical_due_diligence.md`](technical_due_diligence.md) | **Technical DD** — MVM, PoC/CCR, substrate modules, compliance-as-code, CLARITY Skills wiring | 16 workspace crates · L1 `clrty-1` · 100-task pretest |
| 2 | [`tokenomics_model.md`](tokenomics_model.md) | **Tokenomics** — 16M supply, vesting, burn, treasury buckets, MIRRA yield-recycling | Supply cap: **16,000,000 CLRTY** (9 decimals) · `genesis_entropy.json` |
| 3 | [`security_audit_report.md`](security_audit_report.md) | **Security audit synthesis** — MSA-100 + Sovereign-600 credibility and risk mitigation | MSA-100: **100/100 documented** · Sovereign-600: **600 protocols** |
| 4 | [`regulatory_opinion_memo.md`](regulatory_opinion_memo.md) | **Regulatory memo** — Howey Test utility analysis (internal, not legal advice) | Phase 2 compliance data room cross-links |
| 5 | [`roadmap_milestone_tracker.md`](roadmap_milestone_tracker.md) | **Roadmap & milestones** — TGE, MIRRA, GO gates, ATU/pretest, launch readiness | Launch readiness: **92.3%** · `launch_ready: true` |
| 6 | [`quantum_skills_trading_suite.md`](quantum_skills_trading_suite.md) | **Quantum Skills CLI** — MCA, TSR, AVR, EHL institutional trading augmentation | 4 skills · manifest v1 · sequential pipeline only |
| 7 | [`clarity_skills_overview.md`](clarity_skills_overview.md) | **CLARITY Skills execution layer** — evolution, CLI, architecture, operator runbook, investor narrative | 4 Quantum Skills · dual-lock · HUD sync |

---

## All 20 investor documents

| Document | Category | Purpose |
|----------|----------|---------|
| [`technical_due_diligence.md`](technical_due_diligence.md) | Core DD | Living technical deep-dive — whitepaper, PPM, Moniverse theory, repo implementation |
| [`tokenomics_model.md`](tokenomics_model.md) | Core DD | Economic transparency — supply, vesting, burn, treasury, MIRRA |
| [`security_audit_report.md`](security_audit_report.md) | Core DD | MSA-100 + Sovereign-600 audit synthesis for data room |
| [`regulatory_opinion_memo.md`](regulatory_opinion_memo.md) | Core DD | Howey Test utility analysis (internal — not legal advice) |
| [`roadmap_milestone_tracker.md`](roadmap_milestone_tracker.md) | Core DD | Operational milestone tracker — TGE, MIRRA, GO gates, external blockers |
| [`quantum_skills_trading_suite.md`](quantum_skills_trading_suite.md) | Core DD | 4 Quantum Skills — MCA, TSR, AVR, EHL — CLI, tables, dashboards |
| [`clarity_skills_overview.md`](clarity_skills_overview.md) | Core DD | CLARITY Skills execution layer — investors, operators, developers |
| [`FIRST_ACCESS_PACK.md`](FIRST_ACCESS_PACK.md) | **Exclusive** | Volkov Mastermind deployment kit — Private Suite + Terminal Vector |
| [`TIER2_TECH_SPEC.md`](TIER2_TECH_SPEC.md) | **Exclusive** | Tier 2 data primitives index for searchers / allocators |
| [`FIRST_ACCESS_ARCHITECTURE.md`](FIRST_ACCESS_ARCHITECTURE.md) | **Exclusive** | MLX · RAG · ZK · CCIP integration (honest status) |
| [`genesis_participation_protocol.md`](genesis_participation_protocol.md) | Genesis / settlement | Weighted allocation on `clrty-1` before mainnet public launch |
| [`settlement_gatekeeper.md`](settlement_gatekeeper.md) | Genesis / settlement | Programmable settlement layer — attestation, Safe treasury, gatekeeper v1 |
| [`treasury_transparency_dashboard.md`](treasury_transparency_dashboard.md) | Treasury / HUD | 100% asset map · Visual-Depth L1–L2 · dashboard sync |
| [`hyperion_hud.md`](hyperion_hud.md) | Treasury / HUD | Hyperion Borderlands investor HUD — industrial command interface spec |
| [`website_and_investor_portal.md`](website_and_investor_portal.md) | Treasury / HUD | Public website + private investor portal — architecture, gate, data pipeline, security |
| [`moniverse_economic_engine.md`](moniverse_economic_engine.md) | Positioning | Autonetic Moniversion as value-accrual physics |
| [`abstraction_logic_curriculum.md`](abstraction_logic_curriculum.md) | Positioning | Visual-depth class (L0→L3) — circulatory-system mental model |
| [`cognitive_edge_training.md`](cognitive_edge_training.md) | Positioning | Institutional Decision Intelligence mini-class (NDA-tier) |
| [`register_binding_visualization.md`](register_binding_visualization.md) | Positioning | Wallet → L3 node graph visualization |
| [`chain_shift_reporting_template.md`](chain_shift_reporting_template.md) | Reporting | Monthly Chain-Shift brief template (SIM100 + live API) |
| [`clarity_visual_protocol.md`](clarity_visual_protocol.md) | Reporting | CVP — institutional reporting aesthetic (canonical) |
| [`comms_doctrine.md`](comms_doctrine.md) | Reporting | Structural vs promotional comms doctrine |
| [`signal_normalization.md`](signal_normalization.md) | Reporting | High-impact signal allowlist — remove noisy investor data |

**Supporting hub:** [`docs/investor_kit.md`](../investor_kit.md) §0 · §16b  
**Compliance data room:** [`docs/compliance/data_room/INDEX.md`](../compliance/data_room/INDEX.md)

---

## Performance & ROI — CLRTY DATA CENTER

Primary investor metrics surface (Notion) with Google Sheets as source of truth for founder/OpEx inputs.

**B2B infrastructure:** [`../monetization/API_FIRST_B2B_INFRASTRUCTURE.md`](../monetization/API_FIRST_B2B_INFRASTRUCTURE.md) · [`../monetization/MONETIZATION_LAYERS.md`](../monetization/MONETIZATION_LAYERS.md)

| Resource | Path |
|----------|------|
| **ROI template (repo)** | [`performance_roi_tracking_template.md`](performance_roi_tracking_template.md) |
| **Sheets setup** | [`data_center_sheets_template.md`](data_center_sheets_template.md) |
| Metric manifest | `CLRTY_SUBSTRATE/boot/data_center_manifest.json` |
| Live API | `GET /v1/listing/metrics` · `GET /v1/data-center/snapshot` |
| Sync | `make data-center-sync` |

```bash
make data-center-sync
curl -s https://api.clarity-fintech.com/v1/data-center/snapshot | jq '.featured_group,.scaling_status'
```

---

## CLARITY Product Suite (web)

Public product catalog — 13 systems, build programs, commerce layer. Comparative architecture on **clrty-1** (not Solana mainnet).

| Resource | Path |
|----------|------|
| **Products hub** | [`frontend/products/index.html`](../../frontend/products/index.html) |
| Product suite spec | [`docs/products/CLARITY_PRODUCT_SUITE.md`](../products/CLARITY_PRODUCT_SUITE.md) |
| Commerce layer | [`docs/products/COMMERCE_LAYER.md`](../products/COMMERCE_LAYER.md) |
| NeuroStable (NSD) | [`docs/products/NEUROSTABLE_NSD.md`](../products/NEUROSTABLE_NSD.md) |
| HELIX (P01) | [`docs/investor/helix_engine.md`](../investor/helix_engine.md) · [`docs/protocol/helix_hidden_exchange_layer.md`](../protocol/helix_hidden_exchange_layer.md) |
| Machine catalog | `CLRTY_SUBSTRATE/boot/products_suite_manifest.json` |

**Status honesty:** badges on each page — **partial** where repo code exists (HELIX, CortexPay, Alpha Engine, API/SDK); **planned** for DeSci, full PoI/PoR economics.

---

## Security architecture cross-links

| Layer | Scope | Manifest | Latest audit report |
|-------|-------|----------|---------------------|
| MSA-100 | PT-001–100, Zones I–IV | `CLRTY_SUBSTRATE/boot/security_layers_manifest.json` | `var/compliance/security_layers_report.json` |
| Sovereign Perimeter | SP-001–500, Zones A–T | `CLRTY_SUBSTRATE/boot/sovereign_protocols_manifest.json` | `var/compliance/sovereign_protocols_report.json` |
| Atomic Defense | SP-501–600, Zones U–Y | same manifest, band `atomic` | same report |

**Architecture docs:** [`SOVEREIGN_600_ARCHITECTURE.md`](../security/SOVEREIGN_600_ARCHITECTURE.md) · [`MASS_SECURITY_ARCHITECTURE.md`](../security/MASS_SECURITY_ARCHITECTURE.md) · [`INVESTOR_SECURITY_SUMMARY.md`](../audit/INVESTOR_SECURITY_SUMMARY.md)

**Latest audit snapshot (2026-06-19):**

| Report | Total | Implemented | Partial | Documented % | Gate |
|--------|-------|-------------|---------|--------------|------|
| MSA-100 | 100 layers | 74 | 26 | **100.0%** | PASS |
| Sovereign-600 | 600 protocols | 65 | 435 | **83.3%** | PASS |
| Atomic band (501–600) | 100 | — | — | **100.0%** readiness | PASS |

---

## Quantum Skills — CLI & paths

**Overview guide:** [`clarity_skills_overview.md`](clarity_skills_overview.md) — execution layer narrative, architecture, runbook  
**Manifest:** `CLRTY_SUBSTRATE/boot/quant_skills_manifest.json`  
**Runtime table:** `var/trading/quant_skills_table.json`  
**Investor HUD copy:** `frontend/investor/data/quant_skills_manifest.json`, `quant_skills_table.json`

| Skill ID | Name | Tag | Rust module |
|----------|------|-----|-------------|
| `metric-collapse-arbitrage` | Metric-Collapse Arbitrage (MCA) | quant | `clrty-cli-core/src/skills/mca.rs` |
| `topological-state-rebalance` | Topological State-Rebalancing (TSR) | quant | `clrty-cli-core/src/skills/tsr.rs` |
| `attestation-verify` | Attestation-Verified Routing (AVR) | compliance | `clrty-cli-core/src/skills/avr.rs` |
| `entropy-heartbeat-check` | Entropy-Heartbeat Liquidation (EHL) | risk | `clrty-cli-core/src/skills/ehl.rs` |

**CLI commands** (`clarity-cli` / `clrty` binary):

```bash
cargo build --workspace
cargo run -p clarity-cli -- skill run metric-collapse-arbitrage --account=0x... --capital=1000000 --risk-mode=standard
cargo run -p clarity-cli -- skill status --account=0x...
cargo run -p clarity-cli -- skill halt --account=0x...
cargo run -p clarity-cli -- strategy run --steps=mca,tsr,avr,ehl --capital=1000000 --risk-profile=standard
cargo test -p clrty-cli-core
```

**Gates (manifest):** max 3 concurrent per IP · one skill per account · sequential pipeline only.

---

## Machine-readable artifacts

| File | Purpose |
|------|---------|
| `CLRTY_SUBSTRATE/boot/genesis_entropy.json` | Genesis supply + entropy seal |
| `CLRTY_SUBSTRATE/boot/baseline_metrics/genesis_tier_allocations.json` | Sets matrix / tier allocations |
| `CLRTY_SUBSTRATE/boot/tokenomics_manifest.json` | Locked tokenomics parameters |
| `CLRTY_SUBSTRATE/boot/data_center_manifest.json` | Performance & ROI metric catalog (22 metrics) |
| `CLRTY_SUBSTRATE/boot/mainnet_listing_config.json` | CEX lock-ups / vesting |
| `CLRTY_SUBSTRATE/boot/pretest_manifest.json` | 100-task systemic pretest map |
| `CLRTY_SUBSTRATE/bridge_perimeter/connection_registry.json` | Bridge connection hashes |
| `var/compliance/listing_compliance_report.json` | Task 38 listing pack |
| `var/compliance/bridge_connection_audit_report.json` | INF-25 bridge audit |
| `var/launch/launch_readiness_report.json` | All-in-one launch battery |
| `var/launch/mainnet_contract_gates.json` | Mainnet contract GO gates |
| `var/pretest/systemic_readiness.json` | Systemic pretest readiness |
| `var/pretest/full_pretest_report.json` | 100-task pretest output |
| `frontend/investor/data/*.json` | Investor HUD sync (via `build_treasury_data.sh`) |

---

## Verification commands

Run from repository root after `cargo build --workspace`:

```bash
# Workspace build
cargo build --workspace

# Genesis + chain
cargo run -p clarity-cli -- chain genesis-verify
cargo run -p clarity-cli -- chain status

# Security audits → JSON reports
bash scripts/audit/verify_security_layers.sh      # → var/compliance/security_layers_report.json
bash scripts/audit/verify_sovereign_protocols.sh  # → var/compliance/sovereign_protocols_report.json

# Launch readiness (92.3% readiness_pct, launch_ready: true as of 2026-06-19)
bash scripts/launch/launch_readiness.sh --continue --skip-foundry

# Full validation battery
bash scripts/test/full_validation.sh --skip-foundry
bash scripts/test/full_pretest.sh --continue --skip-foundry

# Listing + bridge compliance
bash scripts/audit/generate_listing_compliance_pack.sh
bash scripts/audit/verify_bridge_connection_hashes.sh

# Investor dashboard JSON sync
bash scripts/investor/build_treasury_data.sh

# Supply integrity
cargo test -p clrty-substrate supply_checksum
cargo test -p clrty-substrate settlement listing_config

# Quantum Skills unit tests
cargo test -p clrty-cli-core
```

**Quick smoke:** `bash scripts/verify_launch.sh`

---

## Frontend investor surfaces

**Canonical guide:** [`website_and_investor_portal.md`](website_and_investor_portal.md) — public hub, gate model, investor pages, `build_treasury_data.sh`, security posture.

| Path | Audience | Notes |
|------|----------|-------|
| `frontend/index.html` | Public | Products hub, live chart, section navigation |
| `frontend/portal/gate.html` | Gate | Investor Access — sessionStorage tier gate |
| `frontend/investor/clrty-graph.html` | Private | Command Nexus — Q1–Q4 HUD + 100-node graph |
| `frontend/investor/nexus.html` | Private | Redirect → Command Nexus |
| `frontend/investor/investor-hub.html` | Private | 100 nano clarity list + funnel bar |
| `frontend/investor/treasury-dashboard.html` | Private | Launch readiness, Sovereign 600, MSA, quantum skills |
| `frontend/investor/treasury-exclusive.html` | Private | Exclusive treasury + funding panels |
| `frontend/portal/index.html` | Private | Investor portal dashboard |
| `frontend/docs/content/business/investor-portal-guide.md` | Public docs portal | Stub linking to canonical guide |
| `frontend/docs/content/validation/security-audit-report.md` | Public docs portal | Security audit (web) |
| `frontend/docs/content/validation/roadmap-milestone-tracker.md` | Public docs portal | Roadmap (web) |
| `frontend/docs/content/business/tokenomics-model.md` | Public docs portal | Tokenomics (web) |

Local preview: `cd frontend && python3 -m http.server 8080` → `http://localhost:8080/portal/gate.html`

---

## Notion workspace links

| Page | URL | Repo mirror |
|------|-----|-------------|
| CLRTY — Institutional Vision | [Notion](https://app.notion.com/p/CLRTY-383704760d248039950eef8816181040) | [`technical_due_diligence.md`](technical_due_diligence.md) |
| Early Access to CLRITY | [Notion](https://app.notion.com/p/Early-Access-to-CLRITY-37f704760d24800298b3ede45d52ce4d) | [`genesis_participation_protocol.md`](genesis_participation_protocol.md) |
| MVM / PoC synthesis | [Notion](https://app.notion.com/p/37f704760d24800298b3ede45d52ce4d) | [`technical_due_diligence.md`](technical_due_diligence.md) §2 |
| Launch strategy (554-day) | — | [`CLRTY_Live_Market_Notion.md`](../simulation/CLRTY_Live_Market_Notion.md) |

Import launch strategy markdown into Notion: [`docs/simulation/CLRTY_Live_Market_Notion.md`](../simulation/CLRTY_Live_Market_Notion.md) (Notion-compatible + chart embeds).

---

## Related documentation index

- **Repository map:** [`docs/architecture/REPO_MAP.md`](../architecture/REPO_MAP.md)
- **Full manifest:** [`FULL_PROJECT_MANIFEST.md`](FULL_PROJECT_MANIFEST.md)
- **Documentation index:** [`docs/DOCUMENTATION_INDEX.md`](../DOCUMENTATION_INDEX.md)
- **Platform surfaces:** [`docs/protocol/PLATFORM_SURFACE_MAP.md`](../protocol/PLATFORM_SURFACE_MAP.md)
- **Whitepaper:** [`docs/whitepaper.md`](../whitepaper.md)
