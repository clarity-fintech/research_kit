"""Panic seller — contracts ask side on drops."""
from synthetic_book import SyntheticBook


def react(book: SyntheticBook, prev_mid: float) -> None:
    if book.mid < prev_mid * 0.999:
        book.ask_depth_usdc *= 0.8
        book.mid *= 0.998
        book.bid = book.mid - 0.005
        book.ask = book.mid + 0.005
