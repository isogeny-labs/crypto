# null model: exact yield |T_m ∩ <G>|/q for random same-size factor bases on E0 alone (random x's, no isogeny)
import sys
M = int(sys.argv[1]); NREP = int(sys.argv[2]); SEED = int(sys.argv[3])
sys.argv = ["x", "1", str(M), "E0:fixedK", "/tmp/unused"]
exec(open("ic_isogeny.py").read().split("# ---------- S3 / S4 cross-check")[0])
a, b, B, fb0 = build_fb(E0, M, "fixedK", curves[0]); K = len(fb0); add, neg, mul = mk(a)
r = random.Random(SEED)
for i in range(NREP):
    xs = set()
    while len(xs) < K:
        x = r.randrange(p)
        if E0.is_x_coord(F(x)): xs.add(x)
    fb = [tup(E0.lift_x(F(x))) for x in sorted(xs)]
    T, cnt = build_table(fb, M, add, neg)
    print(f"NULL\t{M}\t{float(sum(1 for S in T if mul(q, S) is None))/float(q):.5f}", flush=True)
