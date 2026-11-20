#!/usr/bin/env python3
"""Pilot summary (not data). usage: python3 pilot/analyse.py <ecbench> <pilot sessions dir>  -> pilot/pilot_summary.json, stdout table.
Measured ratios (ecbench compare, ratio of totals, --resamples 1000) of off and sib{4,8,16} against inc and of inc against
rho-neg, beside the count model's prediction (pilot/model_pilot.json)."""
import json, subprocess, sys
B, D = sys.argv[1], sys.argv[2]
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]; WS = ["0", "0.1", "1"]
model = json.load(open("pilot/model_pilot.json"))
def cmp(row, a, b, w):
    j = json.loads(subprocess.run([B, "compare", "--dir", f"{D}/{row}", "--a", a, "--b", b, "--weight", w, "--resamples", "1000",
                                   "--seed", "20261120", "--json"], capture_output=True, text=True, check=True).stdout)
    return {"ratio": j["ops"]["ratio_b_over_a"], "ci95": j["ops"]["ci95"]}
out = {}
for row in ROWS:
    out[row] = {arm: {w: cmp(row, "inc", arm, w) for w in WS} for arm in ["off", "sib4", "sib8", "sib16"]}
    out[row]["gap_inc"] = {w: cmp(row, "rho-neg", "inc", w)["ratio"] for w in WS}
    for m in ("4", "8", "16"):
        print(row, f"m={m:>2}", " ".join(f"w{w}: {out[row]['sib'+m][w]['ratio']:.3f} [{out[row]['sib'+m][w]['ci95'][0]:.3f},{out[row]['sib'+m][w]['ci95'][1]:.3f}] model {model[row]['pred'][m][str(float(w))]:.3f}" for w in WS))
    print(row, "off/inc", [round(out[row]["off"][w]["ratio"], 6) for w in WS], "gap inc", [round(out[row]["gap_inc"][w], 3) for w in WS])
json.dump(out, open("pilot/pilot_summary.json", "w"), indent=1)
