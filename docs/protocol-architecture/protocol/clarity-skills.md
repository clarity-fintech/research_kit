# Clarity Skills

Proprietary quant skill library for CLRTY agents — modular, installable, **one skill per execution context**.

## Core rule

`QuantSkillEngine.enforceUniqueness(skillId)` — only one active quant skill may run in a given execution context. This prevents conflicting strategies and enforces market diversification discipline.

## API (JS scaffold)

```javascript
const engine = new QuantSkillEngine();
engine.install({ id: 'market-arbitrage', name: 'Market Arbitrage', run: (ctx) => ({ ok: true }) });
await engine.execute('market-arbitrage', { ts: Date.now() });
engine.release();
```

## Default skill catalog (scaffold)

| ID | Name | Tag |
|----|------|-----|
| market-arbitrage | Market Arbitrage | quant |
| payment-executor | Payment Executor | payments |
| lead-scraper | Lead Scraper | growth |
| entropy-monitor | Entropy Monitor | infra |

Frontend: [`frontend/products/clarity-skills.html`](../../frontend/products/clarity-skills.html)

**Investor / operator guide:** [`docs/investor/clarity_skills_overview.md`](../../investor/clarity_skills_overview.md) — 4 Quantum Skills, dual-lock CLI, runbook, HUD sync.

## Future Rust mapping

| Module | Role |
|--------|------|
| `clrty-signal-bridge/` | Signal validation |
| `arbitrage_core/` | Producer execution |
| skills-service (planned) | Registry + monetization |

## Comparison to generic "skills"

Unlike general-purpose agent plugins, Clarity Skills are **quantitative, sequential, and mutually exclusive per context** — aligned with PoC argmax(E − λR) and entropy-linked fee routing.
