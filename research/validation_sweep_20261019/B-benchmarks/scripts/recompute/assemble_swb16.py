import json, os
BASE = "research/validation_sweep_20261019/B-benchmarks/"
OUT = BASE + "recompute/SWB-RECOMPUTE-16.jsonl"
SC = BASE + "scripts/recompute/"
GIT = "ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00"
NOID = {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"}
L = "research/lit2026_w5_lane_batched_f4_20261011/"
W12 = "research/lit2026_w12_flagged_claims_20261011/"
GLV = "research/isogeny_loop_glv_20261012/"
HASH = "research/f4_f5_hash_20261012/"
PDP = "research/pdp_split_dreg_20261011/"
P61 = "research/p61_isogeny_class_20261014/"
HIST = "research/f4_f5_history_20261012/"

def stop(row_no, slug, why):
    return {"row_no": row_no, "slug": slug, "claims": [],
            "identical": dict(NOID), "offrepo_refs": [], "aa_lines": [], "isolation_lines": [],
            "protocol_git_date": "", "first_raw_git_date": "", "script": "",
            "notes": "STOP (card STOP_WHEN): " + why + ". The card lists no STUDY DIRS and empty MEASURED/HEADLINE for this row, so no computation was made; the card's rule is to report the stop and continue."}

rows = []
rows.append({"row_no": 1001, "slug": "20261014-p61-class-pilot",
  "claims": [],
  "identical": dict(NOID),
  "offrepo_refs": [
    P61 + "README.md:34: Durable experiment data are stored on the active UGREEN SMB export below",
    P61 + "README.md:35: `/Volumes/data/ec-hardness-atlas-20261012/`. The requested alias",
    P61 + "README.md:36: `/Volumes/UgreenNAS` is not mounted on this host.",
    P61 + "README.md:39: requires the SMB mount, a free-space floor, full source identities, the exact",
    P61 + "reports/run-20261014T0902Z/test-results.json:4: \"target_directory\": \"/Volumes/SSD990-2/crypto/target-p61-d064-20261014\"",
    P61 + "reports/run-20261014T0902Z/certified-prefix-x000000000000-x000000016704-manifest.json:36: \"path\": \"/Volumes/data/ec-hardness-atlas-20261012/p61-class-streaming-v1/checkpoints/certified-v2-checkpoint-000000016704.json\""],
  "aa_lines": [],
  "isolation_lines": [
    "benchmarks/20261014-p61-class-pilot/README.md:6: Measured on a shared, unisolated macOS 26.6 arm64 host with 8 producer threads:",
    P61 + "reports/run-20261014T0902Z/timings.json: \"isolation\": \"unisolated; other research and desktop processes were active\""],
  "protocol_git_date": "", "first_raw_git_date": GIT + " (" + P61 + "reports/run-20261014T0902Z/timings.json)",
  "script": SC + "1001.py",
  "notes": "HEADLINE is '`p61-i0025` selected-class launch study.' and contains no numeric claim, so zero claims. The timings.json segment values are in the MEASURED body text, which the card's HEADLINE rule does not select; the script prints them for reference only. No PROTOCOL file in the study directory, so protocol_git_date is empty. timings.json, README.md and the other committed study files share git add-commit ca00c151 (2026-10-17). The row's raw file list has no per-row protocol path. Row 1001 uses no store: reference."})

rows.append({"row_no": 1002, "slug": "20261012-lit2026-w5-gate-first-rref",
  "claims": [
    {"claimed": "1.64 (original/candidate word-XOR ratio, first value)", "computed": "1.6417 (K1_17 f6p: 12635914 / 7696852)", "definition": "orig word_xors / gaterref word_xors, f6p cell K1_17, run-20261012T0008Z", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"},
    {"claimed": "1.93 (word-XOR ratio, second value)", "computed": "1.9299 (K1_23 f6p: 51918433 / 26902588)", "definition": "orig word_xors / gaterref word_xors, f6p cell K1_23", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"},
    {"claimed": "2.19 (word-XOR ratio, third value)", "computed": "2.1944 (K0_23 f6p: 124949801 / 56940731)", "definition": "orig word_xors / gaterref word_xors, f6p cell K0_23", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"},
    {"claimed": "2.41 (word-XOR ratio, fourth value)", "computed": "2.4138 (K0_31 f6p: 278237634 / 115269611)", "definition": "orig word_xors / gaterref word_xors, f6p cell K0_31", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"},
    {"claimed": "2.85 (process-CPU previous/candidate, low end)", "computed": "2.8543 (K1_17 f6p)", "definition": "median over 5 paired rounds of cpu_ms prev / cpu_ms gaterref (study table: 'CPU previous / gaterref, median [min-max]'); range min over f6p cells", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"},
    {"claimed": "3.69 (process-CPU previous/candidate, high end)", "computed": "3.6936 (K0_31 f6p)", "definition": "same as above, max over f6p cells", "files": [L + "reports/run-20261012T0008Z/summary.json"], "status": "match"}],
  "identical": dict(NOID),
  "offrepo_refs": [
    L + "README.md:113: export CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/targets/lit2026-w5",
    L + "README.md:115: # original arm: f6_ic_probe built at 4b6e09b78 in /Volumes/SSD990-2/crypto/wt/lit2026-baseline"],
  "aa_lines": [L + "README.md:257: L0 only. The original/previous null control in run 1 ranges 0.24–4.60 per round, so"],
  "isolation_lines": [L + "README.md:132: Isolation L0, unpinned. A test build of the library ran during part of run 1 and the",
    L + "README.md:257: L0 only. The original/previous null control in run 1 ranges 0.24–4.60 per round, so",
    L + "README.md:288: - L0 only; wall time admits no claim; CPU time is L0 too."],
  "protocol_git_date": "", "first_raw_git_date": GIT + " (" + L + "reports/run-20261012T0008Z/summary.json)",
  "script": SC + "1002.py",
  "notes": "Computed from run-20261012T0008Z/summary.json (aggregated by the study from raw/*.jsonl; per-target raw jsonl not parsed). The word-XOR ratios use per-arm word_xors, which the summary marks word_xors_deterministic. The CPU claim uses the study's stated table rule (median of paired per-round ratios); the ratio of arm medians instead gives 2.85 (K1_17), 3.42 (K1_23), 3.44 (K0_23), 3.79 (K0_31), so the upper bound differs by rule (3.69 vs 3.79). Stored summary cpu_round_ratios medians equal the recomputed values. No PROTOCOL file in study. Headline has no identical-output claim, so the identical check was not run."})

rows.append({"row_no": 1003, "slug": "20261012-lit2026-w12-zero-minor",
  "claims": [
    {"claimed": "260 (zero-minor/rho counted field-operation cost, low end)", "computed": "candidate A (mean of 4 rho seeds per target): 260.371 (equals stored); candidate B (median of 4 rho seeds per target): 273.698", "definition": "geomean over 8 targets of zero-minor ops / rho ops at 16 bits (paired_ratio in scaling.json); per-target rho reduction not stated", "files": [W12 + "reports/run-20261012T0009Z/scaling.json"], "status": "cannot_define"},
    {"claimed": "6715 (zero-minor/rho counted field-operation cost, high end)", "computed": "candidate A (mean): 6715.008 (equals stored); candidate B (median): 7547.226", "definition": "geomean over 8 targets at 26 bits; per-target rho reduction not stated", "files": [W12 + "reports/run-20261012T0009Z/scaling.json"], "status": "cannot_define"},
    {"claimed": "search-phase log-cost slope 0.905", "computed": "0.904887 (OLS of log2 search_ops on log2 r, n=47 zero-minor runs)", "definition": "OLS slope over zero-minor runs with stored search_ops; equals fits.json value", "files": [W12 + "reports/run-20261012T0009Z/scaling.json", W12 + "reports/run-20261012T0009Z/fits.json"], "status": "match"},
    {"claimed": "[0.769, 1.021] (95% bootstrap interval of the slope)", "computed": "not reproduced; stored bootstrap 0.769494-1.02052 (fits.json); OLS t-interval 0.745655-1.064119 for reference", "definition": "95% bootstrap over runs within curves; resample count and RNG seed not stated in the committed study", "files": [W12 + "reports/run-20261012T0009Z/fits.json"], "status": "cannot_define"}],
  "identical": dict(NOID),
  "offrepo_refs": [
    W12 + "README.md:243: export CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/targets/lit2026-w12",
    W12 + "README.md:244: export TMPDIR=/Volumes/SSD990-2/crypto/targets/lit2026-w12/tmp   # system disk was full",
    W12 + "README.md:251: lit2026_w12_zero_minor run ... --out <scratch>/replay.json"],
  "aa_lines": [],
  "isolation_lines": [W12 + "README.md:156: ## Results (run-20261012T0009Z, L0, counted field ops; full table in the run report)",
    W12 + "README.md:235: - **Wall times are L0** on a loaded shared host (load 79–106). They are reported only",
    W12 + "README.md:262: **Host.** Shared Apple M4 Pro, macOS (Darwin 25.6.0), isolation L0."],
  "protocol_git_date": "", "first_raw_git_date": GIT + " (" + W12 + "reports/run-20261012T0009Z/scaling.json)",
  "script": SC + "1003.py",
  "notes": "scaling.json has 240 run records (192 rho.negation, 48 zero_minor, of which 47 have search_ops); ops = muls + invs as stored. Stored OLS slopes for all-phase field ops (0.783045), d=3 runs (0.631113), and rho (0.377192) recompute exactly. Paired-ratio rule: the study states a geomean over targets (write_journey.py:36) and does not state how the 4 rho seeds per target are reduced; the arithmetic mean reproduces all six stored values, the median does not. replay-compare.txt reads 'rows a=240 b=240 identical=240 different=0' (not in the headline, identical check not run under step 5). No PROTOCOL file in study."})

rows.append({"row_no": 1004, "slug": "20261012-glv-gls-4d-confirmation",
  "claims": [
    {"claimed": "105,273 ns (projective GLV-GLS-4 median)", "computed": "105272.945 ns (median of 30 rounds_ns = stored median_ns)", "definition": "median ns, arm 'glvgls4 (projective loop)', bench_run2.json", "files": [GLV + "bench_run2.json"], "status": "match"},
    {"claimed": "114,823 ns (GLS-2 median)", "computed": "114822.84 ns", "definition": "median ns, arm 'gls2', bench_run2.json", "files": [GLV + "bench_run2.json"], "status": "match"},
    {"claimed": "1.091x (GLS-2 / projective GLV-GLS-4)", "computed": "1.0907 (114822.84 / 105272.945)", "definition": "ratio of arm medians", "files": [GLV + "bench_run2.json"], "status": "match"},
    {"claimed": "A/A 0.989", "computed": "0.9887 (211894.85 / 214311.96)", "definition": "wnaf median / wnaf (A/A) median, bench_run2.json", "files": [GLV + "bench_run2.json"], "status": "match"}],
  "identical": dict(NOID),
  "offrepo_refs": [],
  "aa_lines": [GLV + "README.md:24: | benchmark at D = −619 | **measured, two runs on two hosts.** ... The A/A control reads 0.987.",
    GLV + "README.md:122: | wNAF again (A/A) | 214,312 | 205,225 | 0.989 | — |",
    GLV + "README.md:127: (0.056) is larger than either run's A/A deviation, and its cause is not",
    GLV + "README.md:148: - GLV-GLS-4 was faster than GLS-2 at every D tested. The A/A control ranges"],
  "isolation_lines": [GLV + "README.md:95: - `taskset -c 3`, host Intel Xeon 2.10 GHz (cloud container, not isolated).",
    GLV + "README.md:128: isolated: host, load, and decomposition time (2.9 vs 5.3 µs) all changed.",
    "benchmarks/20261012-glv-gls-4d-confirmation/README.md:7: Host and isolation: Different Intel Xeon @2.80 GHz, 4 CPUs; pinned core 3; load 1.23→1.21 with another build starting; not isolated.",
    "benchmarks/20261012-glv-gls-4d-confirmation/README.md:9: Absolute timing comparisons stay within each host; between-run cause is not isolated."],
  "protocol_git_date": "", "first_raw_git_date": GIT + " (" + GLV + "bench_run2.json)",
  "script": SC + "1004.py",
  "notes": "bench_run2.json is the confirmation run (README: 'Confirmation run 2 (bench_run2.*, 2026-10-12T02:16Z)'). 30 rounds, 256 pairs, correctness_all_pass true. Stored median_ns values equal the medians of rounds_ns. No off-repo references found in the study files. No PROTOCOL file in study directory."})

rows.append({"row_no": 1005, "slug": "20261012-f4-fp-vs-msolve"}.copy() and stop(1005, "20261012-f4-fp-vs-msolve", "STUDY DIRS is empty for this row"))
rows.append({"row_no": 1006, "slug": "20261012-f4-f5-performance-history",
  "claims": [
    {"claimed": "Seven frozen perf-history snapshots", "computed": "7 (count of files in records/history/)", "definition": "count of committed history snapshot JSON files", "files": [HIST + "records/history/" + x for x in sorted(os.listdir(HIST + "records/history"))], "status": "match"}],
  "identical": dict(NOID),
  "offrepo_refs": [],
  "aa_lines": [],
  "isolation_lines": ["benchmarks/20261012-f4-f5-performance-history/README.md:7: Host and isolation: Historical hosts/compilers/profilers differ; connections break at changed fingerprints; worker-thread coverage incomplete.",
    "benchmarks/20261012-f4-f5-performance-history/README.md:9: History rendering quotes existing records; runtime has no stored spread or isolation eligibility."],
  "protocol_git_date": "", "first_raw_git_date": GIT + " (" + HIST + "records/history/20261008T150820Z-93ff0c72333c.json)",
  "script": SC + "1006.py",
  "notes": "REPORT.md:14 says seven existing main-revision snapshots were imported, matching the 7 files. Test-name matches for 'scratch'/'store' in records/library-expected-tests.txt (e.g. line 1216 'from_scratch', line 1280 'million_store') are test identifiers, not off-repo references. No PROTOCOL file in study directory."})

rows.append(stop(1008 - 0 if False else 1008, "20261012-f4-f5-fused-readback", "STUDY DIRS is empty for this row"))
rows.append(stop(1009, "20261012-f4-f5-dense-packing", "STUDY DIRS is empty for this row"))

rows.append({"row_no": 1007, "slug": "20261012-f4-f5-native-hashing",
  "claims": [
    {"claimed": "Five paired A/B/A' rounds", "computed": "5 (paired_rounds; 5 baseline_ir and 5 candidate_ir per arm)", "definition": "count of paired process rounds in comparison.json", "files": [HASH + "records/comparison.json"], "status": "match"},
    {"claimed": "eleven frozen Boolean systems", "computed": "11 (systems)", "definition": "count of systems in comparison.json", "files": [HASH + "records/comparison.json"], "status": "match"},
    {"claimed": "3.715% fewer complete-F5 instruction references", "computed": "3.7145% (1 - median candidate IR / median baseline IR = 92406706 / 95971529); stored 3.7149%", "definition": "reduction of median over 5 rounds per arm", "files": [HASH + "records/comparison.json"], "status": "match"},
    {"claimed": "95% interval 3.714-3.726%", "computed": "not recomputed; stored 3.714413 to 3.726211", "definition": "95% bootstrap, 2000 resamples, bootstrap_seed 0x243f6a8885a308d3; resampling procedure not in committed scripts", "files": [HASH + "records/comparison.json"], "status": "cannot_define"},
    {"claimed": "45.136% fewer standalone criterion instruction references", "computed": "45.136% (1 - 3362140 / 6128134); stored 45.13599%", "definition": "reduction of median over 5 rounds per arm", "files": [HASH + "records/comparison.json"], "status": "match"},
    {"claimed": "45.121-45.155%", "computed": "not recomputed; stored 45.121134 to 45.155403", "definition": "95% bootstrap as above", "files": [HASH + "records/comparison.json"], "status": "cannot_define"},
    {"claimed": "F4 is neutral under the 2% gate", "computed": "verdict field 'neutral'; |median_log_ratio| 0.000352 < log_threshold 0.019803 (equal to ln 1.02)", "definition": "study gate applied to the stored log ratio", "files": [HASH + "records/comparison.json"], "status": "match"}],
  "identical": {"claimed": True, "n_compared": 627, "n_equal": 627, "n_differ": 0,
    "files": [HASH + "records/primary-a/records.jsonl", HASH + "records/primary-b/records.jsonl"], "status": "match"},
  "offrepo_refs": [
    HASH + "records/baseline-sha256.txt:1: 78fd88fdf422c899bf44ffe7aa10a2b3babe8793dc5619a5c16ebd163b8c0928  /workspace/scratch/f4-f5-hash/a/bench",
    HASH + "records/primary-b/command.txt:1: VALGRIND_LIB=/workspace/scratch/f4-f5-hash/valgrind/usr/libexec/valgrind timeout -k 5 180 /workspace/scratch/f4-f5-hash/valgrind/usr/bin/valgrind --tool=callgrind ... /workspace/scratch/f4-f5-hash/b/bench profile 3"],
  "aa_lines": [HASH + "REPORT.md:11: These are Callgrind instruction counts from an instrumented native driver, including allocator accounting, with merged worker coverage. They are not elapsed-time measurements or hardware retired-instruction counters. Primary profiles contain three calls per system and arm; verdicts use five paired A/B/A′ process rounds, each covering one call on every system."],
  "isolation_lines": [HASH + "REPORT.md:11: These are Callgrind instruction counts ... They are not elapsed-time measurements or hardware retired-instruction counters. (no isolation tier stated in REPORT.md or benchmarks/20261012-f4-f5-native-hashing/README.md)"],
  "protocol_git_date": GIT + " (" + HASH + "PROTOCOL.md)", "first_raw_git_date": GIT + " (" + HASH + "records/primary-a/records.jsonl)",
  "script": SC + "1007.py",
  "notes": "Identity check: primary-a (baseline, a/bench) and primary-b (candidate, b/bench) each have 99 records (3 arms x 11 systems x 3 repeats), joined on (arm, case, seed, repeat): all 99 keys common. Compared fields: output_digest, criterion_digest, input_digest, rank, word_ops, and report counters (reduce_word_ops, criterion_word_ops, rank, rows_pruned, criterion_rows, cols, rows_built). primary-a has no command.txt; the baseline binary hash is in records/baseline-sha256.txt. Stored intervals (bootstrap_seed 0x243f6a8885a308d3, 2000 resamples) were not reproduced. The F5 headline percentage recomputes to 3.7145 from arm medians vs stored 3.7149 (relative difference 0.01%)."})

rows.append(stop(1005, "20261012-f4-fp-vs-msolve", "STUDY DIRS is empty for this row") if False else stop(1005, "20261012-f4-fp-vs-msolve", "STUDY DIRS is empty for this row"))

rows.append({"row_no": 1010, "slug": "20261011-pdp-split-hybrid",
  "claims": [
    {"claimed": "192-run grid", "computed": "192 (24 'run' records in each of 8 files in runs-v2/)", "definition": "count of kind=='run' records in runs-v2/*.jsonl", "files": [PDP + "runs-v2/" + f for f in ["3-0-13-4.jsonl","3-0-13-5.jsonl","3-1-11-4.jsonl","3-1-17-4.jsonl","3-1-17-5.jsonl","3-1-19-4.jsonl","4-0-13-3.jsonl","4-1-11-3.jsonl"]], "status": "match"},
    {"claimed": "grid exact (192-run grid exact)", "computed": "192 of 192 run records have complete_and_exact true", "definition": "count of complete_and_exact == true among runs-v2 run records", "files": [PDP + "runs-v2/"], "status": "match"},
    {"claimed": "22.8x strong rho (closest relation-phase projection)", "computed": "22.751 (cell 3-1-11-4, icv1-f2m11-t67-05f5aa36, unsplit g=4: projected S_relations 956.08 / strong rho mean_s 42.0239)", "definition": "minimum over all cells and variants with a projection of projection.s_relations / rho_strong.mean_s", "files": [PDP + "ANALYSIS.json"], "status": "match"},
    {"claimed": "391x signed-Frobenius rho", "computed": "390.625 (956.08 / 2.4476), same cell and variant", "definition": "minimum over all cells and variants of projection.s_relations / rho_signed_frobenius mean_s", "files": [PDP + "ANALYSIS.json"], "status": "match"}],
  "identical": dict(NOID),
  "offrepo_refs": [
    PDP + "PROTOCOL.md:158: Smoke runs on seed 1, kept in the scratchpad and not committed:",
    PDP + "AMENDMENT_1.md:24: The diagnostic is the scratch reproduction in the PR description, and the",
    PDP + "cairn/README.md:34: ran the commands unjailed. The accepted runs used a scratch node root that"],
  "aa_lines": [],
  "isolation_lines": [PDP + "README.md:48: L2 on most runs.",
    PDP + "README.md:156: - **Wall time is descriptive.** The PDP runs had no isolation (cloud",
    "benchmarks/20261011-pdp-split-hybrid/README.md:7: Host and isolation: PDP shared cloud container,no isolation; matchedrho L2 on most records; projection uses calibration from another host."],
  "protocol_git_date": GIT + " (" + PDP + "PROTOCOL.md)", "first_raw_git_date": GIT + " (" + PDP + "runs-v2/3-0-13-4.jsonl)",
  "script": SC + "1010.py",
  "notes": "ANALYSIS.json stores replica_runs_compared 192 and replica_runs_mismatched 3; README.md:63-66 states the 3 differences are verified counts. 'Exact' is taken as the complete_and_exact flag. The minimum ratios are at the same cell and variant; the study's README table row 3:1:11:4 unsplit g=4 gives 22.8 and 391. Projection uses a pinned ns_per_word_xor calibrated on another host (README.md:154). No A/A or null-control line found in README, PROTOCOL or AMENDMENT."})

rows = [dict(t) for t in sorted({r["row_no"]: r for r in rows}.values(), key=lambda r: r["row_no"])]
with open(OUT, "w") as f:
    for r in rows:
        f.write(json.dumps(r, ensure_ascii=False) + "\n")
print("wrote", OUT, len(rows), "rows")
for r in rows:
    st = {}
    for c in r["claims"]:
        st[c["status"]] = st.get(c["status"], 0) + 1
    print(r["row_no"], r["slug"], st, "identical:", r["identical"]["status"])
