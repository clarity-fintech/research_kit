# Wallet Chainlist — clrty-1

Chain metadata for wallet providers (Moniversion, MetaMask-style networks, hardware wallets).

**Spec:** [clrty-1.md](clrty-1.md) · **Consumer guide:** [consumer_wallet_guide.md](../consumer_wallet_guide.md)

---

## Network entry

```json
{
  "chainId": "0x4B2",
  "chainIdDecimal": 1202,
  "chainName": "CLRTY L1",
  "chainSlug": "clrty-1",
  "nativeCurrency": {
    "name": "CLRTY",
    "symbol": "CLRTY",
    "decimals": 9
  },
  "rpcUrls": [
    "https://rpc.clarity-fintech.com",
    "https://api.clrty.dev/rpc"
  ],
  "wsUrls": [
    "wss://rpc.clarity-fintech.com/ws"
  ],
  "blockExplorerUrls": [
    "https://explorer.clrty.network"
  ],
  "iconUrls": [
    "https://clrty.network/shared/assets/clrty-logo.png"
  ],
  "infoURL": "https://clrty.network/blockchain/index.html"
}
```

`chainId` hex: `0x4B2` = 1202 decimal.

---

## EIP-3085 `wallet_addEthereumChain` (EVM-compatible wallets)

```javascript
await ethereum.request({
  method: 'wallet_addEthereumChain',
  params: [{
    chainId: '0x4B2',
    chainName: 'CLRTY L1',
    nativeCurrency: { name: 'CLRTY', symbol: 'CLRTY', decimals: 9 },
    rpcUrls: ['https://rpc.clarity-fintech.com'],
    blockExplorerUrls: ['https://explorer.clrty.network'],
  }],
});
```

Note: clrty-1 is **not** Ethereum — this snippet is for wallets that support custom L1 networks via EIP-3085.

---

## Chainlist.org submission

Submit PR to [ethereum-lists/chains](https://github.com/ethereum-lists/chains) under:

```
_eip155-1202
```

Include `shortName: "clrty"`, `networkId: 1202`, `slip44: TBD`.

---

## Wallet verification checklist

- [ ] Displays chain name **CLRTY L1** (not "Unknown")
- [ ] Shows 9 decimals for `uclrty`
- [ ] Rejects txs when chainId ≠ 1202
- [ ] RPC points to `CLRTY_L1_RPC` — not ETH mainnet
- [ ] No bridge-wrapped token shown as "CLRTY" at L1 launch

---

## Local dev

```bash
CLRTY_L1_RPC=http://127.0.0.1:8545
CLRTY_L1_NUMERIC_CHAIN_ID=1202
```

Clarity Fortress walkthrough step 2: `frontend/labs/walkthrough/steps.json`

---

## Manifest sync

```bash
python3 scripts/clarity-wallet/generate_labs_manifest.py
# Embeds chainlist snippet in clarity-wallet/labs/manifests/clrty_labs_manifest.json
```

---

## Security

Users must verify **clrty-1 only** at L1 launch — see [staying-safe.md](../../frontend/docs/content/start/staying-safe.md).
