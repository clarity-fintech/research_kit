"""Unified telemetry schema for ABM simulation."""
from __future__ import annotations

import json
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any, Iterator


@dataclass
class StateSnapshot:
    mid_price: float
    bid_depth_usdc: float
    ask_depth_usdc: float
    staked_nano: float = 0.0
    entropy_root: str = ""


@dataclass
class TelemetryLog:
    timestamp: str
    event_type: str
    event_id: int
    pre_state: StateSnapshot
    post_state: StateSnapshot
    status: str
    event_hash: str
    agent_actions: list[dict[str, Any]]
    chain_id: str = "clrty-1"


def read_ndjson(path: Path) -> Iterator[dict[str, Any]]:
    if not path.is_file():
        return iter(())
    for line in path.read_text().splitlines():
        line = line.strip()
        if line:
            yield json.loads(line)


def read_ticks(path: Path) -> list[dict[str, Any]]:
    if not path.is_file():
        return []
    return json.loads(path.read_text())


def read_telemetry(path: Path) -> list[dict[str, Any]]:
    return list(read_ndjson(path))


def write_telemetry_log(path: Path, log: TelemetryLog) -> None:
    with path.open("a") as f:
        f.write(json.dumps(asdict(log)) + "\n")
