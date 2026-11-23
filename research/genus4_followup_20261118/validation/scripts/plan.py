#!/usr/bin/env python3
"""Replay subset drawn from a validator nonce (recorded in out/nonce.txt).
Cells are (p, curve_seed, k); every cell is replayed with all its seeds (the CLI runs seeds 1..S).
Usage: plan.py NONCE_HEX > plan.json"""
import hashlib, json, random, sys
nonce = sys.argv[1]
rng = random.Random(int(hashlib.sha256(bytes.fromhex(nonce)).hexdigest(), 16))
C16, C8 = (172, 180, 2640), (363, 393, 407, 457, 474, 519)
cells16 = [(16411, c, k) for c in C16 for k in range(18)]
# part B IC (arms w and x, seeds 1-2): 8 cells -> 16 x rows and 16 w rows, at least one per curve
while True:
    b = rng.sample(cells16, 8)
    if {c for _, c, _ in b} == set(C16):
        break
# part A: one cell per cc at p = 8209 (arm x, seeds 1-2) -> 10 rows at 5 cc values
a = [{"cc": cc, "cell": (8209, rng.choice(C8), rng.randrange(3))} for cc in ("0.5", "0.7", "1.0", "1.4", "2.0")]
# Q: one new-target cell (k = 3..17, seeds 1-2) and one lean-rho cell (k = 0..2, seed 1, not the G3 key)
qn = rng.choice([x for x in cells16 if x[2] >= 3])
qo = rng.choice([x for x in cells16 if x[2] <= 2 and x[1:] != (172, 0)])
# extension drawn after the draws above (they are unchanged): one cell of the resumed curve-2640 block (b_q.jsonl.r1)
qr = rng.choice([x for x in cells16 if x[1] == 2640 and x[2] >= 3])
print(json.dumps({"nonce": nonce, "B_ic_cells": sorted(b), "A_cells": a,
                  "Q_new_cell": qn, "Q_new_seeds": 2, "Q_old_cell": qo, "Q_old_seeds": 1, "Q_r1_cell": qr, "Q_r1_seeds": 1}, indent=1))
