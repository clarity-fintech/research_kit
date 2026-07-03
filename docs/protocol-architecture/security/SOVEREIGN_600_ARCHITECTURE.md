# Sovereign-600 Architecture

Parallel to **MSA-100** (operational pretest layers PT-001–PT-100), the **Sovereign-600** registry catalogs **600 security protocols** (SP-001–SP-600) across **Zones A–Y**.

## Three-tier security stack

```
MSA-100 (operational)     →  Sovereign SP-001–500 (perimeter)  →  Sovereign SP-501–600 (atomic)
Blue Code tasks 201–500     →  ATU phases p57–p60               →  ATU 501–600 / master seal
```

| Tier | ID range | Role |
|------|----------|------|
| MSA-100 | Layers 1–100 / PT-001–100 | Launch pretest, sim validation, settlement perimeter |
| Sovereign Perimeter | SP-001–500 / Zones A–T | Hardware RoT through governance singularity |
| Atomic Defense | SP-501–600 / Zones U–Y | Token + ledger mathematical immutability |

## Zone bands

### Perimeter (SP-001–500)

| Zone | SP range | Theme |
|------|----------|-------|
| A–E | 1–125 | Hardware, consensus, crypto, network, adversarial monitoring |
| F–J | 126–250 | PQC hardening, infra defense, runtime integrity, governance, AI defense |
| K–O | 251–375 | Crypto frontier, autonomous defense, physical-logic, formal governance, edge |
| P–T | 376–500 | Quantum primitives, self-healing, hyper-integrity, sovereign governance, singularity perimeter |

**SP-500** — protocol-level Singularity Lock (perimeter / governance).

### Atomic Defense (SP-501–600)

| Zone | SP range | Theme |
|------|----------|-------|
| U | 501–520 | Cryptographic Token Integrity |
| V | 521–540 | Consensus & Byzantine Defense |
| W | 541–560 | Cryptographic Ledger Hardening |
| X | 561–580 | Protocol-Logic Defense |
| Y | 581–600 | Finality Singularity |

**SP-600** — **Singularity Lock: Deterministic Immutable Ledger** (terminal ledger seal).

Cross-links: `state_manifold/genesis_seal.rs`, `token_core/blue_code/resilience.rs`, `atu_runner` p60 `structural_500`.

## Machine-readable sources

| File | Purpose |
|------|---------|
| `CLRTY_SUBSTRATE/boot/sovereign_canonical_titles.json` | Canonical security taxonomy titles (SP-001–600, Zones A–Y) |
| `CLRTY_SUBSTRATE/boot/sovereign_zone_catalog.json` | Zone structure and implementation catalog (generator layout) |
| `CLRTY_SUBSTRATE/boot/sovereign_protocols_manifest.json` | 600 protocol entries + status |
| `CLRTY_SUBSTRATE/boot/sovereign_protocol_map.json` | SP ↔ MSA ↔ structural task cross-links |
| `var/compliance/sovereign_protocols_report.json` | Audit rollup (perimeter + atomic bands) |

## Verification

```bash
python3 scripts/investor/generate_sovereign_protocols.py
bash scripts/audit/verify_sovereign_protocols.sh
bash scripts/investor/build_treasury_data.sh
```

Gate threshold: **≥80% documented** (implemented + partial) across all 600; atomic band (501–600) tracked separately.

## Relationship to MSA-100

MSA-100 is **not replaced**. SP-001–100 map 1:1 to MSA layers and PT tasks where applicable. Sovereign-600 extends coverage for sovereign-grade and atomic token/ledger defenses documented in investor materials and Blue Code structural tasks 201–500.

See also: [MASS_SECURITY_ARCHITECTURE.md](MASS_SECURITY_ARCHITECTURE.md), [../token/structural_tasks_201_400.md](../token/structural_tasks_201_400.md).
