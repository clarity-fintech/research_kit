# Downloads — CLRTY-1 Research Kit

Everything in this repository is packaged for systems architects, hardware engineers, cryptographic researchers, and protocol analysts.

## Primary Download

| Kit | File | Contents |
|-----|------|----------|
| Full Research Kit | [`dist/research-kit-full.zip`](dist/research-kit-full.zip) | Protocol architecture docs, investor research, simulations, sanitized data snapshots, engine summaries, research topics, and data dictionary |
| Research Data Pack | [`dist/research-data-pack.zip`](dist/research-data-pack.zip) | Sanitized Data Center snapshots, investor data, sheet inputs, MLX samples, protocol docs, investor research, and engine summaries |
| Simulation Research Pack | [`dist/simulation-research-pack.zip`](dist/simulation-research-pack.zip) | Simulators, arbitrage/simulation docs, and engine summaries for model validation |
| Mastermind First Access Pack | [`dist/mastermind-first-access-pack.zip`](dist/mastermind-first-access-pack.zip) | First Access terminal vector, local inference configs, closed alpha API docs, proof-of-fidelity samples, and hosted manifest |

Checksums: [`dist/SHA256SUMS.txt`](dist/SHA256SUMS.txt)

## Git Clone

```bash
git clone https://github.com/clarity-fintech/research_kit.git
cd research_kit
```

## Included Surfaces

- `docs/protocol-architecture/` — blueprint, whitepaper, chain, security, and tokenomics research.
- `docs/investor-research/` — diligence, ROI, HELIX, Quantum Skills, and performance research.
- `docs/simulation-arbitrage/` — simulation and arbitrage traceability.
- `simulators/` — state-space, tokenomics, mirror, and backtest simulator sources.
- `data/` — sanitized Data Center, investor, sheet, and MLX samples.
- `engine-summaries/` — concise summaries for HELIX, quant stack, CortexPay, monetization calculus, VIS intelligence, and MLX.

## Research Workflow

```bash
git clone https://github.com/clarity-fintech/research_kit.git
cd research_kit
open RESEARCH_TOPICS.md
open DATA_DICTIONARY.md

# Optional First Access / Mastermind terminal vector
unzip dist/mastermind-first-access-pack.zip -d /tmp/clrty-mastermind
```

## Verification

Review [`RESEARCH_TOPICS.md`](RESEARCH_TOPICS.md), [`DATA_DICTIONARY.md`](DATA_DICTIONARY.md), and [`MANIFEST.md`](MANIFEST.md).
