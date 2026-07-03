# Simulation & launch strategy docs

Simulation artifacts, ABM architecture, and the **Notion export** for launch strategy narrative. For implementation authority, prefer [architecture/README.md](../architecture/README.md) and linked investor docs.

---

## Files in this folder

| Doc | Description |
|-----|-------------|
| [**CLRTY_Live_Market_Notion.md**](CLRTY_Live_Market_Notion.md) | **Notion export** (~2,100 lines) — strategy, charts, SIM100 narrative, DD synthesis |
| [abm_architecture.md](abm_architecture.md) | Agent-based model architecture |
| [events_100_catalog.md](events_100_catalog.md) | SIM100 event catalog |
| [generate_charts.py](generate_charts.py) | Chart generation script |

---

## Notion export — section index

Use this to jump into [CLRTY_Live_Market_Notion.md](CLRTY_Live_Market_Notion.md) without scrolling the full file.

| Section | Anchor | Contents |
|---------|--------|----------|
| Strategy summary | [#strategy-in-one-paragraph](CLRTY_Live_Market_Notion.md#strategy-in-one-paragraph) | One-paragraph launch thesis |
| Investor DD (June 2026) | [#investor-data-room--june-2026-update](CLRTY_Live_Market_Notion.md#investor-data-room--june-2026-update) | Six canonical DD docs + product suite |
| 554-day plan | [#less-is-more--50-year-plan-first-554-days](CLRTY_Live_Market_Notion.md#less-is-more--50-year-plan-first-554-days) | Less Is More phased rollout |
| Funding model | [#what-massive-funding-actually-means](CLRTY_Live_Market_Notion.md#what-massive-funding-actually-means) | Treasury depth vs on-chain cap |
| Charts | [#charts--visualizations](CLRTY_Live_Market_Notion.md#charts--visualizations) | 34+ visualization references |
| SIM100 results | [#sim100-results-comprehensive](CLRTY_Live_Market_Notion.md#sim100-results-comprehensive) | Convergence narrative + merkle |
| Moniverse mechanisms | [#unique-mechanisms--moniverse-flow](CLRTY_Live_Market_Notion.md#unique-mechanisms--moniverse-flow) | 52 mechanism catalog |
| Token layout | [#blockchain-custom-layout](CLRTY_Live_Market_Notion.md#blockchain-custom-layout) | Layout v2 overview |
| Arbitrage | [#cross-venue-arbitrage-architecture](CLRTY_Live_Market_Notion.md#cross-venue-arbitrage-architecture) | Producer / HELIX arb |
| Five-phase launch | [#the-five-phase-launch-strategy](CLRTY_Live_Market_Notion.md#the-five-phase-launch-strategy) | Phase gates |
| Supply & float | [#supply--float--realistic-12-month-view-funded-scenario](CLRTY_Live_Market_Notion.md#supply--float--realistic-12-month-view-funded-scenario) | 12-month float model |
| Risks | [#risks-stated-plainly](CLRTY_Live_Market_Notion.md#risks-stated-plainly) | Stated risk register |
| Validation | [#validation--proof-points](CLRTY_Live_Market_Notion.md#validation--proof-points) | Proof points + commands |
| Cross-links | [#notion-topic--repo-doc-cross-links](CLRTY_Live_Market_Notion.md#notion-topic--repo-doc-cross-links) | Topic → authoritative repo doc |

---

## Verification commands

```bash
make verify-all-140-steps
bash scripts/sim/run_sim100_convergence.sh
cargo run -p atu_runner -- 10001
```

Reports: `var/compliance/sim100_convergence_report.json`
