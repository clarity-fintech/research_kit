# Producer Engine

**Full guide:** [`arbitrage_program_guide.md`](arbitrage_program_guide.md) — architecture, CLI, Quantum Skills (MCA), dual-lock, runbook, and implementation status.

The `arbitrage_core` crate implements the producer-side loop:

- **dead_man** — ATU 800 heartbeat; triggers bridge pause
- **detect** — spread scan via `quant_stack`
- **route** — venue routing scores
- **risk** — position limits tied to signal bridge `MAX_TRADE_SIZE`
- **bridge_pause** — links dead-man to global pause flag

Run via FMA relayer producer mode:

```bash
fma-relayer --mode producer --signal-bridge ./signal-bridge.sock --dry-run
```
