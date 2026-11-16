"""Score the confirmatory run against PREDICTIONS.json (frozen before it ran)."""
import json, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyse import ci
P = json.load(open("PREDICTIONS.json"))
rows = []
for p in P["predictions"]:
    d = json.load(open(f"data/confirm-F{p['F']}-J{p['J']}.json"))["mean_steps_per_walk"]
    num, den = p["ratio"].split("/")
    v = sum(d[num]) / sum(d[den]); lo, hi = ci(d[num], d[den])
    rows.append({**p, "value": v, "ci": [lo, hi], "pass": lo <= p["prediction"] <= hi})
json.dump({"rows": rows, "passed": sum(r["pass"] for r in rows), "of": len(rows)}, sys.stdout, indent=1)
