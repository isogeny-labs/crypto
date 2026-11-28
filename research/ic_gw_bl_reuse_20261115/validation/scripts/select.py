#!/usr/bin/env python3
"""Validator unit selection for ic_gw_bl_reuse_20261115 (fixed before the nonce was drawn).

usage: select.py NONCE_HEX > plan.json

Universe: q in {262111, 524287, 1048507, 2097091, 4194271} x seed in 1..12 (60 (q, seed) pairs).
Rank by sha256('<nonce>:<q>:<seed>') ascending. Take the first 6; then append in rank order until
>= 2 selected pairs have seed in 7..12, >= 2 have q >= 2097091 and >= 2 have seed in 1..6.
Each selected pair is run with REF and BLR at the default block width (4).
Block-width units (REF and BLR each): among pairs with q <= 1048507 in rank order, the first gets
block 8, the second block 2, the third block 1, the fourth block 3 (gamma 4.0 = the default).
"""
import hashlib, json, sys

QS = [262111, 524287, 1048507, 2097091, 4194271]
nonce = sys.argv[1]
univ = []
for q in QS:
    for s in range(1, 13):
        key = hashlib.sha256(f"{nonce}:{q}:{s}".encode()).hexdigest()
        univ.append((key, q, s))
univ.sort()
sel = []
def ok(sel):
    return (sum(1 for _, q, s, _i in sel if s >= 7) >= 2 and sum(1 for _, q, s, _i in sel if q >= 2097091) >= 2
            and sum(1 for _, q, s, _i in sel if s <= 6) >= 2)
for i, u in enumerate(univ):
    if len(sel) < 6 or not ok(sel):
        sel.append(u + (i,))
    else:
        break
small = [(i, u) for i, u in enumerate(univ) if u[1] <= 1048507]
blocks = []
for (i, u), b in zip(small[:4], [8, 2, 1, 3]):
    blocks.append({"q": u[1], "seed": u[2], "rank": i, "key": u[0], "block": b})
plan = {
    "nonce": nonce,
    "sample": [{"q": q, "seed": s, "rank": i, "key": k, "block": 4} for k, q, s, i in sel],
    "block_units": blocks,
}
json.dump(plan, sys.stdout, indent=1)
print()
