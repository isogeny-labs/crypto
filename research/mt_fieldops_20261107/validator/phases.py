import json, sys, numpy as np
from collections import defaultdict
rows = [json.loads(l) for l in open(sys.argv[1])]
ci = 3
def g(v, T): return ((ci*v[0] + v[1] + v[2])/(ci+3)) / T   # field-op GAE only (h = 0)
order = {"ic4": ["fb","table","relations","solve","targets","verify"], "rho": ["db","targets","verify"], "rho2": ["db","targets","verify"], "bsgs2": ["baby","targets","verify"]}
acc = defaultdict(list)
for r in rows:
    ph = r["phases"]; prev = [0]*5; parts = {}
    for name in order[r["arm"]]:
        cur = ph[name]; parts[name] = g([c-p for c, p in zip(cur, prev)], r["T"]); prev = cur
    assert [r["I"], r["M"], r["S"], r["H"], r["L"]] == prev, "totals != last snapshot"
    acc[(r["pb"], r["T"], r["arm"], r["param"])].append(parts)
print("GAE/target at I = 3M, h = 0, by phase (median over 22 configs)")
for k in sorted(acc):
    if k[2] in ("rho","rho2") and k[3] != 0.7: continue
    if k[2] == "bsgs2" and k[3] != 1.0: continue
    ps = acc[k]; names = list(ps[0]); tot = np.median([sum(p.values()) for p in ps])
    print(k, f"total {tot:.1f}", " ".join(f"{n}={np.median([p[n] for p in ps]):.1f}" for n in names), f"verify_share={np.median([p['verify']/sum(p.values()) for p in ps]):.2f}")
