# Live Spot Orders + Full Market Model

Model-driven MIRRA spot book: place-all resting liquidity, directional pump clips, and keep-alive replenishment.

## One command — pull everything into the repo

```bash
make live-market-pull
```

This runs SIM100, builds the full market model, places all live spot orders, replenishes 3 keep-alive ticks, and syncs artifacts to:

- `var/trading/full_market_model.json`
- `var/trading/live_spot_orders.json`
- `var/helix/order_grid.json`
- `var/helix/active_orders.json`
- `frontend/investor/data/` (dashboard copies)

## CLI

```bash
clrty helix market pull
clrty helix market model --shadow-dir=var/helix
clrty helix orders place-all --shadow-dir=var/helix
clrty helix orders keep-alive --shadow-dir=var/helix --ticks=5
clrty helix orders status --shadow-dir=var/helix
```

## Keep orders active (daemon)

```bash
INTERVAL=30 make live-spot-keep-alive
```

## Full market model

`helix_engine/src/market_model.rs` fuses:

- SIM100 merkle prior
- HELIX book mid / spread
- Predictive oracle signal

Output: `direction` (up/down/flat), `target_mid`, `pump_levels` for spot clips.

## Honest scope

Simulation + shadow book only until Phase 10 settlement and external MM custody are live. Pump mechanics follow producer/consumer slippage gates — not unbounded retail extraction.
