#!/usr/bin/env python3
"""Tokenomics simulator — fee volatility sweeps, NTT cap stress, staking locks (S17)."""
import json
import math
import random

SUPPLY = 16_000_000
NTT_OUTBOUND_CAP = 2_500_000
PHASES = {
    "bootstrap": {"fee_burn": 0.02, "validator_share": 0.5, "lp_share": 0.35},
    "stabilization": {"fee_burn": 0.05, "validator_share": 0.4, "lp_share": 0.35},
    "mature": {"fee_burn": 0.10, "validator_share": 0.35, "lp_share": 0.30},
}


def staking_apy(staked_pct: float, annual_fees: float) -> float:
    if staked_pct <= 0:
        return 0.0
    staked = SUPPLY * staked_pct
    return (annual_fees * PHASES["mature"]["validator_share"]) / staked * 100


def simulate(days: int = 365, daily_volume: float = 1_000_000, fee_volatility: float = 0.0) -> dict:
    circulating = SUPPLY * 0.6
    burned = 0.0
    for d in range(days):
        vol_mult = 1.0 + fee_volatility * math.sin(d / 7.0)
        fees = daily_volume * 0.003 * vol_mult
        phase = "bootstrap" if d < 90 else ("stabilization" if d < 180 else "mature")
        burned += fees * PHASES[phase]["fee_burn"]
    return {
        "days": days,
        "circulating_start": circulating,
        "total_burned": burned,
        "staking_apy_pct": round(staking_apy(0.4, daily_volume * 365 * 0.003), 4),
        "fee_volatility": fee_volatility,
    }


def stress_ntt_outbound(transfers_per_window: int, clip_clrtY: float = 100_000) -> dict:
    total = transfers_per_window * clip_clrtY
    capped = min(total, NTT_OUTBOUND_CAP)
    queued = max(0, total - NTT_OUTBOUND_CAP)
    return {"requested": total, "executed": capped, "queued": queued, "cap": NTT_OUTBOUND_CAP}


def stress_staking_locks(tier4_pct: float = 0.15) -> dict:
    locked = SUPPLY * tier4_pct
    float_supply = SUPPLY - locked
    velocity_reduction = locked / SUPPLY
    return {"tier4_locked": locked, "float_supply": float_supply, "velocity_reduction_pct": velocity_reduction * 100}


def run_scenarios() -> dict:
    return {
        "baseline": simulate(),
        "high_volatility": simulate(fee_volatility=0.5),
        "ntt_stress": stress_ntt_outbound(30, 100_000),
        "staking_defense": stress_staking_locks(),
    }


if __name__ == "__main__":
    print(json.dumps(run_scenarios(), indent=2))
