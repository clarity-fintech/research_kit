# HELIX Hidden Exchange Layer

Architecture index — canonical protocol spec lives at [`docs/protocol/helix_hidden_exchange_layer.md`](../protocol/helix_hidden_exchange_layer.md).

## Workspace layout

| Path | Role |
|------|------|
| `helix_engine/` | HELIX-01..10 Rust crate |
| `CLRTY_SUBSTRATE/boot/helix_manifest.json` | Module registry |
| `var/helix/` | Shadow state persistence |
| `var/trading/helix_runtime_table.json` | Runtime module table (HUD sync) |
| `clrty-cli-core/src/handlers/helix.rs` | CLI: `clrty helix run\|status\|intents\|net\|halt` |
| `clrty-api/src/helix.rs` | REST: `/v1/helix/*` |
| `atu_runner/src/phases/p_helix.rs` | ATU gates 2601–2610 |

## Substrate wiring (Phase 3)

| Subsystem | HELIX touchpoint |
|-----------|------------------|
| `economic_core` | Slippage governor in `helix_engine/substrate_bridge.rs` |
| `arbitrage_core` | Spread scan in `helix_engine/arb_mesh.rs` |
| `quant_stack/fma` | Order book snapshots for mesh |
| `state_manifold` | Canonical commit via `helix_engine/canonical_commit.rs` |
| `deterministic_committer` | Block height hook in substrate bridge |
| AVR skill | Institutional lanes require `attestation-verify` before route |

## Audit

```bash
bash scripts/audit/verify_helix_components.sh
```

Investor brief: [`docs/investor/helix_engine.md`](../investor/helix_engine.md)
