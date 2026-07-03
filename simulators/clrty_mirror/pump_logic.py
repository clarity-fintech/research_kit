"""Pump / extraction / maintenance modes."""
from __future__ import annotations

from synthetic_book import SyntheticBook


def execute_pump(book: SyntheticBook, target_price: float, magnitude: float) -> None:
    for _ in range(50):
        if book.mid >= target_price or magnitude < 1e-9:
            break
        book.apply_pump(magnitude * 0.25)
        magnitude *= 0.9


def maintenance_band(book: SyntheticBook, low: float, high: float) -> None:
    if book.mid < low:
        book.apply_pump(0.001)
    elif book.mid > high:
        book.mid = high
        book.bid = book.mid - 0.005
        book.ask = book.mid + 0.005
