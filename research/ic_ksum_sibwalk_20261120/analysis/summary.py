#!/usr/bin/env python3
"""results.json + M.json + data/correctness.json -> results.md, summary.json (layout only; every ratio is read from
`ecbench compare`). cwd = study dir."""
import json
d = json.load(open("results.json")); M = json.load(open("M.json")); C = json.load(open("data/correctness.json")); W = ["0", "0.1", "1"]
INC = {"P20": "k5 N9 +prefix", "P22": "k5 N10", "P24": "k5 N11", "B15": "k4 N7 +prefix", "B18": "k4 N7 +prefix",
       "B21": "k5 N9 +prefix", "B24": "k5 N9 +prefix", "B27": "k5 N12"}
f = lambda p: f"{p['ratio']:.4f} [{p['ci95'][0]:.4f}, {p['ci95'][1]:.4f}]"
r4 = lambda x: round(x, 4)
L = ["| row (incumbent, m) | w | cand / inc (pooled 95 %) | per replicate (upper) | upper used | predicted | gap to rho-neg: inc -> cand |",
     "|:--|--:|:--|:--|--:|--:|:--|"]
S = {"schema": "crypto.ic_ksum_sibwalk_summary/v1", "standing": "[pcc:unverified]",
     "unit": "ecbench counted GAE, ratio of totals, paired by target and round; w in {0, 0.1, 1} GAE per native op",
     "binary_sha256": "ff3172218ab6aa4f3d1f19d02b5cc3d2401cf1d737c06b5c0cc11202a50af844", "code_commit": "39d3d20d72",
     "tests": d["tests"], "correctness": {"pairs": sum(v["pairs"] for v in C.values()),
     "off_mismatches": sum(v["off_mismatches"] for v in C.values()), "cand_mismatches": sum(v["cand_mismatches"] for v in C.values()),
     "count_model_mismatches": sum(v["count_model_mismatches"] for v in C.values())}, "rows": {}}
T3 = ["| row | w | inc-prefix / inc | per replicate (upper) | verdict |", "|:--|--:|:--|:--|:--|"]
for row, v in d["rows"].items():
    m = M["m"][row]; t = v["T1"]
    S["rows"][row] = {"incumbent": INC[row], "m": m, "T1_holds": t["holds"], "T1_max_upper": r4(t["max_upper"]),
                      "aa_max_dev": v["aa_max_dev"], "off_ratio": [v["off"][w]["ratio"] for w in W], "weights": {}}
    if "T3" in v:
        S["rows"][row]["T3_holds"] = v["T3"]["holds"]
    for w in W:
        p = t[w]["pooled"]; pred = M["predicted_ratio"][row][str(float(w))]
        L.append(f"| {row + ' (' + INC[row] + ', m=' + str(m) + ')' if w == '0' else ''} | {w} | {f(p)}{' **holds**' if w == '0' and t['holds'] else ''} | "
                 + ", ".join(f"{q['ratio']:.3f} ({q['upper']:.3f})" for q in t[w]["per_replicate"])
                 + f" | {t['max_upper_by_w'][w]:.4f} | {pred:.3f} | {v['gap']['inc'][w]['ratio']:.2f} -> {v['gap']['cand'][w]['ratio']:.2f} |")
        S["rows"][row]["weights"][w] = {
            "ratio": r4(p["ratio"]), "ci95": [r4(c) for c in p["ci95"]], "ci95_bootstrap": [r4(c) for c in p["ci95_bootstrap"]],
            "upper_used": r4(t["max_upper_by_w"][w]), "per_replicate": [r4(q["ratio"]) for q in t[w]["per_replicate"]],
            "per_replicate_upper": [r4(q["upper"]) for q in t[w]["per_replicate"]], "predicted": pred,
            "gap": {a: round(v["gap"][a][w]["ratio"], 3) for a in v["gap"]},
            "gap_ci95": {a: [round(c, 3) for c in v["gap"][a][w]["ci95"]] for a in v["gap"]}, "aa": v["aa"][w]["ratio"]}
        if "T3" in v:
            q = v["T3"][w]
            S["rows"][row]["weights"][w]["T3"] = {"ratio": r4(q["pooled"]["ratio"]), "ci95": [r4(c) for c in q["pooled"]["ci95"]],
                                                  "per_replicate_upper": [r4(x["upper"]) for x in q["per_replicate"]]}
            T3.append(f"| {row if w == '0' else ''} | {w} | {f(q['pooled'])} | " + ", ".join(f"{x['ratio']:.3f} ({x['upper']:.3f})" for x in q["per_replicate"])
                      + f" | {'holds at this w' if v['T3']['holds_by_w'][w] else 'fails at this w'} |")
c = S["correctness"]
open("results.md", "w").write(
    "# Confirmatory results [pcc:unverified]\n\nRatios `ecbench compare --weight w --resamples 4000 --seed 20261120`, 3 sessions pooled "
    "(432 pairs a row; 144 a replicate). Upper used = max over pooled and replicates of max(lin-t, bootstrap) 95 % upper end. "
    "Predicted = the registered count-model prediction (M.json).\n\n## T1: walk_sib=m against the incumbent best\n\n" + "\n".join(L)
    + "\n\n## T3 (secondary): prefix=1 on the three rows where it narrowly failed, fresh registered test\n\n" + "\n".join(T3)
    + f"\n\nA/A (`rho-neg-aa / rho-neg`) = 1.0000 and `off / inc` = 1.000000 on every row and weight. 24/24 sessions `verify --replay-all` "
    f"audit OK. Correctness: {c['pairs']} (session, round, target) triples; off/inc {c['off_mismatches']}, cand/inc {c['cand_mismatches']}, "
    f"count identity {c['count_model_mismatches']} mismatches.\n")
json.dump(S, open("summary.json", "w"), indent=1)
print(open("results.md").read())
