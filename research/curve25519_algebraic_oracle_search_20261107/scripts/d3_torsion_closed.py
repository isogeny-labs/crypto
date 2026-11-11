"""D3: base closed under E[8] translation (in twisted Edwards form E[4] acts linearly:
(u, v) -> (-u, -v), (v, -u)-type maps, so orbits hash canonically for free).
Compare, at the same number F' of unknowns, an E[r]-only base B' with its closure B = B' + E[8]:
coverage (hit rate on random R in E[r]) and counted decision cost, testing (i) every point of B,
(ii) one representative per orbit."""
import json, random, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fp
from fp import p, A, r, mul, CNT, batch_inv
D = pathlib.Path(__file__).resolve().parents[1]
rng = random.Random(202611073)
while True:
    P0 = fp.u_lift(rng.randrange(p))
    if P0 is None: continue
    T8 = fp.u_mul(r, P0)
    if fp.u_mul(4, T8) is not None: break
E8 = [None]; Q = T8
for _ in range(7): E8.append(Q); Q = fp.u_add(Q, T8)

def decide(R, pts, Bx):
    xR, yR = R; invs = batch_inv([(xR - x) % p for x, _ in pts]); hit = 0
    for (x1, y1), iv in zip(pts, invs):
        for sg in (1, -1):
            lam = mul((yR - sg * y1) % p, iv); x3 = (mul(lam, lam) - A - xR - x1) % p
            if x3 in Bx: hit = 1
    return hit

out = []
for Fp_ in (128, 256, 384):
    Bp = []; seen = set()
    while len(Bp) < Fp_:
        P = fp.u_rand_Er(rng)
        if P[0] not in seen: seen.add(P[0]); Bp.append(P)
    Bc = [fp.u_add(P, T) for P in Bp for T in E8]; Bc = [P for P in Bc if P is not None]
    Bx_r = {P[0] for P in Bp}; Bx_c = {P[0] for P in Bc}
    res = dict(F_unknowns=Fp_, closure_points=len(Bc))
    for name, pts, Bx in (("Er_only", Bp, Bx_r), ("closure_all_points", Bc, Bx_c), ("closure_reps", Bp, Bx_c)):
        hits = 0; cost = 0; T = 300
        for _ in range(T):
            R = fp.u_rand_Er(rng); fp.reset(); hits += decide(R, pts, Bx); cost += CNT['M']
        res[name] = dict(hit_rate=hits / T, c=cost / T, c_per_unknown=cost / T / Fp_)
    out.append(res); print(res)
json.dump(out, open(D / "data/d3_torsion.json", "w"), indent=1)
