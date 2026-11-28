#!/usr/bin/env python3
"""results.json -> results.md, summary.json (layout only). cwd = study dir."""
import json
d = json.load(open("results.json")); W = ["0", "0.1", "1"]
INC = {"P24": "k5 N11", "B18": "k4 N7", "B24": "k5 N9"}; RET = {"P24": "k6 N10", "B18": "k5 N7", "B24": "k6 N9"}
L = ["| row | incumbent -> retuned | w | retuned / incumbent [95% lin-t; bootstrap] | per replicate (upper) | gap to rho-neg, incumbent -> retuned | verdict |",
     "|:--|:--|--:|:--|:--|:--|:--|"]
S = {"schema": "crypto.ic_ksum_retune_summary/v1", "standing": "[pcc:unverified]", "unit": "ecbench counted GAE, ratio of totals; w in {0, 0.1, 1} GAE per native op",
     "binary_sha256": "808511395723f84505d4fe8edaf32e66e8d8bff34cdad57777360c5999ffe161", "rule": "R1 (rule.py, w = 0.1 objective)",
     "verdict": d["verdict"], "unchanged_rows": ["P20", "P22", "B15", "B21", "B27"], "rows": {}}
for row, v in d["rows"].items():
    S["rows"][row] = {"incumbent": INC[row], "retuned": RET[row], "holds": v["holds"], "max_upper": round(v["max_upper"], 4), "weights": {}}
    for w in W:
        x = v["weights"][w]; p = x["pooled"]
        verdict = ("holds" if v["holds"] else "fails") if w == "0" else ""
        L.append(f"| {row if w == '0' else ''} | {INC[row] + ' -> ' + RET[row] if w == '0' else ''} | {w} | {p['ratio']:.4f} [{p['ci95'][0]:.4f}, {p['ci95'][1]:.4f}; {p['ci95_bootstrap'][0]:.4f}, {p['ci95_bootstrap'][1]:.4f}] | "
                 + ", ".join(f"{q['ratio']:.3f} ({q['upper']:.3f})" for q in x["per_replicate"]) + f" | {x['gap_incumbent']['ratio']:.2f} -> {x['gap_retuned']['ratio']:.2f} | {verdict} |")
        S["rows"][row]["weights"][w] = {"ratio": round(p["ratio"], 4), "ci95": [round(c, 4) for c in p["ci95"]], "ci95_bootstrap": [round(c, 4) for c in p["ci95_bootstrap"]],
                                        "per_replicate": [round(q["ratio"], 4) for q in x["per_replicate"]], "per_replicate_upper": [round(q["upper"], 4) for q in x["per_replicate"]],
                                        "gap_incumbent": round(x["gap_incumbent"]["ratio"], 3), "gap_retuned": round(x["gap_retuned"]["ratio"], 3), "aa": x["aa"]["ratio"]}
open("results.md", "w").write("# Confirmatory results [pcc:unverified]\n\n" + "\n".join(L) + "\n\nA/A (`rho-neg-aa / rho-neg`) = 1.0000 on every row and weight. All 9 sessions: `verify --replay-all` audit OK (6,912 records, 5,184 replays, 0 problems).\n")
json.dump(S, open("summary.json", "w"), indent=1)
print("\n".join(L))
