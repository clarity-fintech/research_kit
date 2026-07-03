"""MIRRA venue — deterministic order book matching engine."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal


Side = Literal["BUY", "SELL"]


@dataclass
class Order:
    side: Side
    price: float
    volume: float
    order_id: int


@dataclass
class MatchResult:
    filled_volume: float
    avg_price: float
    slippage_bps: float


@dataclass
class MIRRAOrderBook:
    venue: str = "MIRRA"
    bids: dict[float, float] = field(default_factory=dict)
    asks: dict[float, float] = field(default_factory=dict)

    def seed_liquidity(self, mid: float = 1.005, depth: float = 5_000_000.0) -> None:
        self.bids[mid - 0.005] = depth
        self.asks[mid + 0.005] = depth

    def best_bid(self) -> float:
        return max(self.bids.keys()) if self.bids else 0.0

    def best_ask(self) -> float:
        return min(self.asks.keys()) if self.asks else 0.0

    def mid(self) -> float:
        bb, ba = self.best_bid(), self.best_ask()
        if bb > 0 and ba > 0:
            return (bb + ba) / 2
        return bb or ba

    def match_order(self, order: Order) -> MatchResult:
        """Deterministic matching — walks price levels without front-running."""
        if order.side == "BUY":
            return self._process_buy(order)
        return self._process_sell(order)

    def _process_buy(self, order: Order) -> MatchResult:
        remaining = order.volume
        cost = 0.0
        filled = 0.0
        ref = self.best_ask() or order.price
        for price in sorted(self.asks.keys()):
            if price > order.price or remaining <= 0:
                break
            take = min(remaining, self.asks[price])
            self.asks[price] -= take
            if self.asks[price] <= 0:
                del self.asks[price]
            filled += take
            cost += take * price
            remaining -= take
        avg = cost / filled if filled else ref
        slip = abs(avg - ref) / ref * 10_000 if ref else 0.0
        return MatchResult(filled_volume=filled, avg_price=avg, slippage_bps=slip)

    def _process_sell(self, order: Order) -> MatchResult:
        remaining = order.volume
        proceeds = 0.0
        filled = 0.0
        ref = self.best_bid() or order.price
        for price in sorted(self.bids.keys(), reverse=True):
            if price < order.price or remaining <= 0:
                break
            take = min(remaining, self.bids[price])
            self.bids[price] -= take
            if self.bids[price] <= 0:
                del self.bids[price]
            filled += take
            proceeds += take * price
            remaining -= take
        avg = proceeds / filled if filled else ref
        slip = abs(ref - avg) / ref * 10_000 if ref else 0.0
        return MatchResult(filled_volume=filled, avg_price=avg, slippage_bps=slip)
