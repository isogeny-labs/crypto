> **PROVISIONAL, legacy Python, NOT FOR MERGE: violates the repo's no-Python rule; kept as labelled historical evidence; matched native re-measurement required before any claim.**

## Provisional status (added 2026-10-26, after the run; the rest of this file is the text `summarize.py` generated)

- **The run was not halted early.** All 48 scheduled attempts ran (24 per arm, 12 targets x 2 arms x 2
  formulations), 0 skipped, 0 timeouts. The 4500 s CPU cap was not reached: 1360.8 s of attempt CPU
  (22.7 min) was used, in about 23 minutes of wall time, 01:18 to 01:41 UTC. The coordinator's instruction to stop Python research
  compute (the repository's "Implementation language: no Python" rule) reached me while I was waiting; on checking, the run had already finished by itself at 01:41:00 UTC.
- **Not run because of that instruction:** the optional n = 19 block (2 phase-0 + 2 random-phase planted targets,
  300 s, both arms), any repeat trial, and the PDF (`report.pdf`). The n = 19 command is not recorded as a next step
  here; the next step is a native re-measurement (see the PR body).
- **Scripts run once after the run, both quick, no solver:** `verify.py` (passed: 48 of 48 attempts present, 32
  relations re-verified on the curve, 16 UNSAT answers matched by exhaustive pair matching, 0 problems) and
  `summarize.py --no-pdf`.
- **Manual edits to the generated text below:** this status block and the banner were added, the file was renamed from
  `RESULTS.md`, and the last bullet of "Figures and sources" (which promised `report.pdf`) was replaced.

Measured, per arm, legacy Python harness, one trial, CPU seconds of the arm's attempts (wall in the tables below):

| arm | attempts | verified | UNSAT | timeout | independent rank (of 5) | CPU s | CPU s / new independent relation |
|---|---|---|---|---|---|---|---|
| A, `subgroup_xor` | 12 | 4 | 8 | 0 | 4 | 204.9 | 51.2 |
| B, `subgroup_xor` | 12 | 12 | 0 | 0 | 5 | 412.4 | 82.5 |
| A, `line_subgroup` | 12 | 4 | 8 | 0 | 4 | 273.2 | 68.3 |
| B, `line_subgroup` | 12 | 12 | 0 | 0 | 5 | 470.3 | 94.1 |
| A, both forms pooled | 24 | 8 | 16 | 0 | 4 | 478.1 | 119.5 |
| B, both forms pooled | 24 | 24 | 0 | 0 | 5 | 882.7 | 176.5 |

Observations from these numbers (labelled as such, no inference at this sample size):

- Arm A proved UNSAT on all 8 targets outside its search space (sets a and c), per formulation, and verified all 4
  phase-0 planted targets. Arm B verified all 12 targets in both formulations.
- With every attempt charged, arm B's cost per newly independent relation is 1.61 (`subgroup_xor`) and 1.38
  (`line_subgroup`) times arm A's, on both clocks. The protocol's H1 criterion is therefore not met on this sample.
- Where both arms verified the same phase-0 planted target, arm B took more CPU time than arm A in all 8 pairs,
  by factors from 2.2 to 98.7 (a 0.5 s arm-A solve sits behind the 98.7).
- Target order in the schedule is fixed (set a, then b, then c), so the "cost when each rank was first reached" table
  depends on that order (arm A's first relation comes after its four set-a UNSAT proofs). The all-attempts totals do not.
- Arm A's rank stopped at 4 of 5 because only the four phase-0 targets are decomposable in its search space.
  Arm B reached 5 after 7 attempts (`subgroup_xor`) and 5 attempts (`line_subgroup`); its later relations were dependent.

# Phase-aware Frobenius summation: paired fixed-budget comparison, n = 13

Stage diagnostic on a toy field. `S`, end-to-end cost and speedup are unset.
No ECDLP speedup is claimed, and nothing here transfers to another size.
Protocol: [`PROTOCOL.md`](PROTOCOL.md) (registered 2026-10-11, addendum 2026-10-26).
Related draft PR #1630 (`PHASE_EXPERIMENTS.md`) asks for exactly this
controlled comparison; this directory does not depend on its branch.

## What ran

- Curve `y^2 + xy = x^3 + 1` over `F_(2^13)`, odd subgroup order r = 2003
  (prime), lambda = 89 (root of `lambda^2 + lambda + 2` mod r with `pi(G) = [lambda]G`).
- Factor space: seeded 4-dimensional subspace of `ker Tr` with
  5 representatives (65 admissible x once
  Frobenius phases are free). Basis `0x16d1, 0x19a, 0x15a6, 0x7df`; the same space as `../README.md`.
- 12 frozen targets (a: 4, b: 4, c: 4);
  manifest SHA-256 `8cb7f7989e5ea35656b34e7ba8bf7104e0c081277b8dbbd6b0f7507369b01231`.
- Arms: **A** representatives only; **B** one-hot Frobenius phases. Formulations: `subgroup_xor`, `line_subgroup`.
  Per attempt: 120 s solver CPU, one CryptoMiniSat thread
  (pycryptosat 5.16.0), one process, ABBA order, fresh solver per attempt.
- Host: Intel(R) Xeon(R) Processor @ 2.10GHz, 4 cores shared with other jobs, Linux-6.18.44-fc-v153-x86_64-with-glibc2.39, Python 3.13.16.
  Load average at start 2.07. Wall seconds include any contention;
  CPU seconds are the solver clock.
- Command: `/usr/bin/python3 run_paired.py --out results/paired_n13.jsonl --budget 120 --forms subgroup_xor,line_subgroup --cpu-cap 4500`
- Attempts run: 48 of 48 scheduled;
  skipped by the CPU cap: 0. Total attempt CPU: 1361 s (22.7 min).
- Audit: `verify.py` PASSED: 32 relations checked on the curve,
  16 UNSAT answers checked against exhaustive pair matching,
  0 problems.

## Structural fact (brute force, no solver)

Of the 12 frozen targets, **4** are decomposable as a signed sum of four distinct
representatives (arm A's search space) and **12** as a signed sum of four distinct admissible
x-coordinates once phases are free (arm B's). Source: `witness_sealed.json`, read only by `verify.py`.
Arm B's effective factor base is **13x larger** than arm A's (65 against 5 x-coordinates)
while the relation matrix keeps the same 5 columns. That is the claimed mechanism: equal linear
algebra, larger decomposable set. It is also why arm A can only ever return relations on the phase-0
planted targets here, and why a comparison that only counts solved targets favours arm B before any
timing is read.

## Headline: seconds per newly independent verified relation

Every attempt of the arm is charged (verified, UNSAT, timeout). "New independent" is incremental rank of the
coefficient rows over Z/2003 in receipt order; rank cannot exceed 5.

| scope | arm | attempts | verified | UNSAT | timeout | skipped | rejected tuples | rank | dependent verified | wall s | CPU s | wall s / new indep. | CPU s / new indep. | verified / attempt | wall s / attempt |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| subgroup_xor | A | 12 | 4 | 8 | 0 | 0 | 0 | 4 / 5 | 0 | 207.4 | 204.9 | 51.9 | 51.2 | 0.333 | 17.3 |
| subgroup_xor | B | 12 | 12 | 0 | 0 | 0 | 0 | 5 / 5 | 7 | 417.7 | 412.4 | 83.5 | 82.5 | 1.000 | 34.8 |
| line_subgroup | A | 12 | 4 | 8 | 0 | 0 | 0 | 4 / 5 | 0 | 276.7 | 273.2 | 69.2 | 68.3 | 0.333 | 23.1 |
| line_subgroup | B | 12 | 12 | 0 | 0 | 0 | 0 | 5 / 5 | 7 | 476.6 | 470.3 | 95.3 | 94.1 | 1.000 | 39.7 |
| both forms pooled | A | 24 | 8 | 16 | 0 | 0 | 0 | 4 / 5 | 4 | 484.1 | 478.1 | 121.0 | 119.5 | 0.333 | 20.2 |
| both forms pooled | B | 24 | 24 | 0 | 0 | 0 | 0 | 5 / 5 | 19 | 894.3 | 882.7 | 178.9 | 176.5 | 1.000 | 37.3 |

H1 criterion (PROTOCOL.md) **not met** on this sample (H0 is not rejected).

- `subgroup_xor`: B / A seconds per newly independent relation = 1.61 (wall), 1.61 (CPU); B below A on wall: no, on CPU: no.
- `line_subgroup`: B / A seconds per newly independent relation = 1.38 (wall), 1.38 (CPU); B below A on wall: no, on CPU: no.
- `subgroup_xor`: targets that arm A proved UNSAT and arm B decomposed: 8 (a0, a1, a2, a3, c0, c1, c2, c3).
- `line_subgroup`: targets that arm A proved UNSAT and arm B decomposed: 8 (a0, a1, a2, a3, c0, c1, c2, c3).

Rank is capped at 5, so attempts made after an arm reaches full rank still add cost and no rank;
the table below gives the cost charged when each rank was first reached.

## Cost charged when each rank was first reached

| scope | arm | rank reached | attempts charged | wall s | CPU s |
|---|---|---|---|---|---|
| subgroup_xor | A | 1 | 5 | 71.4 | 70.5 |
| subgroup_xor | A | 2 | 6 | 75.2 | 74.2 |
| subgroup_xor | A | 3 | 7 | 77.7 | 76.7 |
| subgroup_xor | A | 4 | 8 | 80.4 | 79.4 |
| subgroup_xor | B | 1 | 1 | 21.4 | 21.1 |
| subgroup_xor | B | 2 | 2 | 35.9 | 35.5 |
| subgroup_xor | B | 3 | 3 | 75.5 | 74.5 |
| subgroup_xor | B | 4 | 4 | 115.3 | 113.9 |
| subgroup_xor | B | 5 | 7 | 212.6 | 210.1 |
| line_subgroup | A | 1 | 5 | 111.5 | 110.0 |
| line_subgroup | A | 2 | 6 | 117.8 | 116.3 |
| line_subgroup | A | 3 | 7 | 118.3 | 116.8 |
| line_subgroup | A | 4 | 8 | 141.9 | 140.1 |
| line_subgroup | B | 1 | 1 | 44.0 | 43.2 |
| line_subgroup | B | 2 | 2 | 92.4 | 90.7 |
| line_subgroup | B | 3 | 3 | 128.5 | 126.4 |
| line_subgroup | B | 4 | 4 | 155.9 | 153.6 |
| line_subgroup | B | 5 | 5 | 193.1 | 190.2 |
| pooled | A | 1 | 5 | 71.4 | 70.5 |
| pooled | A | 2 | 6 | 75.2 | 74.2 |
| pooled | A | 3 | 7 | 77.7 | 76.7 |
| pooled | A | 4 | 8 | 80.4 | 79.4 |
| pooled | B | 1 | 1 | 21.4 | 21.1 |
| pooled | B | 2 | 2 | 35.9 | 35.5 |
| pooled | B | 3 | 3 | 75.5 | 74.5 |
| pooled | B | 4 | 4 | 115.3 | 113.9 |
| pooled | B | 5 | 7 | 212.6 | 210.1 |

## By target set

Set a: planted with random Frobenius phases. b: planted with all phases 0. c: uniform random in the odd subgroup.
CPU s are totals over the attempts shown; the last column is the median per attempt.

| form | set | arm | verified / attempts | UNSAT | timeout | CPU s total | CPU s median |
|---|---|---|---|---|---|---|---|
| subgroup_xor | a | A | 0/4 | 4 | 0 | 67.3 | 17.8 |
| subgroup_xor | a | B | 4/4 | 0 | 0 | 113.9 | 30.1 |
| subgroup_xor | b | A | 4/4 | 0 | 0 | 12.1 | 2.9 |
| subgroup_xor | b | B | 4/4 | 0 | 0 | 147.3 | 34.1 |
| subgroup_xor | c | A | 0/4 | 4 | 0 | 125.5 | 28.6 |
| subgroup_xor | c | B | 4/4 | 0 | 0 | 151.3 | 40.8 |
| line_subgroup | a | A | 0/4 | 4 | 0 | 106.4 | 27.1 |
| line_subgroup | a | B | 4/4 | 0 | 0 | 153.6 | 39.5 |
| line_subgroup | b | A | 4/4 | 0 | 0 | 33.8 | 5.0 |
| line_subgroup | b | B | 4/4 | 0 | 0 | 150.4 | 41.9 |
| line_subgroup | c | A | 0/4 | 4 | 0 | 133.1 | 30.6 |
| line_subgroup | c | B | 4/4 | 0 | 0 | 166.3 | 40.5 |

## Paired attempts, same target and formulation

Last column: B / A total CPU where both arms verified.

| form | target | A status | A CPU s | B status | B CPU s | B / A CPU |
|---|---|---|---|---|---|---|
| subgroup_xor | a0 | unsat | 8.3 | verified | 21.1 | - |
| subgroup_xor | a1 | unsat | 23.4 | verified | 14.4 | - |
| subgroup_xor | a2 | unsat | 20.6 | verified | 39.0 | - |
| subgroup_xor | a3 | unsat | 15.0 | verified | 39.3 | - |
| subgroup_xor | b0 | verified | 3.2 | verified | 28.1 | 8.80 |
| subgroup_xor | b1 | verified | 3.8 | verified | 31.6 | 8.38 |
| subgroup_xor | b2 | verified | 2.5 | verified | 36.5 | 14.71 |
| subgroup_xor | b3 | verified | 2.6 | verified | 51.1 | 19.31 |
| subgroup_xor | c0 | unsat | 44.0 | verified | 42.1 | - |
| subgroup_xor | c1 | unsat | 29.3 | verified | 39.5 | - |
| subgroup_xor | c2 | unsat | 28.0 | verified | 46.3 | - |
| subgroup_xor | c3 | unsat | 24.2 | verified | 23.3 | - |
| line_subgroup | a0 | unsat | 29.2 | verified | 43.2 | - |
| line_subgroup | a1 | unsat | 25.0 | verified | 47.5 | - |
| line_subgroup | a2 | unsat | 29.1 | verified | 35.8 | - |
| line_subgroup | a3 | unsat | 23.0 | verified | 27.1 | - |
| line_subgroup | b0 | verified | 3.7 | verified | 36.7 | 9.98 |
| line_subgroup | b1 | verified | 6.3 | verified | 14.9 | 2.37 |
| line_subgroup | b2 | verified | 0.5 | verified | 47.1 | 98.74 |
| line_subgroup | b3 | verified | 23.3 | verified | 51.8 | 2.22 |
| line_subgroup | c0 | unsat | 36.0 | verified | 40.2 | - |
| line_subgroup | c1 | unsat | 20.1 | verified | 40.7 | - |
| line_subgroup | c2 | unsat | 25.1 | verified | 30.2 | - |
| line_subgroup | c3 | unsat | 51.9 | verified | 55.2 | - |

## Verified relations

Each row is a relation `log R = sum c_j log V_j` over the orbit representatives with `c_j` the sum of
`+-lambda^k` over the summands using representative j. Checks: terms recomputed on the curve, signed sum equals the
target, coefficients equal the lambda-power sums, and the log identity holds mod r (exhaustive log table).

| arm | form | target | terms | coefficients mod r | all checks | rank after (form scope) |
|---|---|---|---|---|---|---|
| B | subgroup_xor | a0 | +pi^5(V1) +pi^10(V0) +pi^1(V0) -pi^2(V1) | [1102,1999,0,0,0] | yes | 1 |
| B | subgroup_xor | a1 | +pi^7(V0) -pi^11(V2) +pi^9(V0) -pi^11(V0) | [1095,0,1981,0,0] | yes | 2 |
| B | subgroup_xor | a2 | +pi^5(V2) -pi^7(V2) +pi^9(V4) -pi^4(V1) | [0,1734,1275,0,484] | yes | 3 |
| B | subgroup_xor | a3 | +pi^11(V0) -pi^0(V4) -pi^5(V1) -pi^2(V0) | [113,95,0,0,2002] | yes | 4 |
| A | subgroup_xor | b0 | +pi^0(V1) -pi^0(V4) +pi^0(V3) +pi^0(V2) | [0,1,1,1,2002] | yes | 1 |
| B | subgroup_xor | b0 | +pi^0(V0) +pi^9(V4) -pi^4(V4) +pi^6(V0) | [1561,0,0,0,215] | yes | 4 |
| B | subgroup_xor | b1 | +pi^5(V4) +pi^7(V0) +pi^9(V4) +pi^5(V2) | [633,0,1908,0,389] | yes | 4 |
| A | subgroup_xor | b1 | +pi^0(V0) +pi^0(V1) +pi^0(V2) -pi^0(V3) | [1,1,1,2002,0] | yes | 2 |
| A | subgroup_xor | b2 | -pi^0(V1) +pi^0(V3) -pi^0(V4) -pi^0(V0) | [2002,2002,0,1,2002] | yes | 3 |
| B | subgroup_xor | b2 | -pi^4(V1) -pi^6(V3) -pi^6(V2) +pi^11(V2) | [0,1734,465,443,0] | yes | 5 |
| B | subgroup_xor | b3 | -pi^2(V3) +pi^8(V2) -pi^11(V4) -pi^1(V4) | [0,0,253,91,1892] | yes | 5 |
| A | subgroup_xor | b3 | -pi^0(V1) +pi^0(V3) -pi^0(V2) -pi^0(V4) | [0,2002,2002,1,2002] | yes | 4 |
| B | subgroup_xor | c0 | -pi^8(V2) +pi^5(V0) +pi^7(V4) +pi^6(V3) | [1908,0,1750,1560,633] | yes | 5 |
| B | subgroup_xor | c1 | +pi^7(V1) -pi^6(V1) -pi^0(V3) -pi^6(V4) | [0,1076,0,2002,443] | yes | 5 |
| B | subgroup_xor | c2 | +pi^0(V4) -pi^4(V0) +pi^10(V0) +pi^11(V0) | [766,0,0,0,1] | yes | 5 |
| B | subgroup_xor | c3 | +pi^5(V3) +pi^8(V4) +pi^8(V0) +pi^11(V1) | [253,22,0,1908,253] | yes | 5 |
| B | line_subgroup | a0 | +pi^10(V4) -pi^12(V2) -pi^5(V0) +pi^8(V1) | [95,253,45,0,1013] | yes | 1 |
| B | line_subgroup | a1 | -pi^11(V0) +pi^2(V2) +pi^10(V4) -pi^2(V3) | [1981,0,1912,91,1013] | yes | 2 |
| B | line_subgroup | a2 | -pi^8(V3) +pi^4(V3) -pi^11(V2) -pi^6(V0) | [443,0,1981,16,0] | yes | 3 |
| B | line_subgroup | a3 | -pi^9(V2) +pi^2(V3) -pi^6(V3) +pi^7(V4) | [0,0,1519,352,633] | yes | 4 |
| A | line_subgroup | b0 | +pi^0(V2) -pi^0(V4) +pi^0(V3) +pi^0(V1) | [0,1,1,1,2002] | yes | 1 |
| B | line_subgroup | b0 | -pi^6(V1) +pi^2(V1) -pi^4(V1) -pi^0(V4) | [0,83,0,0,2002] | yes | 5 |
| B | line_subgroup | b1 | +pi^0(V0) -pi^11(V2) +pi^4(V3) +pi^10(V4) | [1,0,1981,269,1013] | yes | 5 |
| A | line_subgroup | b1 | +pi^0(V2) -pi^0(V3) +pi^0(V1) +pi^0(V0) | [1,1,1,2002,0] | yes | 2 |
| A | line_subgroup | b2 | -pi^0(V1) -pi^0(V4) -pi^0(V0) +pi^0(V3) | [2002,2002,0,1,2002] | yes | 3 |
| B | line_subgroup | b2 | -pi^11(V4) -pi^6(V0) +pi^1(V1) -pi^10(V1) | [443,1079,0,0,1981] | yes | 5 |
| B | line_subgroup | b3 | -pi^8(V0) -pi^7(V3) +pi^12(V4) +pi^4(V3) | [1750,0,0,1639,1958] | yes | 5 |
| A | line_subgroup | b3 | +pi^0(V3) -pi^0(V2) -pi^0(V4) -pi^0(V1) | [0,2002,2002,1,2002] | yes | 4 |
| B | line_subgroup | c0 | +pi^0(V0) +pi^8(V3) +pi^0(V3) +pi^4(V3) | [1,0,0,523,0] | yes | 5 |
| B | line_subgroup | c1 | -pi^7(V3) -pi^3(V3) +pi^9(V2) +pi^12(V3) | [0,0,484,1412,0] | yes | 5 |
| B | line_subgroup | c2 | +pi^8(V2) +pi^7(V2) -pi^12(V0) +pi^9(V2) | [45,0,1370,0,0] | yes | 5 |
| B | line_subgroup | c3 | -pi^9(V4) +pi^0(V3) +pi^10(V1) -pi^2(V0) | [91,1013,0,1,1519] | yes | 5 |

## Limits of this evidence

- One timing trial per cell: no repeat of any attempt. Solver runs are deterministic given the clause order, so
  repetition would vary mainly the CPU clock; run-to-run spread was not measured.
- 12 targets, 2 formulation(s), one factor space, one field. The 4 + 4 + 4 target design makes arm A
  unable to produce a relation on 8 of 12 targets by construction (structural fact above).
- The relation matrix has 5 columns, so "independent relations" saturates at 5; the per-independent-relation
  figures are therefore dominated by how many attempts each arm spends before and after saturation. Both are shown.
- The targets carry no `[a]G + [b]Q` structure, so no unknown-`Q` column is modelled (PROTOCOL.md addendum 3).
- Wall time on a shared host. CPU seconds are the stated budget clock. Neither prices linear algebra, the factor-base
  build, or scalar recovery; this is a stage diagnostic, not `S`, and no rho comparison was run.
- The hypothesis rule above is applied to this sample only; with these counts it is a description, not an inference.
- The harness is Python (legacy tooling under `AGENTS.md`); the SAT backend is native CryptoMiniSat. A native rerun is a
  prerequisite for any performance claim.

## Graphs checked

`docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`, `docs/ic/leaderboard.json`: unchanged. This is a
stage diagnostic with no whole-pipeline `S`, no ratio to a boundary, and no matched rho, so it adds no point under
`AGENTS.md` section 7a. The study-local figures are new.

## Figures and sources

- `figure_n13.svg` (source: `summarize.py`, data: `results/paired_n13.jsonl`), `mechanism.svg`.
- Receipts: `results/paired_n13.jsonl`; relations: `results/relations_n13.jsonl`; rank data:
  `results/rank_n13.json`; audit: `results/verify_n13.json`.
- Code: `freeze_targets.py`, `run_paired.py`, `rank.py`, `verify.py`, `summarize.py`, `test_paired.py`;
  parent harness `../encode.py`, `../run.py`, `../brute_check.py`.
- Report PDF: not built (run wound down early on instruction); no typst, LaTeX, wkhtmltopdf or weasyprint is installed and
  `pandoc` has no PDF engine. `figure_n13.svg` and `mechanism.svg` are the visuals.

## Scaling to m = 51, 53, 83, 131

Nothing in this section is a result of this study. It lists what is already
recorded and what a harness would need. No projection is made.

Known from the parent harness (`../README.md`, `../results/`):

| field | representatives | admissible x with phases | phase model (recorded) | source |
|---|---|---|---|---|
| n = 13, s = 4 | 5 | 65 | about 3.6k variables, 2.5k AND gates; solved at 300 s | `../README.md`, this study |
| n = 19, s = 6 | 22 | 418 | about 7.5k variables, 5.4k AND gates; 0/4 planted at 1200 s for each of two forms | `../README.md` |

- Recorded fact: with phases unknown, none of 8 planted n = 19 attempts finished
  in 1200 s (about 4.9k s total per form), while the n = 13 planted targets
  finished in a median of about 50 s at 300 s. That is a lower bound from
  timeouts, not a fitted rate.
- Recorded fact: representatives only at n = 19 timed out on all 12 completed
  300 s attempts.
- Not known: the variable and gate counts of this encoding at m = 51, 53, 83
  and 131 were not computed here, and the factor-space dimension `s` that makes
  a four-summand decomposition possible at those sizes was not determined.
  `encode.py` could report them (`Model.size()`) without solving; that
  measurement was not run.
- Not known: whether phase-aware encoding helps once the solver can finish at
  all. At n = 13 both arms finish, which is the only regime compared here.

What the harness would need before any claim at those sizes:

1. A native (Rust or C++) circuit builder and runner. `AGENTS.md` requires this
   for new research comparisons; this harness is Python and the m = 83 gate in
   `AGENTS.md` section 8a also needs the frozen m = 83 curve, basis and
   subgroup manifest (`z^83 + z^45 + z^2 + z + 1`, odd part prime).
2. Moduli and subgroup data for n = 51, 53, 83, 131 in the field code
   (`verify_claims.py` carries moduli for 13, 19, 29 and 131 only), plus
   discrete-log machinery that does not tabulate the whole subgroup: the
   brute-force log table used here has `r` entries, which is impossible at
   these sizes. Relation verification would instead be the curve-sum check
   only, or a known-log planted instance.
3. Instances where the phase-aware arm can finish. At n = 19 neither arm did
   within the recorded budgets, so a paired comparison there needs longer
   budgets or a better encoding first (relative or binary phase encodings are
   planned, not implemented, in PR #1630).
4. A relation-collection design with more than one target per factor base:
   rank saturates at the number of orbit columns, so measuring seconds per
   independent relation at scale needs many targets of the form `[a]G + [b]Q`
   against a column count that grows with the factor base.
5. Full-cost accounting under `AGENTS.md` sections 2 and 8: precomputation,
   failed attempts, linear algebra, final scalar recovery, matched rho. This
   study prices one stage on toy fields.
