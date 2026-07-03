#!/usr/bin/env python3
"""Synthetic CLRTY order book for mirror simulation."""
from __future__ import annotations

import json
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any


@dataclass
class SyntheticBook:
    venue: str = "clrty-l1-sim"
    bid: float = 1.0
    ask: float = 1.01
    mid: float = 1.005
    bid_depth_usdc: float = 5_000_000.0
    ask_depth_usdc: float = 5_000_000.0

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)

    def apply_pump(self, magnitude: float) -> None:
        self.mid *= 1.0 + magnitude
        self.bid = self.mid - 0.005
        self.ask = self.mid + 0.005
        self.ask_depth_usdc *= max(0.5, 1.0 - magnitude)

    def save(self, path: Path) -> None:
        path.write_text(json.dumps(self.to_dict(), indent=2))

    @classmethod
    def load(cls, path: Path) -> "SyntheticBook":
        data = json.loads(path.read_text())
        return cls(**data)
