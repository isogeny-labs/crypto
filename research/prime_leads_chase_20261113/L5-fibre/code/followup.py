"""Amendment A1: additive-coincidence statistics over many curves. Usage: followup.py OUT.json bits ncurves Bt..."""
import json, os, random, statistics as st, sys
sys.path.insert(0, os.path.dirname(__file__))
import numpy as np
from ec import find_curve, fibre_points, bsgs_log, in_G
from widths import sumset_bitmaps
OUT, bits, nc = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
Bts = [int(x) for x in sys.argv[4:]]
res = {f'{a}_{Bt}': {'c2': [], 'c3': []} for Bt in Bts for a in ('A1', 'A2', 'I', 'N')}
def W(logs, r):
    maps, _, _ = sumset_bitmaps(sorted(set(logs)), r, 3)
    return int(maps[2].sum()), int(maps[3].sum())
for ci in range(nc):
    C = find_curve(bits, 50000 + 17 * ci)
    E, p, r, g, t = C['E'], C['p'], C['r'], C['g'], C['t']
    log = bsgs_log(E, g, r); rng = random.Random(ci)
    for Bt in Bts:
        arms = {}
        cand = list(range(p)); rng.shuffle(cand); F = []
        for xp in cand:
            F += fibre_points(C, xp)
            if len(F) >= Bt: break
        arms['A1'] = F
        F, xp = [], 0
        while len(F) < Bt: F += fibre_points(C, xp); xp += 1
        arms['A2'] = F
        F, x = [], 0
        while len(F) < Bt: F += [P for P in E.lift(x) if x != t and in_G(E, r, P)]; x += 1
        arms['I'] = F
        for a, F in arms.items():
            logs = [log(P) for P in F]; B = len(set(logs))
            w2, w3 = W(logs, r)
            # baseline: random symmetric sets of the same size (10 seeds)
            nw = []
            for s in range(10):
                rs = random.Random(ci * 1000 + s * 7 + B); ks = set()
                while len(ks) < B // 2: ks.add(rs.randrange(1, (r + 1) // 2))
                nw.append(W(list(ks) + [r - k for k in ks], r))
            m2 = st.mean(x[0] for x in nw); m3 = st.mean(x[1] for x in nw)
            res[f'{a}_{Bt}']['c2'].append(m2 - w2); res[f'{a}_{Bt}']['c3'].append(m3 - w3)
            if a == 'A1':
                res[f'N_{Bt}']['c2'] += [m2 - x[0] for x in nw]; res[f'N_{Bt}']['c3'] += [m3 - x[1] for x in nw]
    print(ci, p, flush=True)
summ = {}
for k, v in res.items():
    summ[k] = {c: dict(mean=st.mean(v[c]), se=(st.stdev(v[c]) / len(v[c]) ** 0.5 if len(v[c]) > 1 else 0), n=len(v[c])) for c in v}
json.dump(dict(raw=res, summary=summ), open(OUT, 'w'), indent=1)
for k, v in summ.items(): print(k, {c: (round(d['mean'], 2), round(d['se'], 2)) for c, d in v.items()})
