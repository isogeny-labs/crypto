import json, math, os
BASE = "/home/user/crypto/research/isogeny_fibers_20261010/round4/16_3_vertical_margin"
rat = json.load(open(os.path.join(BASE, "rational.json")))["rows"]
row = [r for r in rat if r["ell"] == 16777259][0]
velu = row["velu_1pt"]["Meq"]; sq = row["sqrtvelu_1pt"]["Meq"]
print("velu_1pt Meq at ell=16777259:", velu)
print("sqrtvelu_1pt Meq at ell=16777259:", sq)
print("ratio velu/sqrtvelu:", round(velu / sq, 4))
def ols(pts):
    n = len(pts); xs = [p[0] for p in pts]; ys = [p[1] for p in pts]
    mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx; a = my - b * mx
    res = [y - (a + b * x) for x, y in zip(xs, ys)]
    s2 = sum(e * e for e in res) / (n - 2)
    se = math.sqrt(s2 / sxx)
    return n, a, b, se
# t critical 95% two-sided; table values for df
TCRIT = {14: 2.1448, 15: 2.1314}
for series, field, lo in [("sqrtvelu_1pt", "sqrtvelu_1pt", 16), ("sqrtvelu_1pt", "sqrtvelu_1pt", 10), ("velu_1pt", "velu_1pt", 10)]:
    pts = [(r["log2_ell"], math.log2(r[field]["Meq"])) for r in rat if r["ell"] >= 2 ** lo]
    n, a, b, se = ols(pts)
    t = TCRIT.get(n - 2)
    print(f"{series} ell>=2^{lo}: n={n} exponent={b:.4f} ci95=[{b - t*se:.4f},{b + t*se:.4f}] log2c={a:.2f}" if t else f"{series} ell>=2^{lo}: n={n} exponent={b:.4f} se={se:.5f} (df={n-2}, no t table)")
print("count rational rows:", len(rat), "ell>=2^16:", sum(1 for r in rat if r["ell"] >= 2**16), "ell>=2^10:", sum(1 for r in rat if r["ell"] >= 2**10))
print("---- per-ell aggregation candidates (rows grouped by ell, 3 instances each) ----")
from collections import defaultdict
g = defaultdict(list)
for r in rat:
    g[r["ell"]].append(r)
print("instances at 16777259:", [(x["seed"], x["velu_1pt"]["Meq"], x["sqrtvelu_1pt"]["Meq"]) for x in g[16777259]])
print("distinct ell:", len(g), "ell>=2^16:", sum(1 for e in g if e >= 2**16), "ell>=2^10:", sum(1 for e in g if e >= 2**10))
def fit_pts(agg, lo):
    pts = []
    for e, rs in sorted(g.items()):
        if e < 2**lo: continue
        vals = [r["sqrtvelu_1pt"]["Meq"] for r in rs]
        if agg == "mean_then_log": y = math.log2(sum(vals) / len(vals))
        elif agg == "mean_of_log": y = sum(math.log2(v) for v in vals) / len(vals)
        elif agg == "first_instance": y = math.log2(vals[0])
        pts.append((math.log2(e), y))
    return pts
T = {15: 2.1314}
for agg in ["mean_then_log", "mean_of_log", "first_instance"]:
    pts = fit_pts(agg, 16)
    n, a, b, se = ols(pts)
    t = {15: 2.1314}.get(n - 2, 2.1314 if n - 2 == 15 else None)
    print(f"sqrtvelu_1pt ell>=2^16 agg={agg}: n={n} exponent={b:.4f} ci95=[{b-2.1314*se:.4f},{b+2.1314*se:.4f}] (t df=15) log2c={a:.2f}")
