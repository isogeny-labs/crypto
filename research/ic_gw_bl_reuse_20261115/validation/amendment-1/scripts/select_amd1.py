#!/usr/bin/env python3
"""Validator unit selection for amendment 1 of ic_gw_bl_reuse_20261115 (fixed before the nonce was drawn).
usage: select_amd1.py NONCE_HEX > plan.json
Universe q in {262111, 524287, 1048507, 2097091, 4194271} x seed 1..12. Rank by sha256('<nonce>:amd1:<q>:<seed>').
Take the first 4; extend in rank order until >= 1 seed in 7..12, >= 1 with q >= 2097091 and >= 1 seed in 1..6.
Block-width units: among pairs with q <= 1048507 in rank order, the first gets block 8, the second block 2.
Each selected configuration runs REF, BLR1 and the oracle build AUD1; AUD1 also runs on every producer cell
(block 4, seeds 1-6) not already selected."""
import hashlib, json, sys
QS = [262111, 524287, 1048507, 2097091, 4194271]
nonce = sys.argv[1]
univ = sorted((hashlib.sha256(f"{nonce}:amd1:{q}:{s}".encode()).hexdigest(), q, s) for q in QS for s in range(1, 13))
def ok(sel):
    return (any(s >= 7 for _, q, s, _i in sel) and any(q >= 2097091 for _, q, s, _i in sel) and any(s <= 6 for _, q, s, _i in sel))
sel = []
for i, u in enumerate(univ):
    if len(sel) < 4 or not ok(sel):
        sel.append(u + (i,))
    else:
        break
small = [(i, u) for i, u in enumerate(univ) if u[1] <= 1048507]
blocks = [{"q": u[1], "seed": u[2], "rank": i, "key": u[0], "block": b} for (i, u), b in zip(small[:2], [8, 2])]
json.dump({"nonce": nonce, "sample": [{"q": q, "seed": s, "rank": i, "key": k, "block": 4} for k, q, s, i in sel],
           "block_units": blocks}, sys.stdout, indent=1)
print()
