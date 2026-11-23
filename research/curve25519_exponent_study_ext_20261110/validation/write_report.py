#!/usr/bin/env python3
"""Assemble ../validator-report.json from validation/recompute.json (written by validate.py) and the
validator's findings. Run after validate.py: python3 -I validation/write_report.py"""
import hashlib
import json
import os
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.normpath(os.path.join(HERE, ".."))
R = json.load(open(os.path.join(HERE, "recompute.json")))


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


F = R["fits"]
f20, f22, f4 = F["k20_m3"], F["k22_m3"], F["k20_m4"]
TL = R["timeline"]
CR = R["chain_rule"]
IC = R["independent_curve"]
B = R["balanced_total"]


def iv(f, lo="lo95", hi="hi95"):
    return f"{f['slope']:.4f} [{f[lo]:.4f}, {f[hi]:.4f}]"


hashes = R["artifact_hashes_sha256"]
raw = [{"path": p, "sha256": h} for p, h in sorted(hashes.items()) if p.startswith(("cells/", "data/"))]
art = {p: h for p, h in sorted(hashes.items()) if not p.startswith(("cells/", "data/"))}

headline = []
notes = {
    "k20 m3 odd d>=9 lo (README -8.76)": "README value follows the 3-decimal t table (4.303 at 2 df; table value -8.7557); "
    "exact t_0.975,2 = 4.30265 gives -8.7546, i.e. -8.75 (erratum E2)",
    "k22 m3 d>=9 hi (README 10.28)": "README value follows the 3-decimal t table (2.776 at 4 df; table value 10.2845); "
    "exact t_0.975,4 = 2.77645 gives 10.2855, i.e. 10.29 (erratum E2)",
    "parent k20 m3 d>=9 lo (README -8.75)": "same 4-cell fit as the 'odd d >= 9' reading; README quotes it as -8.75 here and -8.76 there (E2)",
    "ratio d12/d9 (5x)": "5.25x", "ratio d18/d19 (110x)": "110.1x", "ratio d14/d12 (same)": "1.0023 (0.23 % dearer)",
    "ratio d28/d27 (0.77x)": "0.7653",
}
for c in R["readme_claims"]:
    h = {"name": c["name"], "claimed": c["claimed"], "recomputed": round(c["recomputed"], 6), "abs_tol": c["tol"],
         "agrees": c["agrees"]}
    if c["name"] in notes:
        h["note"] = notes[c["name"]]
    headline.append(h)

cells_ev = "; ".join(f"{n}: {v['started']} + {v['wall_s']} s -> run.log {v['log_time'][11:19]}Z, added {v['add_commit']} {v['add_commit_time']}"
                     for n, v in TL["cells"].items())
gaps = TL["gaps_between_cells_s_and_load_continuity"]

checks = [
    {"id": "C1", "requirement": "freeze order: PROTOCOL.md and scripts/run_ext.py frozen at 0afe01c59d; post-freeze fix 4424e037d8 "
     "touches only the hash-line selection; every cell committed after both",
     "result": "pass",
     "evidence": f"git (read-only) on {R['validated_sha']}: 0afe01c59d ({R['freeze']['freeze_time']}) adds PROTOCOL.md, "
     "data/frozen.sha256, scripts/run_ext.py. 4424e037d8 (" + R["freeze"]["fix_time"] + ") changes only scripts/run_ext.py, one "
     "statement (1 line removed, 2 added): `want` now takes the frozen.sha256 line ending in 'curve25519_family' instead of the "
     "first token of the file. PROTOCOL.md and frozen.sha256 are unchanged from 0afe01c59d to the validated commit; run_ext.py "
     "is unchanged from 4424e037d8 to the validated commit. All 8 cell files were added once and never modified "
     "(dff753bac0 2026-11-10T15:41:38Z to d1e2cde524 2026-11-12T03:14:08Z), all after the fix. The first cell started "
     "15:39:43Z, 4 s after the fix commit. run.log line 1 is the refused pre-fix launch (binary compared with the PROTOCOL.md hash "
     "95ca6954...).",
     "notes": "scripts/analyse_ext.py first committed in ab932fd654 (2026-11-12T00:34:21Z, 7 of 8 cells done) and modified in "
     "723609651a (03:15:21Z) after the last cell (erratum E8). The 2026-11-10 session log says 'Runner launched 15:44Z'; the "
     "records say 15:39:43Z."},
    {"id": "C2", "requirement": "protocol vs runner vs run log: cell lists, limits (12 GB, 10,800 s), seeds, order, chain rule",
     "result": "pass with one deviation (E1)",
     "evidence": f"run_ext.py constants read with ast (not executed): CAP {R['runner']['CAP']}, MEM_KB {R['runner']['MEM_KB']}, "
     f"N {R['runner']['N']}, SEED {R['runner']['SEED']}; command `ulimit -v {{MEM_KB}}; RAYON_NUM_THREADS=1 {{BIN}} f4sub "
     "{k} sub {d} 0 mont {m} sym {N} {SEED + 1000 * m + d} {CAP}`. p - 1 = 1,048,572 = 2^2 * 3^3 * 7 * 19 * 73; the even "
     f"divisors in (9, 57) are {R['runner']['even_divisors_9_to_57']} and in (9, 19) {R['runner']['even_divisors_9_to_19']}, "
     "as listed. CHAINS = " + json.dumps(R["runner"]["CHAINS"]) + " (the m = 3 list stops at 133, the protocol says "
     "'...'; immaterial). Execution order in run.log (k20 m3 12, 14, 18; k20 m4 12; k20 m3 28, 36; k20 m4 19; k22 m3 55) is the "
     "protocol's chain order. Union chain rule simulated on parent + extension cells: k20 m3 should run "
     f"{CR['k20_m3']['union_rule_should_run']} (ran {CR['k20_m3']['ran']}); k22 m3 {CR['k22_m3']['union_rule_should_run']} "
     f"(ran {CR['k22_m3']['ran']}); k20 m4 {CR['k20_m4']['union_rule_should_run']} (ran {CR['k20_m4']['ran']}): d = 19 ran "
     "outside the union rule because the runner keeps chains 2 and 4 as separate lists.",
     "notes": "No parent-complete cell was rerun. Replaced parent cells (union rule): (20, 4, 19) and (22, 3, 55); the parent's "
     "censored d = 57 is kept as the parent's. The producer's analysis.json lists the same replacements."},
    {"id": "C3", "requirement": "binary identity: archive sha256 = frozen line = every cell's ext.bin_sha256",
     "result": "pass",
     "evidence": f"sha256({R['binary']['path']}) = {R['binary']['sha256']}; data/frozen.sha256 line ending "
     "'curve25519_family' has the same value; all 8 cells' ext.bin_sha256 equal it. data/frozen.sha256 is byte-identical to the "
     f"parent's (sha256 {R['binary']['frozen_sha256_file_hash'][:16]}..., equal to the parent validator's record). run.log has "
     f"{TL['binary_ok_lines']} 'binary ok' lines (launch and three relaunches) and {TL['refused_launch_lines']} refused pre-fix "
     "launch. The same file is at /home/user/ctarget-f1/release/curve25519_family (same sha256), mtime 2026-11-08T23:39:58Z."},
    {"id": "C4", "requirement": "cheap replay of k20 m3 d = 12 and d = 14 with the frozen binary",
     "result": "pass (identical)",
     "evidence": "Run by the validator at 2026-11-12T03:20:44Z and 03:21:00Z with the runner's exact command line and the "
     "recorded seeds 20264120 and 20264122 (cap 10800, ulimit -v 12000000): exit 0, 16.4 s and 18.1 s. Both JSON outputs equal "
     "the cell records in every field except per-target 'ms' (target_x, field_ops, inconsistent, solving_degree, "
     "solving_degree_max, degree_reached, steps, max_rows, max_cols, basis_len, timed_out, complete, mean_field_ops, levels, "
     "v_size, n_vars, sym_terms, input_terms, input_degree, seed, cap_s). Outputs: validation/replay_k20_m3_sym_d12_g0.out, "
     "validation/replay_k20_m3_sym_d14_g0.out.",
     "notes": "The dispatch's literal command used seed 20265112 (and 20265114 for d = 14), which is 20261108 + 4004 (+ 4006), "
     "not the stated formula 20261108 + 1000 m + d = 20264120 (20264122) that the cells record. The recorded seeds were used; "
     "the literal seeds would draw other targets. replay.sh documents the command."},
    {"id": "C5", "requirement": "targets regenerated from the seed rule with independent code",
     "result": "pass",
     "evidence": f"Own family-curve code: k = 20, p = {IC['k20']['p']} (largest prime < 2^20, = 5 mod 8), A = {IC['k20']['A']} "
     f"(smallest valid A; {IC['k20']['a_candidates_counted']} candidates point-counted by a Legendre sum), #E = {IC['k20']['n']} "
     f"= 8 * {IC['k20']['r']}, twist {IC['k20']['nt']} = 4 * {IC['k20']['nt'] // 4}, G = {tuple(IC['k20']['G'])} (smallest u >= 2 "
     "of order r; equal to the parent validator's). LCG st = st * 6364136223846793005 + 1442695040888963407 mod 2^64, "
     "e = 1 + (st >> 11) mod (r - 1), target = x(eG). 24/24 complete-cell targets equal per_target.target_x; the parent's "
     f"k20 m3 d57 first target regenerates as {IC['parent_d57_first_target']['regenerated']} (recorded 610025). First targets the "
     "aborted cells would have drawn: k20 m3 d36 " + str(IC["targets"]["k20_m3_sym_d36_g0"]["first_target_x_if_run"]) +
     ", k20 m4 d12 " + str(IC["targets"]["k20_m4_sym_d12_g0"]["first_target_x_if_run"]) + ", k20 m4 d19 " +
     str(IC["targets"]["k20_m4_sym_d19_g0"]["first_target_x_if_run"]) + ", k22 m3 d55 " +
     str(IC["targets"]["k22_m3_sym_d55_g0"]["first_target_x_if_run"]) + f" (k = 22: p = {IC['k22']['p']}, A = 43186 checked, "
     f"r = {IC['k22']['r']}).",
     "notes": "At k = 22 the A conditions were checked, not its minimality (minimality was checked by the parent validator)."},
    {"id": "C6", "requirement": "the 'inconsistent' flag of all 24 completed targets, by independent arithmetic",
     "result": "pass",
     "evidence": "Own Semaev S_3 for y^2 = x^3 + a x + b (Weierstrass model of the Montgomery curve, a = "
     f"{IC['k20']['aW']}, b = {IC['k20']['bW']}): zero on 200/200 random relations P1 + P2 = P3 and non-zero on 200/200 "
     "perturbed ones; S_4 = Res_U(S_3(x1, x2, U), S_3(x3, x4, U)) at formal degree 2, zero on 200/200 random 4-term relations. "
     "For each target X and V = {A/3 + z : z^d = 1} (the 'mont' frame: Weierstrass x of Montgomery x in mu_d), no multiset "
     "{x1, x2} and x3 in V gives S_4(x1, x2, x3, X) = 0. So 24/24 systems have no solution over the algebraic closure, as "
     "recorded (inconsistent = true, basis_len = 1).",
     "notes": "Checked with summation polynomials, not by point enumeration over F_{p^2} (the parent validator's method)."},
    {"id": "C7", "requirement": "cell records: structure, writer, internal arithmetic (sabotage check)",
     "result": "pass",
     "evidence": "8/8 files re-serialise byte-identically with json.dumps(indent=1), the runner's writer. The ext block's keys and "
     "order equal the runner's (study, cap_s, mem_kb, load_before, load_after, wall_s, started_utc, bin_sha256, exit). Complete "
     "cells: top-level keys equal the parent's complete cells; per-target keys equal the parent's; mean_field_ops equals "
     "sum(field_ops)/6 exactly; targets_done 6; every target complete and not timed out; v_size = d; levels = Omega(d) "
     "(3, 2, 3, 3); n_vars = m(Omega + 1); seed = 20261108 + 1000 m + d; p = 1048573; largest per-target time 651 s (d = 28). "
     "Censored cells: key list equals the parent's crash cells (k, m, form, d_or_t, gidx, capped, failure, stderr); "
     "failure 'exit -6' = ext.exit -6; stderr 'memory allocation of 147016 / 131072 / 732376 / 524288 bytes failed' (d36, m4 d12, "
     "m4 d19, k22 d55); wall 4520.6 / 6226.4 / 1026.7 / 5285.9 s < 10,800 s. The parent's 73 cell files and analysis.json "
     "hash to the parent validator's recorded values (unchanged since that validation). run.log (9 versions) and env.txt "
     "(4 versions) are append-only across the commits.",
     "notes": "The engine's 'study' field in every cell is 'curve25519_exponent_study_20261108' (constant in the binary); the "
     "ext block names the extension. No sign of hand editing."},
    {"id": "C8", "requirement": "README section 2 cell table",
     "result": "pass",
     "evidence": "T 7.748e8 / 7.766e8 / 2.4275e10 / 2.0161e11; solving degree 7 / 10 / 7 / 11 (= solving_degree_max; "
     "solving_degree is 3 / 2 / 4 / 4); max columns 7,759 / 10,651 / 13,616 / 54,949; wall 16 / 18 / 298 / 3,693 / 4,521 / "
     "6,226 / 1,027 / 5,286 s; prime factorisations 2.2.3, 2.7, 2.3.3, 2.2.7, 2.2.3.3, 2.2.3, 19, 5.11; outcomes; per-target "
     "relative spread at most 6.1e-6 (< 1e-5 as stated); all 24 targets inconsistent. Exit codes: 0 for the 4 complete cells, "
     "-6 with an allocation failure for the 4 censored ones."},
    {"id": "C9", "requirement": "section 3 fits: preregistered gate (>= 4 complete cells, d >= 9, span >= 4x), OLS of ln T on "
     "ln |V|, two-sided 95 % t-interval with n - 2 df, on the union of cells",
     "result": "pass; two printed bounds differ by 0.01 from exact quantiles (E2)",
     "evidence": f"Own OLS and Student-t quantiles (incomplete beta, checked against the closed form at 2 df). Gate: k20 m3 "
     f"{f20['n_d_ge_9']} cells, span {f20['span_d_ge_9']:.3f} (not applicable); k20 m4 {f4['n_d_ge_9']} cell; k22 m3 "
     f"{f22['n_d_ge_9']} cells, span {f22['span_d_ge_9']:.3f} (not applicable); d = 36 would give 36/9 = 4.0. Readings (exact t): "
     f"k20 m3 d >= 9 {iv(f20['fit_d_ge_9'])}; all {iv(f20['fit_all'])} (span {f20['span_all']:.2f}); even {iv(f20['fit_even'])}; "
     f"odd d >= 9 {iv(f20['fit_odd_d_ge_9'])}; k22 m3 d >= 9 {iv(f22['fit_d_ge_9'])}; all {iv(f22['fit_all'])} "
     f"(span {f22['span_all']:.2f}); k20 m4 all cells 3, span 3.0 (gate not met; the 3-cell fit is "
     f"{B['m4_three_cell_fit']['slope']:.2f} [{B['m4_three_cell_fit']['lo95']:.1f}, {B['m4_three_cell_fit']['hi95']:.1f}]). "
     "With the 3-decimal t table the producer's analysis.json is reproduced to 4e-15.",
     "notes": "Parent comparison values: parent analysis.json k20 m3 fit_all 6.4422 [3.4904, 9.3941] (6 cells) and the parent "
     "README's d >= 9 value 5.56 [-8.75, 19.88] (4 cells) are quoted correctly."},
    {"id": "C10", "requirement": "ratios, local slopes and chain-variable counts in the Answer and section 3",
     "result": "pass with wording erratum (E7)",
     "evidence": f"T(18)/T(19) = {R['ratios']['d18_over_d19']:.2f} ('110x'); T(12)/T(9) = {R['ratios']['d12_over_d9']:.3f} "
     f"('5x'); T(14)/T(12) = {R['ratios']['d14_over_d12']:.4f} ('the same'); T(28)/T(27) = {R['ratios']['d28_over_d27']:.4f} "
     "('0.77x'). Local slopes in d order: " + ", ".join(f"{a}-{b}: {s:.3f}" for a, b, s in f20["local_slopes"]) +
     " (README rounds all correctly). Omega(19) = 1, Omega(14) = 2, Omega(12) = Omega(18) = Omega(28) = 3, equal to the cells' "
     "'levels'; n_vars = 3(Omega + 1)."},
    {"id": "C11", "requirement": "predictions P1-P6 scored against PROTOCOL.md wording",
     "result": "pass except P3 scoring (E1)",
     "evidence": "P1 not testable (d = 57 not rerun; the chain stopped at d = 36): agrees. P2 failed (4 even cells complete, "
     "but 8 cells span 3.11x < 4x, so the estimator does not apply): agrees. P3: d = 19 censored by memory after 1,026.7 s, but "
     "under the protocol's union chain rule it should not have run (k20 m4 ended at d = 12); the README scores 'passed' without "
     "saying so. P4 failed (d = 12 censored; 14, 18 not run): agrees. P5 passed (exit -6, allocation failure, 5,285.9 s): "
     "agrees. P6 passed in part: 4 'binary ok' launches and all ext.bin_sha256 equal the frozen hash; the d = 57 part is not "
     "testable; the rerun cells d19 (m4) and d55 (k22) recorded no target in either study, though by the seed rule both "
     "would draw the same first target (973646 and 1093934): agrees."},
    {"id": "C12", "requirement": "run.log, env.txt, cells' started_utc / wall_s and commit times agree",
     "result": "pass with timing erratum (E3) and host erratum (E5)",
     "evidence": "Every run.log cell line falls in the cell's [started_utc + wall_s] window (1 s truncation) and carries the "
     "same mean or 'capped'; every cell was committed after it ended: " + cells_ev + ". Gaps between consecutive cells: "
     + "; ".join(f"{a[:-3]} -> {b[:-3]} {g:.1f} s{' (load continuous)' if lc else ''}" for a, b, g, lc in gaps) +
     ". env.txt relaunch 2026-11-10T19:41:55Z = d18 start; 2026-11-11T20:14:22Z vs m4 d12 start 20:14:23Z; third restart "
     "noted at 01:44Z, d55 restarted 01:45:27Z, commit a287158bbe at 01:45:43Z. Kernels in env.txt: "
     + ", ".join(TL["env_kernels"]) + ".",
     "notes": "Inferred from wall times (field_ops are deterministic; wall time assumed comparable): the first d18 attempt "
     f"(started 15:40:17Z) would have ended by {TL['d18_first_attempt_latest_end_if_uninterrupted'][11:19]}Z and the first m4 d12 "
     f"attempt (started 19:46:53Z) by {TL['m4_d12_first_attempt_latest_end_if_uninterrupted'][11:19]}Z on 2026-11-10; neither "
     "left a file, so the runner died hours before the restart times the README lists."},
    {"id": "C13", "requirement": "load averages quoted in section 5",
     "result": "pass with omission (E4)",
     "evidence": "1-min load before each cell: d12 1.51, d14 1.40, d18 3.02, m4 d12 2.52, d28 1.00, d36 1.00, m4 d19 1.00, "
     "k22 d55 4.60; after: 1.40, 1.28, 1.04, 1.00, 1.00, 1.00, 1.00, 1.00. Loads of exactly 1.0 are consistent with one "
     "runnable process for hours (fixed-point load average). load_after of each cell equals load_before of the next where "
     "the runner ran them back to back."},
    {"id": "C14", "requirement": "pcc tagging and claim status",
     "result": "pass (mechanical)",
     "evidence": "python3 tools/pcc/pcc.py lint-prose README.md: 0 findings. No pcc claim is registered; the README says every "
     "measured number is an unverified assertion."},
    {"id": "C15", "requirement": "REPORT and figure consistent with README",
     "result": "pass with cosmetic erratum (E11)",
     "evidence": "Every number in README.md occurs in REPORT.typ and in the text of REPORT.pdf (pdftotext; 3 pages, created "
     "2026-11-12T03:17:49Z). exponent_ext.png plots the union cells (parent circles, extension diamonds), censored cells as "
     "vertical lines, 'estimator not applicable' labels, no fit line."},
    {"id": "C16", "requirement": "controls and reporting contract (reference arm, certificate, pricing band, disclosures)",
     "result": "pass for the claims made; rho wording erratum (E9)",
     "evidence": "The study claims no solve, relation, closure or improvement, so no certificate is required; the verdicts stay "
     "'undetermined'. The reference quantities of the claim are the preregistered lever thresholds (0 and 0.5) and the generic "
     "exponent 1/2, all stated. No cross-method ranking, so the 0 / 0.1 / 1 GAE band does not apply; no m = 83 or ECC2K-130 "
     "material. No rho arm was measured, yet section 6 says the study is 'reported with its gap to rho' (E9)."},
]

errata = [
    {"id": "E1", "severity": "moderate",
     "location": "README section 1 (P3 row), section 2 ('not attempted, by the chain rule'), section 4; data/run.log",
     "issue": "PROTOCOL.md 'Cells and order' applies the chain rule to the union of parent and extension cells in d order, with "
     "the even cells below the parent's censored d running first (12, 14, 18 before 19 at m = 4). Under that rule the k20 m4 "
     "chain ends at d = 12 (censored, memory) and d = 19 is never attempted. The frozen runner keeps chains 2 (12, 14, 18) and 4 "
     "(19, 21, ...) as separate lists, and cell_state() looks only at the cell's own d. So it ran k20 m4 d19 (1,026.7 s) after "
     "d12 had censored. run.log prints 'chain k20 m4: censored at d=12; larger d not attempted' and then runs d19. The README "
     "scores P3 'passed' on this cell, says d = 14 and 18 were not attempted '(chain rule)', and does not disclose the "
     "protocol/runner inconsistency. P1 was scored 'not testable' under the same union rule. No fit or verdict changes "
     "(the cell censored).",
     "fix": "Disclose that the frozen runner and the protocol text disagree at m = 4. Score P3 as 'not reached under the "
     "protocol's union chain rule; the frozen runner ran d = 19 anyway (separate chain lists) and it censored by memory after "
     "1,027 s, as P3 expected'."},
    {"id": "E2", "severity": "low",
     "location": "README Answer 'Other readings' and section 3 table",
     "issue": "The intervals use 3-decimal t quantiles (4.303 at 2 df, 2.776 at 4 df, the parent's frozen analyse.py table). "
     "Exact quantiles (4.30265, 2.77645) change two printed bounds: odd d >= 9 lower bound -8.7546 (README -8.76), and "
     "k22 d >= 9 upper bound 10.2855 (README 10.28). The README also quotes the same 4-cell fit (d = 9, 19, 21, 27) as "
     "5.56 [-8.75, 19.88] (parent value, exact t) and as 5.56 [-8.76, 19.88] ('odd cells', table t). No verdict changes.",
     "fix": "State the quantile source. Quote one value for the identical fit (-8.75 with exact t, or -8.76 with the frozen "
     "table) and 10.29 or 10.28 to match."},
    {"id": "E3", "severity": "low",
     "location": "README section 5 'Three container restarts'; data/env.txt",
     "issue": "The listed times (2026-11-10 19:40Z, 2026-11-11 20:12Z, 2026-11-12 01:43Z) are when a fresh container was "
     "observed (uptime 0). They are not when the runner died. The first d18 attempt started 15:40:17Z and, at the rerun's "
     "297.7 s, would have written its cell by about 15:45Z; no file exists. So the runner died around 15:40-15:45Z, about 4 h "
     "before 19:40Z and just after the session's 15:41:38Z commit. The first k20 m4 d12 attempt started 19:46:53Z and would "
     "have aborted (memory) by about 21:31Z on 2026-11-10; it left no file either. So the runner died about 23 h before the "
     "restart observed at 20:12Z on 2026-11-11. Idle time between cells: 4.0 h (d14 to d18) and 24.5 h (d18 to m4 d12); "
     "73 min of d55 was lost. env.txt gives the third restart as 01:44Z, twice; the README says 01:43Z. The session logs "
     "attribute the restarts to idle containers being reclaimed.",
     "fix": "Give the windows in which the runner died and the idle intervals. Use env.txt's 01:44Z. Name the likely cause "
     "(the container of an idle session was reclaimed)."},
    {"id": "E4", "severity": "low",
     "location": "README section 5 'Host'",
     "issue": "'1.0 to 1.5 for most cells, 3.0 before d = 18 and 4.6 before k = 22 d = 55' leaves out k20 m4 d12, whose "
     "1-min load before the cell was 2.52.",
     "fix": "Add 2.5 before k20 m4 d12."},
    {"id": "E5", "severity": "low",
     "location": "README section 5 'Host' and PROTOCOL 'Limits and host'; data/env.txt",
     "issue": "env.txt records kernel 6.18.44-fc-v338 at the launch and at the first relaunch, and 6.18.44-fc-v378 at the "
     "second. Nothing (kernel, nproc, memory) is recorded at the third relaunch. So the container and its kernel build changed "
     "during the run, but the README says 'the same shared 4-core, 15 GB container as the parent's'. field_ops are not "
     "affected, and wall time is not claimed.",
     "fix": "Disclose the change and record the environment at every relaunch."},
    {"id": "E6", "severity": "low",
     "location": "README Answer ('before its first target finished', 'every m = 4 cell at or beyond d = 12', 'the memory of "
     "the F4 matrix'); section 1 P6 row; data/analysis.json P6 outcome",
     "issue": "The engine (src/bin/curve25519_family.rs, f4sub) prints its JSON once, at the end of a cell, and the runner "
     "discards stdout after a crash. So the records do not show how many targets finished before an abort. The failing "
     "allocation (131,072 to 732,376 bytes) does not show which structure filled the address space either. Only d = 12 and "
     "d = 19 were attempted at m = 4.",
     "fix": "Write: 'aborted with an allocation failure under ulimit -v before the cell finished (the number of finished "
     "targets is not recorded)', and 'both m = 4 cells attempted'. Label the F4-matrix attribution as an inference."},
    {"id": "E7", "severity": "low",
     "location": "README section 3 last slope sentence; Answer bullet 'Cost does not follow |V| alone'",
     "issue": "'The sign changes at each parity change of d' is false at 9 -> 12: d goes from odd to even and the slope is "
     "+5.76, the same sign as 7 -> 9. The negative slopes are 18 -> 19 and 27 -> 28 only. 'The even cells are much dearer than "
     "their odd neighbours' fails for d = 28 against d = 27 (0.77x), as the same sentence concedes. The Omega(d) effect itself "
     "is supported by an exploratory joint fit (observation 1).",
     "fix": "Say that negative local slopes occur at 18 -> 19 and 27 -> 28, and limit 'dearer than odd neighbours' to "
     "d = 12, 14 and 18."},
    {"id": "E8", "severity": "low",
     "location": "README section 4, third bullet",
     "issue": "scripts/analyse_ext.py is described as written 'while the last chain was running (7 of 8 cells done)'. That "
     "holds for its first version (ab932fd654, 2026-11-12T00:34:21Z). The 'other readings' code (the fit despite the gate and "
     "the even/odd split) was added in 723609651a at 03:15:21Z, after the final cell had been committed (d1e2cde524, "
     "03:14:08Z). The diff adds readings only; the estimator and the P1-P6 rules are unchanged.",
     "fix": "Disclose the second commit and its timing."},
    {"id": "E9", "severity": "low",
     "location": "README 'Class' paragraph ('Rho remains ahead on every track') and section 6 last bullet",
     "issue": "Section 6 calls the study 'a closure measurement on the exponent question, reported with its gap to rho as in "
     "the parent'. The README, analysis.json and REPORT give no gap-to-rho figure for any new cell. The rho sentence has no "
     "support in this study's artifacts; it is labelled [pcc:unverified]. The verdicts stay 'undetermined', so 'closure "
     "measurement' overstates what was found.",
     "fix": "Add the parent's distance-to-rho row for the new cells, with the parent's unit caveat (its E5), or drop 'reported "
     "with its gap to rho'. Replace 'closure measurement' with 'resource-limit measurement'."},
    {"id": "E10", "severity": "low",
     "location": "README section 3, balanced-total paragraph",
     "issue": "'With s >= 4.5 in every reading ... no balance below the generic 1/2 exists at either m' rests on point "
     "estimates. The lower bounds of the odd d >= 9 reading (-8.75) and the k22 d >= 9 reading (-1.27) are below the m = 3 "
     "threshold 0. The only m = 4 fit, 9.86 [-59.9, 79.6] over 3 cells, is not reported. The smallest point estimate, 4.5087, "
     "passes 4.5 by 0.009.",
     "fix": "Write 'at the point estimates' and give the intervals."},
    {"id": "E11", "severity": "low",
     "location": "exponent_ext.svg / exponent_ext.png (scripts/figure.py)",
     "issue": "Both panel titles say '12 GB / 10,800 s', but the circles are parent cells measured under 3.5 GB / 1,200 s. "
     "There is no legend entry for the m = 4 markers (the legend is drawn only in the k = 22 panel). The k22 d55 censored label "
     "is hidden under the legend. One label reads '1 cells'.",
     "fix": "Cosmetic: give the limits per origin, add the m = 4 legend entries, and move the legend."},
]

observations = [
    "1. Exploratory, not preregistered: OLS of ln T on ln d and Omega(d) over all complete cells. k20 m3 (10 cells): slope "
    f"{f20['exploratory_lnd_plus_omega']['slope_lnd']:.2f} [{f20['exploratory_lnd_plus_omega']['ci_lnd'][0]:.2f}, "
    f"{f20['exploratory_lnd_plus_omega']['ci_lnd'][1]:.2f}], Omega coefficient {f20['exploratory_lnd_plus_omega']['coef_omega']:.2f} "
    f"[{f20['exploratory_lnd_plus_omega']['ci_omega'][0]:.2f}, {f20['exploratory_lnd_plus_omega']['ci_omega'][1]:.2f}]. k22 m3 "
    f"(8 cells): {f22['exploratory_lnd_plus_omega']['slope_lnd']:.2f} [{f22['exploratory_lnd_plus_omega']['ci_lnd'][0]:.2f}, "
    f"{f22['exploratory_lnd_plus_omega']['ci_lnd'][1]:.2f}], Omega {f22['exploratory_lnd_plus_omega']['coef_omega']:.2f}. The "
    "Omega coefficient excludes 0, which supports the README's point that factorisation shape matters. The size slopes "
    "remain far above both lever thresholds.",
    "2. ulimit -v 12000000 is in KiB: 1.2288e10 bytes (11.44 GiB) of virtual address space. The host has 15 GB of RAM and "
    "no swap. Exit -6 (SIGABRT from Rust's allocation-failure handler) fits a refused mmap under ulimit; the kernel OOM killer "
    "would give -9.",
    "3. The README's 'negligible probability' that a target is consistent is loose. The parent census at k20 m3 d27 covers "
    "344 of 131,203 subgroup points (0.26 %) by sums of E-points alone, and twist points add roots. It does not matter here, "
    "because all 24 flags were verified independently (C6).",
    "4. HEAD moved during validation, from 236e925fb7 to 11af0196bc (merge of origin/main). The study's files are "
    "byte-identical at both. The merge changed 6 lines of src/bin/curve25519_family.rs; that cannot affect the archived "
    "binary. Source-to-binary binding was not proven in the parent either.",
    "5. Session-log statements that disagree with the records (outside the study directory): the 2026-11-10 log says "
    "'Runner launched 15:44Z' (records: 15:39:43Z). The 2026-11-12 log says REPORT.pdf has 6 pages (pdfinfo: 3) and that the "
    "runner was relaunched at 01:44Z (d55 started 01:45:27Z).",
    "6. Process items for the coordinator, disclosed in the README: no bench.record/v1 record, no EKS route, no Conductor "
    "task, no S3 resume checkpoint, no pcc claim. The binary is not in the artifact store; it exists only on this host (two "
    "paths), as in the parent's E12.",
    "7. The README's 'solving degree' column is the cells' solving_degree_max (7, 10, 7, 11). The records also carry "
    "solving_degree (3, 2, 4, 4). The parent uses the same convention.",
    "8. The censored cells' ext.load_before at d28, d36 and m4 d19 is exactly [1.0, 1.0, 1.0]. That is expected after hours "
    "of one runnable process.",
]

not_done = [
    "k20 m3 d18 (298 s) and d28 (3,693 s) were not replayed, under the dispatch's cost limit. Their target_x values "
    "regenerate, their flags verify and their structure checks pass, but their field_ops were not re-measured.",
    "The 4 censored cells were not rerun, so the memory-failure points were not reproduced.",
    "The pre-freeze smoke test (d = 4, m = 2, 1 target, 'about 30 s') has no artifact and was not verified.",
    "'The second check-in trigger reached the session about 20 hours late' and 'a tracked background waiter kept the "
    "session active' cannot be checked from the artifacts. They are consistent with the sub-second gaps between cells after "
    "the second relaunch.",
    "Source-to-binary binding was not proven by a rebuild; nothing was built.",
    "The minimality of A at k = 22 was not searched; only its conditions were checked.",
    "'Out of reach of this engine's dense F4 at any plausible memory' (section 6) is an extrapolation and was not tested.",
]

scripts = {f: sha(os.path.join(HERE, f)) for f in ("validate.py", "write_report.py", "replay.sh",
                                                    "replay_k20_m3_sym_d12_g0.out", "replay_k20_m3_sym_d14_g0.out",
                                                    "recompute.json")}
scripts["note"] = ("validation/ in this study. validate.py recomputes everything from the raw files without importing or "
                   "calling the producer's scripts; replay.sh documents the two replays; write_report.py writes this file "
                   "from recompute.json.")

report = {
    "schema": "validator-report/v1",
    "study": "research/curve25519_exponent_study_ext_20261110",
    "validated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%MZ"),
    "validator": "validator subagent (independent review; did not produce the study)",
    "independent": True,
    "status": "pass_with_errata",
    "disposition": "validated",
    "summary": (
        "The freeze order holds. PROTOCOL.md and the runner were frozen at 0afe01c59d; the only later change is the "
        "one-statement hash-line fix 4424e037d8, made before any cell; all 8 cell files were added once, afterwards. The "
        "archived binary matches the frozen hash and every cell's record. A replay of k20 m3 d = 12 and d = 14 reproduces "
        "both cells exactly, apart from timing. Independent code regenerates all 24 completed targets from the seeds and "
        "confirms all 24 'inconsistent' flags with its own summation polynomials. The cell table, ratios, local slopes, "
        "chain-variable counts and fits recompute. With exact t quantiles two printed bounds move by 0.01, because the "
        "producer used the parent's 3-decimal table (E2). The 'undetermined' verdicts at m = 3 and m = 4 follow from the "
        "protocol, and the four memory censorings (exit -6, allocation failure, all below 10,800 s) are as recorded. One "
        "moderate erratum: the frozen runner ran k20 m4 d = 19 after d = 12 had censored, against the protocol's union chain "
        "rule, and the README scores P3 'passed' on that cell without disclosing it (E1). Errata E2-E11 cover rounding, "
        "restart timing, load and host disclosure, unverifiable wording about when the cells aborted, the parity narrative, "
        "a post-data edit of the analysis script, the rho/closure wording, the balanced-total qualifier and the figure. None "
        "changes a measured number or a preregistered verdict."),
    "commit": {
        "branch": "claude/exponent-ext-20261110",
        "validated_sha": R["validated_sha"],
        "head_at_validation": R["head_at_run"],
        "study_diff_validated_sha_to_head": R["study_diff_validated_to_head"] or "none (study files byte-identical)",
        "study_commits": ["0afe01c59d (freeze)", "4424e037d8 (pre-data runner fix)", "dff753bac0", "87c6196920", "efa948ae52",
                          "61c2da358a", "f83d2a772d", "bf1c72e912", "25cd56421e", "ab932fd654", "908e807205", "6ad8f4d977",
                          "a287158bbe", "9222ef7c2c", "d1e2cde524", "723609651a", "236e925fb7"],
        "protocol_freeze_commit": "0afe01c59d (2026-11-10T15:38:04Z)",
        "git_use": "read-only (rev-parse, log, show, diff, ls-tree, status); no commits, no pushes",
        "tree_state": "study files on disk equal the blobs at the validated sha; only validation/ and validator-report.json "
                      "are new (written by this validator)",
    },
    "binary": {
        "path": R["binary"]["path"], "sha256": R["binary"]["sha256"], "matches_frozen": R["binary"]["matches_frozen"],
        "mtime_utc": "2026-11-08T23:39:58Z",
        "note": "Executed only for the two replays (C4), with the runner's command line. Not copied and not rebuilt. The same "
                "file, with the same sha256, is also at /home/user/ctarget-f1/release/curve25519_family.",
    },
    "artifact_hashes_sha256": art,
    "raw_data": raw,
    "headline": headline,
    "checks": checks,
    "errata": errata,
    "observations": observations,
    "not_done": not_done,
    "validator_scripts": scripts,
}
json.dump(report, open(os.path.join(STUDY, "validator-report.json"), "w"), indent=1)
open(os.path.join(STUDY, "validator-report.json"), "a").write("\n")
print("wrote validator-report.json:", len(checks), "checks,", len(errata), "errata,",
      sum(1 for h in headline if not h["agrees"]), "headline disagreements")
