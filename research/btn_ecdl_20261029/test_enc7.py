import random, time, sys
from btn import Field, Curve, contract_cut
from relation import build
n = 7
F = Field(n, (1 << 7) | 3); C = Curve(F)
pts = [(x, y) for x in range(1 << n) for y in range(1 << n) if C.on((x, y))]
rng = random.Random(3)
full = [1] * (1 << n)
for _ in range(int(sys.argv[1]) if len(sys.argv) > 1 else 2):
    R = rng.choice(pts)
    brute = sum(1 for A in pts for B in pts if A[0] != B[0] and C.add(A, B) == R)
    net, iface, _, _ = build(F, R, (full, full))
    t = time.time(); cnt = contract_cut(net.nv, net.cons, iface[:n])
    print("n=7 R", R, "brute", brute, "network", cnt, "OK" if brute == cnt else "MISMATCH", f"{time.time()-t:.0f}s", flush=True)
    assert brute == cnt
