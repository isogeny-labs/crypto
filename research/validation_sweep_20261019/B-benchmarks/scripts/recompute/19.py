# SWB-RECOMPUTE-20 row 19: recompute headline numbers from committed raw records only.
import csv, json, sys
CSV = "/home/user/crypto/benchmarks/20261017-gpu-nist-jac-seed/data.csv"
rows = list(csv.DictReader(open(CSV, newline="")))
out = {}
for r in rows:
    name = r["curve"]
    jac, aff, sp = r["jacobian_ms_per_seed"], r["affine_ms_per_seed"], r["speedup"]
    rec = {"scalar_bits": r["scalar_bits"], "iters": r["iters"], "mismatches": r["mismatches"],
           "inversions_before": r["inversions_before"], "inversions_after": r["inversions_after"],
           "csv_speedup": sp}
    if jac != "NA" and aff != "NA":
        rec["computed_speedup_affine_over_jacobian"] = "%.4f" % (float(aff) / float(jac))
        rec["jacobian_ms_per_seed"] = "%.4f" % float(jac)
        rec["affine_ms_per_seed"] = "%.4f" % float(aff)
    out[name] = rec
print(json.dumps(out, indent=1, sort_keys=True))
