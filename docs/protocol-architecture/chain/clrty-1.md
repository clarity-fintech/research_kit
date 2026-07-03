# CLRTY-1 Technical Specification

**Chain ID:** `clrty-1` · **Numeric ID:** `1202` · **Denom:** `uclrty` (9 decimals) · **Status:** L1-only launch authority

**Related:** [CLRTY1_ONLY_SCOPE.md](CLRTY1_ONLY_SCOPE.md) · [clrty-1-fma.md](clrty-1-fma.md) · [wallet-chainlist.md](wallet-chainlist.md) · [validators-sentry.md](validators-sentry.md)

**Machine-readable genesis:** [`CLRTY_SUBSTRATE/boot/genesis_entropy.json`](../../CLRTY_SUBSTRATE/boot/genesis_entropy.json)

---

## 1. Identity

| Field | Value |
|-------|-------|
| Network name | CLRTY L1 |
| Chain ID (string) | `clrty-1` |
| Chain ID (numeric) | `1202` |
| Native denom | `uclrty` |
| Decimals | 9 |
| Total supply cap | 16,000,000 CLRTY |
| Mint authority | `null` (post-genesis) |
| Freeze authority | `null` |
| RPC env var | `CLRTY_L1_RPC` |
| WebSocket env var | `CLRTY_L1_WS` |

At launch, **only** `clrty-1` is authoritative for supply, allocations, and settlement. Omnichain mirrors are deferred — see [DEFERRED_BRIDGE.md](../l1_launch/DEFERRED_BRIDGE.md).

---

## 2. Consensus — Proof of Convergence (PoC)

Validators maximize **E(x) − λR(x)** where:

- **E(x)** — execution efficiency (throughput, inclusion latency, HELIX net settlement quality)
- **R(x)** — structural risk (topology drift, entropy spike, capital flight signals)
- **λ** — adaptive penalty from the entropy sink engine (`entropy_sink_engine/`)

### Block structure

| Component | Module |
|-----------|--------|
| PoC engine | `CLRTY_SUBSTRATE/poc_consensus/` |
| λ heartbeat | `entropy_sink_engine/ccr_orchestrator.rs` |
| Topology gate | 50 ms atmospheric sync window |
| State root | `state_manifold/` |

### Finality model

Deterministic finality under PoC tuple commit — simulated in `simulators/state_space/`. Production P2P networking is an external blocker; see [EXTERNAL_BLOCKERS.md](../l1_launch/EXTERNAL_BLOCKERS.md).

---

## 3. Execution — Manifold VM (MVM)

Parallel execution runtime with CCR Set-aware scheduling:

| Layer | Function |
|-------|----------|
| MVM scheduler | Parallel tx lanes by Set tier |
| EntropyBus | Fee + opcode metering |
| CCR orchestrator | Set 99→1 tier migration on telemetry |
| Token core | Extensions: compliance, delegation, metadata, permissions, privacy |

Set 1 (singularity) receives zero-gas kernel routing per CCR policy.

---

## 4. Genesis & tokenomics

Genesis allocations (from `genesis_entropy.json`):

| Bucket | Amount (CLRTY) |
|--------|----------------|
| Treasury | 4,000,000 |
| Validators | 3,000,000 |
| Liquidity | 4,000,000 |
| Ecosystem | 3,000,000 |
| Public | 2,000,000 |

Locked parameters: [`docs/tokenomics/TOKENOMICS_LOCKED.md`](../tokenomics/TOKENOMICS_LOCKED.md)

Supply checksum oracle: `supply_checksum.rs` — attests circulating, staked, locked, burned.

---

## 5. Settlement stack

```
User Intent → PRISM → Sentinels → HELIX (L0.5) → clrty-1 → NSD / Tokenization
```

| Layer | Spec |
|-------|------|
| HELIX | [helix_hidden_exchange_layer.md](../protocol/helix_hidden_exchange_layer.md) |
| Settlement | `CLRTY_SUBSTRATE/settlement/` |
| Platform router | `settlement/platform_router.rs` |

HELIX nets flows in a shadow book before canonical commit to `state_manifold`.

---

## 6. RPC & API surfaces

### Environment

```bash
CLRTY_L1_RPC=http://127.0.0.1:8545
CLRTY_L1_WS=ws://127.0.0.1:8545/ws
CLRTY_L1_CHAIN_ID=clrty-1
CLRTY_L1_NUMERIC_CHAIN_ID=1202
```

### Binaries

| Binary | Role |
|--------|------|
| `clarityd` | L1 node — `genesis-verify`, `status`, `sim-block` |
| `clrty-api` | REST + JSON-RPC on `:8545` |
| `clarity-cli` | Operator CLI — `clrty chain`, `clrty bridge status` |

### Key routes

| Surface | Route | Purpose |
|---------|-------|---------|
| REST status | `GET /v1/status` | Chain heartbeat |
| Indexer | `GET /v1/indexer/clrty-l1` | Block/event ingest |
| Clarity Fortress walkthrough | `GET /v1/labs/walkthrough` | Builder funnel manifest |
| Clarity Fortress sections | `GET /v1/labs/sections` | SEC-* section index |
| JSON-RPC | `POST /rpc` | `getSlot`, `getHealth`, `simulateTransaction`, `requestAirdrop` |

Provision guide: [l1_rpc_provision.md](../omnichain/l1_rpc_provision.md)

---

## 7. Security architecture

| Doc | Scope |
|-----|-------|
| [CLRTY1_MDA.md](../security/CLRTY1_MDA.md) | Moniversion Defense Architecture |
| [CLRTY1_MSD.md](../security/CLRTY1_MSD.md) | Mass Security Defense — 100 nano tasks |
| [MASS_SECURITY_ARCHITECTURE.md](../security/MASS_SECURITY_ARCHITECTURE.md) | MSA-100 layers |

VIS perimeter overlay: [VIS_CLRITY_PROTOCOL_MAP.md](../compliance/VIS_CLRITY_PROTOCOL_MAP.md)

---

## 8. Validator set

Validator singularity set: `CLRTY_SUBSTRATE/boot/validator_singularity_set.json`

Sentry topology: [validators-sentry.md](validators-sentry.md)

---

## 9. Wallet & chainlist

Wallet network metadata: [wallet-chainlist.md](wallet-chainlist.md)

Consumer guide: [consumer_wallet_guide.md](../consumer_wallet_guide.md)

---

## 10. Clarity Fortress & developer funnel

**Clarity Fortress** — twelve-step builder walkthrough at `https://dev.clrty.io/labs`

| Artifact | Path |
|----------|------|
| Walkthrough steps | `frontend/labs/walkthrough/steps.json` |
| Fortress API client | `frontend/shared/labs-api.js` |
| Manifest generator | `scripts/clarity-wallet/generate_labs_manifest.py` |
| Smoke verify | `bash scripts/labs/verify_labs_smoke.sh` |

---

## 11. Verification

```bash
# Genesis seal
cargo run -p clarity-cli -- node genesis-verify --plain

# L1 launch simulation
bash scripts/predeploy/l1_launch_simulation.sh --quick

# Clarity Fortress manifest + smoke
python3 scripts/clarity-wallet/generate_labs_manifest.py
bash scripts/labs/verify_labs_smoke.sh

# Supply immutability
cargo test -p clrty-substrate immutability_audit
```

---

## 12. Deferred (not L1 launch scope)

| Component | Phase |
|-----------|-------|
| LayerZero OFTv2 | Phase 10 |
| Wormhole NTT | Phase 10 |
| FMA EVM contracts | Phase 10 |
| External chain RPC (ETH, Base, Arb, Solana) | Phase 10 |

See [CLRTY1_ONLY_SCOPE.md](CLRTY1_ONLY_SCOPE.md) and [clrty-1-fma.md](clrty-1-fma.md).
