# Platform Surface Map

Master mapping: product page → module → API for CLRTY ($CLRTY).

## Products

| Page | Module | API / Route |
|------|--------|-------------|
| [Products overview](../../frontend/index.html) | Site hub | — |
| [Developer Platform](../../frontend/products/developer-platform.html) | clrty-api, clrty-cli-core | `GET /v1/status` |
| [x402](../../frontend/products/x402.html) | protocol/x402 (spec) | scaffold |
| [Agent Registry](../../frontend/products/agent-registry.html) | agent-service (scaffold) | register/search/rank |
| [Clarity Skills](../../frontend/products/clarity-skills.html) | QuantSkillEngine | enforceUniqueness() |
| [Actions & Blinks](../../frontend/products/actions-blinks.html) | clarityLink() | `GET /v1/stream` |
| [Agents & AI](../../frontend/products/agents-ai.html) | arbitrage_core | producer loop |

## Developer & payment tools

| Page | Module | API / Route |
|------|--------|-------------|
| [Commerce Kit](../../frontend/tools/commerce-kit.html) | payments scaffold | createCheckout() |
| [CLARITY PAY](../../frontend/tools/clarity-pay.html) | settlement/ | `GET /v1/compliance/treasury` |
| [RPC Providers](../../frontend/tools/rpc-providers.html) | clarityd | `GET /v1/indexer/clrty-l1` |
| [Payments Tooling](../../frontend/tools/payments-tooling.html) | payment-service (scaffold) | split/recurring |
| [Token Extensions](../../frontend/tools/token-extensions.html) | token_core/ | CCR Sets 99→1 |
| [Digital Assets](../../frontend/tools/digital-assets.html) | asset-service (scaffold) | Phase 10 |

## Docs tabs (42+ pages)

[`frontend/docs/index.html`](../../frontend/docs/index.html) — Start · Products · Build · Business · Validation

## Network & community

| Page | Data source |
|------|-------------|
| [Network Hub](../../frontend/network/hub.html) | `/v1/status`, `/v1/pretest/status`, investor JSON |
| [Validators](../../frontend/network/validators.html) | validator_singularity_set.json |
| [Status](../../frontend/network/status.html) | status.clarity.com (production) |

## Investor portal

| Page | API |
|------|-----|
| [Portal](../../frontend/portal/index.html) | launch_readiness JSON |
| [Governance](../../frontend/portal/governance.html) | `POST /v1/governance/vote` |
| [Settings/KYC](../../frontend/portal/settings.html) | `GET /v1/compliance/wallet/:wallet/status` |

## System flow

```
User → Agent → Skills (1 active) → Action/Blink
                ↓
         Payment Engine → Token → Settlement
                ↓
         x402 Protocol → L1 finality (clrty-1)
```

## JS scaffold library

[`frontend/lib/clarity-scaffold.js`](../../frontend/lib/clarity-scaffold.js)

## Protocol specs

- [x402.md](x402.md)
- [clarity-skills.md](clarity-skills.md)
- [agent-protocol.md](agent-protocol.md)
- [payment-standard.md](payment-standard.md)
