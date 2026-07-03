#!/usr/bin/env python3
"""Generate all SIM100 / live market chart PNGs for Notion doc."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).parent / "charts"
OUT.mkdir(parents=True, exist_ok=True)
TICKS = json.loads((ROOT / "simulators/state_space/fixtures/batch_100_ticks.json").read_text())

plt.rcParams.update({
    "figure.facecolor": "#ffffff", "axes.facecolor": "#fafafa",
    "axes.edgecolor": "#cccccc", "grid.color": "#eeeeee", "font.size": 10,
    "text.color": "#111111", "axes.labelcolor": "#666666", "xtick.color": "#666666",
    "ytick.color": "#666666",
})
# CVP Raw Mono — black & white chart generation
C = {
    "canvas": "#ffffff", "surface": "#fafafa", "accent": "#111111", "text": "#111111",
    "muted": "#666666", "highlight": "#000000",
    "treasury": "#111111", "validators": "#333333", "liquidity": "#555555",
    "ecosystem": "#777777", "public": "#999999", "bid": "#333333", "ask": "#666666",
    "price": "#111111", "circ": "#444444", "staked": "#333333", "locked": "#555555",
    "burned": "#888888", "flat": "#cccccc",
}
REGIMES = [(1, 20, "#eee"), (21, 25, "#ccc"), (26, 45, "#eee"), (46, 50, "#ddd"),
           (51, 75, "#bbb"), (76, 85, "#ddd"), (86, 95, "#ccc"), (96, 100, "#eee")]

EVENTS100 = [t["event_id"] for t in TICKS]
MID = [t["mid_price"] for t in TICKS]
BID = [t["bid_depth"] / 1e6 for t in TICKS]
ASK = [t["ask_depth"] / 1e6 for t in TICKS]
LAMBDA = [t["lambda"] for t in TICKS]
MCAP = [5.6 * m for m in MID]

# Canvas VOL_PROXY from clrty-live-trading.canvas.tsx
VOL = [0.0, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0, 0.16, 0.16, 0.17, 0.17, 0.17, 0.17, 0.17, 0.17, 0.18, 0.18, 3600.08, 3700.08, 3800.08, 3900.08, 4000.08, 4100.08, 4200.08, 4300.08, 4400.08, 4500.09, 1045.5, 2093.09, 3145.92, 4207.14, 0.0, 1.29, 1.1, 0.93, 0.79, 0.67, 0.57, 0.49, 0.41, 0.35, 0.3, 10000.0, 10000.0, 10000.0, 10000.0, 10000.0, 0.17, 0.15, 0.14, 0.12, 0.11, 0.1, 0.09, 0.08, 0.07, 0.07, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 5000.01, 5000.01, 0.01, 5000.01, 5000.01, 0.01, 5000.01, 5000.01, 0.01, 5000.01, 0.01, 0.01, 0.01, 0.01, 0.01]

SPREAD = [12.5, 15.0, 17.5, 19.9, 22.4, 24.9, 27.4, 29.9, 32.4, 34.9, 37.3, 39.8, 42.3, 44.8, 47.2, 49.7, 52.2, 54.7, 57.1, 59.6, 59.6, 59.6, 59.6, 59.6, 59.6, 61.2, 62.7, 64.3, 65.9, 67.4, 69.0, 70.5, 72.0, 73.6, 75.1, 77.5, 80.0, 82.4, 84.8, 87.3, 89.7, 92.1, 94.5, 97.0, 99.4, 99.4, 99.4, 99.4, 99.4, 99.4, 59.5, 19.1, 41.5, 81.7, 120.9, 158.8, 194.9, 228.9, 260.5, 289.7, 222.4, 144.3, 58.3, 50.6, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 137.0, 139.4, 141.7, 144.0, 146.3, 148.6, 150.9, 153.2, 155.5, 157.7, 160.0, 162.3, 164.5, 166.8, 169.0, 171.2]
IMB = [0.5, 1.0, 1.49, 1.99, 2.49, 2.98, 3.48, 3.98, 4.47, 4.97, 5.47, 5.96, 6.46, 6.95, 7.45, 7.94, 8.44, 8.93, 9.42, 9.92, 9.92, 9.92, 9.92, 9.92, 9.92, 10.23, 10.55, 10.86, 11.18, 11.49, 11.79, 12.1, 12.41, 12.71, 13.02, 13.51, 13.99, 14.48, 14.97, 15.45, 15.94, 16.42, 16.91, 17.39, 17.87, 17.87, 17.87, 17.87, 17.87, 17.87, 9.91, 1.82, -6.3, -14.34, -22.19, -29.76, -36.98, -43.77, -50.1, -55.94, -42.48, -26.85, -9.66, 8.12, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.41, 25.87, 26.34, 26.8, 27.26, 27.72, 28.18, 28.64, 29.09, 29.55, 30.0, 30.45, 30.9, 31.35, 31.8, 32.25]

LIVE_EV = [1, 6, 11, 16, 21, 26, 31, 36, 41, 46, 51, 56, 61, 66, 71, 76, 81, 86, 91, 96, 100]
LIVE_PRICE = [MID[i - 1] for i in LIVE_EV]
LIVE_DEPTH = [(BID[i - 1] + ASK[i - 1]) / 2 for i in LIVE_EV]
LIVE_MCAP = [MCAP[i - 1] for i in LIVE_EV]

E25 = list(range(1, 26))
MID25 = [MID[i - 1] for i in E25]
BID25 = [BID[i - 1] for i in E25]
ASK25 = [ASK[i - 1] for i in E25]
LAM25 = [LAMBDA[i - 1] for i in E25]


def shade(ax):
    for f, t, c in REGIMES:
        ax.axvspan(f, t, alpha=0.12, color=c)


def flat_band(ax, end=35):
    ax.axvspan(1, end, alpha=0.08, color=C["flat"])


# 01 genesis pie
fig, ax = plt.subplots(figsize=(7, 5))
ax.pie([4, 4, 3, 3, 2], labels=["Treasury", "Liquidity", "Validators", "Ecosystem", "Public"],
       colors=[C[k] for k in ["treasury", "liquidity", "validators", "ecosystem", "public"]],
       autopct="%1.1f%%", startangle=90, wedgeprops=dict(width=0.45, edgecolor="white"))
ax.set_title("Genesis Allocation — 16M CLRTY Hard Cap", fontweight="bold")
fig.tight_layout()
fig.savefig(OUT / "01_genesis_allocation.png", dpi=150, bbox_inches="tight")
plt.close()

# 02 supply base
MONTHS = [f"M{i}" for i in range(13)]
circ, st, lk, br = [5.6] * 13, [2.4, 2.41, 2.43, 2.44, 2.45, 2.46, 2.48, 2.49, 2.5, 2.52, 2.53, 2.54, 2.56], [6.4] * 8 + [6.2, 6.0, 5.8, 5.6, 5.4], [0, 0.002, 0.004, 0.006, 0.012, 0.017, 0.023, 0.033, 0.044, 0.055, 0.066, 0.077, 0.087]
x = np.arange(13)
b1 = np.array(circ)
b2 = b1 + np.array(st)
b3 = b2 + np.array(lk)
fig, ax = plt.subplots(figsize=(10, 5))
ax.bar(x, circ, label="Circulating", color=C["circ"], alpha=0.85)
ax.bar(x, st, bottom=b1, label="Staked", color=C["staked"], alpha=0.85)
ax.bar(x, lk, bottom=b2, label="Locked", color=C["locked"], alpha=0.85)
ax.bar(x, br, bottom=b3, label="Burned", color=C["burned"], alpha=0.85)
ax.set_xticks(x)
ax.set_xticklabels(MONTHS)
ax.set_ylabel("Millions CLRTY")
ax.set_title("Supply State — 12 Month Base Model", fontweight="bold")
ax.legend(fontsize=8)
ax.grid(axis="y", alpha=0.5)
fig.tight_layout()
fig.savefig(OUT / "02_supply_12month_base.png", dpi=150, bbox_inches="tight")
plt.close()

# 03 funded
xf, cf, sf, lf, bf = [0, 1, 2], [3.2, 5.0, 5.8], [2.4, 2.6, 2.8], [9.0, 7.2, 5.8], [0, 0.045, 0.18]
b1 = np.array(cf)
b2, b3 = b1 + np.array(sf), b1 + np.array(sf) + np.array(lf)
fig, ax = plt.subplots(figsize=(8, 5))
ax.bar(xf, cf, label="Circulating", color=C["circ"], alpha=0.85)
ax.bar(xf, sf, bottom=b1, label="Staked", color=C["staked"], alpha=0.85)
ax.bar(xf, lf, bottom=b2, label="Locked", color=C["locked"], alpha=0.85)
ax.bar(xf, bf, bottom=b3, label="Burned", color=C["burned"], alpha=0.85)
ax.set_xticks(xf)
ax.set_xticklabels(["M0", "M6", "M12"])
ax.set_ylabel("Millions CLRTY")
ax.set_title("Supply State — Funded Launch (M0/M6/M12)", fontweight="bold")
ax.legend(fontsize=8)
ax.grid(axis="y", alpha=0.5)
fig.tight_layout()
fig.savefig(OUT / "03_supply_funded_scenario.png", dpi=150, bbox_inches="tight")
plt.close()

# 04 sim100 sampled
fig, (a1, a2) = plt.subplots(2, 1, figsize=(10, 7), sharex=True)
a1.plot(LIVE_EV, LIVE_PRICE, "o-", color=C["price"], lw=2, ms=4)
a1.axhline(1, color="#999", ls="--")
a1.axvspan(51, 75, alpha=0.15, color="red")
flat_band(a1)
a1.set_ylabel("USDC")
a1.set_title("SIM100 Price & Liquidity — flat E1-35 is correct OQ1/OQ2 output", fontweight="bold")
a2.plot(LIVE_EV, LIVE_DEPTH, "o-", color=C["bid"], lw=2, ms=4, label="Depth")
a2.plot(LIVE_EV, LIVE_MCAP, "s--", color=C["validators"], lw=1.5, ms=3, label="Mcap proxy")
a2.axvspan(51, 75, alpha=0.15, color="red")
a2.set_xlabel("Event #")
a2.set_ylabel("M USD")
a2.legend()
a2.grid(alpha=0.4)
fig.tight_layout()
fig.savefig(OUT / "04_sim100_price_depth.png", dpi=150, bbox_inches="tight")
plt.close()

# 05 full price
fig, ax = plt.subplots(figsize=(12, 4))
ax.plot(EVENTS100, MID, color=C["price"], lw=1.5)
ax.fill_between(EVENTS100, MID, 0.99, alpha=0.08, color=C["price"])
ax.axhline(1, color="#999", ls="--")
shade(ax)
flat_band(ax)
ax.annotate("Flat $1.005\n(OQ1 ledger)", xy=(17, 1.0055), fontsize=9, ha="center", color="#555")
ax.set_xlabel("Event #")
ax.set_ylabel("Mid (USDC)")
ax.set_title("100-Event Session — +12.5% net; 35 events flat at par band", fontweight="bold")
ax.set_xlim(1, 100)
ax.set_ylim(0.99, 1.14)
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "05_trading_session_price.png", dpi=150, bbox_inches="tight")
plt.close()

# 06 bid ask
fig, ax = plt.subplots(figsize=(12, 4))
ax.plot(EVENTS100, BID, color=C["bid"], lw=1.5, label="Bid")
ax.plot(EVENTS100, ASK, color=C["ask"], lw=1.5, label="Ask")
shade(ax)
flat_band(ax)
ax.set_xlabel("Event #")
ax.set_ylabel("Depth (M USDC)")
ax.set_title("Bid vs Ask — E1-35 stable depth growth", fontweight="bold")
ax.legend()
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "06_trading_bid_ask_depth.png", dpi=150, bbox_inches="tight")
plt.close()

# 07 spread imbalance
fig, (a1, a2) = plt.subplots(2, 1, figsize=(12, 6), sharex=True)
a1.plot(EVENTS100, SPREAD, color=C["validators"], lw=1.5)
shade(a1)
a1.set_ylabel("Spread (bps)")
a1.set_title("Microstructure — flat E1-20 ~12-60 bps; stress peaks ~290 bps", fontweight="bold")
a1.grid(alpha=0.3)
a2.plot(EVENTS100, IMB, color=C["price"], lw=1.5)
a2.axhline(0, color="#999", ls="--")
shade(a2)
a2.set_xlabel("Event #")
a2.set_ylabel("Imbalance (%)")
a2.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "07_trading_spread_imbalance.png", dpi=150, bbox_inches="tight")
plt.close()

# 08 regime pie
fig, ax = plt.subplots(figsize=(6, 5))
ax.pie([45, 30, 15, 10], labels=["Stable 45", "Adversarial 30", "Expansion 15", "Accumulation 10"],
       colors=["#9B9B9B", "#E85555", "#50C878", "#4A90D9"], autopct="%1.0f%%", startangle=90)
ax.set_title("SIM100 Regime Mix — majority stable/normal ops", fontweight="bold")
fig.tight_layout()
fig.savefig(OUT / "08_regime_mix.png", dpi=150, bbox_inches="tight")
plt.close()

# 09 lambda full
fig, ax = plt.subplots(figsize=(12, 3))
ax.plot(EVENTS100, LAMBDA, color=C["validators"], lw=1.5)
shade(ax)
flat_band(ax)
ax.set_xlabel("Event #")
ax.set_ylabel("λ")
ax.set_title("EntropyBus λ — linear drift 0.101→0.200 even in flat price window", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "09_lambda_heartbeat.png", dpi=150, bbox_inches="tight")
plt.close()

# 10 vol + lambda
fig, (a1, a2) = plt.subplots(2, 1, figsize=(12, 5), sharex=True)
a1.plot(EVENTS100, VOL, color=C["bid"], lw=1)
a1.set_ylabel("Vol proxy")
a1.set_title("Activity Vol Proxy + λ (full session)", fontweight="bold")
flat_band(a1)
a1.annotate("Near-zero vol\n(stable)", xy=(12, 50), fontsize=8)
a2.plot(EVENTS100, LAMBDA, color=C["validators"], lw=1.5)
a2.set_xlabel("Event #")
a2.set_ylabel("λ")
a2.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "10_vol_proxy_lambda.png", dpi=150, bbox_inches="tight")
plt.close()

# 11-14 sim tick e1-25
fig, ax = plt.subplots(figsize=(10, 3))
ax.plot(E25, MID25, "o-", color=C["price"], lw=2, ms=5)
ax.axhline(1.005, color="#999", ls="--")
ax.axvspan(1, 20, alpha=0.15, color="#ccc")
ax.axvspan(21, 25, alpha=0.15, color="#E85555")
ax.set_ylim(1.003, 1.007)
ax.set_title("SIM Tick E1-25 — flat mid @ $1.005 (OQ1 genesis/ledger)", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "11_sim_tick_price_e1_25.png", dpi=150, bbox_inches="tight")
plt.close()

fig, ax = plt.subplots(figsize=(10, 3))
ax.plot(E25, BID25, "o-", color=C["bid"], lw=2, label="Bid")
ax.plot(E25, ASK25, "s-", color=C["ask"], lw=2, label="Ask")
ax.axvspan(1, 20, alpha=0.1, color="#ccc")
ax.set_title("SIM Tick E1-25 — bid grows; ask flat at $5M", fontweight="bold")
ax.legend()
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "12_sim_tick_bid_ask_e1_25.png", dpi=150, bbox_inches="tight")
plt.close()

fig, ax = plt.subplots(figsize=(10, 3))
ax.plot(E25, LAM25, "o-", color=C["validators"], lw=2)
ax.set_title("SIM Tick E1-25 — λ drift 0.101→0.125 during flat price", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "13_sim_tick_lambda_e1_25.png", dpi=150, bbox_inches="tight")
plt.close()

fig, ax = plt.subplots(figsize=(5, 5))
ax.pie([20, 5], labels=["Stable 20", "Adversarial 5"], colors=["#9B9B9B", "#E85555"], autopct="%1.0f%%", startangle=90)
ax.set_title("SIM Tick E1-25 Regime Mix", fontweight="bold")
fig.tight_layout()
fig.savefig(OUT / "14_sim_tick_regime_e1_25.png", dpi=150, bbox_inches="tight")
plt.close()

# 15 flat window e1-35
fig, ax = plt.subplots(figsize=(10, 3))
e35, p35 = list(range(1, 36)), [MID[i - 1] for i in range(1, 36)]
ax.plot(e35, p35, color=C["price"], lw=2)
ax.fill_between(e35, p35, 1.003, alpha=0.2, color=C["flat"])
ax.axhline(1.005, color="#999", ls="--")
ax.set_ylim(1.002, 1.008)
ax.annotate("OQ1 genesis + stable book\nNo price discovery yet — correct output", xy=(18, 1.0062), ha="center", fontsize=10,
            bbox=dict(boxstyle="round", facecolor="white", edgecolor="#ccc"))
ax.set_title("Flat Stable Window E1-35 @ $1.005", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "15_flat_stable_window.png", dpi=150, bbox_inches="tight")
plt.close()

# 16 mcap full
fig, ax = plt.subplots(figsize=(12, 4))
ax.plot(EVENTS100, MCAP, color=C["validators"], lw=1.5)
shade(ax)
flat_band(ax)
ax.annotate("Flat mcap ~$5.63M\n(5.6M float × $1.005)", xy=(17, 5.64), fontsize=9, color="#555")
ax.set_title("Market Cap Proxy — 5.6M float × mid", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "16_mcap_proxy_full.png", dpi=150, bbox_inches="tight")
plt.close()

# 17 funded vs unfunded
fig, ax = plt.subplots(figsize=(8, 5))
labels = ["Unfunded peak\n(SIM E46)", "Unfunded stress\n(SIM E75)", "Funded TGE\n(low)", "Funded TGE\n(high)"]
vals = [7.0, 0.47, 15.0, 25.0]
ax.bar(labels, vals, color=["#4A90D9", "#E85555", "#50C878", "#50C878"], alpha=0.85)
ax.set_ylabel("Combined book depth (M USDC)")
ax.set_title("Funded vs Unfunded Depth at Launch", fontweight="bold")
ax.grid(axis="y", alpha=0.5)
fig.tight_layout()
fig.savefig(OUT / "17_funded_vs_unfunded_depth.png", dpi=150, bbox_inches="tight")
plt.close()

# 18 — 554-day launch timeline (horizontal phases)
PHASES = [
    ("Pre-TGE\nP0–1", 90, "#9B9B9B"),
    ("TGE\nDay 0", 1, "#4A90D9"),
    ("Bootstrap\nD1–90", 90, "#50C878"),
    ("Stabilize\nD91–180", 90, "#E8A838"),
    ("Cliff +\nMature burn", 1, "#E85555"),
    ("CEX/B2B\nD181–365", 185, "#4A90D9"),
    ("Institutional\nD366–554", 189, "#50C878"),
]
fig, ax = plt.subplots(figsize=(12, 3))
left = 0
for label, width, color in PHASES:
    ax.barh(0, width, left=left, height=0.5, color=color, alpha=0.85, edgecolor="white")
    ax.text(left + width / 2, 0, label, ha="center", va="center", fontsize=8, color="white", fontweight="bold")
    left += width
ax.set_xlim(0, 554)
ax.set_xlabel("Day")
ax.set_yticks([])
ax.set_title("554-Day Launch Window — Phase Ranges (Days 0–554)", fontweight="bold")
ax.axvline(554, color="#333", ls="--", lw=1)
ax.text(554, 0.35, "Day 554", ha="right", fontsize=9)
fig.tight_layout()
fig.savefig(OUT / "18_554_day_timeline.png", dpi=150, bbox_inches="tight")
plt.close()

# 19 — burn phase ramp 2%/5%/10%
DAYS = np.arange(0, 555)
BURN_PCT = np.where(DAYS < 90, 2, np.where(DAYS < 180, 5, 10))
fig, ax = plt.subplots(figsize=(10, 4))
ax.fill_between(DAYS, BURN_PCT, alpha=0.3, color=C["burned"])
ax.plot(DAYS, BURN_PCT, color=C["burned"], lw=2)
ax.axvline(90, color="#999", ls="--", alpha=0.7)
ax.axvline(180, color="#999", ls="--", alpha=0.7)
ax.annotate("2% bootstrap", xy=(45, 2.3), fontsize=9)
ax.annotate("5% stabilization", xy=(135, 5.3), fontsize=9)
ax.annotate("10% mature", xy=(367, 10.3), fontsize=9)
ax.set_xlim(0, 554)
ax.set_ylim(0, 12)
ax.set_xlabel("Day")
ax.set_ylabel("Fee burn share (%)")
ax.set_title("Burn Phase Ramp — 2% → 5% → 10% (Days 0–554)", fontweight="bold")
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "19_burn_phase_ramp.png", dpi=150, bbox_inches="tight")
plt.close()

# 20 — vesting unlock curve (funded scenario)
VDAYS = np.array([0, 90, 180, 270, 365, 450, 554])
VCIRC = np.array([3.2, 3.8, 5.0, 5.4, 5.8, 6.0, 6.1])
VLOCK = np.array([9.0, 8.4, 7.2, 6.6, 5.8, 5.4, 5.1])
fig, ax = plt.subplots(figsize=(10, 5))
ax.fill_between(VDAYS, VCIRC, alpha=0.4, color=C["circ"], label="Circulating")
ax.fill_between(VDAYS, VCIRC + VLOCK, VCIRC, alpha=0.4, color=C["locked"], label="Locked (vesting)")
ax.plot(VDAYS, VCIRC, color=C["circ"], lw=2)
ax.plot(VDAYS, VCIRC + VLOCK, color=C["locked"], lw=2, ls="--")
ax.axvline(180, color="#E85555", ls="--", alpha=0.7, label="First cliff ~D180")
ax.set_xlim(0, 554)
ax.set_ylabel("Millions CLRTY")
ax.set_xlabel("Day")
ax.set_title("Vesting Unlock Curve — Funded Scenario (Locked vs Circulating)", fontweight="bold")
ax.legend(fontsize=8)
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "20_vesting_unlock_curve.png", dpi=150, bbox_inches="tight")
plt.close()

# 21 — 50-year cumulative burn projection (log scale)
YEARS = np.arange(0, 51)
# Model: ramping volume + 10% mature burn → asymptotic cumulative burn
annual_burn = np.array([
    0.18, 0.35, 0.52, 0.68, 0.82, 0.95, 1.06, 1.15, 1.22, 1.28,
    1.32, 1.35, 1.37, 1.38, 1.39, 1.39, 1.40, 1.40, 1.40, 1.40,
] + [1.40] * 31)  # millions CLRTY per year, tapering
cum_burn = np.cumsum(annual_burn)
fig, ax = plt.subplots(figsize=(10, 5))
ax.semilogy(YEARS, cum_burn * 1e6, color=C["burned"], lw=2)
ax.fill_between(YEARS, cum_burn * 1e6, alpha=0.15, color=C["burned"])
ax.axhline(16e6, color="#999", ls="--", label="16M hard cap")
ax.set_xlabel("Year")
ax.set_ylabel("Cumulative burned (CLRTY, log scale)")
ax.set_title("50-Year Supply Projection — Cumulative Fee Burn (Model)", fontweight="bold")
ax.legend()
ax.grid(alpha=0.3, which="both")
fig.tight_layout()
fig.savefig(OUT / "21_50yr_burn_projection.png", dpi=150, bbox_inches="tight")
plt.close()

# 22 — mechanism activation heatmap (bar by period)
PERIODS = ["Pre-TGE", "Day 0", "D1–90", "D91–180", "D~180", "D181–365", "D366–554", "D555+"]
MECH_COUNTS = [8, 12, 14, 16, 10, 18, 15, 12]
fig, ax = plt.subplots(figsize=(10, 4))
bars = ax.bar(PERIODS, MECH_COUNTS, color=[C["treasury"], C["price"], C["circ"], C["staked"],
                                            C["burned"], C["locked"], C["validators"], C["flat"]], alpha=0.85)
ax.set_ylabel("Primary mechanisms active")
ax.set_title("Moniverse Mechanism Activation by Launch Period", fontweight="bold")
ax.set_xticklabels(PERIODS, rotation=30, ha="right")
ax.grid(axis="y", alpha=0.5)
fig.tight_layout()
fig.savefig(OUT / "22_mechanism_activation.png", dpi=150, bbox_inches="tight")
plt.close()

# 23 — OQ batch results
OQ_LABELS = ["OQ1\nGenesis", "OQ2\nLiquidity", "OQ3\nAdversarial", "OQ4\nRecovery"]
OQ_EVENTS = [25, 25, 25, 25]
OQ_PASS = [100, 100, 100, 100]
fig, ax = plt.subplots(figsize=(8, 5))
x = np.arange(4)
w = 0.35
ax.bar(x - w / 2, OQ_EVENTS, w, label="Events", color=C["price"], alpha=0.85)
ax.bar(x + w / 2, OQ_PASS, w, label="Pass rate (%)", color=C["circ"], alpha=0.85)
ax.set_xticks(x)
ax.set_xticklabels(OQ_LABELS)
ax.set_ylabel("Count / %")
ax.set_title("OQ Batch Results — SIM100 (seed 42, all PASS)", fontweight="bold")
ax.legend()
ax.grid(axis="y", alpha=0.5)
fig.tight_layout()
fig.savefig(OUT / "23_oq_batch_results.png", dpi=150, bbox_inches="tight")
plt.close()

# 24 — arbitrage spread opportunity (E76-85 shaded)
ARB_EV = [1, 10, 20, 30, 40, 50, 60, 70, 76, 80, 85, 90, 100]
ARB_SP = [SPREAD[i - 1] for i in ARB_EV]
fig, ax = plt.subplots(figsize=(10, 4))
ax.plot(ARB_EV, ARB_SP, "o-", color=C["validators"], lw=2, ms=5)
ax.axvspan(76, 85, alpha=0.2, color=C["circ"], label="OQ4 cross_arb E76–85")
ax.set_xlabel("Event #")
ax.set_ylabel("Spread (bps)")
ax.set_title("Arbitrage Spread Opportunity — Post-Stress Recovery Window", fontweight="bold")
ax.legend()
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(OUT / "24_arb_spread_opportunity.png", dpi=150, bbox_inches="tight")
plt.close()

# 25 — SIM100 dashboard 2x2 composite
DEPTH_COMB = [b + a for b, a in zip(BID, ASK)]
fig, axes = plt.subplots(2, 2, figsize=(12, 8))
axes[0, 0].plot(EVENTS100, MID, color=C["price"], lw=1.5)
shade(axes[0, 0])
axes[0, 0].set_title("Mid Price")
axes[0, 0].set_ylabel("USDC")
axes[0, 0].grid(alpha=0.3)
axes[0, 1].plot(EVENTS100, DEPTH_COMB, color=C["bid"], lw=1.5)
shade(axes[0, 1])
axes[0, 1].set_title("Combined Depth")
axes[0, 1].set_ylabel("M USDC")
axes[0, 1].grid(alpha=0.3)
axes[1, 0].plot(EVENTS100, SPREAD, color=C["validators"], lw=1.5)
shade(axes[1, 0])
axes[1, 0].set_title("Spread (bps)")
axes[1, 0].set_xlabel("Event #")
axes[1, 0].grid(alpha=0.3)
axes[1, 1].plot(EVENTS100, LAMBDA, color=C["staked"], lw=1.5)
shade(axes[1, 1])
axes[1, 1].set_title("λ Heartbeat")
axes[1, 1].set_xlabel("Event #")
axes[1, 1].grid(alpha=0.3)
fig.suptitle("SIM100 Dashboard — Full Session (seed 42)", fontweight="bold", y=1.01)
fig.tight_layout()
fig.savefig(OUT / "25_sim100_dashboard.png", dpi=150, bbox_inches="tight")
plt.close()

# 26 — layout v2 256-byte regions
REGIONS = [
    ("v1 header\n0x00–0x7F", 128, "#4A90D9"),
    ("register\nslots\n0x80–0x9F", 32, "#50C878"),
    ("zero-entropy\nlane 0xA0–0xBF", 32, "#E8A838"),
    ("bare-metal\npriority 0xC0–0xDF", 32, "#E85555"),
    ("inter-shard\nflags 0xE0–0xEF", 16, "#9B9B9B"),
    ("commit block\n0xF0–0xF7", 8, "#4A90D9"),
    ("exec nonce\n0xF8–0xFF", 8, "#50C878"),
]
fig, ax = plt.subplots(figsize=(12, 3))
left = 0
for label, size, color in REGIONS:
    ax.barh(0, size, left=left, height=0.6, color=color, alpha=0.85, edgecolor="white")
    if size >= 16:
        ax.text(left + size / 2, 0, label, ha="center", va="center", fontsize=7, color="white", fontweight="bold")
    left += size
ax.set_xlim(0, 256)
ax.set_xlabel("Byte offset")
ax.set_yticks([])
ax.set_title("TokenStorageLayoutV2 — 256-Byte On-Chain Account Layout", fontweight="bold")
fig.tight_layout()
fig.savefig(OUT / "26_layout_v2_regions.png", dpi=150, bbox_inches="tight")
plt.close()

# 31 — Chain-Shift gradient template (manual monthly fill)
fig, ax = plt.subplots(figsize=(10, 5))
ax2 = ax.twinx()
ev_sample = np.array([1, 20, 35, 51, 60, 75, 85, 100])
lam_sample = np.array([0.101, 0.115, 0.136, 0.145, 0.158, 0.165, 0.170, 0.175])
spr_sample = np.array([15, 40, 60, 80, 180, 290, 137, 160])
ax.plot(ev_sample, lam_sample, "o-", color=C["staked"], lw=2, label="λ (pulse)")
ax2.plot(ev_sample, spr_sample, "s--", color=C["validators"], lw=2, label="Spread (bps)")
ax.axvspan(51, 75, alpha=0.12, color=C["muted"], label="Adversarial band")
ax.set_xlabel("Event #")
ax.set_ylabel("λ", color=C["text"])
ax2.set_ylabel("Spread (bps)", color=C["muted"])
ax.set_title("Chain-Shift Gradient Template — fill monthly", fontweight="bold", color=C["text"])
ax.text(50, 0.108, "TEMPLATE", fontsize=9, color=C["muted"], ha="center")
fig.legend(loc="upper left", bbox_to_anchor=(0.12, 0.88), fontsize=8)
fig.tight_layout()
fig.savefig(OUT / "31_chain_shift_gradient_template.png", dpi=150, bbox_inches="tight")
plt.close()

# 32 — Register-binding node graph
fig, ax = plt.subplots(figsize=(11, 6))
ax.set_xlim(0, 10)
ax.set_ylim(0, 6)
ax.axis("off")
nodes = {
    "Wallet": (1, 3),
    "Deposit": (2.5, 3),
    "Layout256B": (4, 3),
    "RegBlock": (5.5, 3),
    "L3_4MiB": (7, 3),
    "Manifold": (8.5, 3),
    "Val1": (6.5, 5),
    "Val2": (8.5, 5),
    "PoC": (7.5, 1.2),
}
edges = [
    ("Wallet", "Deposit"), ("Deposit", "Layout256B"), ("Layout256B", "RegBlock"),
    ("RegBlock", "L3_4MiB"), ("L3_4MiB", "Manifold"),
    ("L3_4MiB", "Val1"), ("Manifold", "Val2"), ("Val1", "PoC"), ("Val2", "PoC"),
]
for name, (x, y) in nodes.items():
    box = dict(boxstyle="round,pad=0.3", facecolor=C["muted"] if "Val" in name or name == "PoC" else C["text"], alpha=0.9)
    ax.text(x, y, name.replace("_", "\n"), ha="center", va="center", fontsize=8, color="white", fontweight="bold", bbox=box)
for a, b in edges:
    x1, y1 = nodes[a]
    x2, y2 = nodes[b]
    ax.annotate("", xy=(x2 - 0.35, y2), xytext=(x1 + 0.35, y1), arrowprops=dict(arrowstyle="->", color="#666", lw=1.5))
ax.set_title("Register Binding — Wallet Capital → L3 Partition → Validator Mesh", fontweight="bold")
fig.tight_layout()
fig.savefig(OUT / "32_register_binding_nodegraph.png", dpi=150, bbox_inches="tight")
plt.close()

# 33 — Treasury transparency 100% (CVP L1)
labels33 = ["Treasury", "Validators", "Liquidity", "Ecosystem", "Public"]
vals33 = [4, 3, 4, 3, 2]
cols33 = [C["treasury"], C["validators"], C["liquidity"], C["ecosystem"], C["public"]]
fig, ax = plt.subplots(figsize=(8, 5))
wedges, texts, autotexts = ax.pie(vals33, labels=labels33, colors=cols33, autopct="%1.1f%%",
    startangle=90, wedgeprops=dict(width=0.42, edgecolor=C["canvas"], linewidth=1),
    textprops=dict(color=C["text"]))
for t in autotexts:
    t.set_color(C["accent"])
    t.set_fontweight("bold")
ax.set_title("Treasury Transparency — 100% Genesis Map (16M CLRTY)", fontweight="bold", color=C["text"])
fig.patch.set_facecolor(C["canvas"])
fig.tight_layout()
fig.savefig(OUT / "33_treasury_transparency_100pct.png", dpi=150, bbox_inches="tight", facecolor=C["canvas"])
plt.close()

# 34 — Abstraction Logic L0→L3 ladder (CVP L3 / Chart 30 asset)
fig, ax = plt.subplots(figsize=(10, 4))
levels = ["L0\nSurface", "L1\nFlow", "L2\nPressure", "L3\nSystems"]
depth = [1, 2, 3, 4]
grays = ["#cccccc", "#999999", "#555555", "#111111"]
bars = ax.barh(levels, depth, color=grays, alpha=0.95, edgecolor=C["canvas"], linewidth=0.5)
ax.set_xlim(0, 5)
ax.set_xlabel("Visual depth", color=C["muted"])
ax.set_title("Abstraction Logic — L0→L3 Visual-Depth Ladder", fontweight="bold", color=C["text"])
ax.tick_params(colors=C["muted"])
for b, d in zip(bars, depth):
    ax.text(d + 0.1, b.get_y() + b.get_height() / 2, f"depth {d}", va="center", color=C["text"], fontsize=9)
fig.patch.set_facecolor(C["canvas"])
fig.tight_layout()
fig.savefig(OUT / "34_abstraction_logic_ladder.png", dpi=150, bbox_inches="tight", facecolor=C["canvas"])
plt.close()

print(f"Generated {len(list(OUT.glob('*.png')))} PNG files in {OUT}")
