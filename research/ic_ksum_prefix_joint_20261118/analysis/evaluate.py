#!/usr/bin/env python3
"""Registered evaluation (PROTOCOL.md section 3). usage (cwd = study dir): python3 analysis/evaluate.py <ecbench> <sessions dir>
Writes results.json. Every ratio is `ecbench compare ... --weight w --resamples 4000 --seed 20261118` (ops ratio of totals,
paired by target and round); this script only reads the comparisons and applies the registered thresholds."""
import json, subprocess, sys
B, D = sys.argv[1], sys.argv[2]
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]; WS = ["0", "0.1", "1"]
LEVERS = {"L1": ("inc", "prefix"), "L2": ("inc", "joint")}
SECONDARY = {"L1|L2": ("joint", "prefix-joint"), "L2|L1": ("prefix", "prefix-joint"), "both": ("inc", "prefix-joint")}
ARMS = ["inc", "prefix", "joint", "prefix-joint"]


def cmp(dirs, a, b, w):
    args = [B, "compare"] + sum([["--dir", d] for d in dirs], []) + ["--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261118", "--json"]
    j = json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)
    if "ops" in j:  # one session: ecbench.comparison/v2
        o = j["ops"]; c = j["curves"][0]
        ratio, ci, boot, pairs, ok = o["ratio_b_over_a"], o["ci95"], c.get("ci95_bootstrap"), o["pairs"], o.get("status") == "ok"
    else:  # several sessions: ecbench.pooled_comparison/v1
        ratio, ci, boot, pairs, ok = j["ratio_b_over_a"], j["ci95"], j.get("ci95_bootstrap"), j["pairs"], bool(j.get("admissible"))
    return {"ratio": ratio, "ci95": ci, "ci95_bootstrap": boot, "pairs": pairs,
            "upper": max(ci[1], (boot or [0, 0])[1]), "lower": min(ci[0], (boot or [9, 9])[0]), "admissible": ok}


res = {"rows": {}, "levers": {}}
for row in ROWS:
    dirs = [f"{D}/{row}-r{i}" for i in (1, 2, 3)]
    R = {"aa": {w: cmp(dirs, "rho-neg", "rho-neg-aa", w) for w in WS},
         "gap": {arm: {w: cmp(dirs, "rho-neg", arm, w) for w in WS} for arm in ARMS}}
    aa_dev = max(abs(R["aa"][w]["ratio"] - 1) for w in WS)
    for lever, (a, b) in LEVERS.items():
        L = {w: {"pooled": cmp(dirs, a, b, w), "per_replicate": [cmp([d], a, b, w) for d in dirs]} for w in WS}
        max_up = max(max([L[w]["pooled"]["upper"]] + [p["upper"] for p in L[w]["per_replicate"]]) for w in WS)
        L["max_upper"] = max_up
        L["holds"] = max_up < 1 and (1 - max_up) > aa_dev
        L["significantly_worse"] = any(L[w]["pooled"]["lower"] > 1 for w in WS)
        R[lever] = L
    for name, (a, b) in SECONDARY.items():
        R[name] = {w: cmp(dirs, a, b, w) for w in WS}
    R["aa_max_dev"] = aa_dev
    res["rows"][row] = R
for lever in LEVERS:
    held = [r for r in ROWS if res["rows"][r][lever]["holds"]]
    worse = [r for r in ROWS if res["rows"][r][lever]["significantly_worse"]]
    res["levers"][lever] = {"rows_holding": held, "rows_significantly_worse": worse, "improves": bool(held) and not worse}
json.dump(res, open("results.json", "w"), indent=1)
for row in ROWS:
    v = res["rows"][row]
    for lever in LEVERS:
        L = v[lever]
        print(row, lever, "holds" if L["holds"] else "fails", f"max_upper={L['max_upper']:.4f}",
              *(f"w{w}: {L[w]['pooled']['ratio']:.4f} [{L[w]['pooled']['ci95'][0]:.4f}, {L[w]['pooled']['ci95'][1]:.4f}]" for w in WS))
    print(row, "both", *(f"w{w}: {v['both'][w]['ratio']:.4f}" for w in WS),
          "gap inc->both", *(f"w{w}: {v['gap']['inc'][w]['ratio']:.2f}->{v['gap']['prefix-joint'][w]['ratio']:.2f}" for w in WS))
print(json.dumps(res["levers"]))
