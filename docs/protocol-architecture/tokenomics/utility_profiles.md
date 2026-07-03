# Structural Token Utility Profiles (S07)

| Profile | Utility | On-chain mapping |
|---------|---------|------------------|
| Validator | Node registration fees, PoC block rewards | `validators` bucket, `validator_network/` |
| Parallel execution licensee | MVM gas + Blue Code priority | `mvm_execution/`, Set tier fee deflection |
| Governance staker | Snapshot voting weight | `FmaStakingVault` tiers 1–4, `snapshot_voting.rs` |
| B2B integrator | API access + attestation relay | `clrty-api`, `fma-relayer` |
| Retail holder | CCR Set migration, referral rewards | `backlog/`, `referral_registry.rs` |

## Fee sinks
Execution gas → validator share + LP share + burn (adaptive phases in `simulate.py`).
