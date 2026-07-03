# HELIX Hidden Exchange Layer

L0.5 execution mesh between operators and clrty-1 canonical state.

## Overview

HELIX is the hidden exchange layer documented as product **P01** in the CLARITY suite. It implements HELIX-01..10 modules in `helix_engine/`.

## Architecture

See [`docs/investor/helix_engine_architecture.md`](../investor/helix_engine_architecture.md).

## Product page

[`frontend/products/suite/category-i-execution/helix-engine.html`](../../frontend/products/suite/category-i-execution/helix-engine.html)

## Verification

```bash
scripts/audit/verify_helix_components.sh
clrty execute 2601
clrty helix status
```

Manifest: `CLRTY_SUBSTRATE/boot/helix_manifest.json`

## Phases

| Phase | Deliverable | Status |
|-------|-------------|--------|
| 0 | Products hub + suite pages | implemented |
| 1 | Docs + manifest + audit | implemented |
| 2 | helix_engine crate + CLI + treasury panel | partial |
| 3 | economic_core, arbitrage_core, AVR wire | partial |
| 4 | ATU 2601–2610 launch gates | partial |

Cross-links: sovereign protocols manifest, Products hub, CortexPay CORTEX-03 payment router.
