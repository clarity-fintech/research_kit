# CLRTY Performance & ROI Tracking Template

Repo-authoritative mirror of the **CLRTY DATA CENTER** metric catalog. Live values sync to Notion via `make data-center-sync`.

**Manifest:** [`CLRTY_SUBSTRATE/boot/data_center_manifest.json`](../../CLRTY_SUBSTRATE/boot/data_center_manifest.json)  
**Sheets template:** [`data_center_sheets_template.md`](data_center_sheets_template.md)  
**API:** `GET /v1/listing/metrics` · `GET /v1/data-center/snapshot`

---

## I. Extraction Metrics — Founder ROI

| Metric | Slug | Formula |
|--------|------|---------|
| Daily Fee Revenue | `daily_fee_revenue_usd` | `transaction_velocity_24h × tax_rate_modeled` |
| Founder Daily Draw | `founder_daily_draw_usd` | `daily_fee_revenue_usd × founder_draw_pct` |
| Treasury Growth Rate | `treasury_growth_rate_usd` | `daily_fee_revenue_usd × treasury_take_pct` |
| Yield Efficiency | `yield_efficiency` | `staking_rewards_24h / total_staked_usd` |
| Personal ROI | `personal_roi` | `(fees + sales − liquidity − marketing) / initial_capital` |
| Break-Even Timeline | `break_even_days` | `liquidity_injection / founder_daily_draw` |

## II. Velocity Metrics — Ecosystem Health

| Metric | Slug |
|--------|------|
| Daily Active Users | `dau` |
| Transaction Velocity | `transaction_velocity_24h` |
| Staking Penetration Rate | `staking_penetration_rate` |
| Holder Retention (30d) | `holder_retention_30d` |
| Average Transaction Size | `avg_transaction_size_usd` |
| Taxation Efficiency | `taxation_efficiency` |

## III. Demand & Utility — Growth

| Metric | Slug |
|--------|------|
| Access Conversion Rate | `access_conversion_rate` |
| Discount Arbitrage Rate | `discount_arbitrage_rate` |
| Burn Rate (Deflationary) | `burn_rate_deflationary` |
| Social Sentiment | `social_sentiment_index` |

## IV. Operational — Brand Scaling

| Metric | Slug |
|--------|------|
| Burn Rate (OpEx) | `burn_rate_opex_usd` |
| Network Reach Growth | `network_reach_growth` |
| Cost Per Acquisition | `cpa_usd` |
| Runway | `runway_days` |

## V. Private Monetization — On-Chain Income (Notion + Data Center)

| Metric | Slug | Source |
|--------|------|--------|
| Execution Fees (24h) | `execution_fees_24h_usd` | `var/monetization/income_ledger.jsonl` |
| Governance/Protocol Fees (24h) | `governance_fees_24h_usd` | founder + treasury split from ledger |
| Model Royalty Income (24h) | `royalty_income_24h_usd` | L09 `model_royalty` stream |
| Marketplace Commission (24h) | `marketplace_commission_24h_usd` | L08 stream |
| Private Stream Cumulative | `private_income_cumulative_usd` | all-time ledger |
| Fee Router Status | `fee_router_status` | 1=modeled, 2=live (post-deploy) |

**Default-on:** `CLRTY_EXECUTION_FEE_DEFAULT=1` (set `0` to disable logging in dev).  
**API:** `GET /v1/monetization/income` · synced via `make data-center-sync` to Notion (same database as I–IV).

---

## Scaling indicator

- **Optimized:** daily fee revenue rising while transaction velocity steady  
- **Stagnant:** flat revenue + access conversion below target → focus on utility demand  
- **Growing:** otherwise — monitor next threshold  

## CVP honesty

Metrics tied to 4% on-chain tax are **`modeled`** until EVM tax deploy. See [`TOKENOMICS_LOCKED.md`](../tokenomics/TOKENOMICS_LOCKED.md) and [`signal_normalization.md`](signal_normalization.md).
