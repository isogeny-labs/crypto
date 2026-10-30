import random, sys
import numpy as np
from btn import Field, Curve, contract
from relation import build

n = int(sys.argv[1]) if len(sys.argv) > 1 else 7
F = Field(n, {3: 0b1011, 5: 0b100101, 7: (1 << 7) | 3, 9: (1 << 9) | (1 << 4) | 1}[n])
C = Curve(F)
pts = [(x, y) for x in range(1 << n) for y in range(1 << n) if C.on((x, y))]
print("n", n, "affine points", len(pts))
rng = random.Random(1)
full = [True] * (1 << n)
# brute count of (A1,A2) with A1+A2=R, x1!=x2, both on curve
for trial in range(3):
    R = rng.choice(pts)
    brute = 0
    for A in pts:
        for B in pts:
            if A[0] != B[0]:
                S = C.add(A, B)
                if S == R: brute += 1
    net, iface, _, _ = build(F, R, (full, full))
    cnt = contract(net.nv, net.cons)
    print("R", R, "brute", brute, "network", cnt, "OK" if brute == cnt else "MISMATCH")
    assert brute == cnt
print(net.stats())
import btn; print("max bucket width", btn.MAXW)
