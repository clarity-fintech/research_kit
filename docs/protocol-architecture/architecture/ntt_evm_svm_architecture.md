# NTT EVM/SVM Architecture (S04)

## Primary bridge: Wormhole NTT

| Layer | EVM | SVM |
|-------|-----|-----|
| Manager | Wormhole `NttManager` | `clrty_ntt_bridge` program |
| Events | `TransferSent`, `TransferReceived` | Same canonical names |
| Rate limit | `outbound_rate_limit` mapping | `BridgeState.queued_amount` |
| Cap | 16M × 10^9 base units | `GLOBAL_HARD_CAP` |

## Execution Gate (off-chain)
`quant_stack/fma/execution_gate.rs` + `fma-relayer`:
- Poll CEX/DEX depth every 250ms
- `should_queue=true` when slippage > dynamic threshold (default 0.05%)
- HuggingFace weights adjust threshold daily

## Supply accounting
All mints route through `supply_harmonizer.rs` + `supply_oracle.rs`.

## Fallback
LayerZero OFTv2 secondary path — same harmonizer ledger.
