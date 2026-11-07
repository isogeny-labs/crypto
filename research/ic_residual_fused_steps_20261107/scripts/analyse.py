"""Ratios of mean steps per walk between arms, with a paired bootstrap 95% CI
(reps share seeds across arms, so arm pairs are paired by rep)."""
import json, random, sys, math
def ci(a, b, B=4000, seed=7):
    rng = random.Random(seed); n = len(a); out = []
    for _ in range(B):
        idx = [rng.randrange(n) for _ in range(n)]
        out.append(sum(a[i] for i in idx) / sum(b[i] for i in idx))
    out.sort(); return out[int(.025*B)], out[int(.975*B)]
rows = []
for path in sys.argv[1:]:
    d = json.load(open(path)); m = d["mean_steps_per_walk"]
    for num, den, pred in [("fused","base",math.sqrt(1.5)),("fusedind","fused",1.0),("fused_ns","base_ns",math.sqrt(2)),("base_ns","base",math.sqrt(2)),("fused_ns","fused",math.sqrt(2))]:
        r = sum(m[num]) / sum(m[den]); lo, hi = ci(m[num], m[den])
        rows.append({"file": path, "F": d["F"], "ratio": f"{num}/{den}", "value": r, "ci": [lo, hi], "prediction": pred, "pred_in_ci": lo <= pred <= hi})
json.dump(rows, sys.stdout, indent=1)
