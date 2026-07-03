"""Arbitrage agent — mean-reverts wide spreads."""
from synthetic_book import SyntheticBook


def react(book: SyntheticBook) -> None:
    spread = book.ask - book.bid
    if spread > 0.02:
        book.mid = (book.bid + book.ask) / 2
        book.bid = book.mid - 0.005
        book.ask = book.mid + 0.005
