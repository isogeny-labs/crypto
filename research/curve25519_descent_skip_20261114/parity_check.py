#!/usr/bin/env python3
"""Check used by Part B: the parity of the coset label equals chi(x_M) (the one-bit class).
Run: python3 -I parity_check.py > parity_check.out.txt"""
import random
src = open("frame_selection.py").read().split("C, S3, S1 = [], [], []")[0]
ns = {}
exec(compile(src, "fs", "exec"), ns)
leg, label, rhs, sqrt, p = ns["leg"], ns["label"], ns["rhs"], ns["sqrt"], ns["p"]
rnd = random.Random(7); ok = tot = 0
for _ in range(3000):
    xm = rnd.randrange(1, p); y = sqrt(rhs(xm))
    if y is None or y == 0: continue
    tot += 1; ok += ((-1) ** label((xm, y)) == leg(xm))
print("points %d, (-1)^label == chi(x_M): %d (%.4f)" % (tot, ok, ok / tot))
