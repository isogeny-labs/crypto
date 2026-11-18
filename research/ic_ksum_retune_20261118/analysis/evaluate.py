#!/usr/bin/env python3
"""Registered evaluation (PROTOCOL.md section 3). usage (cwd = study dir): python3 analysis/evaluate.py <ecbench> <sessions dir>
Writes results.json. Every ratio is `ecbench compare ... --weight w --resamples 4000 --seed 20261118` (ratio of totals)."""
import json, subprocess, sys
B, D = sys.argv[1], sys.argv[2]
ROWS = ["P24", "B18", "B24"]; WS = ["0", "0.1", "1"]


def cmp(dirs, a, b, w):
    args = [B, "compare"] + sum([["--dir", d] for d in dirs], []) + ["--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261118", "--json"]
    j = json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)
    o = j["ops"]; c = j["curves"][0]
    return {"ratio": o["ratio_b_over_a"], "ci95": o["ci95"], "ci95_bootstrap": c.get("ci95_bootstrap"), "pairs": o["pairs"],
            "upper": max(o["ci95"][1], (c.get("ci95_bootstrap") or [0, 0])[1]), "lower": min(o["ci95"][0], (c.get("ci95_bootstrap") or [9, 9])[0]),
            "admissible": o.get("status") == "ok", "s_a": j["a"]["mean_s"], "s_b": j["b"]["mean_s"]}


res = {"rows": {}}
for row in ROWS:
    dirs = [f"{D}/{row}-r{i}" for i in (1, 2, 3)]
    R = {}
    for w in WS:
        R[w] = {"pooled": cmp(dirs, "incumbent", "retuned", w), "per_replicate": [cmp([d], "incumbent", "retuned", w) for d in dirs],
                "aa": cmp(dirs, "rho-neg", "rho-neg-aa", w),
                "gap_incumbent": cmp(dirs, "rho-neg", "incumbent", w), "gap_retuned": cmp(dirs, "rho-neg", "retuned", w)}
    max_up = max(max([R[w]["pooled"]["upper"]] + [p["upper"] for p in R[w]["per_replicate"]]) for w in WS)
    aa_dev = max(abs(R[w]["aa"]["ratio"] - 1) for w in WS)
    holds = max_up < 1 and (1 - max_up) > aa_dev
    worse = any(R[w]["pooled"]["lower"] > 1 for w in WS)
    res["rows"][row] = {"weights": R, "max_upper": max_up, "aa_max_dev": aa_dev, "holds": holds, "significantly_worse": worse}
held = [r for r, v in res["rows"].items() if v["holds"]]
res["verdict"] = {"rows_holding": held, "rows_significantly_worse": [r for r, v in res["rows"].items() if v["significantly_worse"]],
                  "rule_improves": bool(held) and not any(v["significantly_worse"] for v in res["rows"].values()),
                  "all_changed_rows_hold": len(held) == len(ROWS)}
json.dump(res, open("results.json", "w"), indent=1)
for row, v in res["rows"].items():
    print(row, "holds" if v["holds"] else "fails", f"max_upper={v['max_upper']:.4f}",
          *(f"w{w}: {v['weights'][w]['pooled']['ratio']:.4f} [{v['weights'][w]['pooled']['ci95'][0]:.4f}, {v['weights'][w]['pooled']['ci95'][1]:.4f}] gap {v['weights'][w]['gap_incumbent']['ratio']:.2f}->{v['weights'][w]['gap_retuned']['ratio']:.2f}" for w in WS))
print(res["verdict"])
