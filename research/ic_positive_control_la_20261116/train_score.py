#!/usr/bin/env python3
"""PROTOCOL section 4: score each cc by the geometric mean of S/R total_w01
over (p, curve, seed); pick the lowest, ties to the larger cc. Writes TRAINING.md."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
GRID = ["0.7", "1.0", "1.4", "2.0"]
scores, detail = {}, {}
for cc in GRID:
    rows = json.load(open(os.path.join(HERE, f"train_cc{cc}.json")))["rows"]
    rho = {(r["p"], r["curve_idx"], r["seed"]): r for r in rows if r["kind"] == "rho"}
    logs, per_p, bad = [], {}, 0
    for r in rows:
        if r["kind"] != "ic":
            continue
        bad += not r["certified"]
        k = (r["p"], r["curve_idx"], r["seed"])
        x = r["total_w01"] / rho[k]["total_w01"]
        logs.append(math.log(x))
        per_p.setdefault(r["p"], []).append(x)
    scores[cc] = math.exp(sum(logs) / len(logs))
    detail[cc] = {
        "n": len(logs),
        "uncertified": bad,
        "per_p_geomean": {p: math.exp(sum(map(math.log, v)) / len(v)) for p, v in sorted(per_p.items())},
    }
best = min(GRID, key=lambda c: (scores[c], -float(c)))
lines = ["# Training sweep result (PROTOCOL section 4) [pcc:unverified]", "",
         "Curves `curve_seed0 = 5001` (disjoint from the test curves), arm S, primes 4099, 8209, 16411, 2 curves × 2 seeds.", "",
         "| cc | runs | uncertified | score: geomean S/R at w = 0.1 | 4099 | 8209 | 16411 |", "|--:|--:|--:|--:|--:|--:|--:|"]
for cc in GRID:
    pp = detail[cc]["per_p_geomean"]
    lines.append(f"| {cc} | {detail[cc]['n']} | {detail[cc]['uncertified']} | {scores[cc]:.3f} | "
                 + " | ".join(f"{pp[p]:.3f}" for p in sorted(pp)) + " |")
lines += ["", f"**cc\\* = {best}** (lowest score; ties to the larger cc).",
          f"S* is reported as the end-to-end operating point: {'yes' if scores[best] < scores['2.0'] else 'no (cc = 2 is best)'}."]
open(os.path.join(HERE, "TRAINING.md"), "w").write("\n".join(lines) + "\n")
json.dump({"scores": scores, "detail": detail, "cc_star": best}, open(os.path.join(HERE, "training.json"), "w"), indent=1)
print("\n".join(lines))
