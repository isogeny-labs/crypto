#!/usr/bin/env python3
"""Nonce-ranked sample of (q, seed) units for the NRM/LZ replay (validator; written before the nonce).
Universe: q in {262111, 524287, 1048507, 2097091, 4194271} x seed 1..12. Rank by sha256('<nonce>:<q>:<seed>').
Take the first 6 ranked; then extend in rank order until the sample has >= 2 seeds in 7..12,
>= 2 units with q >= 2097091 and >= 2 seeds in 1..6 (replay against committed records).
A/A (CAND without env): the first two sampled units with seed in 1..6 (falls back to the first two).
Usage: select_units.py NONCE > sample.json"""
import hashlib, json, sys
Q = [262111, 524287, 1048507, 2097091, 4194271]
nonce = sys.argv[1]
ranked = sorted(((hashlib.sha256(f"{nonce}:{q}:{s}".encode()).hexdigest(), q, s) for q in Q for s in range(1, 13)))
def ok(sm):
    return (sum(1 for _, q, s in sm if s >= 7) >= 2 and sum(1 for _, q, s in sm if q >= 2097091) >= 2
            and sum(1 for _, q, s in sm if s <= 6) >= 2)
sample = ranked[:6]
i = 6
while not ok(sample):
    sample.append(ranked[i]); i += 1
aa = [u for u in sample if u[2] <= 6][:2] or sample[:2]
out = {"nonce": nonce, "sample": [{"rank": r, "key": k, "q": q, "seed": s} for r, (k, q, s) in enumerate(sample)],
       "aa": [{"q": q, "seed": s} for _, q, s in aa]}
json.dump(out, sys.stdout, indent=1)
