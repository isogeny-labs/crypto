#!/usr/bin/env python3
"""VALIDATOR: assemble the validation artifacts of research/mt_timed_runpod_20261112 from the validator's outputs in a
scratch directory W (recompute.json, results_check.json, source_diff.json, payload_check*.json, replay_*.json,
ec_check.json, build.json) and the written judgement in scripts/findings.json and scripts/claim.json.
Writes into V = the validation directory: replay-certificate.json, validator-report.json, claim-report.json and the
evidence files recompute.json, results_check.json, source_diff.json, payload_check.json, payload_check_a1.json,
ec_check.json, replay_check.json.
usage: make_artifacts.py <W> <V>"""
import collections
import hashlib
import json
import os
import shutil
import sys

W, V = sys.argv[1:3]
S = os.path.normpath(os.path.join(V, ".."))
REPO = os.path.normpath(os.path.join(S, "../.."))
VALIDATED = "43fc3b0e61907dbc12b95ddd471678535311b011"
PROTOCOL = "fde73fb405"
A1 = "7934b7e343"
HEAD_SEEN = "51ca1eb6d68ec237152723982b0f913ff5c3095b"
WHO = "independent validator subagent (claude-opus-5-5), 2026-11-12; validator code in validation/scripts/, not the producer's"
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
L = lambda n: json.load(open(os.path.join(W, n)))

rc, rk, sd = L("recompute.json"), L("results_check.json"), L("source_diff.json")
pc, pa = L("payload_check.json"), L("payload_check_a1.json")
rfs, rff = L("replay_full_std.json"), L("replay_full_fast.json")
rds, rdf = L("replay_dump_std.json"), L("replay_dump_fast.json")
ec, build = L("ec_check.json"), L("build.json")
findings = json.load(open(os.path.join(V, "scripts/findings.json")))
claim = json.load(open(os.path.join(V, "scripts/claim.json")))
for n in ("recompute.json", "results_check.json", "source_diff.json", "payload_check.json", "payload_check_a1.json", "ec_check.json"):
    shutil.copyfile(os.path.join(W, n), os.path.join(V, n))
# compact replay record (all keys, both builds; dump sample)
rep = dict(schema="validator.replay-check/v1", nonce=rds["summary"]["nonce"], nonce_drawn=build["nonce_drawn"],
           full=dict(std=rfs["summary"], fast=rff["summary"]), dump_sample=dict(std=rds["summary"], fast=rdf["summary"]),
           fields_compared="every field the binary prints except ms_arm and ms (incl. ok, targets, I, M, S, H, L, N, phases and the arm-specific counters), against every RunPod repeat of the key",
           keys=dict(std=[[x["key"], x.get("match"), x.get("error")] for x in rfs["results"]],
                     fast=[[x["key"], x.get("match"), x.get("error")] for x in rff["results"]]),
           dump_keys=dict(std=[x["key"] for x in rds["results"]], fast=[x["key"] for x in rdf["results"]]))
json.dump(rep, open(os.path.join(V, "replay_check.json"), "w"), indent=0)

files = ["results/holdout/runs.json", "results/holdout/runs.jsonl", "results/holdout/opcost.jsonl", "results/holdout/host.json",
         "results/holdout/job.log", "results/holdout/DONE", "results/holdout/summary.json", "results/holdout/analysis.txt",
         "PROTOCOL.md", "RESULTS.md", "run.py", "analyze.py", "mk_runs.py", "launch.py", "pod/podjob.py", "plot.py",
         "launch/payload.json", "launch/create.json", "launch/upload.json", "launch/collect.json", "launch/delete.json",
         "attempts/a1/payload.json", "attempts/a1/create.json", "attempts/a1/upload.json", "attempts/a1/collect.json",
         "attempts/a1/delete.json", "attempts/a1/results/runs.jsonl", "attempts/a1/results/host.json",
         "attempts/a1/results/opcost.jsonl", "attempts/a1/results/DONE", "attempts/a1/results/job.log"]
artifact_set = {"research/mt_timed_runpod_20261112/" + f: sha(os.path.join(S, f)) for f in files}
artifact_set["examples/mt_fieldops.rs"] = sha(os.path.join(REPO, "examples/mt_fieldops.rs"))
artifact_set["Cargo.toml"] = sha(os.path.join(REPO, "Cargo.toml"))
artifact_set["Cargo.lock"] = sha(os.path.join(REPO, "Cargo.lock"))
collect = json.load(open(os.path.join(S, "launch/collect.json")))["files_sha256"]
collect_ok = all(collect[k] == sha(os.path.join(S, "results/holdout", os.path.basename(k))) for k in collect)
collect_a1 = json.load(open(os.path.join(S, "attempts/a1/collect.json")))["files_sha256"]
collect_a1_ok = all(collect_a1[k] == sha(os.path.join(S, "attempts/a1/results", os.path.basename(k))) for k in collect_a1)

# ---------------------------------------------------------------- replay certificate
ec_by = {(r["build"],) + tuple(r["key"]): (not r["failures"]) for r in ec["runs"]}
seqs = collections.defaultdict(list)
J = json.load(open(os.path.join(S, "results/holdout/runs.json")))
for r in J["rows"]:
    d = dict(zip(J["fields"], r))
    seqs[(d["build"], d["pb"], d["T"], d["idx"], d["seed"], d["arm"], d["param"], d["doff"])].append(d["seq"])
runs = []
for b, R in (("std", rfs), ("fast", rff)):
    for x in R["results"]:
        k = (b,) + tuple(x["key"])
        runs.append(["/".join(map(str, k)), sorted(seqs[k]), x.get("match") is True, ec_by.get(k)])
counters_ok = all(r[2] for r in runs) and len(runs) == 1224
cert = dict(
    schema="validator.replay-certificate/v1",
    status="verified" if (counters_ok and ec["summary"]["all_pass"] and build["std"]["bit_identical"] and build["fast"]["bit_identical"]) else "failed",
    independent=True, scalars_verified=bool(ec["summary"]["all_pass"]), validator=WHO,
    validated_commit=VALIDATED, protocol_commit=PROTOCOL + " (frozen); amendment A1 " + A1,
    working_tree="study directory, examples/mt_fieldops.rs, Cargo.toml, Cargo.lock, src/ and crates/ clean and identical to 43fc3b0e61 before and after the builds; the validator's own validation/ directory is the only untracked path",
    head_during_validation=f"HEAD moved from 43fc3b0e61 to {HEAD_SEEN[:10]} during validation (benchmarks/records/20261112T1629Z-mt-timed-runpod.json only; study, harness and instances unchanged); not validated",
    artifact_set=artifact_set,
    receipts=dict(collect_hashes_match_committed_results=collect_ok, a1_collect_hashes_match_committed=collect_a1_ok,
                  podjob_sha256_matches_create=pc["podjob_sha256_matches_create"]),
    binary=dict(build=build, rows_bin_sha256=rc["checks"]["bin_sha256_by_build"], host_json_bin_sha256_match_payload=rc["checks"]["host_bin_sha256_match_payload"],
                toolchain="rustc 1.97.0 / cargo 1.97.0"),
    payload=dict(pin=pc["pin"], candidate_is_pinned_payload=pc["candidate_is_pinned_payload"], members=pc["member_names"],
                 all_members_equal_validator_inputs=pc["all_members_equal_validator_inputs"], unexpected_members=pc["unexpected_members"],
                 rebuilt_bit_identical=pc["payload_rebuilt_bit_identical"], gzip_header_mtime_utc=pc["candidate_gzip_header"]["mtime_utc"],
                 a1_pin=pa["pin"], a1_rebuilt_bit_identical=pa["payload_rebuilt_bit_identical"], a1_gzip_mtime_utc=pa["window_search"]["found_utc"],
                 a1_run_py_is_protocol_commit_run_py=pa["a1_run_py_matches_pin"]),
    replay=dict(nonce=rep["nonce"], nonce_drawn=rep["nonce_drawn"], fields_compared=rep["fields_compared"],
                all_keys_rebuilt_binaries=dict(std=rfs["summary"], fast=rff["summary"]),
                dump_build_nonce_sample=dict(std=rds["summary"], fast=rdf["summary"]),
                counters_identical_between_builds_in_rows=rc["checks"]["configuration_keys_with_counters_differing_between_builds"] == 0,
                std_counters_equal_earlier_timed_study=rc["std_counters_vs_earlier_study"]),
    scalar_verification=dict(method="validator EC code (scripts/ec_check.py): instances (p, n prime, cofactor 1, G on curve, [n]G = O); for every dumped run every Q_t on the curve, recovered log == planted log, [log]G == Q_t",
                             summary=ec["summary"], rows_ok_eq_targets=rc["checks"]["ok_ne_targets"] == 0,
                             targets_in_rows=rc["checks"]["targets_total"], targets_verified_in_rows=rc["checks"]["targets_verified_total"]),
    attempt_1=dict(executions=3672, errors=3672, rows_with_timing=0, reason="taskset -c 2: Invalid argument (CPU 2 outside the container's cpuset)"),
    runs_fields=["build/pb/T/idx/seed/arm/param/doff", "seq of the 3 RunPod repeats", "counters replay exactly (rebuilt binary)", "recovered logs EC-verified (dump sample; null = not sampled)"],
    runs=runs)
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)

# ---------------------------------------------------------------- headline numbers
cs = {b: {(c["bits"], c["T"]): c for c in rc[b]["cells"]} for b in ("std", "fast")}
fs = {(r["bits"], r["T"]): r for r in rc["fast_over_std"]}
H = []


def h(name, claimed, rec, nd=2):
    tol = 0.5 * 10 ** (-nd) / abs(claimed) + 1e-12
    H.append(dict(name=name, claimed=claimed, recomputed=rec, rel_tol=tol))


cells = [(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)]
tab = {  # RESULTS.md table, as printed
    "std": [0.884, 0.733, 0.700, 0.791, 0.833, 0.953, 1.053, 1.170, 1.179],
    "fast": [0.893, 0.776, 0.712, 0.739, 0.779, 0.944, 1.011, 1.108, 1.148],
    "ic_fs": [0.892, 0.890, 0.900, 0.918, 0.926, 0.946, 0.963, 0.938, 0.949],
    "rho_fs": [0.878, 0.845, 0.886, 0.981, 0.989, 0.961, 1.003, 0.986, 0.975],
    "bs_std": [1.52, 1.47, 1.46, 1.44, 1.48, 1.54, 1.52, 1.60, 1.58],
    "bs_fast": [1.53, 1.46, 1.41, 1.46, 1.50, 1.62, 1.60, 1.62, 1.60]}
for i, c in enumerate(cells):
    t = f"{c[0]}/{c[1]}"
    h(f"std ic/rho {t}", tab["std"][i], cs["std"][c]["ic_rho_solve"]["median"], 3)
    h(f"fast ic/rho {t}", tab["fast"][i], cs["fast"][c]["ic_rho_solve"]["median"], 3)
    h(f"ic fast/std {t}", tab["ic_fs"][i], fs[c]["ic"]["median"], 3)
    h(f"rho fast/std {t}", tab["rho_fs"][i], fs[c]["rho"]["median"], 3)
    h(f"std ic/bsgs {t}", tab["bs_std"][i], cs["std"][c]["ic_bsgs_solve"]["median"], 2)
    h(f"fast ic/bsgs {t}", tab["bs_fast"][i], cs["fast"][c]["ic_bsgs_solve"]["median"], 2)
for b in ("std", "fast"):
    for pb in (24, 28, 32):
        cr = rc[b]["size"][str(pb)]["cell_median_range"]
        H.append(dict(name=f"{b} {pb}b configurations below 1", claimed={"std": {24: 12, 28: 12, 32: 0}, "fast": {24: 12, 28: 12, 32: 1}}[b][pb],
                      recomputed=rc[b]["size"][str(pb)]["below_1"], rel_tol=1e-9))
head_ranges = {("std", 24): (0.70, 0.88), ("std", 28): (0.79, 0.95), ("std", 32): (1.05, 1.18),
               ("fast", 24): (0.71, 0.89), ("fast", 28): (0.74, 0.94), ("fast", 32): (1.01, 1.15)}
for (b, pb), (lo, hi) in head_ranges.items():
    cr = rc[b]["size"][str(pb)]["cell_median_range"]
    h(f"{b} {pb}b cell-median ic/rho min", lo, cr[0])
    h(f"{b} {pb}b cell-median ic/rho max", hi, cr[1])
icfs = [r["ic"]["median"] for r in rc["fast_over_std"]]
rhofs = [r["rho"]["median"] for r in rc["fast_over_std"]]
h("IC fast/std cell-median min", 0.89, min(icfs)); h("IC fast/std cell-median max", 0.96, max(icfs))
h("rho fast/std cell-median min", 0.85, min(rhofs)); h("rho fast/std cell-median max", 1.00, max(rhofs))
bsall = [c["ic_bsgs_solve"]["median"] for b in ("std", "fast") for c in rc[b]["cells"]]
h("IC/BSGS cell-median min (both builds)", 1.41, min(bsall)); h("IC/BSGS cell-median max (both builds)", 1.62, max(bsall))
h("round-0 drift (median key time rel. to key median)", 1.032, rc["pooled_drift"]["median_rel_by_round"][0], 3)
h("A/A median |log dev| (all keys)", 0.008, rc["pooled_drift"]["median_abs_log_dev"], 3)
h("A/A p95 |log dev|", 0.11, rc["pooled_drift"]["p95_abs_log_dev"], 2)
op = rc["opcost_summary"]
h("GAE ns min", 98, op["gae_ns"][0], 0); h("GAE ns max", 140, op["gae_ns"][1], 0)
h("I/M min", 23, op["I_over_M"][0], 0); h("I/M max", 27, op["I_over_M"][1], 0)
h("std lookup GAE 2^22-2^24 min", 0.74, op["std_lookup_gae_2_22_to_2_24"][0]); h("std lookup GAE 2^22-2^24 max", 1.45, op["std_lookup_gae_2_22_to_2_24"][1])
h("std lookup GAE 2^10-2^16 min", 0.06, op["std_lookup_gae_2_10_to_2_16"][0]); h("std lookup GAE 2^10-2^16 max", 0.11, op["std_lookup_gae_2_10_to_2_16"][1])
mism = [x for x in H if abs(x["claimed"] - x["recomputed"]) > x["rel_tol"] * abs(x["claimed"]) + 1e-12]

decisions_match = all(c["ok"] for c in rk["comparisons"] if c["name"].endswith("decision"))
rep_v = dict(
    schema="validator.report/v1", status="pass" if not mism and cert["status"] == "verified" else "fail", independent=True,
    verdict=("headline validated: every number of the RESULTS.md headline and table recomputes from runs.json (388/388 summary.json fields exact); "
             "both binaries rebuild bit-identical from 43fc3b0e61 in fresh target dirs; all 1224 keys (3672 rows) replay exactly; "
             "301,056 recovered logs of 42 nonce-sampled runs (both builds, all arms, all sizes) verified with the validator's EC code; "
             "the pinned payload rebuilds bit for bit. RESULTS.md needs the non-blocking amendments listed (R-F3..R-F10); "
             "the separate claim statement is not endorsed as worded (claim-report.json, R-F1, R-F2)"),
    disposition="validated", validator=WHO, validated_commit=VALIDATED, protocol_commit=PROTOCOL, amendment_commit=A1,
    claim_scope=("solve-phase wall time (ms_arm) on one pinned CPU (91) of a RunPod cpu5c pod, two builds (std SipHash; mt-fasthash); "
                 "icbest = fastest of ic5, ic5ks, ic5w, ic5rw; rhobest = fastest of rho2xks x 4 kappa and rhodpu x 6; bsgsbest = fastest of bsgs2 x 3; "
                 "key time = median of 3 interleaved repeats; cell = 4 configurations; decision faster/slower if the protocol bootstrap 95% interval "
                 "of the cell median is on one side of 1 (with n = 4 the interval is the range: a 4-of-4 sign rule)"),
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(os.path.join(S, "results/holdout/runs.json"))),
              dict(path="../results/holdout/opcost.jsonl", sha256=sha(os.path.join(S, "results/holdout/opcost.jsonl")))],
    supporting_data=[dict(path="../results/holdout/runs.jsonl", sha256=sha(os.path.join(S, "results/holdout/runs.jsonl")), note="full rows; replayed; in Git, not the artifact store (R-F9)"),
                     dict(path="../results/holdout/host.json", sha256=sha(os.path.join(S, "results/holdout/host.json"))),
                     dict(path="../../mt_timed_20261112/results/holdout/runs.json", sha256=sha(os.path.join(S, "../mt_timed_20261112/results/holdout/runs.json")), note="earlier timed study (session VM), for the host-replication statements and the counter comparison")]
                    + [dict(path=n, sha256=sha(os.path.join(V, n))) for n in ("recompute.json", "results_check.json", "source_diff.json", "payload_check.json", "payload_check_a1.json", "ec_check.json", "replay_check.json", "replay-certificate.json")],
    checks=dict(design=rc["checks"], summary_json_fields_compared=rk["compared"], summary_json_mismatches=len(rk["mismatches"]),
                analysis_regenerated_identical="producer's analyze.py and mk_runs.py rerun on the committed data reproduce summary.json, analysis.txt and runs.json byte for byte",
                decisions_match=decisions_match, source_diff=dict((k, v) for k, v in sd.items() if k != "alias_block"),
                build=build, receipts=cert["receipts"], payload=cert["payload"],
                amendment_A1=dict(commit_time="2026-11-12T16:32:29Z", a2_pod_created="2026-11-12T16:32:34Z", a2_job_started="2026-11-12T16:33:08Z",
                                  a1_rows_with_timing=0, stated_time="16:40Z (wrong, R-F8)", legitimate=True,
                                  change="run.py pins to the middle CPU of sched_getaffinity (PIN=auto); host.json records allowed_cpus; launcher user agent; no workload, analysis or rule change")),
    recompute=dict(method="scripts/recompute.py (validator's own code) from runs.json/runs.jsonl; protocol bootstrap and the exact bootstrap over all 4^4 resamples: identical intervals (the range) and decisions in all 18 build-cells",
                   std=rc["std"]["cells"], fast=rc["fast"]["cells"], size=dict(std=rc["std"]["size"], fast=rc["fast"]["size"]),
                   fast_over_std=rc["fast_over_std"], drift=dict(std=rc["std"]["drift"], fast=rc["fast"]["drift"], pooled=rc["pooled_drift"]),
                   opcost=rc["opcost_summary"], earlier_vm_cells=rc["earlier_vm_cells"], extra=rk["extra"]),
    reporting_contract=dict(one_table_one_unit=True, ratio_reported=True, phases_separated="solve (ms_arm) and solve + shared verification (ms) both computed; same decisions in all 18 build-cells",
                            reference_arm_priced="wall time: every arm's work is inside ms_arm; the counted section 5a band is v5's (not re-run here)",
                            null_A_A="present; not clean (round 0 about 3% slower), disclosed", cheaper_baseline="bsgs2 reported (IC 1.41-1.62x by cell median)",
                            frozen_benchmark="v5 instances (sha256 in ec_check.json), seed 43 of the earlier timed study",
                            external_fastest_implementation="not compared (harness-internal study)",
                            m83_ecc2k130="not mentioned beyond 'nothing here is about ECC2K-130 / m = 83'", interpretation_labelled=True,
                            isolation="shared cloud host, diagnostic seconds (not L2); stated"),
    headline=H, headline_mismatches=mism, findings=findings,
    required_amendments=[f"{f['id']} ({f['severity']}): {f['amendment']}" for f in findings if f["amendment"] != "none"],
    not_done=["no timing reproduction: the RunPod host cannot be re-run from here (instructed not to use RunPod); wall times are judged by internal consistency (R-F12)",
              "the counted section 5a GAE band was not recomputed (wall-time study; the band belongs to the v5 counted study)"])
json.dump(rep_v, open(os.path.join(V, "validator-report.json"), "w"), indent=1, default=str)

# ---------------------------------------------------------------- claim report
st, fa = rc["std"], rc["fast"]
cr0 = lambda b, pb: rc[b]["size"][str(pb)]
fr = lambda r: "%.4f-%.4f" % (r[0], r[1])
class _CR(dict):
    def __getitem__(self, k):
        v = dict.__getitem__(self, k)
        return fr(v) if k in ("cell_median_range", "config_range") else v
cr = lambda b, pb: _CR(cr0(b, pb))
clauses = [
    ["On one pinned CPU of a RunPod Secure Cloud cpu5c pod (AMD EPYC 9575F; a shared cloud host, so the seconds are diagnostic)", True,
     "create.json cloudType SECURE, cpuFlavorId cpu5c; host.json 'AMD EPYC 9575F 64-Core Processor', pin 91 within cpuset 27,56,91,120; not L2"],
    ["36 configurations (the 12 prime-order Weierstrass curves of the v5 study, group orders about 2^24, 2^28 and 2^32, seed 43, T = 1024/4096/16384) x 17 run keys x 3 interleaved repeats for each of two builds", True,
     "3672 rows = 2 x 36 x 17 x 3 in the pre-registered shuffled order; instances prime order, cofactor 1 (ec_check.json)"],
    ["(std HashMap with SipHash; a fast multiplicative hasher, with identical counted operations)", True,
     "source diff = alias/feature only; binaries rebuilt bit-identical; 1224/1224 keys replay exactly; counters equal across builds in all 612 configuration keys"],
    ["every target verified", True, f"ok = targets in all 3672 rows ({rc['checks']['targets_total']} targets); {ec['summary']['targets_checked']} recovered logs re-verified with validator EC code, 0 failures"],
    ["takes 0.70-0.88x (std) and 0.71-0.89x (fast) ... at 2^24 and 0.79-0.95x and 0.74-0.94x at 2^28", False,
     f"numbers are cell medians ({cr('std',24)['cell_median_range']}, {cr('fast',24)['cell_median_range']}, {cr('std',28)['cell_median_range']}, {cr('fast',28)['cell_median_range']}); the statement does not say so and per configuration they are {cr('std',24)['config_range']}, {cr('fast',24)['config_range']}, {cr('std',28)['config_range']}, {cr('fast',28)['config_range']} (R-F1)"],
    ["(faster in 12 of 12 configurations at each size and build)", True, "12/12 below 1 at 2^24 and 2^28 in both builds; all 6 cells faster"],
    ["1.05-1.18x (std; 12 of 12 configurations above 1) and 1.01-1.15x (fast; 11 of 12 above 1) at 2^32", False,
     f"cell medians {cr('std',32)['cell_median_range']} / {cr('fast',32)['cell_median_range']}, counts 12/12 and 11/12 correct; the fast build's T = 1024 cell is unranked (1.011 [0.996, 1.031]) and is not mentioned (R-F2); per configuration {cr('std',32)['config_range']} / {cr('fast',32)['config_range']}"],
    ["the fast hasher speeds the fastest IC by 4-11% and the fastest rho by 0-15%", "conditional",
     f"cell medians of fast/std: IC {min(icfs):.4f}-{max(icfs):.4f} (3.7-11.0% less time), rho {min(rhofs):.4f}-{max(rhofs):.4f} (-0.3% to 15.5%); per configuration IC 0.82-0.98, rho 0.81-1.07: correct only as cell medians (R-F1)"],
    ["the IC takes 1.41-1.62x the time of a paired BSGS", "conditional", f"cell medians {min(bsall):.4f}-{max(bsall):.4f}; per configuration 1.31-1.67 (R-F1)"],
    ["the A/A control drifted (the first of three interleaved rounds about 3% slower), disclosed", True,
     f"median key time relative to its key median by round {[round(x, 4) for x in rc['pooled_drift']['median_rel_by_round']]}; A/A interval contains 1 in 4/9 (std) and 3/9 (fast) cells"]]
cl = dict(
    schema="validator.report/v1", independent=True, validator=WHO, validated_commit=VALIDATED,
    scope="this one statement exactly as worded; nothing else in the study",
    statement_judged=claim["statement_judged"], statement_sha256=hashlib.sha256(claim["statement_judged"].encode()).hexdigest(),
    status="fail", disposition="insufficient_evidence",
    verdict=("not endorsed as worded: every number recomputes, but the ranges are cell medians presented beside per-configuration counts "
             "without saying so (per configuration 0.70-0.98 at 2^24 in the std build, not 0.70-0.88), the fast build's unranked 2^32 T = 1024 cell "
             "is omitted, and the 4-of-4 sign-rule disclosure of the validated predecessor is dropped. The proposed wording is endorsed."),
    proposed_wording=claim["proposed_wording"], proposed_wording_sha256=hashlib.sha256(claim["proposed_wording"].encode()).hexdigest(),
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(os.path.join(S, "results/holdout/runs.json"))),
              dict(path="../results/holdout/opcost.jsonl", sha256=sha(os.path.join(S, "results/holdout/opcost.jsonl")))],
    supporting=dict(replay_certificate="replay-certificate.json (sha256 %s)" % sha(os.path.join(V, "replay-certificate.json")),
                    validator_report="validator-report.json (sha256 %s)" % sha(os.path.join(V, "validator-report.json")),
                    recompute="recompute.json", results_check="results_check.json"),
    clause_checks=[dict(clause=c, ok=o, recomputed=r) for c, o, r in clauses],
    headline=H, headline_mismatches=mism,
    notes=["Endorsed wording = proposed_wording byte for byte; any change to it needs a new judgement.",
           "RESULTS.md's own headline and table are validated (validator-report.json, status pass); this report judges only the statement."])
json.dump(cl, open(os.path.join(V, "claim-report.json"), "w"), indent=1)
print("certificate", cert["status"], "report", rep_v["status"], "claim", cl["status"], "headline", len(H), "mismatches", len(mism))
for m in mism:
    print("  ", m)
