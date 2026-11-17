#!/usr/bin/env python3
"""Score the frozen predictions N1..N9 (PROTOCOL.md) from remeasure.json.

Usage: python3 scorecard.py [remeasure.json] [scorecard.json]
Prints one line per prediction and writes scorecard.json. [pcc:unverified]
"""
import json
import sys

src = sys.argv[1] if len(sys.argv) > 1 else "remeasure.json"
out = sys.argv[2] if len(sys.argv) > 2 else "scorecard.json"
d = json.load(open(src))
T = d["targets"]
rows_all = [(tn, r) for tn, t in T.items() for r in t["rows"]]
sc = {}


def rank(v):
    o = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    for k, i in enumerate(o):
        r[i] = k
    return r


def spearman(x, y):
    if len(x) < 3:
        return None
    rx, ry = rank(x), rank(y)
    n = len(x)
    mx, my = sum(rx) / n, sum(ry) / n
    num = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    den = (sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry)) ** 0.5
    return num / den if den else None


RECORDED = {"v3": ["recorded"], "cover_pushforward": ["ic_jc", "ic_jc_L", "ic_e1_fibre_full", "ic_e1_two_term", "ic_jc_first_collision", "ic_e1_fibre_first_collision", "birthday_e1_first_collision"], "tower_large": ["g2_recorded", "g3_recorded"]}

# N1
below = [(tn, r["p"], a) for tn, r in rows_all for a, e in r["arms"].items() if e["class_neg_band"] == "below"]
bad_rec = [(tn, r["p"], a, e["class_neg_band"]) for tn, r in rows_all for a, e in r["arms"].items() if a in RECORDED[tn] and e["class_neg_band"] not in ("above", "unresolved (lower bound)")]
sc["N1"] = {"holds": not below and not bad_rec, "below_rows": below, "recorded_rows_outside": bad_rec}

# N2: four named classes per step
n2 = []
for tn, r in rows_all:
    n, pl = r["rho"]["negation"], r["rho"]["plain"]
    nn = (n["canonicalisations"] + n["partition_hashes"] + n["dp_lookups"] + n["dp_inserts"]) / n["steps"]
    pp = (pl["partition_hashes"] + pl["dp_lookups"] + pl["dp_inserts"]) / pl["steps"]
    nw = n["native_per_step"]
    n2.append({"target": tn, "p": r["p"], "neg4_per_step": nn, "plain4_per_step": pp, "neg_with_word_per_step": nw, "plain_with_word_per_step": pl["native_per_step"]})
out_neg = [x for x in n2 if not 1.8 <= x["neg4_per_step"] <= 3.5]
out_pl = [x for x in n2 if not 0.9 <= x["plain4_per_step"] <= 1.6]
sc["N2"] = {"holds": not out_neg and not out_pl, "rows": len(n2), "neg_outside": len(out_neg), "plain_outside": len(out_pl),
            "neg4_range": [min(x["neg4_per_step"] for x in n2), max(x["neg4_per_step"] for x in n2)],
            "plain4_range": [min(x["plain4_per_step"] for x in n2), max(x["plain4_per_step"] for x in n2)]}

# N3
n3 = []
for tn, r in rows_all:
    if tn == "cover_pushforward":
        continue
    n = r["rho"]["negation"]
    n3.append({"target": tn, "p": r["p"], "w1_over_w0": (n["total_ops"] + n["native"]) / n["total_ops"], "w1_over_w0_four_classes": (n["total_ops"] + n["native"] - n["word_ops"]) / n["total_ops"],
               "old_over_neg": r.get("old_plain_rho_ops", r.get("old_plain_rho_ops_mean")) / n["total_ops"], "target_v3": tn == "v3"})
a_ok = all(1.3 <= x["w1_over_w0"] <= 4 for x in n3)
a4_ok = all(1.3 <= x["w1_over_w0_four_classes"] <= 4 for x in n3)
b_ok = all(4 <= x["old_over_neg"] <= 10 for x in n3 if x["target_v3"])
sc["N3"] = {"holds": a_ok and b_ok, "first_part_as_measured_incl_word_ops": a_ok, "first_part_four_classes_only": a4_ok, "second_part_v3_old_over_neg_in_4_10": b_ok,
            "w1_over_w0_range": [min(x["w1_over_w0"] for x in n3), max(x["w1_over_w0"] for x in n3)],
            "w1_over_w0_four_classes_range": [min(x["w1_over_w0_four_classes"] for x in n3), max(x["w1_over_w0_four_classes"] for x in n3)],
            "v3_old_over_neg": [x["old_over_neg"] for x in n3 if x["target_v3"]]}

# N4
rows11 = []
for tn, arm in (("v3", "recorded"), ("cover_pushforward", "ic_jc")):
    for r in T[tn]["rows"]:
        e = r["arms"][arm]
        rec = e.get("recorded_ratio_to_old_plain_rho") or e.get("recorded_median_ratio_to_old_plain_rho")
        rows11.append({"target": tn, "p": r["p"], "new_w0": e["ratio_neg_band"][0], "old_w0": rec[0], "ratio": e["ratio_neg_band"][0] / rec[0]})
up = sum(1 for x in rows11 if x["new_w0"] > x["old_w0"])
cov = [x for x in rows11 if x["target"] == "cover_pushforward"]
cov_ok = all(3 <= x["ratio"] <= 12 for x in cov)
sc["N4"] = {"holds": up >= 9 and cov_ok, "rows_up": up, "rows": len(rows11), "cover_ratio_new_over_old": [x["ratio"] for x in cov], "cover_all_in_3_12": cov_ok}

# N5
disagree = []
for tn, r in rows_all:
    for a, e in r["arms"].items():
        if a in RECORDED[tn] or a.endswith("C") or a == "C":
            if not (e["class_neg_band"] == "above" and e["class_neg_cal"] == "above"):
                disagree.append((tn, r["p"], a, e["class_neg_band"], e["class_neg_cal"]))
wcal = []
for tn, r in rows_all:
    t = r["class_times_ns"]
    ta = r["t_add_ns"]
    key = "J_word_impl" if "J_word_impl" in ta else "J_g2"
    wcal.append((tn, r["p"], t["mul_p"] / ta[key]))
w_bad = [x for x in wcal if not 0.002 <= x[2] <= 0.03]
sc["N5"] = {"holds": not disagree and not w_bad, "disagreements_band_vs_cal": disagree, "w_cal_mul_p_outside_0.002_0.03": w_bad,
            "w_cal_mul_p_range": [min(x[2] for x in wcal), max(x[2] for x in wcal)]}

# N6
curves = 0
aut = []
for tn, r in rows_all:
    cl = r.get("curves_checked_all") or r.get("curves_checked")
    if cl:
        curves += len(cl)
        aut += [(tn, r["p"], c) for c in cl if c["automorphism_beyond_negation"]]
    else:
        curves += 1
        if r["rho"]["automorphism_fold"]["automorphism_beyond_negation"]:
            aut.append((tn, r["p"], r["rho"]["automorphism_fold"]))
sc["N6"] = {"holds": not aut and curves >= 32, "curves_checked": curves, "with_automorphism": aut}

# N7
def tot(r, a):
    return r["arms"][a]["total_band"]

n7 = []
cases = [("v3", 1619, "recorded", ["C", "A", "B"]), ("tower_large", 16001, "g3_recorded", ["g3_C", "g3_A", "g3_B"]), ("cover_pushforward", 12841, "ic_jc", ["C", "A", "B"])]
for tn, p, rec, new in cases:
    rr = next((r for r in T[tn]["rows"] if r["p"] == p), None)
    if rr is None:
        n7.append({"target": tn, "p": p, "present": False})
        continue
    old = tot(rr, rec)
    c = tot(rr, new[0])
    best = [min(tot(rr, a)[i] for a in new) for i in range(3)]
    n7.append({"target": tn, "p": p, "present": True, "C_below_recorded_all_prices": all(c[i] < old[i] for i in range(3)), "best_below": all(best[i] < old[i] for i in range(3)),
               "C_total": c, "recorded_total_lower_bound": old, "group_ops_C": rr["arms"][new[0]]["group_ops"], "group_ops_recorded": rr["arms"][rec]["group_ops"]})
sc["N7"] = {"holds": all(x.get("present") and (x["C_below_recorded_all_prices"] or x["best_below"]) for x in n7), "cases": n7}

# N8
ps = {r["p"] for tn, r in rows_all if tn == "tower_large"}
sc["N8"] = {"holds": 32003 in ps and 64007 in ps, "tower_primes_completed": sorted(ps)}

# N9
n9 = {}
for g in ("g2", "g3"):
    xs, ys = [], []
    for r in T["tower_large"]["rows"]:
        e = r["arms"].get(f"{g}_C")
        if e:
            xs.append(r["p"])
            ys.append(e["ratio_neg_band"][0])
    n9[g] = {"spearman": spearman(xs, ys), "n": len(xs)}
sc["N9"] = {"holds": all(v["spearman"] is not None and v["spearman"] >= 0.7 for v in n9.values()), **n9}

for k, v in sc.items():
    print(k, "HOLDS" if v["holds"] else "REFUTED", json.dumps({a: b for a, b in v.items() if a != "holds"})[:300])
json.dump(sc, open(out, "w"), indent=1)
