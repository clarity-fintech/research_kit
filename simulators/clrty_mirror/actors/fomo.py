"""FOMO agent — buys on upward mid moves."""
from synthetic_book import SyntheticBook


def react(book: SyntheticBook, prev_mid: float) -> None:
    if book.mid > prev_mid * 1.001:
        book.bid_depth_usdc *= 0.95
        book.apply_pump(0.002)
