# Sensitivity analyses (validator's, not producer evidence): (A) IC vs rho2x (x-only sibling control);
# (B) solver-only cost (verification phase removed from every arm); (C) seen_pairs uncounted hashes priced; (D) fixed per-cell kappa/mf instead of per-config best-of-3.
import json, sys, numpy as np
from collections import defaultdict
rows = [json.loads(l) for l in open(sys.argv[1])]; r2x = [json.loads(l) for l in open(sys.argv[2])]
CI, HP = (3, 10, 30), (0, 0.1, 1)
cfg = defaultdict(lambda: defaultdict(dict))
for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]][r["param"]] = r
for r in r2x: cfg[(r["pb"], r["T"], r["idx"], r["seed"])]["rho2x"][r["param"]] = r
cells = sorted({k[:2] for k in cfg})
def gae(r, ci, h, solver_only=False, extra_h=0):
    I, M, S, H, L = (r["I"], r["M"], r["S"], r["H"], r["L"])
    if solver_only: I, M, S, H, L = r["phases"]["targets"]
    return ((ci*I + M + S)/(ci + 3) + h*(H + L + extra_h)) / r["targets"]
def boot(x, B=20000):
    lo, hi = 9, -9
    for s in (11, 22, 33):
        rng = np.random.default_rng(s); m = np.median(x[rng.integers(0, len(x), (B, len(x)))], axis=1)
        lo, hi = min(lo, np.quantile(m, .025)), max(hi, np.quantile(m, .975))
    return lo, hi
def table(a_ic, a_ref, f_ic, f_ref, title):
    print("\n##", title)
    for ci in CI:
        out = []; meds = []
        for c in cells:
            ks = sorted(k for k in cfg if k[:2] == c); st = []
            for h in HP:
                x = np.array([f_ic(cfg[k], ci, h) / f_ref(cfg[k], ci, h) for k in ks]); lo, hi = boot(x); st.append((np.median(x), lo, hi)); meds.append(np.median(x))
            rk = "cheaper" if all(s[2] < 1 for s in st) else ("dearer" if all(s[1] > 1 for s in st) else "unranked")
            out.append(f"{c[0]}b/T{c[1]} {rk} (" + "/".join(f"{s[0]:.3f}" for s in st) + ")")
        print(f"I={ci}M: " + "; ".join(out)); print(f"   median range {min(meds):.3f}..{max(meds):.3f}")
ic = lambda v, ci, h: gae(v["ic4"][1.0], ci, h)
best = lambda arm, **kw: (lambda v, ci, h: min(gae(r, ci, h, **kw) for r in v[arm].values()))
table("ic4", "rho2x", ic, best("rho2x"), "(A) IC4 / rho2x (x-only sibling, recomputed on hit), per-config best kappa")
table("ic4", "rho2", ic, best("rho2"), "(A0) IC4 / rho2 as produced (for comparison)")
for arm in ("rho", "rho2", "bsgs2"):
    table("ic4", arm, lambda v, ci, h: gae(v["ic4"][1.0], ci, h, solver_only=True), best(arm, solver_only=True), f"(B) solver-only (verification excluded): IC4 / {arm}")
# (C) price ic4's uncounted seen_pairs hash ops: upper bound = relations + duplicate hits; use the 'relations' count x 1.5 as a generous bound
table("ic4", "rho", lambda v, ci, h: gae(v["ic4"][1.0], ci, h, extra_h=1.5*v["ic4"][1.0]["relations"]), best("rho"), "(C) IC4 with seen_pairs hashes priced (bound 1.5 x relations) / rho")
# (D) fixed parameter per cell: for each generic arm, the single param minimizing the cell median at (ci, h)
def fixed(arm):
    def f(v, ci, h, _cache={}):
        key = (arm, ci, h, v["ic4"][1.0]["pb"], v["ic4"][1.0]["T"])
        if key not in _cache:
            ks = [k for k in cfg if k[:2] == key[3:]]
            _cache[key] = min(cfg[ks[0]][arm], key=lambda p: np.median([gae(cfg[k][arm][p], ci, h) for k in ks]))
        return gae(v[arm][_cache[key]], ci, h)
    return f
for arm in ("rho", "rho2", "bsgs2"):
    table("ic4", arm, ic, fixed(arm), f"(D) generic arm with one fixed best parameter per cell (no per-config oracle): IC4 / {arm}")
