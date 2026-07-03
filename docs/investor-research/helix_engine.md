# HELIX Engine — Investor Brief

**Product:** P01 · **Category:** I — Execution · **Status:** partial (repo-backed)

HELIX is the hidden exchange layer (L0.5) between operator intents and clrty-1 canonical state. It routes capital, commerce, and Mirra yield flows through a deterministic shadow book before net settlement commits to `state_manifold`.

## Why it matters

| Capability | Investor relevance |
|------------|-------------------|
| Intent routing (HELIX-02) | Commerce (CortexPay) and capital flows share one execution mesh |
| Net settlement (HELIX-03) | Bilateral flows net before on-chain commit — lower gas, lower leakage |
| Arb mesh (HELIX-05) | Cross-venue spread scan via `arbitrage_core` — ties to Quantum Skills MCA |
| Kernel tick loop (HELIX-10) | Continuous shadow state — treasury `#helixPanel` on launch dashboard |

## Module map (HELIX-01..10)

Full table: [`helix_engine_architecture.md`](helix_engine_architecture.md)

## Verification

```bash
bash scripts/audit/verify_helix_components.sh
clrty execute 2601
clrty helix status
```

Reports: `var/compliance/helix_components_report.json` · Runtime: `var/trading/helix_runtime_table.json`

## Cross-links

- Protocol: [`docs/protocol/helix_hidden_exchange_layer.md`](../protocol/helix_hidden_exchange_layer.md)
- Product page: [`frontend/products/suite/category-i-execution/helix-engine.html`](../../frontend/products/suite/category-i-execution/helix-engine.html)
- Manifest: `CLRTY_SUBSTRATE/boot/helix_manifest.json`
- ATU band: **2601–2610** in [`docs/atu_master_ledger.md`](../atu_master_ledger.md)
- Skills: `helix-intent-resolve`, `helix-net-settle`, `helix-route` in `quant_skills_manifest.json`
