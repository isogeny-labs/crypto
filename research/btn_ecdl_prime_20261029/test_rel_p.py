import sys, random, time, numpy as np
from btn import Net, contract
from prime_field import Fp, CurveP, on_curve_p, add_relation, _fix_const, _lt_const

p = int(sys.argv[1]) if len(sys.argv) > 1 else 5
a, b = (int(sys.argv[2]), int(sys.argv[3])) if len(sys.argv) > 3 else (1, 1)
F = Fp(p); C = CurveP(F, a, b); d = F.d
pts = C.points()
print(f"p={p} a={a} b={b} affine points {len(pts)}")
rng = random.Random(1)
ok = True
for _ in range(2):
    R = rng.choice(pts)
    brute = sum(1 for A in pts for B in pts if A[0] != B[0] and C.add(A, B) == R)
    net = Net()
    P1 = (net.word(d), net.word(d)); P2 = (net.word(d), net.word(d))
    Rw = (net.word(d), net.word(d)); _fix_const(net, Rw[0], R[0]); _fix_const(net, Rw[1], R[1])
    on_curve_p(net, C, P1); on_curve_p(net, C, P2)
    add_relation(net, C, P1, P2, Rw)
    t = time.time(); cnt = contract(net.nv, net.cons); dt = time.time() - t
    g = cnt == brute; ok &= g
    print(f"  R={R} brute={brute} net={cnt} vars={net.nv} {dt:.0f}s", "OK" if g else "MISMATCH")
sys.exit(0 if ok else 1)
