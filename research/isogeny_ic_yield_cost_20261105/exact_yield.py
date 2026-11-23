# exact relation probability per (curve, m) for the fixed-|FB| factor base: |T_m ∩ <G>| / q,
# plus 2-Sylow structure and the distribution of FB points over the 8 cofactor cosets
import sys
M = int(sys.argv[1]); NAMES = sys.argv[2].split(","); NMC = int(sys.argv[3]) if len(sys.argv) > 3 else 0
sys.argv = ["x", "1", str(M), ",".join(f"{n}:fixedK" for n in NAMES), "/tmp/unused"]
exec(open("ic_isogeny.py").read().split("# ---------- S3 / S4 cross-check")[0])
from collections import Counter
for name in NAMES:
    c = byname[name]; E = c["E"]
    a, b, B, fb = build_fb(E, M, "fixedK", c); add, neg, mul = mk(a)
    T, cnt = build_table(fb, M, add, neg)
    insub = [S for S in T if mul(q, S) is None]
    exact = float(len(insub)) / float(q)
    # coset label of each FB point = its 2-power-torsion component q*P (as a point of E[8])
    cos = Counter(str(mul(q, P)) for P in fb)
    inv = E.abelian_group().invariants()
    print(f"m={M}\t{name}\t|FB|={len(fb)}\t|T|={len(T)}\t|T∩<G>|/q={exact:.4f}\t|T|/#E={float(len(T))/float(N):.4f}\tgroup={inv}\tcosets={sorted(cos.values(), reverse=True)}", flush=True)
    # fixed-n Monte Carlo, no stopping rule: R = a*phi(G) for uniform a
    if len(sys.argv) and NMC:
        G = tup(c["phi"](G0)); r = random.Random(4242); hits = 0
        for _ in range(NMC):
            if mul(r.randrange(1, q), G) in T: hits += 1
        ph = hits / NMC; z = (ph - exact) / math.sqrt(exact*(1-exact)/NMC)
        print(f"MC\tm={M}\t{name}\tn={NMC}\tphat={ph:.4f}\texact={exact:.4f}\tz={z:+.2f}", flush=True)
