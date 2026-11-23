# XP-20261029-packed-filter: PROTOCOL §10 independent validation (TASK-20261029-654f73)

**Validator report:** `validator-report.json` (sha256 `df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a`), status **pass**.  
**Replay certificate:** `replay-certificate.json` (sha256 `b357028b1c2187dc67abfbafe8999bf5f1b1a79e9c74c2471304b16e14f4d533`), status **verified**, scalars_verified **true**.  
**Decision under check:** DEC-20261029-5eb8e6, `cards/VERDICT-P32.md` sha256 `415a89d78efd3079e7e1acd3824ec390247fed11a29baabb3ff727db9c589ea3`.  
**Snapshot validated:** branch `claude/ic-packed-filter-20261029`, commit `ecc7e409a23ebbea91c6fe3bd3f1e1ab69baa7f0`, clean tree, in sync with `origin`; `PROTOCOL.md` (sha256 `703bde26…7e51`), `APPROVAL.md` (`0a35923c…6a40`) and the six specs are unchanged since the freeze commit `f870408c17`.  
**Date:** 2026-10-30. **Role:** validator (independent; own code in `code/`).

A pass says the receipts are admissible evidence and the verdict's numbers follow from them. It does not support an ECDLP claim, does not make the P32 result a counted or baseline improvement, and authorizes no promotion: that is the Coordinator's evidence review.

## Joint verdicts

| joint | result | breaking artifact sought | found |
|:--|:--|:--|:--|
| J1 decision statistic | holds | corrected upper bound ≥ 1, or a headline mismatch | none: corrected upper bound 0.963328 < 1; every J1 number reproduces |
| J2 instrument | holds | a position imbalance between dgbf and dgb-aa, or an admitted pair with a run below L2 | none: 144/144 at positions 1/2 for both; 0 records below L2 in either wall session; 0 re-grade disagreements |
| J3 charged-work identity | holds | any field difference, a counter on the wrong arm or phase, a bitop difference ≠ 384 × counter | none: 0 mismatches over 336 matched records per arm; identity holds on 672 filtered records per session, phase by phase |
| J4 counted route and band | holds | an S′ ratio off by more than its printed precision, or an order reversal | none in the claim under review; one descriptive per-curve figure is truncated in the verdict's prose (below) |
| J5 independent replay | holds | a replay that does not reproduce a scalar or log | none: 1152/1152 identical in each of the four sessions; 1344/1344 scalars verified by own arithmetic in each |
| proves-too-much control | holds | an improvement on an A/A pair or at P24 | none: both A/A intervals contain 1; P24 corrected lower bound > 1 |

Defects found: **none**. One descriptive discrepancy (not counted as a defect; the Coordinator may rule otherwise) is listed in "Discrepancies".

## What I read, and what I did not

- Raw inputs: `P{24,32}/{wall,counted}/eks-results/` and `P28/counted/eks-results/` (session files, `job-exit.json`, `audit.json`, `l2-gate.json`, `control-comparison.json`, `input-spec.json`, `build-info.json`, `binary.sha256`, `SHA256SUMS`); the frozen specs; the P28 receipts the handoff names; S3 (read-only) for the S3 copies, launcher `result.json` and `request.json`.
- Instrument definitions at `3271a25d02` via `git show`: `band.rs` (priced GAE), `record.rs` (`grade_v3`), `docs/units/BITOPS.md` (bitop/v1 prices). `compare.rs` was **not** read: the interval method was derived from the statement (linearised t on the log ratio, independent-SE combination, Welch–Satterthwaite dof) and then found to equal the in-pod instrument output to 4.2e-16.
- **Not read:** `cards/tools/`, any `analysis/` directory (P24, P28, P32), any wall field of a counted session, any rho wall time, any P28 wall wall-field. Counted sessions (and P28 wall) are loaded through `code/vcommon.py:load_counted`, which drops every wall and timing key and truncates the too-short blocker to its prefix before any other code sees the record. Directory listings showed the names of `analysis/` directories and of other sessions' scratch folders; none was opened.

## J1: decision statistic (blind re-derivation)

Method (`code/j1_decision.py`, `code/vstats.py`): pairs keyed by (curve slug, target index, round), measured rounds only, admitted when both runs are verified and at L2 or L3. ns/GAE = Σ solve_wall_ns / Σ total_gae per arm. ratio = ns/GAE(B) / ns/GAE(A). SE of ln ratio by the delta method on the four sums (sample variance, n − 1), t with n − 1 dof, interval exp(ln R ± t·SE). corrected = ratio(dgb→dgbf) / ratio(dgb→dgb-aa), SEs combined as independent, Welch–Satterthwaite dof. Student t quantile by own incomplete-beta code.

| quantity | P32 recomputed | P32 claimed | P24 recomputed | P24 claimed |
|:--|:--|:--|:--|:--|
| pairs (below level) | 288 (0) | 288 (0) | 288 (0) | 288 (0) |
| ns/GAE dgb | 84.552599 | 84.5526 | 73.846686 | — |
| ns/GAE dgbf | 81.044647 | 81.0446 | 74.120456 | — |
| ratio dgbf/dgb [ci95] | 0.958512 [0.956084, 0.960945] | 0.958512 [0.956084, 0.960945] | 1.003707 [1.001532, 1.005888] | — |
| aa_ratio dgb-aa/dgb [ci95] | 0.998337 [0.996146, 1.000534] | 0.998337 [0.996146, 1.000534] | 1.000506 [0.999012, 1.002003] | 1.000506 [0.999012, 1.002003] |
| **corrected dgbf/dgb [ci95]** | **0.960107806 [0.956898, 0.963328]** | 0.960107806 [0.956898, 0.963328] | **1.003200 [1.000564, 1.005842]** | 1.003200 [1.000564, 1.005842] |
| corrected dof | 562.64 | 562.6 | 509.34 | — |
| condition 9: dgbf/dgb-aa | 0.960107806240821 (rel. diff 0.0e+00) | 0.960107806240821 (0.0) | 1.003199518428217 (0.0e+00) | — |
| Σwall(dgbf)/Σwall(dgb-aa); GAE sums equal | 0.9601078062408209; True | = corrected | 1.0031995184282168; True | — |

Reading under PROTOCOL §6.2, recomputed: P32 corrected upper bound 0.963328 < 1 (improvement); P24 corrected lower bound 1.000564 > 1 (regression). The linear-scale interval differs in the sixth decimal (P32 corrected [0.956893, 0.963323]), so the instrument's interval is the log-scale one.

**Cross-check against the instrument.** The in-pod `control-comparison.json` is an `ecbench compare` output (`--a dgbf-edge --b dgb-aa`, aa arms dgbf-edge/dgbf). My code recomputes its `ratio_b_over_a`, `ci95`, `aa_ratio`, `aa_ci95`, `corrected_ratio`, `corrected_ci95`, `corrected_dof` and both ns/GAE with maximum relative differences of 2.3e-16 (P32) and 4.1e-16 (P24).

### Descriptive intervals (decide nothing)

| size | curve | corrected [linearised-t ci95] | target-clustered bootstrap ci95 (cluster = target; 24 per curve, 48 for both; B = 4000) | dgbf-edge/dgb (edge) |
|:--|:--|:--|:--|:--|
| P32 | `icv1-fp32-t89141-95879fe3` | 0.959215 [0.954834, 0.963617] | [0.956933, 0.961605] | 0.957679 |
| P32 | `icv1-fp32-tm68303-f902d2f0` | 0.961017 [0.956294, 0.965762] | [0.958666, 0.963704] | 0.960913 |
| P32 | both (48 clusters) | 0.960108 [0.956898, 0.963328] | [0.958482, 0.961943] | 0.959282 |
| P24 | `icv1-fp24-t217-e1fbffd2` | 1.004782 [1.001166, 1.008411] | [1.002539, 1.007038] | 1.010070 |
| P24 | `icv1-fp24-tm567-4accd0e8` | 1.001749 [0.997935, 1.005577] | [0.998090, 1.005119] | 1.000935 |
| P24 | both (48 clusters) | 1.003200 [1.000564, 1.005842] | [1.001126, 1.005349] | 1.005304 |

Per-round corrected ratio (descriptive stability):

- P32: round 1 0.964606, round 2 0.959237, round 3 0.959433, round 4 0.959079, round 5 0.958218, round 6 0.960343
- P24: round 1 1.002008, round 2 1.004454, round 3 1.000813, round 4 1.007563, round 5 1.001718, round 6 1.002954

**Position control and middle-over-edge contrasts** (PROTOCOL §6.2):

| size | edge estimate dgbf-edge/dgb [ci95] | decision corrected [ci95] (middle positions) | aa_ratio dgb-aa/dgb | dgbf/dgbf-edge | overlap |
|:--|:--|:--|:--|:--|:--|
| P32 | 0.959282 [0.956531, 0.962040] | 0.960108 [0.956898, 0.963328] | 0.998337 [0.996146, 1.000534] | 0.999197 [0.997648, 1.000749] | yes |
| P24 | 1.005304 [1.002677, 1.007938] | 1.003200 [1.000564, 1.005842] | 1.000506 [0.999012, 1.002003] | 0.998411 [0.996270, 1.000557] | yes |

**Null-object controls** (`docs/inventor-protocol.md` §3). Two null objects of the same shape as the decision pair are measured with the same code:
- the A/A pairs (identical oracle on both arms): both wall contrasts above contain 1 at P32, and the counted A/A ratios are exactly 1 at every weight;
- a label-swap null on the matched-position pair (dgb-aa, dgbf): swapping arm labels at random within each pair destroys the arm effect. Its distribution must centre on 1 and narrow like 1/√n as pairs are added, while a real effect stays put:

| P32 pairs n | null median | null 95% band | observed corrected (first n pairs) | null fraction ≤ observed |
|--:|:--|:--|:--|:--|
| 72 | 0.999818 | [0.990605, 1.009555] | 0.961942 | 0.0 |
| 144 | 0.999995 | [0.992388, 1.007595] | 0.959215 | 0.0 |
| 288 | 0.999986 | [0.994665, 1.005242] | 0.960108 | 0.0 |

The null band narrows from ±0.0095 to ±0.0053 as n goes from 72 to 288 (about 1/√n), while the observed ratio holds near 0.96 at every n and in every round. The quantity behaves as a real effect, not an artifact. At P24 the same null gives the observed 1.0032 at the 99.8th percentile, the regression the frozen rule reads.

## J2: instrument

Method (`code/j2_instrument.py`): every condition is read from the job's own files after my own `SHA256SUMS` check. Levels are re-counted from records and compared with `l2-gate.json`; each record's level is re-derived from its blocker prefixes. For the wall sessions, a partial re-grade recomputes each record's grading-v3 L1/L2 gates from its raw isolation and timing fields (foreign, steal and sibling ticks, run-queue delay, preemption, foreign task slices, memory stall, too-short, affinity, CPU moves, threads, CPU/wall, intruders). Positions and predecessors are tabulated from `seq` within each (curve, target, round) block.

| check | P32 wall | P24 wall |
|:--|:--|:--|
| 1 exit 0, single pod | True | True |
| 2 audit ok, complete, 0 problems, 1152/1152 identical | True | True |
| 3 counted A/A exactly 1 at w = 0, 0.1, 1 | True | True |
| 4 spec JSON-equal, build-info, binary sha | True | True |
| 5 l2-gate accepted, all L2/L3, no invalid outcomes | True | True |
| 6 native A/A admission file | True | True |
| records re-graded / raw-gate disagreements | 1344 / 0 | 1344 / 0 |
| level-from-blockers mismatches | 0 | 0 |
| record levels | {'L2': 1344} | {'L2': 1344} |
| algorithm seed shared by the four arms of each block | True | True |
| curves are the §3 curves; seeds 202610296b3/4, 6, 8 | True | True |

**Block positions and predecessors** (measured rounds; identical at P32 and P24):

| arm | position counts (0..3) | predecessor within block |
|:--|:--|:--|
| `dgb` | {'0': 144, '3': 144} | {'(block start)': 144, 'dgb-aa': 144} |
| `dgb-aa` | {'1': 144, '2': 144} | {'dgb': 144, 'dgbf': 144} |
| `dgbf` | {'1': 144, '2': 144} | {'dgb-aa': 144, 'dgbf-edge': 144} |
| `dgbf-edge` | {'0': 144, '3': 144} | {'(block start)': 144, 'dgbf': 144} |

Measured block orders: {'dgb dgb-aa dgbf dgbf-edge': 144, 'dgbf-edge dgbf dgb-aa dgb': 144}; blocks are four consecutive `seq` values: True. `dgbf` and `dgb-aa` each sit 144 times at position 1 and 144 at position 2, and each has 144 filtered and 144 unfiltered predecessors.

**Counted sessions** (v3 §6.1 conditions 1–6 as adapted by dg-wall §6.1; exit-42 clause):

| check | P32 counted | P24 counted | P28 counted (r2, of record) |
|:--|:--|:--|:--|
| 1 audit | True | True | True |
| 2 rho-neg-aa/rho-neg exactly 1 | True | True | True |
| 3 spec and build | True | True | True |
| 4 every ic.pipeline execution L2/L3 | True | True | True |
| 5 rho L2/L3 or too-short L1 only | True | True | True |
| 6 invalid_outcomes empty | True | True | True |
| exit-42 clause | True | True | True |
| below-L2 set from records = l2-gate.json | True | True | True |
| job exit / launcher exit | 42 / [23] | 42 / [23] | 42 / [23] |
| below_l2 (per arm) | {'rho-neg': 7, 'rho-neg-aa': 7} | {'rho-neg': 336, 'rho-neg-aa': 336} | {'rho-neg': 100, 'rho-neg-aa': 99} |

All waived executions are rho executions graded L1 whose only `L2:` blockers begin "solve too short to admit wall time". Their solve-time range is not reported, because it is rho wall time (PROTOCOL §10).

**S3 and launch binding** (read-only, `code/s3check.py`, `code/s3requests.py`, `code/c2_identity.py`): every job has exactly one pod-uid prefix, equal to `job-exit.json`'s `pod_uid`. Every file of every session streamed from S3 hashes equal to the repository copy. Every launcher `request.json` embeds the frozen spec (JSON-equal) and names image `32144a29…817b`, revision `3271a25d02` and ecbench `21701be6…4503`. Launcher exits: P32 and P24 wall 0 (`succeeded`); P32, P24 and P28-r2 counted 23 (`failed`), so `verify-build` did not run for the counted jobs. Their binary is bound by `binary.sha256`, `build-info.json`, every record's `binary_sha256` and the audit's `auditor_binary_sha256`. RULING-P28 C2's identity facts, recomputed for every job including the launcher-less P28 wall: `S3_TERMINAL` true for all six; the image digest is unverifiable without pod status for all six.
P28 session of record: the original counted prefix `runs/ic-dgbf-rho-p28-20261029/` holds 0 objects today, consistent with the logged `S3_KEYCOUNT … 0` and COUNTED = r2 (VERDICT-P32 (B) B5).

## J3: charged-work identity

Method (`code/j3_work.py`): every `dgbf`, `dgbf-edge` and `dgb-aa` record, warm-ups included, is compared with the `dgb` record of the same (curve, target, round). The comparison covers the recovered log, `total_gae`, every unpriced count as band.rs resolves it, and per phase by name `adds`, `doubles`, `scalar_mults`, `gae`, `native` and `work_native` without `table_filter_bit_sets`. Every record's bitop/v1 work is re-priced from its primitive counts with my own reading of BITOPS.md, and the dgbf − dgb difference is checked phase by phase (bitops and primitive counts). `ecbench work` was not run: the handoff limits the binary to the replay.

| check | P32 | P24 |
|:--|:--|:--|
| matched records (dgbf, dgbf-edge, dgb-aa) / mismatches | {'dgb-aa': 336, 'dgbf': 336, 'dgbf-edge': 336} / 0 | {'dgb-aa': 336, 'dgbf': 336, 'dgbf-edge': 336} / 0 |
| `table_filter_bit_sets` absent on dgb, dgb-aa | True | True |
| present once, in `oracle_setup` only, on dgbf, dgbf-edge | True | True |
| one value per curve | {'icv1-fp32-t89141-95879fe3': [1048276], 'icv1-fp32-tm68303-f902d2f0': [1048295]} | {'icv1-fp24-t217-e1fbffd2': [26190], 'icv1-fp24-tm567-4accd0e8': [26196]} |
| records re-priced / mismatches | 1344 / 0 | 1344 / 0 |
| bitop identity (384 × counter, oracle_setup only, prims +6 word_op per bit set) checked / failures | 672 / 0 | 672 / 0 |
| bitop/v1 dgbf/dgb, measured records; difference | 1.0030127; 115,931,990,016 | 1.0051139; 2,896,736,256 |
| condition 10 (wall): p max, signed points max (source) | 3,965,125,361, 2048 (signed_points) | 14,940,703, 324 |
| counted dgb, dgbf = wall records (incl. counter, warm-ups) | True (336, 336) | True (336, 336) |

P32 difference check: 384 × 144 × (1,048,276 + 1,048,295) = 115,931,990,016, equal to the recomputed difference. P28: counted r2 dgb and dgbf equal the P28 wall records in every condition-8 field (True); this used P28 wall counts only. Condition 10 holds on P28 counted (p max 241,737,017, signed points max 814).

Because the counted session re-executed every `dgb` and `dgbf` input, warm-ups included, in a second pod, the equality is also a cross-pod replication of all 672 IC records per size.

## J4: counted route and band

Method (`code/j4_band.py`): S′_w = (G − C + w(U + n_C) + w · table_filter_bit_sets) / √r per record, with U resolved as band.rs does (top-level counter, else Σ phase `native[name]` or `native[base]`). No record carries pinned calibration units, so C = n_C = 0. Ratios are Σ S′(B) / Σ S′(A) over matched measured pairs, overall and per curve; the order is read per curve at each weight.

| session | pair (B/A) | w = 0 | w = 0.1 | w = 1 | per curve at w = 1 | without bit-set term | order strict across band, every curve |
|:--|:--|--:|--:|--:|:--|:--|:--|
| P32-wall | `dgbf/dgb` | 1.000000 | 1.050488 | 1.358633 | 1.3547 / 1.3626 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P32-wall | `dgb-aa/dgb` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P32-wall | `dgbf/dgbf-edge` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P32-counted | `dgbf/dgb` | 1.000000 | 1.050488 | 1.358633 | 1.3547 / 1.3626 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P32-counted | `dgb/rho-neg` | 31.471850 | 29.985805 | 23.277474 | 21.8109 / 24.9707 | 31.471850465 / 29.985804727 / 23.277473863 | True |
| P32-counted | `dgbf/rho-neg` | 31.471850 | 31.499721 | 31.625534 | 29.5476 / 34.0245 | 31.471850465 / 29.985804727 / 23.277473863 | True |
| P32-counted | `rho-neg-aa/rho-neg` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P24-wall | `dgbf/dgb` | 1.000000 | 1.050561 | 1.356044 | 1.3682 / 1.3437 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P24-wall | `dgb-aa/dgb` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P24-wall | `dgbf/dgbf-edge` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P24-counted | `dgbf/dgb` | 1.000000 | 1.050561 | 1.356044 | 1.3682 / 1.3437 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P24-counted | `dgb/rho-neg` | 12.115877 | 11.649806 | 9.452794 | 10.0011 / 8.9560 | 12.115877148 / 11.649805904 / 9.452794065 | True |
| P24-counted | `dgbf/rho-neg` | 12.115877 | 12.238828 | 12.818408 | 13.6839 / 12.0342 | 12.115877148 / 11.649805904 / 9.452794065 | True |
| P24-counted | `rho-neg-aa/rho-neg` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P28-counted | `dgbf/dgb` | 1.000000 | 1.054811 | 1.398006 | 1.4479 / 1.3496 | 1.000000000 / 1.000000000 / 1.000000000 | False |
| P28-counted | `dgb/rho-neg` | 21.044850 | 20.019054 | 15.337880 | 15.3321 / 15.3435 | 21.044850242 / 20.019053538 / 15.337879658 | True |
| P28-counted | `dgbf/rho-neg` | 21.044850 | 21.116316 | 21.442445 | 22.1997 / 20.7079 | 21.044850242 / 20.019053538 / 15.337879658 | True |
| P28-counted | `rho-neg-aa/rho-neg` | 1.000000 | 1.000000 | 1.000000 | 1.0000 / 1.0000 | 1.000000000 / 1.000000000 / 1.000000000 | False |

§5a: `rho-neg` is cheaper than both IC arms at every weight on every curve at P32, P24 and P28 (order strict across the band), so that ranking is permitted. `dgbf` against `dgb` ties at w = 0 and is costlier at w > 0 on both curves, so no counted ranking of `dgbf` ahead of `dgb` is permitted. Mean S′ per arm and weight (for `VALIDATION.json`'s `gae_band`) is in `code/out/j4.json` (`mean_Sprime_a`, `mean_Sprime_b`).

## J5: independent replay

Method (`code/replay.sh`): each session directory was copied and the copy checked against `SHA256SUMS` before and after the replay. `ecbench verify --replay-all` was run with the image binary (sha256 checked) on this host, an Intel Xeon VM; the pods ran on c8a.2xlarge, AMD EPYC 9R45. Warm-ups are refused by the binary's replay modes, so `code/scalars.py` checks every record, warm-ups included, with my own short-Weierstrass arithmetic: [recovered]G = Q, with the curve bound by its ICV1 j-invariant, p + 1 − t = r and [r]G = O.

| session | replays identical | problems | receipt ok | own scalar check (records, warm-ups) | replay env class (pod env class) |
|:--|:--|:--|:--|:--|:--|
| P32-wall | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |
| P32-counted | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |
| P24-wall | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |
| P24-counted | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |
| P28-counted (session of record, extra) | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |
| P28-wall (descriptive only) | 1152/1152 | 0 | True | 1344/1344 (192) | ECBENV3h13e37662a118 (ECBENV3hc1c5e5126853) |

The P28 wall replay is descriptive. It never stands in for the missing in-pod audit (RULING-P28 (a)), and P28 stays undecidable.

## VERDICT-P32 headline numbers

122 of 122 headline entries reproduce: each claimed value at its printed precision (rel_tol = half a unit in the last printed digit; 1e-9 for the 15-digit condition-9 value and the in-pod cross-check entries). The full list, with source lines, is in `validator-report.json` → `headline`. Every §6.1 wall condition (1–10) and counted condition for P24 and P32 is marked in `validator-report.json` → `section_6_1` with its recomputed value; all reproduce.

| claim | claimed | recomputed | reproduced |
|:--|:--|:--|:--|
| P32 ns_per_gae dgb | 84.5526 | 84.5525987 | yes |
| P32 ns_per_gae dgbf | 81.0446 | 81.04464734 | yes |
| P32 ratio_b_over_a dgbf/dgb | 0.958512 | 0.9585116079 | yes |
| P32 aa_ratio dgb-aa/dgb | 0.998337 | 0.9983374801 | yes |
| P32 corrected_ratio dgbf/dgb | 0.960107806 | 0.9601078062 | yes |
| P32 corrected ci95 lower | 0.956898 | 0.9568981834 | yes |
| P32 corrected ci95 upper | 0.963328 | 0.9633281948 | yes |
| P32 condition 9: ratio dgbf/dgb-aa (w=0 file) | 0.960107806240821 | 0.9601078062 | yes |
| P32 position control dgbf-edge/dgb | 0.959282 | 0.9592815969 | yes |
| P32 wall per charged GAE reduction, percent | 3.99 | 3.989219376 | yes |
| P32 reduction interval low, percent | 3.67 | 3.667180524 | yes |
| P32 reduction interval high, percent | 4.31 | 4.310181658 | yes |
| P32 bitop/v1 dgbf/dgb (measured records) | 1.0030127 | 1.003012748 | yes |
| P32 phase sum solve_wall dgb-aa, ms | 48192.2 | 48192.17664 | yes |
| P32 phase sum solve_wall dgbf, ms | 46269.7 | 46269.68499 | yes |
| P32 phase sum target_PDP dgb-aa, ms | 14047.8 | 14047.79331 | yes |
| P32 phase sum target_PDP dgbf, ms | 10413.1 | 10413.11932 | yes |
| P32 phase sum oracle_setup dgb-aa, ms | 6396.3 | 6396.297257 | yes |
| P32 phase sum oracle_setup dgbf, ms | 8167.2 | 8167.163333 | yes |
| P32 phase sum target_query dgb-aa, ms | 26776.7 | 26776.69785 | yes |
| P32 phase sum target_query dgbf, ms | 26765.8 | 26765.77297 | yes |
| P32 target_PDP change, percent | -25.9 | -25.87362942 | yes |
| P32 oracle_setup change, percent | 27.7 | 27.68580016 | yes |
| P32 oracle_setup rise as share of dgb-aa solve wall, percent | 3.67 | 3.674592433 | yes |
| P32 S' dgbf/dgb w=0 (wall) | 1.0 | 1 | yes |
| P32 S' dgbf/dgb w=0.1 (wall) | 1.050488 | 1.050487761 | yes |
| P32 S' dgbf/dgb w=1 (wall) | 1.358633 | 1.358632561 | yes |
| P32 S' dgbf/dgb w=1 per curve t89141 (wall) | 1.3547 | 1.354717861 | yes |
| P32 S' dgbf/dgb w=1 per curve tm68303 (wall) | 1.3626 | 1.362580347 | yes |
| P32 S' dgbf/dgb w=0.1 per curve tm68303 (wall) | 1.0509 | 1.050916477 | yes |
| P32 counted S' dgb/rho-neg w=0.0 | 31.4719 | 31.47185047 | yes |
| P32 counted S' dgbf/rho-neg w=0.0 | 31.4719 | 31.47185047 | yes |
| P32 counted S' dgb/rho-neg w=0.1 | 29.9858 | 29.98580473 | yes |
| P32 counted S' dgbf/rho-neg w=0.1 | 31.4997 | 31.49972088 | yes |
| P32 counted S' dgb/rho-neg w=1.0 | 23.2775 | 23.27747386 | yes |
| P32 counted S' dgbf/rho-neg w=1.0 | 31.6255 | 31.62553392 | yes |
| P32 counted S' dgb/rho-neg w=1 per curve t89141 | 21.81 | 21.81092478 | yes |
| P32 counted S' dgb/rho-neg w=1 per curve tm68303 | 24.97 | 24.97067237 | yes |
| P32 counted S' dgbf/rho-neg w=1 per curve t89141 | 29.55 | 29.54764937 | yes |
| P32 counted S' dgbf/rho-neg w=1 per curve tm68303 | 34.02 | 34.02454742 | yes |
| P24 corrected_ratio dgbf/dgb | 1.003200 | 1.003199518 | yes |
| P24 corrected ci95 lower | 1.000564 | 1.000564415 | yes |
| P24 corrected ci95 upper | 1.005842 | 1.005841562 | yes |
| P24 counted S' dgb/rho-neg w=0.0 | 12.1159 | 12.11587715 | yes |
| P24 counted S' dgbf/rho-neg w=0.0 | 12.1159 | 12.11587715 | yes |
| P24 counted S' dgbf/dgb w=0.0 | 1.0 | 1 | yes |
| P24 counted S' dgb/rho-neg w=0.1 | 11.6498 | 11.6498059 | yes |
| P24 counted S' dgbf/rho-neg w=0.1 | 12.2388 | 12.23882834 | yes |
| P24 counted S' dgbf/dgb w=0.1 | 1.0506 | 1.050560708 | yes |
| P24 counted S' dgb/rho-neg w=1.0 | 9.4528 | 9.452794065 | yes |
| P24 counted S' dgbf/rho-neg w=1.0 | 12.8184 | 12.8184076 | yes |
| P24 counted S' dgbf/dgb w=1.0 | 1.3560 | 1.35604431 | yes |

## Discrepancies (non-blocking)

- **P32 S' dgbf/dgb w=0.1 per curve icv1-fp32-t89141-95879fe3**: printed 1.0500, recomputed 1.0500605204138123. 1.0500605 rounds to 1.0501 at the printed precision; 1.0500 is a truncation. Descriptive per-curve figure; not in (A), (D) or the claim under review; the aggregate 1.050488 and every claim-under-review number reproduce. Not counted as a defect of the evidence; the Coordinator may rule otherwise.

## Heuristic-validation and cost-model checks

- Heuristic validation: not applicable. This is an engineering (constant) wall-clock comparison with no heuristic prediction under test. §7's predicted range was pre-registered; the miss (gain smaller than predicted) is descriptive.
- §6 ladder (`docs/inventor-protocol.md`): not applicable. The improvement is measured at the size where it is claimed (32 bits), and VERDICT-P32 (D) 5 forbids transfer.
- Units declared: ns per charged GAE (wall), GAE and S = GAE/√r (counted), bitop/v1, and S′ with w GAE per unpriced native operation. Flagged assumptions: S′ at w = 1 is a pessimistic band device; `table_insert_attempts` is unpriced on every IC arm; the three bitop/v1 limits of §5 apply. Memory beside time (max RSS, median over measured records, P32): dgb 62350 KiB, dgb-aa 62350 KiB, dgbf 63412 KiB, dgbf-edge 63380 KiB. Every run is a verified solve, so cost is per solve, not per attempt over a success probability.

## Limitations

- `ecbench work` was not run: the handoff limits the image binary to the replay. Condition 8's bitop half was checked against the records' in-pod `work` blocks, which I re-priced from their primitive counts with my own reading of BITOPS.md. The output of `ecbench work` itself was not compared.
- Condition 7 and the identical w = 0.1 and w = 1 wall blocks concern analysis files I was told not to read. Their content was recomputed from records, and my method equals the in-pod instrument to 4.2e-16.
- Warm-ups cannot be replayed by the binary's replay modes. They are covered by my own scalar check and by the counted session's second-pod execution of every dgb and dgbf input (J3), not by `ecbench verify`.
- Image digest: no job's pod image digest is verifiable from S3 without pod status. For the P32 and P24 wall jobs the launcher succeeded, and its `verify-build` ran. For the three counted jobs the launcher exited 23, so `verify-build` did not run, as VERDICT-P32 (C) discloses. Their binary is bound by `binary.sha256`, `build-info.json`, every record's `binary_sha256` and the audit's auditor binary.
- The raw-field re-grade covers the record-level grading-v3 gates. Session-level gates (preflight, thresholds, capsule, eviction) were read from `session.json`, and `irq` is null in every record, so the interrupt gate was not exercised. Counted sessions were checked by level-from-blocker consistency only (their raw fields are wall fields).
- The solve-time range of the waived rho executions (VERDICT-P32: 257.0 to 968.9 µs) is not reproduced: it is rho wall time.
- One session per size, c8a.2xlarge only, two curves per size: the scope of VERDICT-P32 (A) and (D). Nothing here bears on transfer or replication.
- Independence: I was dispatched by the main session that also relayed the producer's cards. I share no code or outputs with the producer, read no `analysis/` file and no `cards/tools/` file, and replayed on a host of another class.
- The launcher exit codes of the P24 and P32 jobs were read from the S3 launcher prefixes (`result.json`), not from local receipts.

## Negative controls on my own checkers

`code/selftest.py` perturbs copies of real records and requires each checker to flag the change: a native count +1, an extra work_native key, a changed log, a wrong pairing, a one-bitop change, a wrong scalar, the counter on the unfiltered arm, and wall-field leakage through the counted view. All fire (`code/out/selftest.json`).

## How to reproduce

```
cd research/ic_packed_filter_20261029/validation/code
python3 -B scalars.py out/scalars.json
python3 -B j1_decision.py 32 out/j1-P32.json; python3 -B j1_decision.py 24 out/j1-P24.json
python3 -B xcheck_inpod.py out/j1-inpod-crosscheck.json
python3 -B j2_instrument.py out/j2.json; python3 -B j3_work.py out/j3.json; python3 -B j4_band.py out/j4.json
python3 -B selftest.py out/selftest.json
PATH=<awsvenv>/bin:$PATH python3 -B s3check.py out/s3check.json  # and s3requests.py, c2_identity.py (read-only S3)
bash replay.sh <ecbench 21701be6…> ../../P32/wall/eks-results <scratch> P32-wall   # likewise per session
python3 -B build_report.py ..; python3 -B write_report_md.py ..
```

`raw_data` paths in `validator-report.json` are relative to a depth-2 evidence directory: `benchmarks/<dir>/` or `research/ic_packed_filter_20261029/`, where `VALIDATION.json` is expected. Each entry also carries `repo_path`. From `validation/` itself they do not resolve.

## validation_report (agents/validator.md)

```yaml
validation_report:
  id: null  # not allocated by the validator (no allocator type); named by path and sha256
  task_id: TASK-20261029-654f73
  run_ids: [ic-dgbf-p32-20261029, ic-dgbf-rho-p32-20261029, ic-dgbf-p24-20261029, ic-dgbf-rho-p24-20261029, ic-dgbf-rho-p28-20261029-r2]
  artifact_checks:
    - SHA256SUMS of all six sessions OK; store copies match .artifacts.json; S3 copies byte-equal; one pod prefix per job equal to job-exit pod_uid
    - frozen specs JSON-equal to input-spec.json, session/spec.json and every launcher request; build-info, binary.sha256, record and auditor binary = 21701be6...4503 from 3271a25d02
  metric_recomputations:
    - J1 P32 corrected dgbf/dgb 0.960107806 [0.956898, 0.963328], dof 562.6, condition 9 exact; P24 1.003200 [1.000564, 1.005842] (own code; in-pod comparison agrees to 4.2e-16)
    - J3 condition 8 and bitop identity on all records incl. warm-ups; bitop/v1 dgbf/dgb 1.0030127 (P32)
    - J4 S' at w = 0, 0.1, 1 for dgbf/dgb, dgb/rho-neg, dgbf/rho-neg at P32, P24, P28; order strict for rho-neg vs both IC arms
  control_checks:
    - counted A/A exactly 1 at every weight (dgb/dgb-aa, dgbf-edge/dgbf, rho-neg-aa/rho-neg); wall A/A intervals contain 1; native A/A admission file conditions hold
    - proves-too-much: no improvement on an A/A pair; P24 regression reproduced; label-swap null centred on 1 and narrowing as 1/sqrt(n)
    - positions 144/144 for dgbf and dgb-aa at positions 1/2; predecessors balanced
  heuristic_validation_checks: [not applicable (engineering constant; no heuristic prediction under test)]
  cost_model_checks:
    - units declared (ns per charged GAE, GAE/sqrt(r), bitop/v1, S' with w GAE per native op); w = 1 flagged pessimistic; table_insert_attempts unpriced on all IC arms; memory (max RSS) reported beside time
  proof_architecture_checks: []
  verdict: passed
  limitations: [see REPORT.md, Limitations]
  artifact_paths:
    - research/ic_packed_filter_20261029/validation/validator-report.json  # sha256 df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a
    - research/ic_packed_filter_20261029/validation/replay-certificate.json  # sha256 b357028b1c2187dc67abfbafe8999bf5f1b1a79e9c74c2471304b16e14f4d533
    - research/ic_packed_filter_20261029/validation/REPORT.md
    - research/ic_packed_filter_20261029/validation/code/
review_attestation:
  joints_owned: [J1 decision statistic, J2 instrument, J3 charged-work identity, J4 counted route and band, J5 independent replay, proves_too_much, blind_rederivation]
  joint_verdicts: {J1: holds, J2: holds, J3: holds, J4: holds, J5: holds, proves_too_much: holds}
  blind_rederivation: {quantity: corrected wall ratio dgbf/dgb at P32, value: 0.960107806240821, ci95: [0.956898, 0.963328], blind_from_respected: true}
  read_sibling_reports: false
  sources_read:
    - crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md, AGENTS.md (Review architecture; Curve identity and measured bounds)
    - crypto-autoresearcher origin/main ledger/handoffs/TASK-20261029-654f73.yaml, ledger/decisions/DEC-20261029-5eb8e6.yaml
    - backlog/XP-20261029-packed-filter/PROTOCOL.md; spec-P24.json, spec-P24-rho.json, spec-P28.json, spec-P28-rho.json, spec-P32.json, spec-P32-rho.json (APPROVAL.md hashed only)
    - research/ic_packed_filter_20261029/cards/VERDICT-P32.md (lines 1-230); RULING-P28.md ((a), seal, C2, C6); RULING-P28-ADDENDUM-1.md (heading and D2' lines via grep)
    - research/ic_packed_filter_20261029/P24/{wall,counted}/eks-results/, P32/{wall,counted}/eks-results/, P28/counted/eks-results/ (raw files; counted sessions through the wall-field-stripping loader)
    - research/ic_packed_filter_20261029/P28/wall/eks-results/ (counts only, wall fields stripped; job-exit, build-info, binary.sha256, input-spec, session.json status, SHA256SUMS)
    - research/ic_packed_filter_20261029/P28/P28-HOSTLOSS.md, s3verify-P28/{check-output.txt, s3receipt.json (first 2.5 KB)}, receipts-P28-rho-r2/{result.json, launch-P28-rho-r2.log, request.json}
    - git show 3271a25d02 src/cryptanalysis/ecbench/band.rs (lines 1-200), record.rs (grade_v3), docs/units/BITOPS.md (conventions, 2026-10-29 filter rule)
    - git show e27727238 backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md section 6.1; git show 6e4804417 backlog/XP-20261029-dg-wall/PROTOCOL.md (counted-session line)
    - docs/VALIDATION_GATE.md; scripts/validation_gate.py (checker functions); scripts/fixtures/validation_gate/positive/*.json; tools/artifacts/artifacts.py (store policy); origin/main benchmarks/20261017-ic-a15-filter-slots/VALIDATION.json (head)
    - docs/session-logs/2026-10-29.md and 2026-10-30.md (S3_KEYCOUNT lines via grep only)
    - S3 (read-only) runs/ic-dgbf-*-20261029*/ listings, pod files, launcher result.json and request.json
  not_read: [research/ic_packed_filter_20261029/cards/tools/, any analysis/ directory, compare.rs, any counted-session wall field, any rho wall time, P28 wall wall fields]
  verdict: passed
```
