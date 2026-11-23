#!/usr/bin/env python3
"""Independent recomputation (python3 -I) of the same-base geometric-mean ratios from
records.jsonl, compared with summary.json. Reads data only; never executes it.
usage: python3 -I sgsecant_recheck.py PANEL_DIR"""
import json, math, sys
d = sys.argv[1]
recs = [json.loads(l) for l in open(f"{d}/records.jsonl")]
summ = json.load(open(f"{d}/summary.json"))
ph = recs[0]["phases"]
def led(r):
    L = r["phase_ledger_mul_add_inv_native"]
    return [tuple(x) for x in L]
def cost(r, metric, incl):
    t = [sum(x[i] for x in led(r)) for i in range(4)]
    c = list(r["construct"])
    if incl:
        t = [a + b for a, b in zip(t, c)]
    mul, add, inv, nat = t
    return {"field": mul + add + inv, "mul_only": mul, "inv_x10": mul + add + 10 * inv,
            "field+0.1native": mul + add + inv + 0.1 * nat, "field+1native": mul + add + inv + nat}[metric]
def ok(r):
    return r["completed"] and r["censored_systems"] == 0 and r["unresolved"] == 0 and r["nonzero_dim_systems"] == 0
idx = {(r["p"], r["d"], r["inst"], r["run"], r["arm"]): r for r in recs}
bad = 0; checked = 0
for cell in summ["cells"]:
    for c in cell["comparisons"]:
        ratios = []
        for (p, dd, inst, run, arm), ra in idx.items():
            if p != cell["p"] or dd != cell["d"] or arm != c["a"]:
                continue
            rb = idx.get((p, dd, inst, run, c["b"]))
            if rb and ok(ra) and ok(rb):
                ratios.append(cost(ra, c["metric"], c["incl_construct"]) / cost(rb, c["metric"], c["incl_construct"]))
        gm = math.exp(sum(map(math.log, ratios)) / len(ratios)) if ratios else float("nan")
        checked += 1
        if len(ratios) != c["n"] or (ratios and abs(gm - c["gm_ratio"]) > 1e-4 * max(1, gm)):
            bad += 1
            print("MISMATCH", cell["p"], cell["d"], c["comparison"], c["metric"], c["incl_construct"], len(ratios), c["n"], gm, c["gm_ratio"])
print(f"recheck: {checked} comparison rows recomputed, {bad} mismatches")
sys.exit(1 if bad else 0)
