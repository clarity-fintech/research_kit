#!/usr/bin/env python3
"""Run Python ABM mirror layer against Rust batch output."""
from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "simulators/state_space/fixtures"
BOOK_PATH = FIXTURES / "synthetic_book.json"

sys.path.insert(0, str(Path(__file__).resolve().parent))

from synthetic_book import SyntheticBook  # noqa: E402
from mirra_order_book import MIRRAOrderBook, Order  # noqa: E402
from actors import fomo, arb, panic, holder  # noqa: E402
from pump_logic import execute_pump, maintenance_band  # noqa: E402
from telemetry import read_ndjson, read_ticks, read_telemetry  # noqa: E402


def run_rust_batch(seed: int) -> None:
    subprocess.run(
        [
            "cargo",
            "run",
            "-p",
            "clrty-state-space-sim",
            "--",
            "batch",
            "--seed",
            str(seed),
            "--events",
            "1-100",
            "--write-fixture",
        ],
        cwd=ROOT,
        check=True,
    )


def mirror_pass(seed: int, skip_rust: bool) -> None:
    if not skip_rust:
        run_rust_batch(seed)
    book = SyntheticBook()
    mirra = MIRRAOrderBook()
    mirra.seed_liquidity(book.mid, book.bid_depth_usdc)
    staked: dict[str, float] = {}
    prev = book.mid
    ndjson = FIXTURES / "batch_100.ndjson"
    events = list(read_ndjson(ndjson))
    for event in events:
        regime = event.get("regime", "Stable")
        if regime == "Expansion":
            execute_pump(book, book.mid * 1.05, 0.01)
        fomo.react(book, prev)
        arb.react(book)
        panic.react(book, prev)
        holder.react(book, staked)
        maintenance_band(book, 0.95, 1.15)
        mirra.seed_liquidity(book.mid, book.bid_depth_usdc)
        if regime == "Adversarial" and event.get("event_id", 0) % 5 == 0:
            mirra.match_order(Order(side="BUY", price=book.ask, volume=100_000.0, order_id=event["event_id"]))
        prev = book.mid
    book.save(BOOK_PATH)
    ticks = read_ticks(FIXTURES / "batch_100_ticks.json")
    telemetry = read_telemetry(FIXTURES / "batch_100_telemetry.ndjson")
    merkle = (FIXTURES / "batch_100_merkle.txt").read_text().strip()
    print(
        f"mirror ok seed={seed} events={len(events)} ticks={len(ticks)} "
        f"telemetry={len(telemetry)} staked={sum(staked.values()):.0f} merkle={merkle}"
    )


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--seed", type=int, default=42)
    p.add_argument("--skip-rust", action="store_true")
    args = p.parse_args()
    mirror_pass(args.seed, args.skip_rust)


if __name__ == "__main__":
    main()
