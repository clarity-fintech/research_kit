# CLRTY-1 Only Launch Scope

**Decision:** CLRTY L1 (`clrty-1` / `uclrty`) is the **sole authoritative ledger** at launch. All supply, allocations, staking, governance, and settlement commit to this chain only.

**Spec:** [clrty-1.md](clrty-1.md) · **Bridge deferral:** [DEFERRED_BRIDGE.md](../l1_launch/DEFERRED_BRIDGE.md)

---

## In scope (launch)

| Area | Deliverable |
|------|-------------|
| **L1 consensus** | PoC engine, entropy sink, 50 ms topology gate |
| **Genesis** | `genesis_entropy.json` — 16M cap, null mint |
| **Token core** | Native `uclrty`, extensions (compliance, delegation, metadata) |
| **Settlement** | HELIX L0.5 → canonical L1 commit |
| **RPC** | `CLRTY_L1_RPC` — clarityd + clrty-api |
| **Validators** | Singularity set + sentry topology |
| **Security** | MDA, MSD-100, MSA-100 pretest gates |
| **Clarity Fortress** | dev.clrty.io/labs twelve-step funnel |
| **Wallets** | Moniversion + chainlist metadata (chainId 1202) |
| **Listing prep** | CEX pack, metadata, compliance reports (no bridge dependency) |

---

## Out of scope (deferred Phase 10)

| Area | Inventory path | Notes |
|------|----------------|-------|
| LayerZero OFTv2 | `bridge_perimeter/layerzero_oft/` | Reference mesh only |
| Wormhole NTT | `bridge_perimeter/wormhole_ntt/` | Global Accountant |
| FMA EVM contracts | `bridge_perimeter/fma/contracts/` | Foundry tests in CI |
| Solana Anchor programs | `bridge_perimeter/programs/` | Compile check only |
| Supply harmonizer | `fma/supply_harmonizer.rs` | Cross-chain cap sync |
| FMA relayer | `fma-relayer/` | Producer mode dry-run |
| External RPC | `ETH_RPC`, `BASE_RPC`, etc. | Alchemy CLI = monitor only |
| Omnichain indexer | Multi-chain workers | L1 indexer only at launch |

---

## Authority rules

1. **Supply truth** — `genesis_entropy.json` + `supply_checksum.rs` on `clrty-1` only.
2. **Bridge status** — `clrty bridge status` reports **deferred**; connection hashes are audit artifacts, not live paths.
3. **Clarity Fortress funnel** — Step 11 (Cross-chain) is **telemetry/education** via Alchemy CLI; execution remains on L1.
4. **Listing** — CEX DDQ cites L1 genesis; bridge mirrors disclosed as Phase 10 roadmap item.

---

## Environment isolation

| File | Purpose |
|------|---------|
| `.env` | Settlement + compliance secrets |
| `.env.l1` | L1 RPC + validator keys (gitignored) |
| `.env.example` | Template — copy vars, never commit values |

```bash
cp .env.example .env.l1
# Set CLRTY_L1_RPC to production endpoint
```

---

## Activation criteria (Phase 10)

Before enabling omnichain:

- [ ] CLRTY L1 mainnet live ≥ 30 days
- [ ] Gate 4 legal clearance (bridge registration)
- [ ] Supply oracle wired to live harmonizer
- [ ] External audit Gates 1–5 complete
- [ ] Board sign-off on bridge cap policy

---

## Verification

```bash
cargo run -p clarity-cli -- bridge status --plain   # expect deferred
cargo run -p clarity-cli -- node genesis-verify --plain
bash scripts/labs/verify_labs_smoke.sh
```
