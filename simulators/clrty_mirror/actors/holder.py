"""Long-term holder — stakes tokens and ignores short-term volatility."""
from synthetic_book import SyntheticBook


def react(book: SyntheticBook, staked: dict[str, float]) -> None:
    wallet = "holder_0"
    staked[wallet] = staked.get(wallet, 0.0) + 10_000_000.0
    book.bid_depth_usdc *= 1.005
