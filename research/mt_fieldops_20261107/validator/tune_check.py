import json, sys
from collections import defaultdict
rows = [json.loads(l) for l in open(sys.argv[1])]
def gae(r, ci, h):  # PROTOCOL.md "Cost unit": [(cI I + M + S)/(cI + 3) + h (H + L)] / T
    return ((ci*r["I"] + r["M"] + r["S"])/(ci + 3) + h*(r["H"] + r["L"])) / r["targets"]
assert all(r["arm"] == "ic4" and r["ok"] == r["targets"] for r in rows)
grid = sorted({(r["param"], r["g"]) for r in rows}); cfgs = sorted({(r["pb"], r["T"], r["seed"], r["idx"]) for r in rows})
print("grid points", len(grid), "configs", len(cfgs), cfgs)
# protocol: instances index 0, seeds 101..103, cells (22,1024),(24,4096),(25,16384)
c = defaultdict(dict)
for r in rows: c[(r["pb"], r["T"], r["seed"], r["idx"])][(r["param"], r["g"])] = gae(r, 10, 0.1)
assert all(len(v) == 15 for v in c.values())
score = {k: sum(v[k]/min(v.values()) for v in c.values())/len(c) for k in grid}
for k, s in sorted(score.items(), key=lambda kv: kv[1])[:5]: print(k, round(s, 4))
best = min(score, key=score.get); print("argmin", best)
fz = json.load(open(sys.argv[2])); print("frozen", fz["mu"], fz["g"], "match:", (fz["mu"], fz["g"]) == best)
# robustness of the choice to the objective's (cI, h)
for ci in (3, 10, 30):
    for h in (0, 0.1, 1):
        cc = defaultdict(dict)
        for r in rows: cc[(r["pb"], r["T"], r["seed"])][(r["param"], r["g"])] = gae(r, ci, h)
        sc = {k: sum(v[k]/min(v.values()) for v in cc.values())/len(cc) for k in grid}
        b = min(sc, key=sc.get); print(f"  cI={ci} h={h}: argmin {b} score {sc[b]:.4f}; (1.0,2.0) {sc[(1.0,2.0)]:.4f}")
