#!/usr/bin/env python3
"""Write the validation-gate artifacts for target A from the validator's own outputs.

Validator V, 2026-11-14. Inputs (all under research/validation_20261114/out/):
recompute_fj.json, tables_md_check.json, replay_compare.json, certs_check.json,
binaries.sha256, art.sha256. Refuses (exit 1) unless every check passed.
Writes out/gate/replay-certificate.json, out/gate/validator-report.json and
prints the sha256 of each, for VALIDATION.json.
Usage: make_gate_a.py REPO_ROOT
"""
import hashlib, json, os, sys

repo = sys.argv[1]
V = os.path.join(repo, "research/validation_20261114")
O = os.path.join(V, "out")
E = os.path.join(repo, "research/full_jacobian_ic_20261113")
HEAD = "224624a91a97639781ff336f2e8dc9a0738035b3"
PR_COMMITS = {"data_and_code": "c88c7b4e0c6409e8b2fa3a6d94294cb5d04ac289",
              "rustfmt_bin": "f1997205729b506658fadbaebfab7de84ba91aca",
              "merge": "cc075763d700e225dd269876d0589a307422da12"}
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
J = lambda n: json.load(open(os.path.join(O, n)))

rc, tm, rp, ce = J("recompute_fj.json"), J("tables_md_check.json"), J("replay_compare.json"), J("certs_check.json")
fail = []
if rc["mismatches"]: fail.append("recompute mismatches")
if tm["failures"]: fail.append("tables.md failures")
if rp["differences"]: fail.append("replay differences")
if ce["n_failures"]: fail.append("certificate failures")
if rc["summary"]["above_band"] != 36 or rc["summary"]["above_cal"] != 36: fail.append("cells")
if rc["rule"]["verdict"] != "unresolved": fail.append("verdict")
if fail:
    print("REFUSED:", fail); sys.exit(1)

fj = json.load(open(os.path.join(E, "full_jacobian.json")))
rec = json.load(open(os.path.join(repo, "benchmarks/records/20261113T1722Z-full-jacobian-ic.json")))
heads = []
for m in rc["record_metrics"]:
    if m["name"].startswith("total_"):
        heads.append({"name": "record:" + m["name"], "claimed": m["value"], "recomputed": m["recomputed"], "rel_tol": 5e-6})
        heads.append({"name": "record:" + m["name"] + ":reference(negation rho on E1, w=0.1)", "claimed": m["reference"], "recomputed": m["ref_recomputed"], "rel_tol": 5e-6})
    else:
        heads.append({"name": "record:" + m["name"], "claimed": m["value"], "recomputed": round(m["recomputed"][0], 3), "rel_tol": 1e-9})
R = {r["p"]: r for r in rc["rows"]}
for row in fj["rows"]:
    p = row["p"]
    for arm in ("A", "B", "A_S", "B_S"):
        for i, w in enumerate(("0", "0.1", "1")):
            heads.append({"name": f"json:p{p}:{arm}:ratio_neg_band[w={w}]", "claimed": row["arms"][arm]["ratio_neg_band"][i],
                          "recomputed": R[p]["arms"][arm]["ratio_neg_band"][i], "rel_tol": 1e-9})
            heads.append({"name": f"json:p{p}:{arm}:ratio_neg_cal[{('0','w_cal','10w_cal')[i]}]", "claimed": row["arms"][arm]["ratio_neg_cal"][i],
                          "recomputed": R[p]["arms"][arm]["ratio_neg_cal"][i], "rel_tol": 1e-9})
for arm in ("A", "B"):
    v = fj["verdict"]["per_arm"][arm]
    heads.append({"name": f"verdict:{arm}:ratio_slope_w0.1", "claimed": v["ratio_slope_w0.1"], "recomputed": rc["rule"][arm]["slope_w0.1"], "rel_tol": 1e-9})
    heads.append({"name": f"verdict:{arm}:ratio_slope_ci95_lower (claimed with t=2.571; recomputed with exact t=2.5706)",
                  "claimed": v["ratio_slope_ci95_lower"], "recomputed": rc["rule"][arm]["ci95_lower_exact_t"], "rel_tol": 2e-3})
heads.append({"name": "cells above at band and calibrated reading (of 36)", "claimed": 36, "recomputed": rc["summary"]["above_band"]})

raw = [{"path": n, "sha256": sha(os.path.join(E, n))} for n in ("full_jacobian.json", "run.log", "PROTOCOL.md", "frozen.sha256", "RESULTS.md", "tables.md")]
raw.append({"path": "../../benchmarks/records/20261113T1722Z-full-jacobian-ic.json", "sha256": sha(os.path.join(repo, "benchmarks/records/20261113T1722Z-full-jacobian-ic.json"))})
binaries = open(os.path.join(O, "binaries.sha256")).read().split("\n")
src_files = ["src/cryptanalysis/chained_transfer/full_jacobian_ic.rs", "src/bin/full_jacobian_ic.rs",
             "src/cryptanalysis/chained_transfer/ic_control.rs", "src/cryptanalysis/chained_transfer/negation_remeasure.rs",
             "src/cryptanalysis/chained_transfer/instrument.rs", "src/cryptanalysis/chained_transfer/mod.rs"]
import subprocess
def blob_sha(c, p):
    return hashlib.sha256(subprocess.run(["git", "-C", repo, "show", f"{c}:{p}"], capture_output=True, check=True).stdout).hexdigest()
src = [{"path": p, "sha256_at_head": blob_sha(HEAD, p), "sha256_at_rustfmt_commit": blob_sha(PR_COMMITS["rustfmt_bin"], p)} for p in src_files]
standing = "[pcc:unverified]: no pcc claim id; no keyring signature"
who = "Validator V (Claude Opus 5.5, validator role, independent of the producers); 2026-11-14"
cert = {
    "schema": "replay-certificate/v1", "status": "verified", "independent": True, "scalars_verified": True,
    "validator": who, "standing": standing,
    "source": {"commit": HEAD, "dirty": False, "pr_commits": PR_COMMITS,
               "commit_note": "built from a git-archive export of origin/main 224624a91a into the validator's scratch (not the shared working tree, which another agent was editing); every target file is byte-equal between f199720572 and 224624a91a",
               "recorded_run_binding": f"record commit {rec['commit']} with commit_dirty={rec['commit_dirty']}: the measured binary was built from uncommitted code; the clippy fixes at full_jacobian_ic.rs:627 and :791 and the rustfmt of the bin came after the run (producer scratch logs 17:39 and commit f199720572); this replay shows the committed code reproduces every non-timing value",
               "source_files": src},
    "build": {"toolchain": "rustc 1.97.0 (2d8144b78 2026-07-07)", "profile": "release", "binaries_sha256": [b for b in binaries if b.strip()]},
    "runs": [
        {"command": "full_jacobian_ic --threads 2 --stage2 25601,51203 --budget-s 2400 --out-dir <scratch>",
         "recorded_command": rec["commands"][0]["argv"], "exit": 0,
         "result": f"every non-timing value of full_jacobian.json identical: {rp['compared_values']} values, 0 differences (all 9 primes, stage 1 and 2); calibrated order classes: {rp['calibrated_class_values']} values, {len(rp['calibrated_class_differences'])} differ"},
        {"command": "val_fj_dump (validator program, research/validation_20261114/scripts/val_fj_dump.rs) + check_fj_certs.py",
         "exit": 0,
         "result": f"certificates re-verified with independent Python arithmetic (E1 group law, Cantor on the quintic model of J_C): {ce['totals']['ic_logs_ok_E1']} of {ce['totals']['ic_logs']} IC logs on E1, {ce['totals']['ic_logs_ok_J']} on J_C, {ce['totals']['rho_logs_ok']} of {ce['totals']['rho_logs']} rho logs on E1; per-run counters re-averaged equal the committed means ({ce['totals']['means_compared']} means, {ce['totals']['means_mismatch']} differ)"},
    ],
}
rep = {
    "schema": "validator-report/v1",
    "target": "research/full_jacobian_ic_20261113 (bench record 20261113T1722Z-full-jacobian-ic, status measured)",
    "validated_commit": f"{HEAD} (origin/main; target files byte-equal to {PR_COMMITS['rustfmt_bin']})",
    "validator": who, "status": "pass", "independent": True, "standing": standing,
    "raw_data": raw,
    "method": ["recompute_fj.py: band and calibrated totals, ratios, order classes of 45 arm-rows (36 IC cells), 64 fits with own Student-t quantile, the frozen rule, F1-F8 numbers, 20 record metrics, RESULTS.md table (2057 values, 0 mismatches)",
               "check_tables_md.py: 635 tables.md cells", "compare_replay.py: replay vs committed json",
               "check_fj_certs.py on val_fj_dump output: curve data, planted logs, every recovered log, j-invariants, #E1/#E2 by point counting, #J by the L-polynomial (p <= 3203) and by annihilation"],
    "headline": heads, "headline_failures": [],
    "verdict_under_frozen_rule": rc["rule"],
    "caveats": [
        "toy sizes only (p = 211 to 51,203), one session on one shared 4-vCPU VM with no isolation; wall times and the best-of-N calibrated weights are indicative",
        "stage 2 has 1 instance and 2 IC seeds per prime; the frozen rule reads stage 1 only",
        "freeze before data is not established by git: PROTOCOL.md, frozen.sha256 and the data enter in one commit (c88c7b4e0c, 18:56:30Z) after the run (17:29:02Z-17:36:10Z); the session scratch shows the protocol's own account is consistent (smoke run with unit class times 17:21:51Z, record opened 17:22:14Z) and that the p = 211 and 419 counters seen in the smoke equal the measured ones (340 values)",
        "IC arms are lower bounds although labelled lower_bound=false: the walk's per-step partition hash (mix64(key_of)), the large-prime HashMap and the RNG draws are not counted, nor the per-instance [h]Z of pick_z; every one of these only raises the IC side, so 'above' is conservative",
        "same unit is a counting convention: an IC group operation is a Cantor step on the word Jacobian (t_add 588-653 ns) and a rho operation an E1 addition (103-152 ns), so every IC/rho ratio is understated by about 4.3-5.7x in time",
    ],
    "findings": [
        "RESULTS.md scorecard F6 says held on per-prime means 0.483-0.536; the frozen F6 is per run, and 3 of 176 whole-J runs exceed 0.6 (p = 211: A k0 s2 0.6031, A k1 s3 0.6031, B k0 s2 0.6034; certs_check.json, val_fj_dump); per-prime means are 0.4830-0.5398, not 0.483-0.536. F6 failed narrowly under its frozen wording",
        "RESULTS.md 'At w = 0 ... the smallest ratio is 18.7, for A at p = 211': B at p = 211 is 17.96 (and the controls 10.9 and 11.9); 18.66 is the smallest for arm A only",
        "RESULTS.md 'fitted rho exponent against ln r 0.234' and 'ln(group ops / sqrt r) on ln p gives 0.420 for A and 0.292 for B, both positive' are fits over all 9 primes (stage 2 included, unlabelled); over the 7 stage-1 primes they are 0.238, 0.074 and -0.033 (B negative). The stage-1 rho fit on ln p (0.236 [0.120, 0.352]) is as stated",
        "RESULTS.md 'All arms are certified per log, with 0 bad relations': relation verification was off in the run (verify_relations false, check_walk false), so bad_relations = 0 is not a check; the logs themselves are certified (360/360 here)",
        "RESULTS.md 'group operations, linear-algebra multiplications and additions are priced on the band': additions (rel_add, la_add) are only in the w=1 sensitivity total_w1_with_additions, as PROTOCOL.md says",
        "PROTOCOL.md seeds '700 + s', '500 + s', '1000 + s per instance' differ from the code: 700 + s + 10k, 500 + s + 10k, 1000 + s + (seed mod 997); deterministic formulas, no sign of seed selection, but the text does not describe the run",
        "p = 102407 (third stage-2 prime of the protocol) is not in full_jacobian.json stage2_not_run, because the rerun dropped it from --stage2; the killed first run is not a command in the bench record (only RESULTS.md F8 and PROGRESS.md mention it)",
        "bench record raw files are uploaded: false (no S3 copy yet)",
    ],
}
os.makedirs(os.path.join(O, "gate"), exist_ok=True)
for n, d in (("replay-certificate.json", cert), ("validator-report.json", rep)):
    p = os.path.join(O, "gate", n)
    with open(p, "w") as f: json.dump(d, f, indent=1); f.write("\n")
    print(n, sha(p))
