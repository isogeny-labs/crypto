#!/usr/bin/env python3
"""Pilot summary (not data). usage (cwd = study dir): python3 pilot/analyse.py <ecbench> <pilot sessions dir>
Ratios are `ecbench compare --a inc --b <arm> --weight w --resamples 4000 --seed 20261118` per row (one session each)."""
import json, subprocess, sys
B, D = sys.argv[1], sys.argv[2]
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]; WS = ["0", "0.1", "1"]


def cmp(d, a, b, w):
    j = json.loads(subprocess.run([B, "compare", "--dir", d, "--a", a, "--b", b, "--weight", w, "--resamples", "4000",
                                   "--seed", "20261118", "--json"], capture_output=True, text=True, check=True).stdout)
    o = j["ops"]
    return {"ratio": o["ratio_b_over_a"], "ci95": o["ci95"], "pairs": o["pairs"]}


out = {}
lines = ["| row | arm | w = 0 | w = 0.1 | w = 1 | arm / rho-neg (w = 0) |", "|:--|:--|:--|:--|:--|--:|"]
for row in ROWS:
    d = f"{D}/{row}"; out[row] = {}
    for arm in ["prefix", "joint", "prefix-joint"]:
        out[row][arm] = {w: cmp(d, "inc", arm, w) for w in WS}
        out[row][arm]["gap_w0"] = cmp(d, "rho-neg", arm, "0")["ratio"]
        lines.append(f"| {row} | {arm} | " + " | ".join(f"{out[row][arm][w]['ratio']:.4f} [{out[row][arm][w]['ci95'][0]:.4f}, {out[row][arm][w]['ci95'][1]:.4f}]" for w in WS) + f" | {out[row][arm]['gap_w0']:.2f} |")
    out[row]["inc_gap_w0"] = cmp(d, "rho-neg", "inc", "0")["ratio"]
json.dump(out, open("pilot/pilot_summary.json", "w"), indent=1)
open("pilot/pilot_table.md", "w").write("\n".join(lines) + "\n")
print("\n".join(lines))
