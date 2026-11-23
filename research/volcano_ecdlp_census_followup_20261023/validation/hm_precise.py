#!/usr/bin/env python3
"""H-M knife-edge check (validator-written): high-precision permutation p for
the ic arm of every M curve (rows shuffled across the 8 models, as s5c), with
2,000,000 shuffles per curve, plus the parametric one-way ANOVA p, and the
Holm-adjusted values across the 12 curves. Usage:
python3 -I hm_precise.py STUDY_DIR ROWS_DIR OUT_JSON [NPERM]"""
import json, os, sys
import numpy as np
from scipy import stats
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vfu_data
STUDY, ROWS, OUT = sys.argv[1:4]
NP = int(sys.argv[4]) if len(sys.argv) > 4 else 2_000_000
df, verts, tcr, checks = vfu_data.load(STUDY, ROWS)
out = {}
for (V, v), g in df[(df.fam == "M") & (df.arm == "ic")].groupby(["V", "vertex"]):
    groups = [g[g.model == m].sort_values(["tidx", "round"])["s"].to_numpy(float) for m in sorted(g.model.unique())]
    x = np.concatenate(groups); k = len(groups); n = len(groups[0]); N = len(x)
    gm = np.array([y.mean() for y in groups]); ssb = n * ((gm - x.mean()) ** 2).sum()
    ssw = sum(((y - y.mean()) ** 2).sum() for y in groups)
    F = (ssb / (k - 1)) / (ssw / (N - k))
    rng = np.random.default_rng([20261024, 98, v])
    cnt = 0; done = 0
    while done < NP:
        b = min(4000, NP - done)
        xs = x[np.argsort(rng.random((b, N)), axis=1)].reshape(b, k, n).mean(2)
        cnt += int((n * ((xs - x.mean()) ** 2).sum(1) >= ssb * (1 - 1e-12)).sum()); done += b
    out[f"{V}:{v}"] = {"F": F, "p_perm": (1 + cnt) / (NP + 1), "n_perm": NP, "p_anova": float(stats.f.sf(F, k - 1, N - k))}
curs = sorted(out)
def holm(ps):
    idx = sorted(range(len(ps)), key=lambda i: ps[i]); m = len(ps); adj = [0.0]*m; run = 0.0
    for r, i in enumerate(idx):
        run = max(run, min(1.0, (m - r) * ps[i])); adj[i] = run
    return adj
for key in ("p_perm", "p_anova"):
    for c, h in zip(curs, holm([out[c][key] for c in curs])):
        out[c][key + "_holm"] = h
for c in curs:
    p = out[c]["p_perm"]; out[c]["p_perm_mc_sd"] = (p * (1 - p) / NP) ** 0.5
out["_n_significant"] = {k: sum(out[c][k + "_holm"] < 0.05 for c in curs) for k in ("p_perm", "p_anova")}
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(out, indent=1))
