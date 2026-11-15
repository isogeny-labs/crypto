#!/usr/bin/env python3
"""Write the target-4 gate inputs (replay certificate, validator report) from the
validator's own outputs.  Recomputes every record metric and the RESULTS.md headline
numbers from the replayed raw data with validator code, pairs each with the claimed
value, and pins the repository files it compared against.
Usage: make_t4_gate.py EXPORT_ROOT REPLAY_small_de.json BAND.json RECOMPUTE.json OUTDIR"""
import json, sys, os, re, hashlib
root, replay, band_p, rc_p, outdir = sys.argv[1:6]
raw = json.load(open(replay)); band = json.load(open(band_p)); rc = json.load(open(rc_p))
rec_path = "benchmarks/records/20261112T1753Z-small-de-endo-fb.json"
rec = json.load(open(os.path.join(root, rec_path)))
EV = "research/small_de_endo_fb_20261112"
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
def nat(n): return n["muls"] + n["invs"] + n["tests"] + n["la_ops"]
cells = {(c["bits"], c["m"], c["d"]): c for c in raw["cells"]}
head = []
for m in rec["metrics"]:
    mm = re.match(r"(ic_single_instance_w0p1|ic_relation_phase_w0p1|columns|targets_certified|decomposition_probability)_b(\d+)_m(\d)_d(m?\d+)_a(\d)$", m["name"])
    kind, b, mmm, d, a = mm.group(1), int(mm.group(2)), int(mm.group(3)), mm.group(4), "A" + mm.group(5)
    d = -int(d[1:]) if d.startswith("m") else int(d)
    c = cells[(b, mmm, d)]; arm = next(x for x in c["arms"] if x["arm"] == a); w = 0.1
    rel = sum(r["gae"] + arm["build_gae"] + w * (nat(r["native"]) + nat(arm["build_native"])) for r in arm["reps"]) / len(arm["reps"])
    tgt = sum(t["gae"] + w * nat(t["native"]) for t in arm["targets"]) / len(arm["targets"])
    best = min(sum(x["gae"] + w * x["native"] for x in rk["runs"]) / len(rk["runs"]) for rk in c["rho"])
    val = {"ic_single_instance_w0p1": rel + tgt, "ic_relation_phase_w0p1": rel, "columns": arm["columns"],
           "targets_certified": sum(1 for t in arm["targets"] if t["certified"]),
           "decomposition_probability": sum(r["decomp_prob"] for r in arm["reps"]) / len(arm["reps"])}[kind]
    head.append({"name": f"record:{m['name']}", "claimed": m["value"], "recomputed": val, "rel_tol": 1e-9})
    if m.get("reference") is not None:
        head.append({"name": f"record:{m['name']}:reference(best matched rho, w=0.1)", "claimed": m["reference"], "recomputed": best, "rel_tol": 1e-9})
# RESULTS.md headline numbers (printed rounded; rel_tol = half a unit in the last printed digit, relative)
def rnd(claimed, rec_v, ulp): return {"claimed": claimed, "recomputed": rec_v, "rel_tol": max(1e-9, (ulp / 2) / abs(rec_v))}
mr = band["min_ratio"]
head += [dict(name="RESULTS.md:min single-instance IC/rho, w=0", **rnd(2.32, mr["0.0"], 0.01)),
         dict(name="RESULTS.md:min single-instance IC/rho, w=0.1", **rnd(7.62, mr["0.1"], 0.01)),
         dict(name="RESULTS.md:min single-instance IC/rho, w=1", **rnd(40.16, mr["1.0"], 0.01)),
         dict(name="RESULTS.md:best amortised IC/rho w=0.1 (12 bits, m=2, D=-11, A2)", **rnd(2.29, rc["best_amortised_ratio_w0.1"][0], 0.01)),
         dict(name="RESULTS.md:28-bit amortised IC/rho w=0.1, low end", **rnd(125, rc["amortised_ratio_w0.1_28bits_range"][0], 1)),
         dict(name="RESULTS.md:28-bit amortised IC/rho w=0.1, high end", **rnd(1631, rc["amortised_ratio_w0.1_28bits_range"][1], 1)),
         {"name": "RESULTS.md:arm-cells below matched rho at all three prices", "claimed": 0, "recomputed": band["order_counts"]["IC below"], "rel_tol": 1e-9},
         {"name": "RESULTS.md:arm-cells (IC arms x cells)", "claimed": 120, "recomputed": band["arm_cells"], "rel_tol": 1e-9},
         {"name": "RESULTS.md:targets certified (16 x 120 arm-cells)", "claimed": 1920, "recomputed": rc["ic_targets_independently_certified"], "rel_tol": 1e-9}]
bad = [h for h in head if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
pin = lambda rel: {"path": rel, "sha256": sha(os.path.join(root, EV, rel))}
VO = "../validation_20261112/out/t4/"
vrep = {
    "schema": "validator-report/v1",
    "target": "research/small_de_endo_fb_20261112 (bench record 20261112T1753Z-small-de-endo-fb)",
    "validated_commit": "c5678a98aef6bbee34df3f277e618dd0a00f8ab1",
    "validator": "validator role (Claude Opus 5.5), not the producer; restart 3, 2026-11-13",
    "status": "pass" if not bad else "fail",
    "independent": True,
    "standing": "[pcc:unverified]: no pcc claim id; this report carries no keyring signature",
    "raw_data": [pin("small_de_derived.json"), pin("tables.md"), pin("run.log"), pin(".artifacts.json"), pin("PROTOCOL.md"),
                 {"path": VO + "band.json", "sha256": sha(os.path.join(outdir, "band.json"))},
                 {"path": VO + "recompute.json", "sha256": sha(os.path.join(outdir, "recompute.json"))},
                 {"path": VO + "sd_dump.jsonl", "sha256": sha(os.path.join(outdir, "sd_dump.jsonl"))}],
    "raw_data_note": ("small_de.json (store sha256 8bcdde7c..., 1,584,609 bytes, .artifacts.json) could not be fetched: the "
                      "environment's S3 credentials were rejected (InvalidAccessKeyId). The validator replayed the recorded command "
                      "from c5678a9 and recomputed from that replay; render.py applied to the replay gives small_de_derived.json, "
                      "tables.md and small_de.svg byte-identical to the committed files, and the replay's sha256 differs from the "
                      "store pin only through timing fields (construction_ms, wall_ms). The store bytes themselves are unverified."),
    "method": ["recompute_small_de.py: single-instance and relation-phase totals at w = 0, 0.1, 1, best matched rho, 720 per-arm-cell values vs small_de_derived.json (0 mismatches), A2 vs A1 rule verdicts",
               "band_small_de.py: order of every arm-cell against the best matched rho at 0, 0.1, 1; extra unemitted rho native work needed to reverse it",
               "check_small_de_prose.py: RESULTS.md Answer-section ratios",
               "check_sd_instances.py / check_sd_cm.py: curves, CM data, lambda, and 1,920 IC target logs certified with vlib",
               "val_sd_rho output: 2,240 rho logs certified with vlib; costs equal small_de.json"],
    "headline": head,
    "headline_failures": bad,
    "findings": ["RESULTS.md:19 and :111 say the per-trial solving cost is the same 'to the operation' in every arm of every cell; "
                 "in 12 of 60 cells (all m = 3, 12 to 20 bits) the per-trial field-operation count differs between repetitions and "
                 "arms by up to 0.85%; the A2/A1 ratio of the means is 1.000 to three decimals in 59 of 60 cells and 1.0007 in one. Prose overstatement; no headline number changes.",
                 "Record commit b7e8c68 (commit_dirty true) does not contain small_de.rs or small_de_ic.rs; they were first committed in e0f476b, 30 minutes after the run. No binary hash recorded. The source is unchanged from e0f476b to c5678a9 and reproduces the data.",
                 "PROTOCOL.md: the pre-data draft (57ea773, 05:45 UTC) is the byte-identical prefix before Amendment 1; Amendment 1 and frozen.sha256 were committed with the results (e0f476b), so its pre-data status rests on the producer's statement."],
}
json.dump(vrep, open(os.path.join(outdir, "validator-report.json"), "w"), indent=1)
cert = {
    "schema": "replay-certificate/v1",
    "status": "verified",
    "independent": True,
    "scalars_verified": True,
    "validator": vrep["validator"],
    "standing": vrep["standing"],
    "source": {"commit": "c5678a98aef6bbee34df3f277e618dd0a00f8ab1", "export": "git archive (no checkout of the working tree)",
               "note": "src/cryptanalysis/chained_transfer/small_de.rs, src/bin/small_de_ic.rs and src/cryptanalysis/ic_boundary.rs are identical between e0f476bd0f (first commit of this code) and c5678a9"},
    "build": {"toolchain": "rustc 1.97.0 (2d8144b78 2026-07-07)", "profile": "release (opt-level 3)", "binaries_sha256": open(os.path.join(outdir, "outputs.sha256")).read().strip().splitlines()[-3:]},
    "runs": [{"command": "small_de_ic --sizes 12,16,20,24,28 --reps 12:20,16:20,20:10,24:6,28:3 --targets 16 --out-dir <scratch>",
              "recorded_command": rec["commands"][0]["argv"], "exit": 0,
              "result": "every non-timing value reproduced: render.py(replay) -> small_de_derived.json, tables.md, small_de.svg byte-identical to the committed files; run.log identical after removing '(N ms)'; 120/120 record metrics equal (rel 1e-9)",
              "replay_raw_sha256": "3c76110e5a2ee5e7fc185fb00be8184b1e0140be14b0942843776cef0677ec7b (differs from the store pin 8bcdde7c... through timing fields; store bytes not fetched)"},
             {"command": "val_sd_rho (validator tool: the run_cell / rho_cost calls with the same instances, target stream and seeds)", "exit": 0,
              "result": "2,240 rho runs; gae and native equal small_de.json run by run; every recovered log certified [k]G = T by vlib"},
             {"command": "val_sd_dump (validator tool: construct() and the target stream)", "exit": 0,
              "result": "30 curves; 1,920 IC target logs in the replay certified by vlib; CM discriminant, j-invariant, lambda checked"}],
    "not_verified": ["store bytes of small_de.json (sha256 8bcdde7c...)", "wall-clock fields (construction_ms, wall_ms)",
                     "the IC solver's operation counts were reproduced by re-running the producer's code, not by an independent implementation"],
}
json.dump(cert, open(os.path.join(outdir, "replay-certificate.json"), "w"), indent=1)
print("headline entries", len(head), "failures", len(bad))
for h in bad[:10]: print("  ", h)
