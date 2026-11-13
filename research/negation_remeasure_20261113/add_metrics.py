#!/usr/bin/env python3
"""Add the headline metrics of remeasure.json to the open bench record.

Usage: python3 add_metrics.py RECORD [remeasure.json]
Value = arm total at the price (band 0, 0.1, 1), reference = the negation-map
rho total at the same price (matched_rho). [pcc:unverified]
"""
import json
import subprocess
import sys

rec = sys.argv[1]
d = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "research/negation_remeasure_20261113/remeasure.json"))
ARMS = {"v3": ["recorded", "C"], "cover_pushforward": ["ic_jc", "C"], "tower_large": ["g2_recorded", "g2_C", "g3_recorded", "g3_C"]}
TAG = {"v3": "v3", "cover_pushforward": "cover", "tower_large": "tower"}
PRICE = ["w0", "w0.1", "w1"]
n = 0
for tn, t in d["targets"].items():
    for r in t["rows"]:
        neg = r["rho"]["negation"]
        nruns = neg["runs"]
        rho_tot = [neg["total_ops"] + w * neg["native"] for w in (0.0, 0.1, 1.0)]
        for a in ARMS[tn]:
            e = r["arms"].get(a)
            if not e:
                continue
            for i, pr in enumerate(PRICE):
                name = f"{TAG[tn]}_p{r['p']}_{a.lower()}_total_{pr}"
                cmd = ["python3", "tools/benchrec.py", "metric", rec, "--name", name, "--value", repr(e["total_band"][i]), "--unit", "gae",
                       "--better", "lower", "--reference", repr(rho_tot[i]), "--reference-kind", "matched_rho", "--n", str(nruns),
                       "--note", f"{tn} arm {a}{' (lower bound: unemitted counters)' if e['lower_bound'] else ''}; reference = negation-map rho total with its native counters priced at the same w, same process"]
                subprocess.run(cmd, check=True, capture_output=True)
                n += 1
        # rho native emission, one metric per row
        for kind in ("negation", "plain"):
            k = r["rho"][kind]
            name = f"{TAG[tn]}_p{r['p']}_rho_{kind}_native_per_step"
            subprocess.run(["python3", "tools/benchrec.py", "metric", rec, "--name", name, "--value", repr(k["native_per_step"]), "--unit", "count",
                            "--better", "neutral", "--n", str(k["runs"]),
                            "--note", "canonicalisations + partition hashes + DP lookups + DP inserts + word ops, per walk step, mean over seeds"], check=True, capture_output=True)
            n += 1
print("metrics added:", n)
