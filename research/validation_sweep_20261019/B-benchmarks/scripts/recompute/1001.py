import json
# HEADLINE of 20261014-p61-class-pilot: "`p61-i0025` selected-class launch study." contains no numeric claim.
T = "research/p61_isogeny_class_20261014/reports/run-20261014T0902Z/timings.json"
t = json.load(open(T))
print("headline numeric claims: 0")
print("timings.json segments:", [(s["label"], s["vertices"], s["generation_elapsed_ms"], s["verification_elapsed_ms"]) for s in t["segments"]])
