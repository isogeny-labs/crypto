import json, sys, numpy as np
from collections import defaultdict
rows = [json.loads(l) for l in open(sys.argv[1])]
extra = {"rhoseq": sys.argv[2], "rho2x": sys.argv[3]}
CI, HP = (3, 10, 30), (0, 0.1, 1)
cfg = defaultdict(lambda: defaultdict(dict))
for r in rows: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]][r["param"]] = r
for arm, path in extra.items():
    for l in open(path):
        r = json.loads(l); cfg[(r["pb"], r["T"], r["idx"], r["seed"])][arm][r["param"]] = r
cells = sorted({k[:2] for k in cfg})
def gae(r, ci, h): return ((ci*r["I"] + r["M"] + r["S"])/(ci + 3) + h*(r["H"] + r["L"])) / r["targets"]
def boot(x, B=20000):
    lo, hi = 9, -9
    for s in (11, 22, 33):
        rng = np.random.default_rng(s); m = np.median(x[rng.integers(0, len(x), (B, len(x)))], axis=1)
        lo, hi = min(lo, np.quantile(m, .025)), max(hi, np.quantile(m, .975))
    return lo, hi
for ref in ("rho", "rhoseq"):
    print(f"\n## IC4 / {ref} (per-config best kappa); medians h=0/0.1/1, rank under the pre-registered rule")
    n_cheaper = 0; meds = []
    for ci in CI:
        out = []
        for c in cells:
            ks = sorted(k for k in cfg if k[:2] == c); st = []
            for h in HP:
                x = np.array([gae(cfg[k]["ic4"][1.0], ci, h) / min(gae(r, ci, h) for r in cfg[k][ref].values()) for k in ks])
                lo, hi = boot(x); st.append((np.median(x), lo, hi)); meds.append(np.median(x))
            rk = "cheaper" if all(s[2] < 1 for s in st) else ("dearer" if all(s[1] > 1 for s in st) else "unranked")
            n_cheaper += rk == "cheaper"
            out.append(f"{c[0]}b/T{c[1]} {rk} (" + "/".join(f"{s[0]:.3f}" for s in st) + (f"; h=1 [{st[2][1]:.3f},{st[2][2]:.3f}]" if rk != "cheaper" else "") + ")")
        print(f"I={ci}M: " + "; ".join(out))
    print(f"   cheaper in {n_cheaper}/18; median range {min(meds):.3f}..{max(meds):.3f}")
# rho vs rhoseq reference strength
print("\n## reference strength: rhoseq / rho (median over 22 configs, I=3M, h=0 and h=1)")
for c in cells:
    ks = sorted(k for k in cfg if k[:2] == c)
    print(c, " ".join(f"h={h}: {np.median([min(gae(r,3,h) for r in cfg[k]['rhoseq'].values())/min(gae(r,3,h) for r in cfg[k]['rho'].values()) for k in ks]):.3f}" for h in (0, 1)))
