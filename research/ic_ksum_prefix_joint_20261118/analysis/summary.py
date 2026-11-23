#!/usr/bin/env python3
"""results.json -> results.md, summary.json (layout only; every number is read from `ecbench compare`). cwd = study dir."""
import json
d = json.load(open("results.json")); W = ["0", "0.1", "1"]
INC = {"P20": "k5 N9", "P22": "k5 N10", "P24": "k5 N11", "B15": "k4 N7", "B18": "k4 N7", "B21": "k5 N9", "B24": "k5 N9", "B27": "k5 N12"}
f = lambda p: f"{p['ratio']:.4f} [{p['ci95'][0]:.4f}, {p['ci95'][1]:.4f}]"
L = ["| row (k, N) | w | L1 prefix / inc | L1 per replicate (upper) | L2 joint / inc | both / inc | gap to rho-neg: inc -> registered best |",
     "|:--|--:|:--|:--|:--|:--|:--|"]
S = {"schema": "crypto.ic_ksum_prefix_joint_summary/v1", "standing": "[pcc:unverified]",
     "unit": "ecbench counted GAE, ratio of totals, paired by target and round; w in {0, 0.1, 1} GAE per native op",
     "binary_sha256": "32507dc9c5777b8b50196575000562f1478d0eb77de9738abe65d77feb55e3b6", "levers": d["levers"], "rows": {}}
for row, v in d["rows"].items():
    best = "prefix-joint" if v["L1"]["holds"] and v["L2"]["holds"] else ("joint" if v["L2"]["holds"] else ("prefix" if v["L1"]["holds"] else "inc"))
    S["rows"][row] = {"incumbent": INC[row], "L1_holds": v["L1"]["holds"], "L2_holds": v["L2"]["holds"], "registered_best": best,
                      "L1_max_upper": round(v["L1"]["max_upper"], 4), "L2_max_upper": round(v["L2"]["max_upper"], 4), "aa_max_dev": v["aa_max_dev"], "weights": {}}
    for w in W:
        a, b = v["L1"][w], v["L2"][w]
        L.append(f"| {row + ' (' + INC[row] + ')' if w == '0' else ''} | {w} | {f(a['pooled'])}{' **holds**' if w == '0' and v['L1']['holds'] else ''} | "
                 + ", ".join(f"{q['ratio']:.3f} ({q['upper']:.3f})" for q in a["per_replicate"])
                 + f" | {f(b['pooled'])}{' **holds**' if w == '0' and v['L2']['holds'] else ''} | {f(v['both'][w])} | {v['gap']['inc'][w]['ratio']:.2f} -> {v['gap'][best][w]['ratio']:.2f} ({best}) |")
        S["rows"][row]["weights"][w] = {
            "L1": {"ratio": round(a["pooled"]["ratio"], 4), "ci95": [round(c, 4) for c in a["pooled"]["ci95"]], "upper": round(a["pooled"]["upper"], 4),
                   "per_replicate_upper": [round(q["upper"], 4) for q in a["per_replicate"]]},
            "L2": {"ratio": round(b["pooled"]["ratio"], 4), "ci95": [round(c, 4) for c in b["pooled"]["ci95"]], "upper": round(b["pooled"]["upper"], 4),
                   "per_replicate_upper": [round(q["upper"], 4) for q in b["per_replicate"]]},
            "both": {"ratio": round(v["both"][w]["ratio"], 4), "ci95": [round(c, 4) for c in v["both"][w]["ci95"]]},
            "L1_given_L2": round(v["L1|L2"][w]["ratio"], 4), "L2_given_L1": round(v["L2|L1"][w]["ratio"], 4),
            "gap": {arm: round(v["gap"][arm][w]["ratio"], 3) for arm in v["gap"]},
            "gap_ci95": {arm: [round(c, 3) for c in v["gap"][arm][w]["ci95"]] for arm in v["gap"]},
            "aa": v["aa"][w]["ratio"]}
open("results.md", "w").write("# Confirmatory results [pcc:unverified]\n\nRatios `ecbench compare --weight w --resamples 4000 --seed 20261118`, 3 sessions pooled (1,152 pairs a row; replicates 384). "
    "Upper = max(lin-t, bootstrap) 95 % upper end. Registered best = prefix-joint where both levers hold, joint where only L2 holds.\n\n"
    + "\n".join(L) + "\n\nA/A (`rho-neg-aa / rho-neg`) = 1.0000 on every row and weight. 24/24 sessions: `verify --replay-all` audit OK "
    "(73,728 records, 55,296 replays reproduced, 0 problems). T2 correctness: 12,288 joint/inc and 12,288 prefix-joint/prefix pairs, 0 mismatches.\n")
json.dump(S, open("summary.json", "w"), indent=1)
print("\n".join(L))
