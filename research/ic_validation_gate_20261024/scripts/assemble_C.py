#!/usr/bin/env python3
"""Assemble the Table C artifacts and record the standing. usage: assemble_C.py REPO
Writes C/replay-certificate.json, C/rho-reference-assessment.json, C/standing.json.
Never writes C/VALIDATION.json: R4 fails for every row (see the assessment's reasons)."""
import hashlib, json, os, statistics, sys, glob
from pathlib import Path

repo = Path(sys.argv[1]); here = repo / "research/ic_validation_gate_20261024"; C = here / "C"
rep = json.load(open(C / "validator-report.json"))
summ = json.load(open(here / "raw/replay_C_summary.json"))
s20 = {(z["a"], z["n"]): z for z in json.load(open(repo / "research/ic_exponent_20260926/analysis.json"))["sizes"]}
ana = json.load(open(repo / "research/ic_descent_20260930/analysis-isolated.json"))
by_curve = {}
for r in summ: by_curve.setdefault(r["curve"].replace(" ", ""), []).append(r)
standing, assess, runs = {}, {}, []
for row in rep["rows"]:
    kn = row["dir"]; a = int(kn[1:].split("n")[0]); n = int(kn.split("n")[1])
    rs = [r for r in summ if r["run"].startswith(kn + "/")]
    u0 = s20[(a, n)]["unit_ns"]
    ub = [x["unit_ns"] for f in glob.glob(str(repo / f"research/ic_descent_20260930/runs-isolated/main/{kn}/M*/r*-baseline.price.json"))
          for x in json.load(open(f))["repetitions"]]
    reasons = []
    if not all(r["identical"] for r in rs): reasons.append("R1: " + ", ".join(r["run"] for r in rs if not r["identical"]))
    reasons.append("R2: targets are generated inside the binary from random_seed; no target point or generator is archived, so [d]G = Q "
                   "cannot be checked by an independent implementation (the binary's own verifier says all_verified)")
    if not row["pass_recompute"]: reasons.append("R3 recompute disagrees")
    reasons.append("R4 same process: the IC figure is s_before / (paired wall-time speedup of candidate over baseline), both from the "
                   "ic_descent_20260930 isolated runs; the reference S is the section-20 batch rho (ic_exponent_20260926, 2026-09-26) priced at "
                   "that study's canonical step: two studies, two processes, not the same unit measurement")
    reasons.append("R4 band: the IC arm is a wall-clock figure and rho a counted figure priced at the canonical step; neither names unpriced "
                   "native counts that a 0 / 0.1 / 1 GAE band could be applied to")
    standing[row["slug"]] = {"curve": row["curve"], "pass": False, "reasons": reasons}
    assess[row["slug"]] = {"same_unit": False, "same_binary_or_process": False, "matches_protocol": True,
                           "strongest_available": "yes at the canonical step (signed Frobenius and negation batch rho); not re-checked against the 2026-10-14 faster rho implementation (counts unchanged)",
                           "gae_band": "not computable", "order_holds_across_band": None,
                           "unit_ns_s20_process": u0, "unit_ns_v2_baseline_median": statistics.median(ub),
                           "unit_ratio_s20_over_v2": u0 / statistics.median(ub),
                           "x_reference_if_rho_unit_scaled_to_v2_process_ESTIMATE": row["recomputed"]["x_reference"] * statistics.median(ub) / u0,
                           "x_reference_board": row["recomputed"]["x_reference"]}
    runs.append({"slug": row["slug"], "dir": kn, "sampled_runs": len(rs), "identical": sum(r["identical"] for r in rs),
                 "recovered_scalars_per_run": 32})
cert = {"schema": "replay-certificate/v1", "status": "verified" if all(r["identical"] for r in summ) else "fail",
        "independent": True, "scalars_verified": False,
        "scalars_note": "replayed natively; all_verified true and the 32 recovered scalars equal the archive; NOT independently checked "
                        "as [d]G = Q because the archive holds no target points. `scalars_verified` is therefore false.",
        "what": "Seeded sample (3 per curve, 27 runs, seed ic_validation_gate_20261024 frozen in PROTOCOL.md before the draw) of the archived "
                "isolated main-comparison price runs, re-executed with `ic price --params` on the current binary, compared on counts, "
                "recovered scalars and all_verified.",
        "runs_replayed": len(summ), "runs_identical": sum(r["identical"] for r in summ),
        "extra_counters_in_new_binary": sorted({c for r in summ for c in r["new_binary_extra_counters"]}),
        "runs": runs, "detail": "../raw/replay_C_summary.json"}
json.dump(cert, open(C / "replay-certificate.json", "w"), indent=1)
json.dump(assess, open(C / "rho-reference-assessment.json", "w"), indent=1)
json.dump(standing, open(C / "standing.json", "w"), indent=1)
print(cert["status"], cert["runs_identical"], "/", cert["runs_replayed"], cert["extra_counters_in_new_binary"])
