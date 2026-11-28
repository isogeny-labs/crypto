# XP-20261029-dg-blocked: independent validation (VAL-20261029-33a286)

Snapshot `claude/ic-dg-blocked-20261029` at `bf899979a6` (equals origin at start and end; working tree clean at start, later dirtied by another session with untracked launch/, README.md and figures/, none used as evidence: launcher receipts are read from S3); frozen protocol `cd04cfd2d1` plus `cards/RULING-P2.md` (A1). Own code under `code/`; the ecbench binary (88070375…412c) was used only for the replay. Evidence admissibility only: this authorizes nothing.

## Verdicts

| b | verdict | reason |
|--:|:--|:--|
| 24 | **fail** | fail: wall-session section 6.1 fails conditions 1, 5, 6, 7 and 9 (exit 42; l2-gate not accepted; no control-comparison.json; decision block pairs_below_level 1; identity rel diff 1.17e-5). The executions below L2 are seq 1144 (dgb-edge, 903.9 us) and seq 1145 (dgb, 913.0 us): one block (icv1-fp24-tm995-ee6c519b, target 22, round 5, reversed order, 28,031 GAE, the cheapest instance), graded L1 only because grading v3 does not admit wall time under 1 ms; dg and dg-aa in the same block took 1.270 and 1.284 ms. The counted session also fails its route: counted dgb seq 1145 (911.2 us) is an ic.pipeline execution below L2, so the exit-42 clause does not hold. The main session's 'undecidable' reading is confirmed; no row is admissible. |
| 28 | **pass_with_caveats** | pass with caveats: wall-session conditions 1-9 and A1 hold and the counted session holds under the exit-42 clause; corrected dgb/dg 0.929953 [0.926605, 0.933313], upper bound below 1, recomputed to 1e-16 of the committed block; 2,304 replays identical. Caveats are of scope and interpretation, not of the instrument: the two curves differ beyond their intervals (0.9379 vs 0.9212); one host class; the gain is a wall-clock component speed-up (counted S identical); the prediction (0.77-0.84) was missed. |
| 32 | **fail** | fail: wall-session section 6.1 fails conditions 1, 5, 6, 7 and 9 (exit 42; l2-gate not accepted; no control-comparison.json; decision block aa_pairs 287; identity rel diff 5.9e-5). The one execution below L2 is seq 1313 (dg-aa, icv1-fp32-tm81759-3ad5e8c5, target 16, round 6), graded L1 for 'L2: 3 foreign task slices on the run CPU during the solve (0 us)'; its solve (195.09 ms) is within 0.07% of dg's in the same block. The counted session holds under the exit-42 clause, but that does not rescue the wall decision. The main session's 'undecidable' reading is confirmed; no row is admissible. |

## Wall statistic, recomputed from `records.jsonl` (w = 0 decision file, `--a dg --b dgb`)

| b | ns/GAE dg | ns/GAE dgb | dgb/dg | aa_ratio dg-aa/dg | corrected dgb/dg | max rel diff vs committed |
|--:|--:|--:|:--|:--|:--|--:|
| 24 | 79.337 | 72.029 | 0.907890 [0.9055, 0.9103] | 1.000435 [0.9984, 1.0024] | **0.907495 [0.904487, 0.910513]** | 2.1e-16 |
| 28 | 73.826 | 68.665 | 0.930092 [0.9269, 0.9333] | 1.000149 [0.9991, 1.0012] | **0.929953 [0.926605, 0.933313]** | 1.2e-16 |
| 32 | 90.320 | 81.066 | 0.897543 [0.8942, 0.9009] | 1.000066 [0.9985, 1.0016] | **0.897483 [0.893867, 0.901115]** | 2.5e-16 |

24 and 32 bits are descriptive only (section 6.1 fails). The w = 0.1 and w = 1 files carry an identical wall block at every size.

Descriptive (decides nothing):

| b | identity: dg-aa→dgb vs corrected (rel diff) | edge control dgb-edge/dg | dgb/dgb-edge | target-clustered (48) linearised | stratified target bootstrap (20,000) |
|--:|:--|:--|:--|:--|:--|
| 24 | 0.907505 (1.2e-05) | 0.908144 [0.9057, 0.9106] (overlaps decision: True) | 0.999720 [0.9984, 1.0011] | [0.90509, 0.90991] | [0.90506, 0.90972] |
| 28 | 0.929953 (0.0e+00) | 0.930331 [0.9270, 0.9336] (overlaps decision: True) | 0.999743 [0.9983, 1.0012] | [0.92610, 0.93382] | [0.92703, 0.93276] |
| 32 | 0.897430 (5.9e-05) | 0.897717 [0.8944, 0.9010] (overlaps decision: True) | 0.999805 [0.9983, 1.0013] | [0.89426, 0.90072] | [0.89428, 0.90052] |

Per curve (corrected dgb/dg):

- 24: `icv1-fp24-t1973-f876d133` 0.9081 [0.9039, 0.9123]; `icv1-fp24-tm995-ee6c519b` 0.9069 [0.9026, 0.9112]
- 28: `icv1-fp28-t9805-a382b8a8` 0.9379 [0.9333, 0.9425]; `icv1-fp28-tm3463-6d2f5699` 0.9212 [0.9168, 0.9257]
- 32: `icv1-fp32-tm43175-08b04f34` 0.8982 [0.8934, 0.9031]; `icv1-fp32-tm81759-3ad5e8c5` 0.8968 [0.8913, 0.9022]

Positions (measured blocks, every size): `dg` and `dgb-edge` at {0: 144, 3: 144}; `dg-aa` and `dgb` at {1: 144, 2: 144}; 144 forward and 144 reversed blocks; `dgb` precedes `dg-aa` in 144 blocks and follows it in 144.

## Section 6.1 and A1, rechecked

| b | wall 1 exit | 2 audit | 3 counted A/A | 4 spec/build | 5 gate | 6 native A/A | 7 decision block | 8 identical work | 9 identity | A1 | counted route | counted = wall dg, dgb |
|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 24 | **fails** (42) | holds | holds | holds | **fails** | **fails** | **fails** | holds | **fails** | holds | **fails** (exit 42) | holds |
| 28 | holds (0) | holds | holds | holds | holds | holds | holds | holds | holds | holds | holds (exit 42) | holds |
| 32 | **fails** (42) | holds | holds | holds | **fails** | **fails** | **fails** | holds | **fails** | holds | holds (exit 42) | holds |

Below-L2 executions (recomputed from `records.jsonl`; equal to `l2-gate.json`):

- 24-bit wall: seq 1144 `dgb-edge` (903.9 µs) and seq 1145 `dgb` (913.0 µs), the same block (`icv1-fp24-tm995-ee6c519b`, target 22, round 5, reversed), 28,031 GAE (the cheapest instance); blocker `L2: solve too short to admit wall time`. `dg-aa` and `dg` in that block took 1.284 and 1.270 ms. 24-bit counted: seq 1145 `dgb` (911.2 µs) is an IC execution below L2, so the exit-42 clause fails.
- 32-bit wall: seq 1313 `dg-aa` (`icv1-fp32-tm81759-3ad5e8c5`, target 16, round 6), blocker `L2: 3 foreign task slices on the run CPU during the solve (0 µs)`; solve 195.09 ms against 194.96 ms for `dg` in the same block. The 32-bit counted session holds (10 rho executions L1, all short solves).
- 28 bits: no wall execution below L2; counted: 273 rho executions L1, all `solve too short` (allowed by the exit-42 clause).

A1 (packed layout): at `be2969ce`, `build_negation_folded_prime_blocked` takes `PackedPairTable` iff `p <= u32::MAX && fb.points.len() <= 1<<16` (`ic_boundary.rs` 5754-5777); `signed_points` in the record is `fb.points.len()`. Max p: 24: 12513203, 28: 180253867, 32: 3837054223; signed points: 24: 324, 28: 814, 32: 2048. Every `dgb`/`dgb-edge` record (wall and counted, warm-ups included) is in the packed regime.

Not relabelling or accounting: each `dgb` and `dgb-edge` record (336 per size, 48 warm-ups) equals the `dg` record of the same curve, target and round in recovered logarithm, total GAE, every unpriced counter, every per-phase count and native map, seed, counters, and also bitop work and factor-base hashes: 0 differences. The counted session's `dg`/`dgb` records equal the wall session's. Only wall time (and, at 32 bits, RSS 80.3 → 61.9 MB) differs.

## Replay (verified, independent)

`ecbench verify --replay-all` on copies of all six sessions on this host (Intel, 4 vCPU): 6912 of 6912 measured runs identical, 0 problems; copies equal the job SHA256SUMS and S3 originals; 8064 record seals recomputed, 0 failures.

Launcher receipts were read from their S3 originals (`runs/<job>/launcher/<id>/`), not from the evidence directory's `launch/` folders, which are untracked at `bf899979a6` (written by another session after the snapshot push; byte-equal to S3). Each receipt's spec equals the frozen spec; one pod per Job, no restart; `status.json` pod, node and exit equal `job-exit.json`. Launcher state is `succeeded`/0 for 28-bit wall and `failed`/23 for the five exit-42 sessions, as the v3 protocol anticipates.

## Matched rho (counted session, rho-neg), mean S at w = 0 / 0.1 / 1

| curve | dg = dgb | rho-neg | dgb/rho-neg (distance remaining) |
|:--|:--|:--|:--|
| `icv1-fp24-t1973-f876d133` | 13.633 / 14.286 / 20.162 | 1.0712 / 1.1679 / 2.0379 | 12.73 / 12.23 / 9.89 |
| `icv1-fp24-tm995-ee6c519b` | 13.753 / 14.389 / 20.106 | 1.0426 / 1.1360 / 1.9766 | 13.19 / 12.67 / 10.17 |
| `icv1-fp28-t9805-a382b8a8` | 20.759 / 21.623 / 29.394 | 0.9619 / 1.0552 / 1.8953 | 21.58 / 20.49 / 15.51 |
| `icv1-fp28-tm3463-6d2f5699` | 21.342 / 22.158 / 29.511 | 0.9703 / 1.0643 / 1.9097 | 21.99 / 20.82 / 15.45 |
| `icv1-fp32-tm43175-08b04f34` | 31.823 / 33.312 / 46.714 | 0.9120 / 1.0025 / 1.8175 | 34.89 / 33.23 / 25.70 |
| `icv1-fp32-tm81759-3ad5e8c5` | 31.517 / 32.988 / 46.222 | 1.0031 / 1.1028 / 1.9999 | 31.42 / 29.91 / 23.11 |

rho-neg < dg = dgb at all three weights on every curve. Strongest available: every curve has p and r prime, cofactor 1, j ∉ {0, 1728} (recomputed from a, b solved on G and targets; equals the ICV1 j), so Aut(E) = {±1}; negation is the only fold (no Frobenius fold over F_p; Frobenius discriminants have large squarefree parts). Records show `automorphisms_used` 2.

## Phase sums (measured runs, ms; descriptive)

| b | oracle_setup dg → dgb | target_PDP dg → dgb | relations dg → dgb | solve dg → dgb |
|--:|:--|:--|:--|:--|
| 24 | 262.2 → 155.6 | 228.4 → 239.1 | 743.9 → 752.4 | 1084.4 → 984.3 |
| 28 | 1769.3 → 1000.0 | 1062.3 → 1427.6 | 3800.1 → 4161.1 | 5766.4 → 5363.2 |
| 32 | 13627.2 → 6149.8 | 10998.0 → 13353.4 | 36366.6 → 38616.1 | 50848.3 → 45638.5 |

The main session's 28-bit reading (set-up 1769 → 1000 ms, target PDP 1062 → 1428 ms) is confirmed. Target PDP wall rises at every size, consistent with a slower probe on the packed table; with the smaller-than-predicted set-up saving at 24 and 28 bits, that is why every corrected ratio lies above its section 7 prediction (24: 0.81-0.87, 28: 0.77-0.84, 32: 0.78-0.85).

## Discrepancies with the main session

None in the numbers: 0.9300 [0.9266, 0.9333], the below-L2 executions (24: seq 1144 `dgb-edge`, 1145 `dgb`; 32: seq 1313 `dg-aa`, '3 foreign task slices (0 µs)'), the record-by-record identity and the phase sums all reproduce. Additions: at 24 bits the counted session also fails its route (counted `dgb` seq 1145, 911.2 µs), which VERDICT.md states and the main-session summary omits; the 28-bit per-curve ratios differ beyond their intervals.

## Before promotion

- Section 10 outputs missing at bf899979a6: no README.md with the position table and the section 6.2 position-control figures, no figure and no PDF (research-visuals) under research/ic_dg_blocked_20261029/. They must be committed before the leaderboard or benchmarks/ change. (An untracked README.md and figures/ appeared in the working tree after the snapshot push; they are uncommitted and were not reviewed.)
- The launch/ folders (requests, launcher receipts, logs) are untracked at bf899979a6; commit them in the Coordinator's snapshot (they are byte-equal to the S3 launcher receipts this validation used) or cite the S3 receipts.
- The evidence review must record which leaderboard section receives the 28-bit row (pair_table_component, or a new oracle-named component section if that section holds set-up figures only), per PROTOCOL section 10.

## Files

`validator-report.json`, `replay-certificate.json`, `VALIDATION.draft.json` (rows: the two 28-bit curves), `code/` (replay.sh, fetch_launcher.sh, dgbstats.py, dgbval.py, dgbseal.py, dgbbuild.py, gatecheck.sh, dgbmd.py), `launcher-receipts/` (S3 request.json and result.json per session), `work/` (recompute.json, seals.json, gatecheck.txt), `replay/` (audit and logs per session), `frozen/` (protocol, approval, ruling, specs, source excerpts), `SHA256SUMS`.
