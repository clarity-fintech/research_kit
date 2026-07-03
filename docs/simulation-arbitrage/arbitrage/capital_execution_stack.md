# Capital Execution Stack

Fifteen MIRRA capital capabilities (CE-01..CE-15) activated by `scripts/helix/capital_execution_cycle.sh`.

## Registry

Canonical manifest: [`manifests/capital_execution_stack.json`](../../manifests/capital_execution_stack.json)

## Tiers

| Tier | Capabilities | Command |
|------|--------------|---------|
| **core** | CE-01, 02, 03, 04, 09, 11 | `make capital-execution-cycle` |
| **full** | All 15 | `CAPITAL_TIER=full make capital-execution-cycle` |

## CLI

```bash
clrty helix capital cycle --tier=core
clrty sys test morning --tier standard
```

## Artifacts

- `var/trading/capital_execution_report.json`
- `var/trading/yield_aggregation.json` (CE-15 daily compounding node)

## Status classes

`implemented | partial | scaffold | deferred` — see manifest per capability.

Honest scope: scaffold/deferred items do not go live until Phase 10 settlement, external MM, and custody are operational.
