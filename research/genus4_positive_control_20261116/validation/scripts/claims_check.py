#!/usr/bin/env python3
"""Every number in the two RESULTS.md files (transcribed by the validator,
with the RESULTS line it comes from), against the validator's recomputation
(recompute.json).  Point estimates must agree to the printed precision
(|claimed - recomputed| <= half a unit in the last printed digit); interval
ends are bootstrap quantiles with another RNG, so they must agree within
max(0.02 * value, 0.006) and lie on the same side of 1."""
import json, sys

R = json.load(open(sys.argv[1]))
OUT = sys.argv[2]
s1 = {int(k): v for k, v in R["s1"].items()}
s2 = R["s2"]
rows = []


def add(study, where, name, claimed, rec, digits=3):
    tol = 0.5 * 10 ** (-digits) + 1e-12
    ok = abs(claimed - rec) <= tol
    rows.append({"study": study, "where": where, "name": name, "claimed": claimed, "recomputed": rec,
                 "kind": "point", "ok": ok})


def addi(study, where, name, claimed, rec):
    (c0, c1, c2), (r0, r1, r2) = claimed, rec
    ok = abs(c0 - r0) <= 0.0005 + 1e-12
    for c, r in ((c1, r1), (c2, r2)):
        ok &= abs(c - r) <= max(0.02 * abs(r), 0.006)
        ok &= (c < 1) == (r < 1)
    rows.append({"study": study, "where": where, "name": name, "claimed": claimed,
                 "recomputed": [round(x, 4) for x in rec], "kind": "interval", "ok": ok})


# ---------------- study 1: components table
for p, v, w, rel, share in [(2053, 0.331, 0.222, 0.988, 0.88), (4099, 0.330, 0.229, 0.990, 0.86), (8209, 0.331, 0.233, 0.998, 0.84)]:
    e = s1[p]
    add(1, "Components", f"LA v/c p={p}", v, e["LA_v_over_c"][0])
    add(1, "Components", f"LA w/c p={p}", w, e["LA_w_over_c"][0])
    add(1, "Components", f"relation w/c p={p}", rel, e["rel_w_over_c"][0])
    add(1, "Components", f"relation share of w's mult. p={p}", share, e["w_rel_share_mean_of_runs"], 2)
    add(1, "Components", f"fallbacks v+w p={p}", 0, e["fallbacks"]["v"] + e["fallbacks"]["w"], 0)
for p, lo, hi in [(2053, 0.219, 0.225), (4099, 0.226, 0.232), (8209, 0.230, 0.236)]:
    addi(1, "Components", f"LA w/c interval p={p}", (s1[p]["LA_w_over_c"][0], lo, hi), s1[p]["LA_w_over_c"])
# band table
band = [
    (2053, "c", [0.009, 0.272, (1.459, 1.148, 2.081)], 0.051),
    (2053, "w", [0.008, 0.192, (1.018, 0.806, 1.436)], 0.021),
    (4099, "c", [0.005, 0.195, (1.047, 0.675, 1.838)], 0.038),
    (4099, "w", [0.005, 0.133, (0.708, 0.456, 1.251)], 0.015),
    (8209, "c", [(0.004, 0.003, 0.005), (0.148, 0.107, 0.212), (0.799, 0.578, 1.142)], 0.030),
    (8209, "v", [(0.004, 0.003, 0.005), (0.105, 0.076, 0.151), (0.564, 0.408, 0.805)], 0.014),
    (8209, "w", [(0.004, 0.003, 0.005), (0.099, 0.072, 0.141), (0.528, 0.382, 0.754)], 0.011),
]
for p, arm, cells, fpm in band:
    for wv, cell in zip(("0.0", "0.1", "1.0"), cells):
        rec = s1[p][f"{arm}/R w={wv}"]
        if isinstance(cell, tuple):
            addi(1, "Band table", f"{arm}/R p={p} w={wv}", cell, rec)
        else:
            add(1, "Band table", f"{arm}/R p={p} w={wv}", cell, rec[0])
    add(1, "Band table", f"{arm}/R FPM p={p}", fpm, s1[p][f"{arm}/R FPM"][0])
# prose
add(1, "Components prose", "v/c total min (0.71)", 0.71, min(s1[p][f"v/c w={w}"][0] for p in s1 for w in (0.1, 1.0)), 2)
add(1, "Components prose", "v/c total max (0.75)", 0.75, max(s1[p][f"v/c w={w}"][0] for p in s1 for w in (0.1, 1.0)), 2)
add(1, "Components prose", "w/c total min (0.66)", 0.66, min(s1[p][f"w/c w={w}"][0] for p in s1 for w in (0.1, 1.0)), 2)
add(1, "Components prose", "w/c total max (0.70)", 0.70, max(s1[p][f"w/c w={w}"][0] for p in s1 for w in (0.1, 1.0)), 2)
add(1, "F2 bullet", "w modular additions p=8209 (9.9e7)", 9.9e7, s1[8209]["w_mean_adds"], -6)
add(1, "F2 bullet", "adds / priced mults p=8209 (1.92)", 1.92, s1[8209]["w_adds_over_muls"], 2)
addi(1, "F2 bullet + P4 row", "'multiplications and additions on both arms as emitted' w/R w=1 p=8209 (1.057 [0.765, 1.499])",
     (1.057, 0.765, 1.499), s1[8209]["w/R w=1 mul+add (F2)"])
add(1, "F2 bullet", "with rho hashes and additions estimated (about 0.63)", 0.63, s1[8209]["w/R w=1 mul+add, rho +3/step (F2 est.)"][0], 2)
loo = [v[2] for v in s1[8209]["LOO w/R w=1"].values()]
add(1, "F4 bullet", "LOO upper bound min (0.74)", 0.74, min(loo), 2)
add(1, "F4 bullet", "LOO upper bound max (0.88)", 0.88, max(loo), 2)
add(1, "verdict bullet", "v/R w=1 p=8209 upper end (0.805)", 0.805, s1[8209]["v/R w=1.0"][2], 2)
add(1, "verdict bullet", "c/R w=1 p=8209 upper end (1.142)", 1.142, s1[8209]["c/R w=1.0"][2], 2)
add(1, "What is next", "w/v LA in multiplications p=8209 (0.705)", 0.705, s1[8209]["LA_w_over_v_mul"])
add(1, "What is next", "w/v LA counting additions p=8209 (0.933)", 0.933, s1[8209]["LA_w_over_v_mul_plus_add"])
add(1, "What is next", "unit-T cap minimum (>= 1.9e5)", 1.9e5, s1[2053]["unitT_cap_min_over_curves"], -4)

# ---------------- study 2
q = {"Q1": [(0.003, 0.002, 0.005), (0.088, 0.064, 0.133), (0.469, 0.341, 0.709)],
     "Q2": [0.003, (0.251, 0.183, 0.380), (1.367, 0.995, 2.068)],
     "Q3": [0.003, (0.098, 0.071, 0.148), (0.142, 0.102, 0.215)],
     "Q4": [0.003, (0.089, 0.064, 0.134), (0.123, 0.089, 0.187)]}
for name, cells in q.items():
    for wv, cell in zip(("0.0", "0.1", "1.0"), cells):
        v = s2[name][wv]
        rec = [v["ratio"], v["lo"], v["hi"]]
        if isinstance(cell, tuple):
            addi(2, "Readings table", f"{name} w={wv}", cell, rec)
        else:
            add(2, "Readings table", f"{name} w={wv}", cell, rec[0])
        rows.append({"study": 2, "where": "Readings table", "name": f"{name} w={wv} cheapest rho", "claimed": "R",
                     "recomputed": v["cheapest"], "kind": "label", "ok": v["cheapest"] == "R"})
v = s2["Qcal"]["cal"]
addi(2, "Readings table", "Qcal", (0.008, 0.006, 0.012), [v["ratio"], v["lo"], v["hi"]])
for nm, cl, rc in [("w_mul,p", 0.0028, R["s2_cal"]["mean"][0]), ("w_add,p", 0.0009, R["s2_cal"]["mean"][1]), ("w_mul,r", 0.0046, R["s2_cal"]["mean"][2])]:
    add(2, "Readings table (Qcal row)", nm, cl, rc, 4)
for name, cl in [("Q1", True), ("Q2", False), ("Q3", True), ("Q4", True), ("Qcal", True)]:
    rows.append({"study": 2, "where": "Readings table", "name": f"{name} below at every price", "claimed": cl,
                 "recomputed": s2["verdict"][name], "kind": "label", "ok": cl == s2["verdict"][name]})
loo = s2["LOO w=1"]
for name, lo, hi in [("Q1", 0.40, 0.66), ("Q2", 1.16, 1.91), ("Q3", 0.12, 0.20), ("Q4", 0.10, 0.17)]:
    pts = [x["ratio"] for x in loo[name].values()]
    add(2, "LOO line", f"{name} LOO min", lo, min(pts), 2)
    add(2, "LOO line", f"{name} LOO max", hi, max(pts), 2)
add(2, "LOO line", "Q1 LOO upper ends min (0.68)", 0.68, min(x["hi"] for x in loo["Q1"].values()), 2)
add(2, "LOO line", "Q1 LOO upper ends max (0.97)", 0.97, max(x["hi"] for x in loo["Q1"].values()), 2)
add(2, "Owner-decision bullet", "Q3 LOO upper ends min (0.20)", 0.20, min(x["hi"] for x in loo["Q3"].values()), 2)
add(2, "Owner-decision bullet", "Q3 LOO upper ends max (0.30)", 0.30, max(x["hi"] for x in loo["Q3"].values()), 2)
add(2, "rho reference", "P/R at w=0 (1.30)", 1.30, s2["Q1"]["0.0"]["P_over_R"], 2)
add(2, "rho reference", "P/R at w=1 min over readings (1.30)", 1.30, min(s2[x]["1.0"]["P_over_R"] for x in ("Q1", "Q2", "Q3", "Q4")), 2)
add(2, "rho reference", "P/R at w=1 max over readings (1.35)", 1.35, max(s2[x]["1.0"]["P_over_R"] for x in ("Q1", "Q2", "Q3", "Q4")), 2)
add(2, "rho reference", "compares per step, negation (17.9)", 17.9, s2["per_step"]["R"]["compares"], 1)
add(2, "rho reference", "compares per step, plain (18.0)", 18.0, s2["per_step"]["P"]["compares"], 1)
add(2, "Reading it", "largest upper end at w<=0.1 (0.380, Q2)", 0.380,
    max(s2[x][w]["hi"] for x in ("Q1", "Q2", "Q3", "Q4") for w in ("0.0", "0.1")), 2)
add(2, "Scorecard P1", "P/R at w=1 in Q3/Q4 (1.351)", 1.351, s2["Q3"]["1.0"]["P_over_R"])

json.dump(rows, open(OUT, "w"), indent=1)
bad = [r for r in rows if not r["ok"]]
print(f"claims checked: {len(rows)}; consistent: {len(rows) - len(bad)}; inconsistent: {len(bad)}")
for r in bad:
    print("  INCONSISTENT", r)
