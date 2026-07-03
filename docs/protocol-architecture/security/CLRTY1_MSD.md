# CLRTY-1 MSD — Mass Security Defense

100 nano-task defense ledger for **clrty-1** L1 launch. Each task (`MSD-001`–`MSD-100`) maps to an MDA ring, MSA layer band, verify command, and honest status.

**Machine-readable manifest:** [`CLRTY_SUBSTRATE/boot/msd_nano_tasks_manifest.json`](../../CLRTY_SUBSTRATE/boot/msd_nano_tasks_manifest.json)

**Related:** [CLRTY1_MDA.md](CLRTY1_MDA.md) · [MASS_SECURITY_ARCHITECTURE.md](MASS_SECURITY_ARCHITECTURE.md) · [security_layers_manifest.json](../../CLRTY_SUBSTRATE/boot/security_layers_manifest.json)

---

## Zone overview

| Zone | Tasks | MDA ring | Focus |
|------|-------|----------|-------|
| **A** | MSD-001–025 | Ring I — Protocol | PoC, EntropyBus, topology, state root |
| **B** | MSD-026–050 | Ring II — Economic | Genesis, supply cap, vesting, listing |
| **C** | MSD-051–075 | Ring III — Execution | HELIX, MIRRA, MLX, simulateTransaction |
| **D** | MSD-076–100 | Ring IV — Perimeter | VIS, Safe, KYC, bridge pause, panic |

```mermaid
flowchart LR
  A[Zone_A_001_025] --> B[Zone_B_026_050]
  B --> C[Zone_C_051_075]
  C --> D[Zone_D_076_100]
```

---

## Task status model

| Status | Meaning |
|--------|---------|
| `implemented` | Artifact exists; verify command passes |
| `partial` | Scaffold or dry-run only |
| `planned` | Spec / manifest stub |
| `deferred` | Phase 10 — bridge / FMA dependent |

---

## Sample tasks (full list in manifest)

| ID | Title | Zone | Status |
|----|-------|------|--------|
| MSD-001 | PoC sim-block commit p50 | A | implemented |
| MSD-007 | 50ms topology gate | A | implemented |
| MSD-015 | Genesis entropy seal | B | implemented |
| MSD-022 | Supply checksum oracle | B | implemented |
| MSD-030 | Listing config alignment | B | implemented |
| MSD-045 | CCR Set tier migration bounds | B | partial |
| MSD-052 | HELIX shadow book dry-run | C | partial |
| MSD-058 | MLX toxicity score hook | C | partial |
| MSD-065 | simulateTransaction RPC | C | implemented |
| MSD-071 | Clarity Fortress walkthrough step 5 gate | C | implemented |
| MSD-078 | VIS N01 identity gatekeeper | D | partial |
| MSD-085 | Bridge pause dead-man switch | D | partial |
| MSD-092 | Safe treasury monitor | D | partial |
| MSD-099 | Clarity Fortress smoke verify script | D | implemented |
| MSD-100 | Launch readiness composite | D | partial |

---

## Verification

```bash
# Manifest integrity (100 tasks)
python3 -c "
import json
m = json.load(open('CLRTY_SUBSTRATE/boot/msd_nano_tasks_manifest.json'))
assert m['version'] == 1
assert len(m['tasks']) == 100
ids = [t['id'] for t in m['tasks']]
assert ids == [f'MSD-{i:03d}' for i in range(1, 101)]
print('OK:', len(ids), 'MSD tasks')
"

# MSA cross-check
bash scripts/audit/verify_security_layers.sh

# Clarity Fortress smoke (MSD-099)
bash scripts/labs/verify_labs_smoke.sh
```

---

## CI integration

MSD-099 (`verify_labs_smoke.sh`) runs in `.github/workflows/ci.yml` when manifest and walkthrough artifacts are present.

---

## Maintenance

Regenerate task stubs after MSA layer changes:

```bash
# Manual edit msd_nano_tasks_manifest.json — one task per MSA band quartile
# Status updates tracked in var/compliance/msd_report.json (future)
```

Honesty rule: **do not mark `implemented` without a passing verify command** in the manifest.
