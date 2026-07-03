# HELIX Engine Architecture

Hidden exchange layer — HELIX-01..10 modules in `helix_engine/`.

## Modules

| ID | Module | Role |
|----|--------|------|
| HELIX-01 | matching_grid | MIRRA book port |
| HELIX-02 | intent_resolver | Commerce/capital intent routing |
| HELIX-03 | net_settlement | Net flow settlement |
| HELIX-04 | synthetic_pairs | Synthetic pair registry |
| HELIX-05 | arb_mesh | Spread scan via arbitrage_core |
| HELIX-06 | liquidity_graph | BFS routing graph |
| HELIX-07 | encrypted_tunnels | Tunnel registry scaffold |
| HELIX-08 | predictive_oracle | Heuristic oracle |
| HELIX-09 | imm_core | IMM spreads + shadow persistence |
| HELIX-10 | kernel | Continuous tick loop (`helixd`) |

## Integration

- CLI: `clrty helix run|status|intents|net|halt`
- API: `/v1/helix/*`
- Skills: `helix-intent-resolve`, `helix-net-settle`, `helix-route`
- Shadow state: `var/helix/`
- Manifest: `CLRTY_SUBSTRATE/boot/helix_manifest.json`
- Substrate bridge: `helix_engine/substrate_bridge.rs` → `economic_core`, `deterministic_committer`
- Sovereign cross-links: SP-276, SP-561, MSA-051

Verify: `scripts/audit/verify_helix_components.sh`
