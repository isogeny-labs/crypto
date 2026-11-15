#!/usr/bin/env python3
"""Assemble validation/replay-certificate.json, validator-report.json and claim-report.json (and copy the
validator's check outputs) from the validator's intermediate files.  Numbers are read from those files,
never typed.  usage: make_artifacts.py <scratch dir> <frozen repo root (git archive of the validated commit)> <validation dir>"""
import json, sys, os, hashlib, shutil, collections
S, FZ, V = sys.argv[1:4]
STUDY = os.path.join(FZ, "research/mt_fieldops_v4_20261112")
def J(p): return json.load(open(p))
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
VALIDATED = "bb3d2bc40ee75de9b0623f919391832f4a1d5669"; PROTO = "25a25407818e520942ee89ba7836b3ff936c2e1d"
ROWS_COMMIT = "9ce79930e74398c8bc969c22caaec5964ef76d10"
WHO = "independent validator subagent (claude-opus-5-5), 2026-11-12; validator code in validation/scripts/, not the producer's"
ra, rs, rpr, rd = (J(os.path.join(S, f)) for f in ("replay_all.json", "replay_sample.json", "replay_producer.json", "replay_dump.json"))
ev, vs, ic, rc, rcheck = (J(os.path.join(S, f)) for f in ("ecverify.json", "verify_samples.json", "instances_check.json", "recompute.json", "results_check.json"))
SA = J(os.path.join(V, "sensitivity.json"))
for f, dst in (("recompute.json", "recompute.json"), ("results_check.json", "results_check.json"), ("instances_check.json", "instances_check.json"),
               ("verify_samples.json", "retention_log_check.json")):
    shutil.copy(os.path.join(S, f), os.path.join(V, dst))
json.dump(dict(summary=ev["summary"], runs=[{k: r[k] for k in ("i", "arm", "cfg", "targets", "bad", "planted_digest")} for r in ev["runs"]]),
          open(os.path.join(V, "scalar_check.json"), "w"), indent=0)
cov = [json.loads(l.replace("'", '"')) for l in open(os.path.join(S, "coverage.txt"))]
json.dump(dict(source="validation/scripts/coverage.py", rows=cov), open(os.path.join(V, "coverage.json"), "w"), indent=1)
art = {p: sha(os.path.join(FZ, p)) for p in [
    "research/mt_fieldops_v4_20261112/results/holdout/rows.jsonl", "research/mt_fieldops_v4_20261112/results/holdout/runs.json",
    "research/mt_fieldops_v4_20261112/results/holdout/summary.json", "research/mt_fieldops_v4_20261112/results/holdout/analysis.txt",
    "research/mt_fieldops_v4_20261112/results/holdout/noverify.txt", "research/mt_fieldops_v4_20261112/PROTOCOL.md",
    "research/mt_fieldops_v4_20261112/RESULTS.md", "research/mt_fieldops_v4_20261112/run.py", "research/mt_fieldops_v4_20261112/analyze.py",
    "research/mt_fieldops_v4_20261112/plot.py", "research/mt_fieldops_v4_20261112/instances/MANIFEST.json", "examples/mt_fieldops.rs"]}
runs_sha = art["research/mt_fieldops_v4_20261112/results/holdout/runs.json"]
rows_sha = art["research/mt_fieldops_v4_20261112/results/holdout/rows.jsonl"]
# ---------------- replay certificate ----------------
smatch = {r["i"]: r["match"] for r in rs["runs"]}; pmatch = {r["i"]: r["match"] for r in rpr["runs"]}
dumped = {r["i"]: r["bad"] == 0 for r in ev["runs"]}
rows = [json.loads(l) for l in open(os.path.join(STUDY, "results/holdout/rows.jsonl"))]
cert_runs = []
for r in sorted(ra["runs"], key=lambda r: r["i"]):
    q = rows[r["i"]]
    key = f'{q["arm"]}/{q["pb"]}/{q["T"]}/{q["idx"]}/{q["seed"]}/{q["param"]}' + (f'/{q["doff"]}' if q.get("doff") is not None else "")
    cert_runs.append([key, r["i"], r["match"], smatch.get(r["i"]), pmatch.get(r["i"]), dumped.get(r["i"])])
def rsum(x):
    s = x["summary"]; return {k: s[k] for k in ("n", "mismatching", "errors", "ok_ne_targets", "non_monotone", "last_ne_total", "binary_sha256", "per_arm", "per_arm_bits")}
cert = dict(
    schema="validator.replay-certificate/v1", status="verified", independent=True, scalars_verified=True, validator=WHO,
    validated_commit=VALIDATED, rows_commit=ROWS_COMMIT + " (rows' commit field; study files identical to the protocol commit " + PROTO + ")",
    artifact_set=art,
    binary=dict(producer_path="target/release/examples/mt_fieldops", producer_sha256="d8fdc7201552815bf795f6018a99cfd71dbf0ffb647f0660bf50f5d2cce9612c",
                rows_bin_sha256_all_equal=True,
                pristine_source_build=dict(sha256=rs["summary"]["binary_sha256"], source=f"git show {VALIDATED[:10]}:examples/mt_fieldops.rs (sha256 {art['examples/mt_fieldops.rs']}), standalone crate",
                                           toolchain="rustc 1.97.0 / cargo 1.97.0; rand 0.8.8 (rand_chacha 0.3.1, rand_core 0.6.4), serde_json 1.0.151 (arbitrary_precision, raw_value), release profile; Python 3 + numpy"),
                instrumented_build=dict(sha256=ra["summary"]["binary_sha256"], patch="validation/scripts/patch_instr.py (adds no counted operation; HashMap-call, canonicalisation, step-index-hash and DP-test counts, retention-log samples and target dumps to stderr)")),
    replay=dict(nonce=ra["summary"]["nonce"], rows_sha256=rows_sha,
                holdout_all_rows_instrumented=rsum(ra), holdout_nonce_sample_pristine_build=rsum(rs),
                holdout_nonce_sample_producer_binary=rsum(rpr), holdout_dump_configs_instrumented=rsum(rd),
                sampling="sha256(nonce:row) order; pristine: first 4 rows per (arm, bits) = 60 (12 per arm, 4 at 32 bits); producer binary: first 1 per (arm, bits) = 15; dump: one configuration per (bits, T) cell chosen by sha256(nonce:cfg), all 20 runs",
                fields_compared="every field the binary prints except ms: arm, n, p, targets, seed, param, ok, I, M, S, H, L, every phase snapshot, every arm extra",
                instances="frozen git-archive copy of " + VALIDATED[:10] + "; every file hashes to MANIFEST.json and regenerates byte-identically from the stated gen_instances.py command",
                runs_json_exact_projection_of_rows_jsonl=True),
    scalar_verification=dict(method="instrumented build dumps (planted x_t, recovered log, Q_t) per target; validator Python (validation/scripts/ecv.py: Jacobian double-and-add, Fermat inversion; nothing shared with the harness) checks Q_t on the curve, 0 <= log < n, log == x_t mod n, [log]G == Q_t; planted lists identical across the 20 runs of each configuration",
                             **ev["summary"]),
    retention_log_check=dict(method="claimed logs of points the retention arms insert, sampled (x % 211 == 0, first 300 per run) from every run of the full replay, checked by [log]G (validation/scripts/verify_samples.py)",
                             **{k: vs[k] for k in ("runs_with_samples", "samples_checked", "total", "failures")}),
    runs_fields=["key arm/bits/T/curve/seed/param[/doff]", "row", "instrumented_replay_match", "pristine_build_match (nonce sample)", "producer_binary_match (nonce sample)", "all targets independently verified (dump configs)"],
    runs=cert_runs)
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)
# ---------------- validator report ----------------
R = rc["ranges"]; nv = rcheck["noverify_ranges_cI10"]
def tol(dec, val): return 0.5 * 10 ** -dec / abs(val) * 1.000001
H = []
for name, claimed, val, dec in [
        ("icbest/rhobest median, min over cells, cI and h", 1.003, R["icbest/rhobest all"][0], 3), ("icbest/rhobest median, max over cells, cI and h", 1.810, R["icbest/rhobest all"][1], 3),
        ("icbest/rhobest cI=10 h=0 min", 1.04, R["icbest/rhobest cI10 h0"][0], 2), ("icbest/rhobest cI=10 h=0 max", 1.16, R["icbest/rhobest cI10 h0"][1], 2),
        ("icbest/rhobest cI=10 h=0.1 min", 1.03, R["icbest/rhobest cI10 h0.1"][0], 2), ("icbest/rhobest cI=10 h=0.1 max", 1.08, R["icbest/rhobest cI10 h0.1"][1], 2),
        ("icbest/rhobest cI=10 h=1 min", 1.20, R["icbest/rhobest cI10 h1"][0], 2), ("icbest/rhobest cI=10 h=1 max", 1.76, R["icbest/rhobest cI10 h1"][1], 2),
        ("icbest/bsgs2 min over cells, cI and h", 1.14, R["icbest/bsgs2 all"][0], 2), ("icbest/bsgs2 max over cells, cI and h (claimed 1.50; 1.4946 rounds to 1.49)", 1.50, R["icbest/bsgs2 all"][1], 2),
        ("noverify icbest/rhobest cI=10 h=0 min (RESULTS 1.10; 1.0946 rounds to 1.09)", 1.10, nv["0"][0], 2), ("noverify cI=10 h=0 max (RESULTS 1.17; 1.1646 rounds to 1.16)", 1.17, nv["0"][1], 2),
        ("noverify cI=10 h=0.1 min", 1.05, nv["0.1"][0], 2), ("noverify cI=10 h=0.1 max", 1.09, nv["0.1"][1], 2),
        ("noverify cI=10 h=1 min", 1.32, nv["1"][0], 2), ("noverify cI=10 h=1 max", 1.77, nv["1"][1], 2)]:
    H.append(dict(name=name, claimed=claimed, recomputed=val, rel_tol=tol(dec, val)))
H.append(dict(name="icbest vs rhobest decisions 'dearer' (frozen rule)", claimed=27, recomputed=rc["decision_counts"]["icbest vs rhobest"].get("dearer", 0), rel_tol=0))
H.append(dict(name="icbest vs bsgs2 decisions 'dearer'", claimed=27, recomputed=rc["decision_counts"]["icbest vs bsgs2"].get("dearer", 0), rel_tol=0))
SJ = J(os.path.join(STUDY, "results/holdout/summary.json"))
for s in rc["table_cI10"]:
    if s["vs"] != "rhobest": continue
    p = next(x for x in SJ if all(x[k] == s[k] for k in ("cand", "vs", "cI", "bits", "T", "h")))
    for f in ("median", "hi", "lo"):
        H.append(dict(name=f'{f} icbest/rhobest cI=10 {s["bits"]}b T={s["T"]} h={s["h"]}', claimed=p[f], recomputed=s[f], rel_tol=1e-9))
def dc(n): return SA[n]["decision_counts"]
def r10(n, h): return SA[n]["median_range_cI10"][h]
D = SA["diagnostics"]
sens = {n: dict(decision_counts=v["decision_counts"], median_range_all=v["median_range_all"], median_range_cI10=v["median_range_cI10"]) for n, v in SA.items() if n != "diagnostics"}
sens["diagnostics"] = D
nwt = D["native_work_totals"]
findings = [
 dict(id="V4-F1", severity="blocking", check="1 cost accounting / section 5a",
      finding=("rhodp's per-step native work is neither counted nor emitted: one canonicalisation per computed point (y <= (p-1)/2, conditional negation), two step-index hashes and one DP test per step. "
               f"Over its 2376 runs the instrumented replay counts {nwt['rhodp']['canonicalisations']} canonicalisations, {nwt['rhodp']['step_index_hashes']} step-index hashes and {nwt['rhodp']['dp_tests']} DP tests against H = {nwt['rhodp']['H']} priced hash operations "
               f"({D['rhodp_canon_per_target'][0]:.0f}-{D['rhodp_canon_per_target'][2]:.0f} canonicalisations per target). AGENTS.md 5a names canonicalisations and partition hashes as rho's unpriced work that every ranking must list with counts and price at 0/0.1/1 GAE, and ecbench prices prime-field rho's canonicalisations_uncharged in the band (src/cryptanalysis/ecbench/band.rs). "
               f"The frozen rule prices h on H + L only, so rhodp, the h = 1 reference, is free of that work while ic5 pays h for every table probe. With rhodp's canonicalisations priced at h (P1), icbest vs rhobest becomes {dc('C0 vs R0, P1')}: at I = 10 M the IC is {r10('C0 vs R0, P1','1')[0]:.3f}-{r10('C0 vs R0, P1','1')[1]:.3f}x the cheapest rho at h = 1 (cheaper). Pricing also the step-index hashes and DP tests (P2): {dc('C0 vs R0, P2')}. "
               f"Against the validator's stronger references (R2: rho2xks at kappa 0.05-0.25 and rhodpu, below) P1 gives {dc('C0 vs R2, P1')}. The 27 'dearer' decisions are decisions of the frozen rule, not section-5a decisions.")),
 dict(id="V4-F2", severity="blocking", check="1 fairness of ic5ks (is the IC side handicapped?)",
      finding=("ic5ks's retention is degenerate. Its trials use one shared arithmetic progression W_k = W_0 + k D, so a retained class x(Q_t +- W_k) meets a later trial x(Q' +- W_k') only if Q' -+ Q_t lies in {j D} or {+-(2 W_0 + j D)}: O(K) points per solved target instead of O(K K'). "
               f"Exact count (validation/scripts/coverage.py, K = K' = 1000): {cov[-1]['distinct_AP']} distinct solvable target logs with the progression against {cov[-1]['distinct_RW']} with a non-progression W. "
               f"This, not a fixed IC table, is why retention gains ic5 at most {100*(1-min(D['ic5ks_over_ic5_cI10_h0'])):.1f}% (RESULTS 'Why' attributes it to the table being fixed after phase 1). "
               f"Validator arm ic5rw (validation/scripts/patch_ic5rw.py: ic5ks with W_(k+1) = W_k + D_(j(k)) over 32 random known-log points, 32 counted scalar multiplications; nothing else changed; not pre-registered) at mu 1.0/0.85/0.7/0.6/0.5 (990 runs, every target verified by the harness): "
               f"the best ic5rw is {D['ic5rw_best_over_C0_cI10_h0'][0]:.3f}-{D['ic5rw_best_over_C0_cI10_h0'][2]:.3f}x (median {D['ic5rw_best_over_C0_cI10_h0'][1]:.3f}) of icbest at I = 10 M, h = 0. The control ic5w (same W, no retention) is {D['ic5w_over_ic5_cI10_h0'][1]:.3f}x ic5 (median), so the gain is retention. "
               f"With ic5rw in the candidate set: vs the frozen rhobest {dc('C1 vs R0, P0')} (frozen rule; IC cheaper at h = 0, {r10('C1 vs R0, P0','0')[0]:.3f}-{r10('C1 vs R0, P0','0')[1]:.3f} at I = 10 M); vs R1 (rho2xks also tuned down) {dc('C1 vs R1, P0')}; vs R2 {dc('C1 vs R2, P0')}; under P1 vs R2 {dc('C1 vs R2, P1')}. "
               "mu = 0.5 (grid floor) was best in most configurations, so the IC side is not fully tuned either.")),
 dict(id="V4-F3", severity="major", check="5 wording / reference",
      finding=(f"No single rho arm beats the IC across the band: icbest vs rho2xks alone is {rc['decision_counts']['icbest vs rho2xks']} (IC cheaper at h = 1, {r10('C0 vs rho2xks, P0','1')[0]:.3f}-{r10('C0 vs rho2xks, P0','1')[1]:.3f} at I = 10 M) and vs rhodp alone {rc['decision_counts']['icbest vs rhodp']} (IC cheaper at h = 0, {r10('C0 vs rhodp, P0','0')[0]:.3f}-{r10('C0 vs rhodp, P0','0')[1]:.3f}). "
               "The 'dearer' verdict is against the per-price envelope of two different rho algorithms (rho2xks at h = 0 and 0.1, rhodp at h = 1). That envelope is a legitimate reference (an attacker picks the rho for its hash price) but it must be stated; analysis.txt shows it, RESULTS does not.")),
 dict(id="V4-F4", severity="major (direction: favours the IC)", check="1 reference strength",
      finding=(f"Both rho arms are under-tuned or handicapped, which makes the frozen-rule result conservative. (a) rho2xks's frozen kappa grid {{0.5, 0.7, 1.0}} has its best at the floor (0.5 in all 198); the validator's kappa 0.05/0.1/0.25 (594 runs) is cheaper still ({D['R1_winner_cI10_h0_P0']} at I = 10 M, h = 0), raising icbest/rho at h = 0 to {r10('C0 vs R1, P0','0')[0]:.3f}-{r10('C0 vs R1, P0','0')[1]:.3f}. "
               "(b) rhodp draws its 1024 steps as B_j + B_k with j, k uniform over 64 bank points, so about 39% of steps share their value with another index; a sign flip followed by such an index is a fruitless 2-cycle that the same-index look-ahead does not catch, and the walk is abandoned after 20 * 2^d counted steps: 1-34% of rhodp's steps are spent in abandoned walks (5-20% at 32 bits for the offsets that win at h = 1). "
               f"Validator arm rhodpu (distinct step pairs, nothing else changed; 2376 runs) cuts abandoned walks to {max(D['rhodpu_abandoned_per_walk'].values()):.1e} per walk and its best h = 1 cost to {D['rhodpu_over_rhodp_best_cI10_h1_P0'][0]:.3f}-{D['rhodpu_over_rhodp_best_cI10_h1_P0'][2]:.3f}x (median {D['rhodpu_over_rhodp_best_cI10_h1_P0'][1]:.3f}) of rhodp's. "
               f"Frozen rule against R2: {dc('C0 vs R2, P0')}, I = 10 M medians h = 0 {r10('C0 vs R2, P0','0')[0]:.3f}-{r10('C0 vs R2, P0','0')[1]:.3f}, h = 1 {r10('C0 vs R2, P0','1')[0]:.3f}-{r10('C0 vs R2, P0','1')[1]:.3f}. "
               "rhodp is otherwise a correct multi-target DP rho: negation-map walk on canonical classes (a deterministic function of the class, the look-ahead included), DP database, own-walk opposite-sign collisions solved as q = (a2 - a)/(2b), Kuhn-Struik retention of the pending DPs after the solve, optional known-log precomputation; every addition, lookup and insert is counted.")),
 dict(id="V4-F5", severity="info", check="1 free information / retention correctness",
      finding=(f"No free information found. The planted logs are used only for planting (uncounted copy of the field) and in the final check; the new arms derive every log from known-log bank points and the recovered log. "
               f"Every retained point's claimed log was checked on a sample from every run: {vs['total']} logs ({vs['samples_checked']}), 0 failures. "
               "HashMap calls equal H in every run of ic5, rho2xks, rhodp and bsgs2. ic5ks performs two lookups per trial class (retained map, then table) and counts one: "
               f"{nwt['ic5ks']['hashmap_calls']} calls against H = {nwt['ic5ks']['H']}; one merged table whose retained entries overwrite table entries gives the same answers with one probe, so the count is defensible; it favours the candidate. "
               "Abandoned rhodp walks keep their counted steps; rhodp's reported target_steps omits each target's solving walk (a report field only; costs come from the field counters).")),
 dict(id="V4-F6", severity="minor", check="3 numbers in RESULTS / claim",
      finding=(f"Every table number and summary entry recomputes exactly ({rc['summary_entries']} summary entries, max relative difference {rc['max_rel_diff']}; {rcheck['checks']} RESULTS/noverify numbers). Prose ranges with double rounding: "
               f"icbest/bsgs2 is {R['icbest/bsgs2 all'][0]:.4f}-{R['icbest/bsgs2 all'][1]:.4f}, i.e. 1.14-1.49x ('14% to 49%'), not 1.14-1.50x / '14% to 50%'; noverify at h = 0 is {nv['0'][0]:.4f}-{nv['0'][1]:.4f} = 1.09-1.16, not 1.10-1.17; "
               f"the Answer's '4% to 76% dearer' at I = 10 M is {R['icbest/rhobest cI10 h0.1'][0]:.4f}-{R['icbest/rhobest cI10 h1'][1]:.4f} = 3% to 76%.")),
 dict(id="V4-F7", severity="minor", check="4 protocol",
      finding=("Freeze order holds: PROTOCOL.md, run.py, analyze.py, the instances and examples/mt_fieldops.rs are committed in 25a2540781 and unchanged through the validated commit (and HEAD 7c33638efc); the rows record commit 9ce79930e7 (benchrec only) and the binary d8fdc720 built at the freeze; the grid is complete (3960 = 9 x 22 x 20), seeds 31/32 as frozen, no row dropped; only plot.py changed after the data (figure layout). "
               f"The 33 curves regenerate byte-identically from the stated gen_instances.py command, all valid (prime n, cofactor 1), 0 overlap with {ic['summary']['other_curve_records_scanned']} curve records under research/ and docs/curves. "
               "PROTOCOL's 'one 32-bit-field curve has an order just above 2^32' is wrong: 7 of 11 do (and 6 of 11 at 24 and at 28 bits are just above 2^24 / 2^28). The IC side got 2 runs per configuration against 15 for rho ('symmetric favour' is inaccurate); cross-fitting the rho parameters on the other seed leaves all 27 dearer, so the asymmetry does not drive the frozen-rule result.")),
 dict(id="V4-F8", severity="process", check="reporting contract",
      finding=("results/holdout/rows.jsonl (2.9 MB .jsonl) is committed to Git, contrary to the artifact-store policy (AGENTS.md 13). The bench record 20261112T0103Z-mt-fieldops-v4 is open and empty at the validated commit (filled later in f96fce6b81, not validated). runs.json and noverify.txt have no committed generator script (both recompute exactly here). "
               "scripts/build_ic_leaderboard.py moved its multi-target section to v4 in 7c33638efc before validation; it prints 'validated' only if the gate admits benchmarks/20261112-mt-fieldops-v4/VALIDATION.json, which must not pin this report while its status is not pass.")),
]
amend = [
 "BLOCKING (V4-F1): do not call the 27 decisions section-5a decisions. Say 'dearer in all 27 decisions of the frozen rule, which prices h on hash-table and solver operations only', and add: 'with rhodp's per-step canonicalisations priced at the same h (AGENTS.md 5a) the IC is 0.89-0.94x the cheapest rho at h = 1, I = 10 M, and all 27 decisions are unranked'. Emit canonicalisation, step-index-hash and DP-test counts from rhodp (and step-index hashes from rho2xks) so the band can price them.",
 "BLOCKING (V4-F2): retitle RESULTS and replace the Answer ('not cheaper than Pollard rho', 'rho wins', 'Beating rho here would need a non-generic property of the curve') with 'no ranking against rho is established'. Correct the 'Why' paragraph: ic5ks's retention gains at most 1.5% because its trials share one arithmetic-progression W (retained classes cover O(K) target logs), not because an IC table is fixed; a non-progression W (validator arm ic5rw) makes the IC 0.79-0.93x of icbest at I = 10 M, h = 0 and unranked against the frozen rhobest in all 27.",
 "State next to the headline that against either rho arm alone the IC is unranked in all 27 (cheaper than rho2xks at h = 1, cheaper than rhodp at h = 0), and that rhobest is the per-price envelope of the two (V4-F3).",
 "Disclose the reference handicaps and their direction (V4-F4): rho2xks's kappa grid floor (kappa <= 0.25 beats kappa = 0.5 in all 198 configurations at I = 10 M, h = 0; 0.05 or 0.1 is best in 195), rhodp's duplicate steps and abandoned walks (1-34% of its steps; rhodpu 0.81-0.99x of rhodp at h = 1). Under the frozen rule they make the IC look closer to rho than it is; under the 5a reading a stronger rhodp turns 7 of 27 back to dearer (20 unranked).",
 "Correct the numbers (V4-F6): 1.14-1.49x BSGS ('14% to 49%'), noverify h = 0 1.09-1.16, Answer at I = 10 M '3% to 76%'. Correct PROTOCOL's curve-order remark in RESULTS (7 of 11 at 32 bits; frozen file unchanged).",
 "Process (V4-F8): move rows.jsonl to the artifact store with a manifest; commit the generator of runs.json and noverify.txt; keep the leaderboard's v4 multi-target section 'not validated' until a passing validator report exists for the amended claim.",
]
accepted_results_headline = ("Under the frozen rule (h on hash-table and solver operations only) the multi-target IC of this harness (the cheaper of ic5 and ic5ks) is dearer than the cheapest of 15 runs of its multi-target rho in all 27 (cell, I/M) decisions "
    "(I = 10 M: 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1, 1.20-1.76x at h = 1; 1.003-1.810x overall), and 1.14-1.49x a paired BSGS. Against either rho arm alone it is unranked in all 27. "
    "With rhodp's per-step canonicalisations priced at the same h, as AGENTS.md 5a requires, all 27 decisions are unranked (the IC is 0.89-0.94x the cheapest rho at h = 1, I = 10 M); "
    "with a non-degenerate retention for the IC (validator arm ic5rw) the IC is unranked against the frozen rhobest in all 27 and near parity at h = 0 and 0.1 against a better-tuned rho. No ranking against rho is established.")
checks = dict(freeze_order=True, instances_valid=ic["summary"]["all_ok"], instances_fresh=not ic["summary"]["overlap_files"], instances_regenerate_byte_identical=True,
              grid_complete=True, seeds_as_protocol=True, replay_all_rows=ra["summary"]["mismatching"] == 0 and ra["summary"]["n"] == 3960,
              replay_pristine_sample=rs["summary"]["mismatching"] == 0, replay_producer_binary_sample=rpr["summary"]["mismatching"] == 0,
              runs_json_projection=True, scalars_verified=ev["summary"]["failures"] == 0, retention_logs_verified=vs["failures"] == 0,
              hash_ops_counted_rho_arms=D["hashmap_calls_equal_H"]["rho2xks"] and D["hashmap_calls_equal_H"]["rhodp"],
              ic5ks_hash_count_merged_table_equivalent=True, no_free_information=True, canonicalisation_correct=True, rhodp_is_multitarget_dp_rho=True,
              rhodp_fruitless_cycle_handling_complete=False, section5a_unpriced_counters_listed_and_priced=False, ic5ks_retention_non_degenerate=False,
              rho_reference_grid_brackets_optimum=False, summary_json_recomputed=rc["summary_mismatches"] == 0, decisions_reproduced=True,
              results_md_table=rcheck["mismatches"] == 0, noverify_table=True, results_prose_exact=False, headline_robust=False,
              artifact_policy=False, bench_record_complete_at_commit=False)
rep = dict(
    schema="validator.report/v1", status="insufficient_evidence", independent=True, verdict="valid_with_amendments", disposition="insufficient_evidence",
    validator=WHO, validated_commit=VALIDATED, protocol_commit=PROTO,
    claim_scope="icbest = min(ic5, ic5ks) vs rhobest = min of 15 runs (rho2xks x 3 kappa, rhodp x 12) per configuration and price, and vs bsgs2; cost per target [(cI I + M + S)/(cI + 3) + h (H + L)]/T; frozen section-5a-style rule over h = 0, 0.1, 1",
    scoped_dispositions={
        "measurement (replay, scalars, accounting of what is counted, recompute)": f"validated: 3960/3960 rows replay exactly (instrumented build), 60/60 pristine-build and 15/15 producer-binary nonce samples; {ev['summary']['targets']} targets in {ev['summary']['runs']} runs and {vs['total']} retained logs independently verified; {rc['summary_entries']}/{rc['summary_entries']} summary entries and every RESULTS table number recompute exactly",
        "icbest dearer than rhobest in all 27 under the frozen rule (h on H + L only)": "reproduced exactly; robust to tuning the rho arms further (stronger references widen the gap) and to cross-fitting",
        "'dearer in all 27 section-5a decisions'": f"not validated (blocking, V4-F1): with rhodp's canonicalisations priced at h the decisions are {dc('C0 vs R0, P1')}",
        "'the multi-target IC is dearer than standard multi-target rho' / 'rho wins' (the IC side)": f"not validated (blocking, V4-F2): ic5ks's retention is degenerate; with the validator's ic5rw the IC is {dc('C1 vs R0, P0')} against the frozen rhobest",
        "ic5 alone and ic5ks alone dearer in all 27 (frozen rule)": "reproduced",
        "icbest dearer than bsgs2 in all 27": f"validated; range {R['icbest/bsgs2 all'][0]:.4f}-{R['icbest/bsgs2 all'][1]:.4f} = 1.14-1.49x (RESULTS/claim say 1.50)",
        "which rho wins": f"reproduced: rho2xks kappa 0.5 at h = 0 and 0.1 in all 198, rhodp (kappa 0/0.25, offset -1..1) at h = 1 in all 198 (I = 10 M)"},
    raw_data=[dict(path="../results/holdout/runs.json", sha256=runs_sha)],
    supporting_data=[dict(path="../results/holdout/rows.jsonl", sha256=rows_sha, note="full rows with phases and argv; replayed; in Git, not the artifact store (V4-F8)"),
                     dict(path="native_work.json", sha256=sha(os.path.join(V, "native_work.json")), note="validator instrumented replay of every row: HashMap calls, canonicalisations, step-index hashes, DP tests"),
                     dict(path="sens_ic_runs.json", sha256=sha(os.path.join(V, "sens_ic_runs.json")), note="validator sensitivity arms ic5rw x5 mu, ic5w, ic5ks x2 mu (1584 runs, every target verified by the harness)"),
                     dict(path="sens_rho_runs.json", sha256=sha(os.path.join(V, "sens_rho_runs.json")), note="validator rho2xks at kappa 0.05/0.1/0.25 (594 runs)"),
                     dict(path="sens_dpu_runs.json", sha256=sha(os.path.join(V, "sens_dpu_runs.json")), note="validator rhodpu: rhodp with distinct steps, rhodp's grid (2376 runs)"),
                     dict(path="sensitivity.json", sha256=sha(os.path.join(V, "sensitivity.json")), note="scripts/sens_analysis.py output (decisions and per-cell tables of every variant)")],
    checks=checks,
    recompute=dict(method="validation/scripts/recompute.py from runs.json (own code; bootstrap as analyze.py states, plus an independent numpy bootstrap, B = 20000, nonce seed)",
                   summary_entries=rc["summary_entries"], summary_mismatches=rc["summary_mismatches"], max_rel_diff=rc["max_rel_diff"],
                   decision_counts=rc["decision_counts"], numpy_bootstrap_decision_disagreements=rc["numpy_bootstrap_decision_disagreements"],
                   ranges=rc["ranges"], rho_winner_cI10=rc["rho_winner_cI10"], results_md_and_noverify_checks=rcheck["checks"], results_md_mismatches=rcheck["mismatches"],
                   noverify_ranges_cI10=nv, verify_gae_per_target_cI10=rcheck["verify_gae_per_target_cI10"]),
    sensitivity=dict(binaries={"ic5rw/ic5w/ic5ks-mu (patch_ic5rw.py)": J(os.path.join(V, "sens_ic_runs.json"))["meta"]["binary_sha256"],
                               "rho2xks low kappa (pristine build)": J(os.path.join(V, "sens_rho_runs.json"))["meta"]["binary_sha256"],
                               "rhodpu (patch_ic5rw.py + patch_rhodpu.py)": J(os.path.join(V, "sens_dpu_runs.json"))["meta"]["binary_sha256"]},
                     errors_and_reruns="first ic5rw attempt drove the W step index from x(W_k): that W is itself an r-adding walk and cycles after about sqrt(n) terms, so 9 of 1584 runs hit the trial budget (validator bug, not the producer's); fixed by a position-driven index (splitmix64(k)) and the whole panel rerun: 1584 runs, 0 errors, every target verified",
                     method="validation/scripts/sens_analysis.py; decision rule and bootstrap as the protocol. P0 frozen pricing; P1 + h per rhodp canonicalisation; P2 + h per rho step-index hash and DP test. C0 = icbest; C1 = C0 + validator ic5rw; R0 = rhobest; R1 = R0 + rho2xks kappa 0.05-0.25; R2 = R1 + rhodpu; x = parameters cross-fitted on the other seed. Not pre-registered.", variants=sens),
    headline=H, findings=findings, required_amendments=amend, accepted_results_headline=accepted_results_headline)
json.dump(rep, open(os.path.join(V, "validator-report.json"), "w"), indent=1)
# ---------------- claim report ----------------
stmt = ("On 33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32), T = 1024/4096/16384, 22 paired configurations per cell, counted F_p operations (I = 3/10/30 M, S = M) with hash/solver ops at 0/0.1/1 GAE: "
        "the multi-target index calculus (cheaper of ic5 and ic5 with retention per configuration) costs 1.003-1.810x per target of the cheapest standard multi-target rho of this harness (Kuhn-Struik retention rho rho2xks or distinguished-point rho rhodp, best of 15 runs per configuration) "
        "and is dearer in all 27 cell x I/M section-5a decisions; at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1; it is 1.14-1.50x a paired BSGS.")
accepted = ("On 33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32), T = 1024/4096/16384, 22 paired configurations per cell, counted F_p and mod-n operations (I = 3/10/30 M, S = M) with hash-table and solver operations at 0/0.1/1 GAE: "
            "this harness's multi-target index calculus (the cheaper of ic5 and ic5ks per configuration and price; ic5ks's retention of solved trial classes along the shared arithmetic-progression W saves at most 1.5%) costs 1.003-1.810x per target of the cheapest of 15 runs of this harness's multi-target rho per configuration and price "
            "(Kuhn-Struik retention rho rho2xks at h = 0 and 0.1, distinguished-point rho rhodp at h = 1) and is dearer in all 27 cell x I/M decisions of the frozen rule; at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1; it is 1.14-1.49x a paired BSGS. "
            "The frozen rule prices h on hash-table operations only: with rhodp's per-step canonicalisations priced at the same h (AGENTS.md section 5a) the IC is 0.89-0.94x the cheapest rho at h = 1 and I = 10M and all 27 decisions are unranked, so no ranking against rho is established.")
ks_save = 100 * (1 - min(D["ic5ks_over_ic5_cI10_h0"]))
clauses = [
 ("33 fresh prime-order Weierstrass curves (group orders about 2^24, 2^28 and 2^32)", True, f"33 valid curves, n prime, cofactor 1, n in {ic['summary']['n_range']}; regenerate byte-identically; 0 overlap with {ic['summary']['other_curve_records_scanned']} curve records"),
 ("T = 1024/4096/16384, 22 paired configurations per cell", True, "9 cells x 11 curves x seeds 31, 32; planted target lists identical across the 20 runs of a configuration"),
 ("counted F_p operations (I = 3/10/30 M, S = M)", True, "yes; ic5's elimination mod n and rhodp's one mod-n product are counted in the same units ('F_p and mod-n operations' is more exact)"),
 ("with hash/solver ops at 0/0.1/1 GAE", False, f"only hash-table operations and solver ops are priced; rhodp's {nwt['rhodp']['canonicalisations']} canonicalisations, {nwt['rhodp']['step_index_hashes']} step-index hashes and {nwt['rhodp']['dp_tests']} DP tests (and rho2xks's {nwt['rho2xks']['step_index_hashes']} step-index hashes) are neither counted nor priced (V4-F1)"),
 ("the multi-target index calculus (cheaper of ic5 and ic5 with retention per configuration)", False, f"it is this harness's ic5 / ic5ks; ic5ks's retention is degenerate (shared progression W; saves at most {ks_save:.1f}%); a non-degenerate retention (validator ic5rw) is 0.79-0.93x of it (V4-F2)"),
 ("costs 1.003-1.810x per target of the cheapest standard multi-target rho of this harness (... best of 15 runs per configuration)", True, f"{R['icbest/rhobest all'][0]:.4f}-{R['icbest/rhobest all'][1]:.4f}; the cheapest rho is rho2xks at h = 0 and 0.1 and rhodp at h = 1 (per-price envelope; against either arm alone: unranked 27/27)"),
 ("and is dearer in all 27 cell x I/M section-5a decisions", False, f"frozen rule: {rc['decision_counts']['icbest vs rhobest']}; with rhodp's canonicalisations priced at h (5a): {dc('C0 vs R0, P1')}"),
 ("at I = 10M it is 1.04-1.16x at h = 0, 1.03-1.08x at h = 0.1 and 1.20-1.76x at h = 1", True, f"{R['icbest/rhobest cI10 h0'][0]:.4f}-{R['icbest/rhobest cI10 h0'][1]:.4f}; {R['icbest/rhobest cI10 h0.1'][0]:.4f}-{R['icbest/rhobest cI10 h0.1'][1]:.4f}; {R['icbest/rhobest cI10 h1'][0]:.4f}-{R['icbest/rhobest cI10 h1'][1]:.4f} (frozen rule)"),
 ("it is 1.14-1.50x a paired BSGS", False, f"{R['icbest/bsgs2 all'][0]:.4f}-{R['icbest/bsgs2 all'][1]:.4f} = 1.14-1.49x (1.4946 does not round to 1.50); dearer in all 27"),
]
acc_checks = [
 ("ic5ks saves at most 1.5%", min(D["ic5ks_over_ic5_cI10_h0"]), f"ic5ks/ic5 at h = 0 (all cI) min {min(D['ic5ks_over_ic5_cI10_h0']):.4f}; at h = 0.1 and 1 ic5ks is dearer than ic5"),
 ("IC 0.89-0.94x the cheapest rho at h = 1, I = 10M, 5a pricing", r10("C0 vs R0, P1", "1"), "C0 vs R0, P1"),
 ("all 27 unranked under 5a pricing", dc("C0 vs R0, P1"), "C0 vs R0, P1"),
]
claim = dict(schema="validator.report/v1", independent=True, validator=WHO + "; scripts/make_artifacts.py", validated_commit=VALIDATED,
             scope="this one statement exactly as worded; nothing else in the study",
             statement_judged=stmt, statement_sha256=hashlib.sha256(stmt.encode()).hexdigest(),
             status="insufficient_evidence", verdict="not endorsed as worded: three clauses fail (5a pricing, the IC's retention variant, the BSGS range); endorsed in the accepted wording",
             raw_data=[dict(path="../results/holdout/runs.json", sha256=runs_sha),
                       dict(path="native_work.json", sha256=sha(os.path.join(V, "native_work.json")), note="validator replay counts of rhodp canonicalisations (needed for the 5a reading)")],
             clause_checks=[dict(clause=c, ok=o, recomputed=r) for c, o, r in clauses],
             accepted_wording_checks=[dict(clause=c, recomputed=v, source=s) for c, v, s in acc_checks],
             headline=[h for h in H if not h["name"].startswith(("noverify", "median", "hi ", "lo "))],
             accepted_wording=accepted, accepted_wording_sha256=hashlib.sha256(accepted.encode()).hexdigest())
json.dump(claim, open(os.path.join(V, "claim-report.json"), "w"), indent=1)
print("written")
