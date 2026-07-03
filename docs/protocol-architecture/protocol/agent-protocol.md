# Agent Protocol

On-chain agent identity, registry, and execution on CLRTY L1.

## Registry (scaffold)

```javascript
const registry = new AgentRegistry();
registry.register({ name: 'Trading Agent', reputation: 0.91 });
registry.search('trading');
registry.rank('agent_0');
```

Frontend: [`frontend/products/agent-registry.html`](../../frontend/products/agent-registry.html)

## Execution flow

```
Agent → Skills (1 active) → Actions → Payment → x402 → L1 finality
```

## Rust mapping

| Component | Path |
|-----------|------|
| Producer loop | `arbitrage_core/` |
| Signal bridge | `clrty-signal-bridge/` |
| Secure model hook | `docs/integration/secure_model_hook.md` |

## Identity

Agents bind to wallet addresses and optional VIS attestation blobs for institutional tiers.

## Status

Registry and reputation scoring are **frontend scaffolds**. Production agent-service is post-L1 launch roadmap.
