"""Counting null for minimal cores: greedy deletion on a RANDOM set of 'decomposable' abscissae of matched density."""
import random, json, sys, statistics as st
import numpy as np
from pysat.solvers import Glucose4
from pdp import Setup, build2, assumptions
from run import MASTER

out = {}
for n, l, nsamp in ((11, 3, 300), (13, 4, 150), (17, 5, 0)):
    seed = MASTER + 1000 * l
    S = Setup(n, l, seed)
    if nsamp:
        I = build2(S); s = Glucose4(bootstrap_with=I['clauses'])
        rng = random.Random(seed + 77); sat = 0
        for _ in range(nsamp):
            x = rng.randrange(1, 1 << n)
            sat += bool(s.solve(assumptions=assumptions(I, x)))
        p = sat / nsamp; src = 'solver on %d uniform abscissae (curve and twist side)' % nsamp
    else:
        meta = json.loads(open('raw_l5.jsonl').readline())
        p = 2 ** (3 * l) / 12 / 2 ** (n - 1); src = 'heuristic 2^(3l)/12 / 2^(n-1) (the run.py draw had 0 decomposable of %d, too few to estimate)' % meta['random_drawn']
    nprng = np.random.default_rng(seed + 5); rng = random.Random(seed + 6)
    sizes = []
    for trial in range(300):
        D = nprng.random(1 << n) < p
        while True:
            x = rng.randrange(1 << n)
            if not D[x]:
                break
        fam = np.array([x]); kept = 0
        order = list(range(n))
        if trial % 2:
            rng.shuffle(order)
        for b in order:
            f2 = np.concatenate([fam, fam ^ (1 << b)])
            if D[f2].any():
                kept += 1
            else:
                fam = f2
        sizes.append(kept)
    out[str(l)] = dict(n=n, density=round(p, 4), density_source=src, null_min_core_mean=round(st.mean(sizes), 2), null_min_core_median=st.median(sizes),
                       null_min=min(sizes), null_max=max(sizes), null_p10=sorted(sizes)[30], null_p90=sorted(sizes)[270], trials=300)
    print(l, out[str(l)], flush=True)
json.dump(out, open('core_null.json', 'w'), indent=1)
