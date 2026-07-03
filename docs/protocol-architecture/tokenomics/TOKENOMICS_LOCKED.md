# Tokenomics — Finalized and Locked

**Chain:** `clrty-1` | **Denom:** `uclrty` | **Status:** Parameters frozen in-repo; board sign-off pending

## Frozen parameters

| Parameter | Value | Source |
|-----------|-------|--------|
| Total supply | 16,000,000 CLRTY | `boot/genesis_entropy.json` |
| Decimals | 9 | genesis + `global_manifold_state.rs` |
| Mint authority | `null` (immutable) | genesis |
| Freeze authority | `null` (immutable) | genesis |

## Genesis allocations

| Bucket | CLRTY | % |
|--------|-------|---|
| Treasury | 4,000,000 | 25% |
| Validators | 3,000,000 | 18.75% |
| Liquidity | 4,000,000 | 25% |
| Ecosystem | 3,000,000 | 18.75% |
| Public | 2,000,000 | 12.5% |

Distribution tier detail: [distribution_tiers.md](distribution_tiers.md).

## Integrity checksum

Computed by `economic_engine/tokenomics/supply_checksum.rs`:

```
allocation_checksum = sha256(total_supply.to_le_bytes() || denom.as_bytes())
```

**Committed hash:** `df3f767fecd60974c517d954ed0e28b92728c21b507dc54139025f09075f2e61`

Manifest: [`CLRTY_SUBSTRATE/boot/tokenomics_manifest.json`](../CLRTY_SUBSTRATE/boot/tokenomics_manifest.json)

Verify:

```bash
cargo run -p clarity-cli -- chain genesis-verify
```

## Sign-off (external — board to complete)

| Field | Value |
|-------|-------|
| Name | _pending_ |
| Role | _pending_ |
| Date | _pending_ |
| Board resolution ref | _pending_ |

After sign-off, set `locked_at` and `sign_off` fields in `tokenomics_manifest.json` and run genesis seal ceremony on production state.

## Post-lock changes

Require governance snapshot vote (Task 51) + 48h timelock (Task 49). No supply inflation — only fee redistribution and burn per adaptive tokenomics phases.
