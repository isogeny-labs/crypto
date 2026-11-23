#!/usr/bin/env python3
"""Condensed old-vs-new tables for RESULTS.md from remeasure.json. [pcc:unverified]"""
import json, sys
d = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "remeasure.json"))
def f(x):
    if x is None: return "n/a"
    if abs(x) >= 1000: return f"{x:,.0f}"
    if abs(x) >= 100: return f"{x:.0f}"
    if abs(x) >= 10: return f"{x:.1f}"
    return f"{x:.3g}"
def tri(v): return " / ".join(f(x) for x in v)
REC = {"v3": ["recorded"], "cover_pushforward": ["ic_jc", "ic_e1_two_term", "ic_e1_fibre_full", "ic_jc_first_collision", "ic_e1_fibre_first_collision", "birthday_e1_first_collision"], "tower_large": ["g2_recorded", "g3_recorded"]}
NEW = {"v3": ["L", "A", "B", "C"], "cover_pushforward": ["A", "B", "C"], "tower_large": ["g2_C", "g3_C"]}
changes = []
for tn, t in d["targets"].items():
    print(f"\n#### {tn}: recorded arms, old (recorded ratio to the plain `rho_reference`) versus new (to the negation-map rho)\n")
    print("| p | arm | group ops | old ratio 0 / 0.1 / 1 | old class | new ratio 0 / 0.1 / 1 | new class (band) | cal. ratio 0 / w_cal / 10 w_cal | class (cal) | lower bound |")
    print("|--:|--|--:|--|--|--|--|--|--|--|")
    for r in t["rows"]:
        for a in REC[tn]:
            e = r["arms"].get(a)
            if not e: continue
            rec = e.get("recorded_ratio_to_old_plain_rho") or e.get("recorded_ratio_to_old_plain_rho_la_priced") or e.get("recorded_median_ratio_to_old_plain_rho")
            rc = e.get("recorded_class")
            if rec and rc != e["class_neg_band"]: changes.append((tn, r["p"], a, rc, e["class_neg_band"]))
            print(f"| {r['p']} | {a} | {f(e['group_ops'])} | {tri(rec)} | {rc} | {tri(e['ratio_neg_band'])} | {e['class_neg_band']} | {tri(e['ratio_neg_cal'])} | {e['class_neg_cal']} | {'yes' if e['lower_bound'] else 'no'} |")
    print(f"\n#### {tn}: improved arms against the negation-map rho\n")
    print("| p | arm | group ops | native (by class) | ratio 0 / 0.1 / 1 | class | cal. ratio | class (cal) |")
    print("|--:|--|--:|--|--|--|--|--|")
    for r in t["rows"]:
        for a in NEW[tn]:
            e = r["arms"].get(a)
            if not e: continue
            nat = ", ".join(f"{k} {f(v)}" for k, v in e["native"].items())
            print(f"| {r['p']} | {a} | {f(e['group_ops'])} | {nat} | {tri(e['ratio_neg_band'])} | {e['class_neg_band']} | {tri(e['ratio_neg_cal'])} | {e['class_neg_cal']} |")
print("\nCLASS CHANGES (recorded class vs new class):")
for c in changes: print(" ", c)
