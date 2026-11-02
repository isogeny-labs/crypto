# POST-HOC (not pre-registered): SA from random and from box+diamond starts, seeds 1..3, same 2000-move budget.
import json, random, math, itertools, time, sys
from armB import Dcalc, staircase_opt, fpoly, degsum, ksum, P
res = {}
rng0 = random.Random(0); f, crv = fpoly(2, P, rng0)
for l in (3, 5):
    R = 2*l + 4; dc = Dcalc(f, 2, 2, l); v, hs = staircase_opt(l, R)
    def score(M):
        if not M: return 0.0
        Ds, Ns = dc.bound(M, l, R); return sum(Ds)/degsum(M)
    cells = list(itertools.product(range(R+1), repeat=2))
    for seed in (1, 2, 3):
        rng = random.Random(seed)
        start = {"rand50": set(c for c in cells if rng.random() < 0.5),
                 "diamond": set(s for s in cells if abs(s[0]-s[1]) <= 0.6*R)}
        for nm, cur in start.items():
            cs = score(sorted(cur)); best = (cs, sorted(cur)); T = 0.01; t1 = time.time()
            for it in range(2000):
                c = rng.choice(cells); nxt = set(cur) ^ {c}; ns = score(sorted(nxt))
                if ns >= cs or rng.random() < math.exp((ns-cs)/T): cur, cs = nxt, ns
                if cs > best[0]: best = (cs, sorted(cur))
                T *= 0.997
            res[f"l{l}_s{seed}_{nm}"] = dict(best=best[0], stair_opt=float(v), gap=best[0]-float(v), M=best[1], secs=time.time()-t1)
            print(l, seed, nm, best[0], float(v), best[0]-float(v), flush=True)
json.dump(res, open("raw/armB2_posthoc.json", "w"), indent=1)
