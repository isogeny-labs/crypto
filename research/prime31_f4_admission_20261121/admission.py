#!/usr/bin/env python3
"""Exact Prime31 run50 F4 admission arithmetic, not a solver or timing benchmark."""
from math import comb

H = (2327, 983, 421, 207, 87, 34, 22, 9)
N = 4096
MISSES = 6
H1024 = (580, 233, 124, 50, 23, 6, 6, 2)
HISTORICAL_BSGS_1024_MS = 170.113

def call_counts(hist=H, misses=MISSES):
    assert sum(hist) + misses == N
    return tuple(sum(hist[i:]) + misses for i in range(len(hist)))

def dense_envelope(v, d):
    m = comb(v + d, d)
    return m, 4 * m * m / 2 ** 40

def main():
    calls = call_counts()
    assert calls == (4096, 1769, 786, 365, 158, 71, 37, 15)
    assert sum(calls) == 7297
    assert sum(H1024) == 1024
    calls_1024 = sum((i + 1) * n for i, n in enumerate(H1024))
    assert calls_1024 == 1827
    print("7-leaf 1024 first-hit histogram:", H1024)
    print("7-leaf 1024 calls:", calls_1024)
    print("historical 1024-target BSGS sensitivity ms/7-leaf-call (all else free):", HISTORICAL_BSGS_1024_MS / calls_1024)
    print("7-leaf arm calls:", calls)
    print("total calls:", sum(calls))
    print("per-target calls:", sum(calls) / N)
    print("7-leaf per-call / 8-leaf per-call crossover:", (N - MISSES) / sum(calls))
    for v in (16, 31, 64, 74):
        d4, _ = dense_envelope(v, 4)
        d5, tib = dense_envelope(v, 5)
        print(f"vars={v}: degree<=4 {d4}, degree<=5 {d5}, 32-bit dense square TiB={tib:.9f}")
    assert dense_envelope(64, 4)[0] == 814385
    assert dense_envelope(64, 5)[0] == 11238513

if __name__ == "__main__":
    main()
