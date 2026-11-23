#!/usr/bin/env python3
"""Sensitivity of C2 (genus 5 vs genus 4 at matched N) to the unit (lesson L-e388a81efa37).

The protocol's unit is one Jacobian group operation of the curve's own genus (GAE); a genus-5 Cantor
step and a genus-4 one are different amounts of field work.  Here every arm is re-priced in F_p
multiplications: IC = cantor_mul (the field mults inside its group operations) + w * (rel_mul + la_mul
+ rel_add + la_add) for w in {0, 0.1, 1}; the ideal negation reference = sqrt(pi r / 4) steps times the
mean cantor_mul per group operation measured on the same genus and prime (the IC rows' own ratio).
Reading only, not a replacement for the frozen one.  Writes validation/out/unit_sensitivity.json.
"""
import gzip, json, math, os
V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
PO = os.path.join(os.path.dirname(S), "genus4_prime_order_20261118")
def rows(f):
    op = gzip.open if f.endswith(".gz") else open
    return [json.loads(l) for l in op(f, "rt") if l.strip()]
files = {"g4_p16411_x*": os.path.join(PO, "b_p16411_xstar.jsonl.gz"), "g4_p32771_x*": os.path.join(PO, "b_p32771_xstar.jsonl.gz"),
         "g5_p2357_x5*": os.path.join(S, "b5_p2357_x5star.jsonl"), "g5_p4099_x5*": os.path.join(S, "b5_p4099_x5star.jsonl")}
out = {}
for k, f in files.items():
    rr = [r for r in rows(f) if r["kind"] == "run"]
    mpg = sum(r["cantor_mul"] for r in rr) / sum(r["group_ops"] for r in rr)
    R = sum(math.sqrt(math.pi * r["r"] / 4) for r in rr) / len(rr)
    out[k] = {"cantor_mul_per_group_op": mpg, "ratio_fmul_unit": {}}
    for w in (0.0, 0.1, 1.0):
        ic = sum(r["cantor_mul"] + w * (r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]) for r in rr) / len(rr)
        out[k]["ratio_fmul_unit"][str(w)] = ic / (R * mpg)
out["C2_fmul_unit"] = {f"{a}/{b}": {w: out[a]["ratio_fmul_unit"][w] / out[b]["ratio_fmul_unit"][w] for w in ("0.0", "0.1", "1.0")}
                       for a, b in (("g5_p2357_x5*", "g4_p16411_x*"), ("g5_p4099_x5*", "g4_p32771_x*"))}
json.dump(out, open(os.path.join(V, "out", "unit_sensitivity.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
