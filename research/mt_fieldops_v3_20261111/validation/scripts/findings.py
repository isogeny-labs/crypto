#!/usr/bin/env python3
"""Validator findings, required amendments and scoped dispositions for both studies (numbers from out/)."""
import json, sys
V = sys.argv[1]; O = f"{V}/out"
J = lambda p: json.load(open(p))
sA = J(f"{O}/sens_v3.json"); sB = J(f"{O}/sens_b.json"); rcA = J(f"{O}/results_check_v3.json"); rcB = J(f"{O}/results_check_b.json")
st = rcA["ic5_struct_32b_T1024"]
FA = dict(
 status="insufficient_evidence",
 verdict="valid_with_amendments",
 disposition="insufficient_evidence",
 claim_scope="ic5 (k = 4, mu = 1.0) vs rhobest = cheapest of the 12 pre-registered runs of this harness's anchored fixed-database rho family (rho, rho2, rho2x, rhoseq x kappa 0.5/0.7/1.0), vs bsgs2 and vs ic4; cost per target in GAE incl. the shared verification phase; PROTOCOL.md section-5a rule over cI {3, 10, 30} x h {0, 0.1, 1}",
 scoped_dispositions={
  "measurement (replay, scalars, accounting, recompute)": "validated: 4488/4488 rows replay exactly; 108 runs / 774,144 targets independently verified; 540/540 summary entries and every RESULTS.md table number recompute exactly",
  "ic5 cheaper than ic4 in all 36 (cell, cI) (improvement over the multi-target IC baseline; section-3 class engineering)": "validated",
  "ic5 dearer than bsgs2 in all 36 (1.11x-1.49x)": "validated",
  "ic5 cheaper than the pre-registered harness rhobest in all 36": "reproduced exactly as a statement about that reference; not robust: removing two avoidable overheads of the harness rho leaves 30 cheaper and 6 unranked",
  "headline 'ic5 cheaper than the strongest rho in every cell' / 'moves the crossover against the strongest rho down to T = 1024'": "insufficient_evidence (blocking): the standard multi-target rho rule (Kuhn-Struik retention) is missing from the reference; the validator's measured retention arm makes ic5 dearer at h = 0 and 0.1 in all 36 and the 5a decision unranked in all 36"},
 checks={"freeze_order": True, "amendment_A1_substance": True, "amendment_A1_wording": False, "instances_valid": True, "instances_fresh": True,
         "tune_recomputed_and_frozen_before_holdout": True, "tune_holdout_disjoint": True, "grid_complete": True,
         "replay_all_rows": True, "replay_pristine_source_sample": True, "replay_producer_binary_sample": True, "runs_json_projection": True,
         "scalars_verified": True, "negative_control": True, "ic5_accounting_complete": True, "no_free_information": True,
         "summary_json_recomputed": True, "verdicts_agree": True, "results_md_table": True, "noverify_table": True, "headline_within_tol": True,
         "rho_reference_kappa_grid_brackets_optimum": True, "rho_reference_strongest_standard": False,
         "artifact_policy": False, "bench_record_complete_at_commit": False, "results_prose_exact": False},
 ks_runs=dict(rho2xks=1056, ic5ks=528, errors_or_unverified=0, build="validation/scripts/patch_ks.py on the frozen source, same toolchain"),
 findings=[
  dict(id="A-F1", severity="blocking", check="5 fairness (reference strength)", finding=(
   "The rho reference has no Kuhn-Struik retention, the defining rule of standard multi-target rho: once target t is solved, every point its walk computed has a known log and can be stored for the later targets. The anchored harness keeps a fixed phase-1 database, so its walk cost is about sqrt(2 n T) against about sqrt(n T) with retention. "
   "The validator added retention to the harness's own strongest arm (rho2xks = rho2x plus one counted hash insert per retained point; no other change, restart overhead kept) and ran it on all 264 holdout configurations at kappa 0.05, 0.1, 0.25, 0.5 (1056 runs, every target verified by the harness's counted check). "
   "With rho2xks added to rhobest, ic5 is dearer at h = 0 in all 36 (cell, cI) (median 1.069 to 1.337, every 95% lower bound > 1) and at h = 0.1 (1.046 to 1.181, every lower bound > 1). It is cheaper only at h = 1 (0.908 to 0.975), where the retention arm pays a hash insert for every stored point and the anchored rho2x stays the reference. Under the frozen section-5a rule all 36 decisions become unranked (as measured; also without the verification phase and with the restart cost removed). "
   "kappa 0.05, the smallest value tried, was best for rho2xks in 181 of 264 configurations at cI = 10, h = 0, so this reference is still not fully tuned and the measured gap is conservative. "
   "At h = 1 a distinguished-point Kuhn-Struik rho on {P, -P} classes needs about 2 hash operations per theta steps instead of 1 to 2 per step. A model estimate (validation/scripts/dp_estimate.py; not measured) puts ic5 at 1.14x to 2.17x that rho's cost per target at h = 1. With it, ic5 would be dearer at every h. "
   "Giving ic5 the same retention does not close the gap: ic5ks, measured on all 264 configurations, gains 0.0% to 2.0%, because the trial classes Q_t +- W_k from one shared progression collide across targets only along the differences (k - j) D. "
   "This repository already implements the standard multi-target rho as ecbench rho.multitarget_pooled (docs/ecbench/README.md). The headline 'cheaper than the strongest rho in every cell' and the answer to Q1 ('moves the crossover against the strongest rho down to T = 1024') are not supported."),
   evidence="validation/VALIDATOR_REPORT.md section 6; validation/scripts/patch_ks.py, run_ks.py, sens.py, dp_estimate.py; validator scratch out/ks_rows.jsonl, out/sens_v3.json"),
  dict(id="A-F2", severity="amendment", check="5 fairness (residual overheads after the fix)", finding=(
   "After the fix the rho arms still carry avoidable overheads, measured with an instrumented build that left every counter unchanged on all 4488 rows: "
   "(a) one counted addition per 32-step restart whose start point is neither stored nor looked up: 2.2% to 2.6% of rho2x's total cost (cI = 10, h = 0; 2.5% to 3.0% for rho); "
   "(b) repeated phase-2 lookups: 1.5% to 3.5% of rho2/rho2x lookups. They come from sibling self-duplicates, and from a new duplicate that the chained starts introduced: with probability 1/64 the restart increment B_k equals the previous walk's first arm, and the new walk then retraces the old one (plain rho at 32 b: 1.5% of lookups, the 1/64 x 31/32 rate); "
   "(c) phase-1 walks keep stepping along stored paths after a merge, until the next restart: rho2x 0.519 to 0.626 steps per database point against an ideal of about 0.5. "
   "If (a) and (b) are removed (validator counterfactual on measured counters), ic5 vs rhobest is cheaper in 30 of 36 and unranked in 6, all six at cI = 30 (largest median 1.011). Removing (a) alone leaves 34 cheaper and 2 unranked."),
   evidence="VALIDATOR_REPORT.md section 6; validation/scripts/patch_dbg.py, sens.py"),
  dict(id="A-F3", severity="amendment", check="4 protocol (amendment A1 wording)", finding=(
   "A1 is legitimate in substance. The bounded start set made the 32-bit holdout impossible to finish (db_target 1.468M points against at most about 1.464M reachable). The fix touches only the five restart hunks of the rho family (git diff f71b42cc4f 75dbc70b36). The six partial rows are consistent with the job order, and ic5's parameters, the instances, the seeds and the decision rule are unchanged. "
   "Two statements in A1 are false as written. (1) 'makes every rho arm cheaper or equal': per run it does not. In the Study B rerun, 3.5% to 15.7% of rho-family runs became dearer, by up to 4.3%, although rhobest fell in every cell. (2) 'no repeated walks': the chained starts repeat a walk with probability 1/64 per restart (A-F2b)."),
   evidence="research/mt_rho_start_fix_20261111 results; VALIDATOR_REPORT.md section 2"),
  dict(id="A-F4", severity="amendment", check="reporting (wording)", finding=(
   "The title, headline and Q1 answer say 'the strongest rho'. The reference is the cheapest of 12 pre-registered runs of this harness's anchored rho family (v2 validator F4 asked for this wording too). Of the 2376 (configuration, cI, h) settings, the winner was rho2x at kappa 0.7 in 2122, rho2 at 0.7 in 187, rho2x at 0.5 in 55, rho2 at 0.5 in 11 and rhoseq at 0.7 in 1."),
   evidence="validator out/recompute_v3.json rhobest_winner_counts"),
  dict(id="A-F5", severity="amendment", check="reporting (prose numbers)", finding=(
   f"'needs about 61 relations' is one run. At 32 b, T = 1024 the relation counts are {st['ic5_relations'][0]} to {st['ic5_relations'][2]} (median {st['ic5_relations'][1]}). '1396 measured vs 1380 predicted' is one run; the cell has {st['ic5_trials_per_target'][0]:.0f} to {st['ic5_trials_per_target'][2]:.0f} trials per target (median {st['ic5_trials_per_target'][1]:.0f}) against n/(2K) = 1380. The 'Why ic5 wins' section is interpretation and should be labelled so. The experiment does not test it, although the phase split supports it."),
   evidence="validator out/results_check_v3.json"),
  dict(id="A-F6", severity="amendment", check="reporting (section 3 class, generic nature)", finding=(
   "ic5 vs ic4 is an improvement over the multi-target IC baseline (class engineering: S fell). It is a success under the owner's criterion and is validated in all 36 (cell, cI). ic5's lead over the harness rho is generic, not index-calculus-specific. The relation search plus mod-n elimination costs 0.08% to 2.5% of ic5. A known-log factor base (F_i = r_i G) with the same table and trials, as ecbench's ic.known_log_ksum_v1, would cost 0.977x to 1.0005x ic5 (validator model on the measured phases). bsgs2, a generic method with the same shared-inversion lever, is 1.11x to 1.49x cheaper than ic5 in every cell. RESULTS should state both facts next to the rho ratio."),
   evidence="validation/scripts/generic_ic5.py; out/generic_ic5.json"),
  dict(id="A-F7", severity="process", check="artifact policy (AGENTS.md section 13)", finding=(
   "RESULTS.md says rows.jsonl is in the artifact store, but at 70d0b40bc3 results/holdout/rows.jsonl (3,156,592 bytes) is committed to Git and the directory has no .artifacts.json. results/tune/rows.jsonl (137,984 bytes) is also over the 64 KiB limit for data files. The replay needs rows.jsonl (phases, argv). Upload it and verify its hash (sha256 46ddf9ec...62bd) before it leaves Git."),
   evidence="git ls-files; ls research/mt_fieldops_v3_20261111"),
  dict(id="A-F8", severity="process", check="reproducibility bindings / bench record", finding=(
   "At the validated commit, benchmarks/records/20261111T0551Z-mt-fieldops-v3.json is open with no commands, no raw entries and no metrics. The run command (started 08:03:15Z, 1372 s) exists only in the uncommitted working tree. The benchmarks/20261111-mt-fieldops-v3 entry the record names does not exist. "
   "Rows bind commit e821c5fbaf and the binary sha256 773fb2ca... (all 4488). The rows have no dirty-tree flag and no toolchain. Counts are deterministic: a standalone build of the committed source (rustc 1.97.0, rand 0.8.8, serde_json 1.0.151) reproduces every replayed row. The run was on the local host, not isolab-eks, and Limits says so. It is not an ecbench session (section 12); Limits says 'this harness's, not ecbench's'."),
   evidence="frozen benchmarks/records at 70d0b40bc3; git status during validation"),
  dict(id="A-F9", severity="note", check="5 fairness (ic5 accounting)", finding=(
   "No uncounted field, hash or linear-algebra work was found in ic5. Every operation is counted, consistent with ic4 and bsgs2: factor-base square roots (counted Tonelli-Shanks), the (k-1)-term full points (I + 4M + 2S per pair), the table (I + 2M + 2S and 2 H per pair of classes), relation trials (xpair + H per class, + H dedupe, + 2 counted padd per hit), mod-n elimination and back substitution (every product M, every inverse I; one run at 32 b, T = 1024 used 4,577 M and 48 I), target trials (xpair + H per class, 2 counted padd per hit) and verification. "
   "The planted logs xs are used only to plant Q_t (an uncounted copy of the field) and in the final verification. Every arm leaves F_p and mod-n additions, subtractions and negations uncounted, and the source header says so. bsgs2, ic4 and ic5 share the same xpair and hash conventions."),
   evidence="examples/mt_fieldops.rs at 70d0b40bc3 lines 542-793, 1240-1247"),
  dict(id="A-F10", severity="note", check="reporting (verification constant)", finding=(
   "The shared verification phase costs 31.4 to 47.6 GAE per target at cI = 10. It is 64% of ic5's cost at 22 b, T = 16384, and pulls every ratio toward 1. RESULTS discloses it and reports noverify.txt, which recomputes exactly (36 of 36 rows)."),
   evidence="out/results_check_v3.json"),
  dict(id="A-F11", severity="note", check="m = 83 / ECC2K-130", finding="Not engaged: these are 22- to 32-bit prime-field curves and no transfer is claimed. The row text should say so in one line.", evidence="RESULTS.md"),
  dict(id="A-F13", severity="note", check="scope (HEAD moved)", finding=(
   "During validation HEAD advanced from 70d0b40bc3 to a9907c9254. Two commits are outside the validated set. "
   "866d8b9d0a adds the bench-record command, metrics and raw references; the raw sha256 values match this certificate's artifact_set, and the records stay open with uploaded: false. "
   "a9907c9254 points the leaderboard's multi-target section at the v3 ic5 rows, labelled 'validated' once benchmarks/20261111-mt-fieldops-v3/VALIDATION.json exists. That directory does not exist yet. "
   "Under A-F1 this report does not support a 'validated' vs-rho row: status is insufficient_evidence."),
   evidence="git log 70d0b40bc3..a9907c9254"),
  dict(id="A-F12", severity="note", check="scope", finding="The working tree was dirty during validation (benchmarks/records modified, uncommitted). Every check reads a git-archive copy of 70d0b40bc3, and the study directories in the working tree were identical to HEAD. research-kb was not available in this container (no executable), so the standard-rho literature points rest on the validator's own analysis and on the repository's ecbench documentation.", evidence="git status; scripts/research-kb")],
 required_amendments=[
  "BLOCKING: drop 'the strongest rho' from the title, headline, section-5a bullet and Q1 answer. Replace it with 'the cheapest of 12 pre-registered runs of this harness's anchored, fixed-database rho family (no retention of solved walks, no distinguished points)'.",
  "BLOCKING: state next to the ic5/rhobest ratio that a standard multi-target rho is cheaper. With Kuhn-Struik retention added to rho2x (validator, 1056 runs), ic5 is 1.07x-1.34x that rho at h = 0 and 1.05x-1.18x at h = 0.1, cheaper only at h = 1 (0.91-0.98), so the 5a decision is unranked in all 36 (cell, cI). A distinguished-point rho is estimated (not measured) to be cheaper at h = 1 as well. Do not report a crossover against rho at T = 1024 (Q1 answer) or anywhere else until a Kuhn-Struik/DP rho arm (e.g. ecbench rho.multitarget_pooled, or retention in this harness) is measured under the frozen rule.",
  "Report the residual reference overheads (restart cost 2.2-3.0%, repeated lookups 1.5-3.5%, phase-1 merge waste), and say that removing the first two leaves 30 cheaper and 6 unranked even against the harness family.",
  "Amend A1: the fix does not make every rho run cheaper or equal (3.5%-15.7% of Study B runs are dearer, by up to 4.3%), and chained starts repeat a walk with probability 1/64 per restart.",
  "Headline the validated improvement: ic5 cheaper than ic4 (the multi-target IC baseline) in all 36 (cell, cI), 0.47-0.99. Class it engineering under AGENTS.md section 3, and keep the bsgs2 ratio (1.11-1.49, dearer everywhere) as the distance remaining to the best generic method in the harness.",
  "State that ic5's lead over the harness rho is generic: the IC-specific phases are 0.08%-2.5% of its cost, and a known-log k-sum table would cost the same.",
  "Correct '61 relations' to 48-111 (median 53.5) and give the cell range for the trials check. Label 'Why ic5 wins' as interpretation.",
  "Correct 'rows.jsonl (artifact store)': push rows.jsonl (and tune/rows.jsonl) through tools/artifacts with a .artifacts.json manifest. Commit the bench record with its command, raw hashes and metrics, and the benchmarks/ entry.",
  "Add one line saying ECC2K-130 / m = 83 is not engaged."])
FB = dict(
 status="insufficient_evidence",
 verdict="valid_with_amendments",
 disposition="insufficient_evidence",
 claim_scope="Correction of the rho-family restart defect and rerun of the 1584 v2 rho jobs; v2 IC4 vs rhobest (cheapest of the 12 harness rho runs), each rho arm and bsgs2 on the merged rows; v2 analyze.py rules",
 scoped_dispositions={
  "the restart defect and its fix (chained starts) in examples/mt_fieldops.rs": "validated: the diff is exactly the five restart hunks; log bookkeeping verified on every target; ic4, ic5 and bsgs2 untouched (the 528 v2 ic4/bsgs2 rows replay exactly with the fixed source)",
  "corrected numbers (table, 1.4%-4.8%, 0.951-0.991, 0.9% at 25 b cI 30 h 1, three T = 4096 decisions unranked -> dearer)": "validated: 324/324 summary entries, the 6-row table and every prose number recompute exactly",
  "'the v2 headline survives: IC4 still cheaper than the cheapest of four rho arms at T = 16384'": "reproduced against the harness family only; not robust: with the restart cost and repeated lookups removed, 2 of 6 T = 16384 (cell, cI) stay cheaper; with Kuhn-Struik retention (measured) all 6 become unranked (IC4 1.12x-1.17x at h = 0)",
  "'Consequences' (claim superseded, leaderboard and benchmarks README carry the corrected range)": "not done at 70d0b40bc3: the leaderboard still shows the pre-fix v2 numbers as validated"},
 checks={"freeze_order": True, "fix_correct": True, "fix_scope_rho_only": True, "replay_all_rows": True, "replay_pristine_source_sample": True,
         "merge_exact": True, "same_jobs_as_v2": True, "scalars_verified": True, "summary_json_recomputed": True, "results_md_table": True,
         "results_prose_numbers": True, "consequences_applied_at_commit": False, "rho_reference_strongest_standard": False, "artifact_policy": False},
 sensitivity={k: {kk: vv for kk, vv in v.items() if kk != "entries"} for k, v in sB["readings"].items()},
 findings=[
  dict(id="B-F1", severity="note", check="the fix", finding=(
   "The defect is as described. Restarts drawn as sums of a fixed 64-point bank allow only C(66, 3) = 45,760 phase-1 walks (about 1.46M points) and 2,080 starts per target. The fix (chained starts s <- s + B_k, tp <- tp + B_k from Q) keeps the log bookkeeping right: every target of all 1584 rerun rows verifies, and 36 dumped runs (258,048 targets) were verified independently. The diff f71b42cc4f..75dbc70b36 of examples/mt_fieldops.rs is exactly the five restart hunks, so ic4, ic5 and bsgs2 are untouched. All 528 v2 ic4/bsgs2 rows, made by the v2 binary (bbcc68cc), replay exactly from the fixed source. That justifies the merge."),
   evidence="git diff; validator out/dbg_b_merged.jsonl, out/eccheck_b.json"),
  dict(id="B-F2", severity="note", check="recompute", finding=(
   "Every number recomputes exactly from runs.json and merged.jsonl: 324/324 summary entries; the 6-row RESULTS table (v2 and corrected ranges, decisions); rhobest cost reduction 1.41%-4.75%; corrected T = 16384 medians 0.951-0.991; 25 b, cI = 30, h = 1 median 0.9908 [0.9861, 0.9927]; decisions moved 24b/4096/cI30, 25b/4096/cI10, 25b/4096/cI30 (unranked -> dearer). merged.jsonl is byte-exact: the v2 ic4/bsgs2 lines plus rows.jsonl. The new rho jobs have the same argv as the v2 jobs."),
   evidence="validator out/recompute_b.json, out/results_check_b.json"),
  dict(id="B-F3", severity="amendment", check="reporting (consequences stated as done)", finding=(
   "'Consequences' says claim PCC-20261111-fa13c8 is superseded by 'the claim recorded with this correction', and that the leaderboard component section and benchmarks/20261111-mt-fieldops-ic4/README.md carry the corrected range. At 70d0b40bc3 none of this is true. No new pcc claim exists. docs/ic/LEADERBOARD.md and the benchmarks README were last changed in 4fbb181006, before the fix, and still show the pre-fix IC4/rho ratios (24 b, T = 16384: 0.933) as 'validated'. The session log lists these as open threads. Either apply them in the same PR or reword them as pending."),
   evidence="git log -- docs/ic/LEADERBOARD.md benchmarks/20261111-mt-fieldops-ic4/README.md pcc/claims"),
  dict(id="B-F4", severity="blocking", check="5 fairness (reference strength)", finding=(
   "'The v2 headline survives' holds only against the harness's anchored fixed-database rho family. "
   "Residual overheads after the fix, measured: restart cost 2.2%-2.3% of rho2x's cost, repeated lookups 2.1%-3.1%. With both removed, IC4 is cheaper at T = 16384 only at cI = 3 (2 of 6 (cell, cI)); 24 and 25 b at cI = 10 become unranked and at cI = 30 dearer. "
   "With Kuhn-Struik retention added to rho2x (validator rho2xks, 528 runs on the same 132 configurations, every target verified by the counted check), IC4 at T = 16384 is 1.131 and 1.157 at h = 0 and 1.092 and 1.109 at h = 0.1 (cI = 10; every lower bound > 1). It is cheaper only at h = 1 (0.976, 0.984), so all six T = 16384 decisions become unranked and the other 12 are dearer. At h = 1 a distinguished-point Kuhn-Struik rho is estimated (model, not measured) to cost 1/1.46 to 1/1.31 of IC4 at T = 16384 (IC4/DP-rho 1.31-1.46; 1.31-2.20 over all cells). "
   "The correction is valid as an accounting correction of the reference, but the claim that IC4 beats rho at T = 16384, now on the leaderboard, is not supported against a standard multi-target rho."),
   evidence="validator out/sens_b.json, out/ks_b_rows.jsonl, out/dp_estimate_b.json"),
  dict(id="B-F5", severity="amendment", check="reporting (wording)", finding=(
   "'(rhobest cost, per cell, c_I and h)' is correct at the level of the rhobest median. Per run the fix is not monotone: 96.5% (rho), 95.5% (rho2), 95.7% (rho2x) and 84.3% (rhoseq) of runs became cheaper, and some became dearer by up to 4.3%. The PROTOCOL's 'unboundedly many distinct starts' is right, but the chained starts repeat the previous walk with probability 1/64 per restart (Study A, A-F2b). At kappa 1.0 phase 1 still takes 1.18-1.26 steps per new point for rho (ideal about 1.0)."),
   evidence="validator out/results_check_b.json"),
  dict(id="B-F6", severity="process", check="artifact policy / bench record", finding=(
   "results/merged.jsonl (1,446,495 bytes) and results/rows.jsonl (1,055,444 bytes) are committed to Git, against AGENTS.md section 13. The bench record 20261111T0800Z-mt-rho-start-fix is open at 70d0b40bc3 with no raw entries or metrics. It does record the command (run.py, started 08:01:01Z, 133.7 s, exit 0, commit_dirty false). The run was on the local host, not ecbench and not isolab-eks."),
   evidence="git ls-files; frozen benchmarks/records")],
 required_amendments=[
  "BLOCKING: reword 'The v2 headline survives' to 'IC4 stays cheaper than the cheapest of the 12 runs of this harness's anchored, fixed-database rho family at T = 16384'. Add that the result is not robust: removing the residual restart cost and repeated lookups leaves only cI = 3 cheaper, and with Kuhn-Struik retention (validator, measured) IC4 is 1.12x-1.17x dearer at h = 0 and the decision is unranked. Correct claim PCC-20261111-fa13c8, the leaderboard section and the benchmarks README to this before citing the v2 row as 'IC4 cheaper than rho'.",
  "Either apply the 'Consequences' (new pcc claim, leaderboard and benchmarks README corrected) in the same PR, or reword them as pending. At 70d0b40bc3 they are not done, and the leaderboard still shows the pre-fix 0.91-0.95 numbers as validated.",
  "Say that the fix is not monotone per run (up to +4.3% on 3.5%-15.7% of runs) and that chained starts repeat a walk with probability 1/64 per restart.",
  "Move merged.jsonl and rows.jsonl to the artifact store with a manifest, and close the bench record with raw hashes."])
json.dump(FA, open(f"{V}/findings_A.json", "w"), indent=1); json.dump(FB, open(f"{V}/findings_B.json", "w"), indent=1)
print("ok")
