#!/usr/bin/env python3
"""L3: chain / flat ratios at equal (m, d) against the recorded chain cells (exponent study + extension, binary dc2c30a4)
and this run's chain controls; flat SD; and, as a non-preregistered reading, the parent's section-5 estimator
(OLS of ln T on ln |V| over complete cells with d >= 9, >= 4 cells spanning >= 4x) applied to the flat cells."""
import glob, json, math, os
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..", "..")
rec = {}
for f in glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k20_m[23]_sym_d*_g0.json")) + \
         glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/k20_m[23]_sym_d*_g0.json")):
    j = json.load(open(f))
    if not j["capped"] and j.get("per_target"):
        rec[(j["m"], j["d_or_t"])] = (j["mean_field_ops"], max(t["solving_degree_max"] for t in j["per_target"]))
    elif j["capped"]:
        rec[(j["m"], j["d_or_t"])] = ("censored", None)
flat, ctrl = {}, {}
for f in sorted(glob.glob(os.path.join(H, "cells", "*.json"))):
    j = json.load(open(f))
    if j.get("capped") or not j.get("per_target"): continue
    key = (j["m"], j["d_or_t"]); val = (j["mean_field_ops"], max(t["solving_degree_max"] for t in j["per_target"]), j["ext"]["wall_s"])
    (flat if j["base"] == "subflat" else ctrl)[key] = val
print("binary control (chain cells rerun with the new binary vs recorded):")
for k, v in sorted(ctrl.items()):
    print(f"  m={k[0]} d={k[1]}: rerun T={v[0]:.6g} recorded T={rec.get(k, ('n/a',))[0]} equal={rec.get(k, (None,))[0] == v[0]}")
print("\nflat cells vs recorded chain cells:")
rows = []
for k, v in sorted(flat.items()):
    r = rec.get(k, ("n/a", None))
    ratio = (r[0] / v[0]) if isinstance(r[0], float) else None
    rows.append((k[0], k[1], v[0], v[1], r[0], r[1], ratio, v[2]))
    print(f"  m={k[0]} d={k[1]:4d}: flat T={v[0]:.4g} SD={v[1]} ({v[2]:.0f}s) | chain T={r[0] if not isinstance(r[0], float) else f'{r[0]:.4g}'} SD={r[1]} | chain/flat={'%.1f' % ratio if ratio else r[0]}")
T975 = {2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306}
def fit(xs, ys):
    n = len(xs); mx, my = sum(xs) / n, sum(ys) / n; sxx = sum((x - mx) ** 2 for x in xs)
    s = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx; c = my - s * mx
    se = math.sqrt(sum((y - c - s * x) ** 2 for x, y in zip(xs, ys)) / (n - 2) / sxx); t = T975.get(n - 2, 1.96)
    return s, s - t * se, s + t * se
print("\nnon-preregistered reading: parent section-5 estimator on the flat cells (d >= 9):")
for m in (2, 3):
    sel = sorted((d, v[0]) for (mm, d), v in flat.items() if mm == m and d >= 9)
    if len(sel) >= 4 and sel[-1][0] / sel[0][0] >= 4:
        s, lo, hi = fit([math.log(d) for d, _ in sel], [math.log(t) for _, t in sel])
        print(f"  m={m}: n={len(sel)} span={sel[-1][0] / sel[0][0]:.1f}x slope s={s:.2f} [{lo:.2f}, {hi:.2f}]  (lever thresholds: m=3 s<0, m=4 s<0.5)")
    else:
        print(f"  m={m}: estimator not applicable (n={len(sel)})")
json.dump({"rows": rows}, open(os.path.join(H, "analysis.json"), "w"), indent=1)
