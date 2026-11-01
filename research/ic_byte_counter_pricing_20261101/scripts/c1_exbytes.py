#!/usr/bin/env python3
"""C1 and the distance to rho of research/ic_residual_scale_20261101 under the band rule of
option B (byte counters listed, not priced), next to the numbers as built.

Reads only saved comparisons: the committed ones in sessions/<S>/comparisons/ (w = 0, which the
rule does not change) and the re-run ones in analysis/exbytes/comparisons/ (w = 0.1 and 1,
written by regen_exbytes.py).  C1 as in the study's PROTOCOL.md and scripts/summarise.py: for
B in {1, 4} and X in {dp, eq} the paired 95% interval of S(res{B}X)/S(res{B}) lies below 1 at
w = 0, 0.1 and 1 (cells on p40 and p42; p36 is shown, not counted).

Usage: c1_exbytes.py STUDY_DIR   (writes STUDY_DIR/analysis/exbytes/C1.md and c1.json)
"""
import glob, json, os, sys

study = sys.argv[1]
W = ["0", "0.1", "1"]


def point(ops, w):
    return ops["ratio_b_over_a"], ops["ci95"]


def cells(sess, a, b):
    """{'built': {w: (ratio, ci)}, 'rule': {w: (ratio, ci)}} for the pair a -> b."""
    d = os.path.join(study, "sessions", sess, "comparisons")
    main = json.load(open(os.path.join(d, f"{a}__{b}.json")))["ops"]
    pts = main["band"]["ratio_b_over_a"]
    built = {"0": point(main, 0)}
    rule = {"0": point(main, 0)}  # w = 0 has no operation count: identical under the rule
    for kind, folder, tag in (("built", d, None), ("rule", os.path.join(study, "analysis", "exbytes", "comparisons"), sess + "__")):
        pat = os.path.join(folder, (tag or "") + f"{a}__{b}__*.json")
        for p in glob.glob(pat):
            c = json.load(open(p))
            o = c["ops"]
            wt = c.get("pricing_weight")
            if wt is None:
                continue
            key = {0.1: "0.1", 1.0: "1"}.get(round(float(wt), 6))
            if key is None:
                continue
            (built if kind == "built" else rule)[key] = point(o, wt)
    return built, rule


def below1(x):
    return all(w in x and x[w][1][1] < 1 for w in W)


def fmt(x):
    return "  ".join(f"{x[w][0]:.3f} [{x[w][1][0]:.3f}, {x[w][1][1]:.3f}]" if w in x else "?" for w in W)


out, lines = {}, []
lines.append("| curve | cell (b/a) | as built: w = 0 / 0.1 / 1 | below 1 | byte counter not priced: w = 0 / 0.1 / 1 | below 1 |")
lines.append("|:--|:--|:--|:--:|:--|:--:|")
c1 = {"built": [], "rule": []}
for curve in ("p36", "p40", "p42"):
    sess = "S-" + curve
    for B in (1, 4):
        for X in ("dp", "eq"):
            a, b = f"res{B}", f"res{B}{X}"
            built, rule = cells(sess, a, b)
            out[f"{curve} {b}/{a}"] = {"built": built, "rule": rule, "below1_built": below1(built), "below1_rule": below1(rule)}
            if curve != "p36":
                c1["built"].append(below1(built))
                c1["rule"].append(below1(rule))
            lines.append(f"| {curve} | {b}/{a} | {fmt(built)} | {'yes' if below1(built) else 'NO'} | {fmt(rule)} | {'yes' if below1(rule) else 'NO'} |")
lines.append("")
lines.append(f"C1 (p40, p42; 8 cells): as built {sum(c1['built'])}/8 met; byte counter not priced {sum(c1['rule'])}/8 met.")

# Distance to matched rho: point ratios of totals, S(arm)/S(rho), from the band at each weight.
lines.append("")
lines.append("Distance to matched rho (arm / rho, ratio of totals), w = 0 / 0.1 / 1:")
lines.append("")
lines.append("| curve | arm | as built | byte counter not priced |")
lines.append("|:--|:--|:--|:--|")
dist = {}
for curve in ("p36", "p40", "p42"):
    sess = "S-" + curve
    bj = json.load(open(os.path.join(study, "analysis", f"band-{sess}.json")))
    nj = json.load(open(os.path.join(study, "analysis", "exbytes", f"band-{sess}.json")))
    ws = bj["weights"]
    idx = [ws.index(w) for w in (0.0, 0.1, 1.0)]
    bi = {r["arm"]: r["mean_s"] for r in bj["rows"]}
    ni = {r["arm"]: r["mean_s"] for r in nj["rows"]}
    for arm in ("res1", "res1dp", "res1eq", "res4", "res4dp", "res4eq"):
        if arm not in bi or "rho" not in bi:
            continue
        f = lambda t, r: [t[arm][i] / t["rho"][i] if t[arm][i] is not None and t["rho"][i] else None for i in idx]
        b_, n_ = f(bi, 0), f(ni, 0)
        dist[f"{curve} {arm}"] = {"built": b_, "rule": n_}
        s = lambda v: " / ".join("?" if x is None else f"{x:.2f}" for x in v)
        lines.append(f"| {curve} | {arm} | {s(b_)} | {s(n_)} |")
os.makedirs(os.path.join(study, "analysis", "exbytes"), exist_ok=True)
open(os.path.join(study, "analysis", "exbytes", "C1.md"), "w").write("\n".join(lines) + "\n")
json.dump({"c1": out, "distance_to_rho": dist}, open(os.path.join(study, "analysis", "exbytes", "c1.json"), "w"), indent=1)
print("\n".join(lines))
