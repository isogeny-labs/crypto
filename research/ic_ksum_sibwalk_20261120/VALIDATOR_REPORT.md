# Validator report VAL-20261120-7085ad: `ic_ksum_sibwalk_20261120` (k-sum `walk_sib=m` sibling trial walk, Table B)

This report independently validates the producer's evidence on branch `claude/f4-f5-algorithm-speed-bvp7qd`. It covers three
commits:

- `39d3d20d72`: code (`TargetSource::StrideRenewSib`, `KSumTable::probe_x`, `x_sibling_pair_counted`);
- `1189d9f96a`: protocol registration (03:46:43Z);
- `c1b719d9ce`: results (03:57:58Z), which was HEAD with a clean tree throughout this validation.

`git diff 39d3d20d72 c1b719d9ce -- src crates Cargo.toml Cargo.lock` is empty. The validator did not design, write or run any
part of the study. It did not edit, repair or regenerate any producer file. Validator nonce:
`5b4c82dad8e2095a19f807852a7c4312`, drawn 2026-11-20T04:01:22Z, after the results commit.

Independence is limited in two ways:

- The validator is a subagent of the same Claude Code session as the producer. It shares the scratchpad, the producer's
  binary and the raw tarballs, and it is independent by context only.
- The study has no pcc claim. Every number below is an **unverified assertion `[pcc:unverified]`** until the Coordinator
  archives this report.

## Verdicts

"Passed" means three things. The receipt is admissible evidence. The registered test T1 holds on that row, as claimed. The
headline numbers recompute exactly. The claim is a T0-scope, constant-factor engineering gain over the validated incumbent
(`research/ic_ksum_prefix_joint_20261118`, VAL-20261118-facd3e). It changes no exponent. It is not a claim below rho, and it
authorises no leaderboard or `benchmarks/README.md` row by itself (section 7).

**T1 (`cand / inc`, ratio of totals, 432 pairs a row; rule: upper 95 % end < 1 at w = 0, 0.1 and 1, pooled and in each of
the 3 replicates, where the upper end is the larger of linearised-t and bootstrap).** The validator's own `ecbench compare`
reruns are identical to the producer's `results.json.gz`, with a maximum difference of 0.0 over every compared number.

| row (m) | w = 0 [lin-t 95 %] | w = 0.1 | w = 1 | max upper end (pooled and 3 replicates), w = 0 / 0.1 / 1 | T1 verdict |
|:--|:--|:--|:--|:--|:--|
| P20 (6) | 0.7894 [0.7854, 0.7933] | 0.8175 [0.8142, 0.8208] | 0.9175 [0.9162, 0.9189] | 0.8008 / 0.8270 / 0.9214 | **passed** |
| P22 (8) | 0.7786 [0.7749, 0.7823] | 0.8074 [0.8043, 0.8105] | 0.9116 [0.9104, 0.9128] | 0.7918 / 0.8184 / 0.9160 | **passed** |
| P24 (12) | 0.7632 [0.7601, 0.7663] | 0.7942 [0.7917, 0.7968] | 0.9058 [0.9048, 0.9068] | 0.7724 / 0.8019 / 0.9088 | **passed** |
| B15 (2) | 0.9158 [0.9119, 0.9198] | 0.9270 [0.9236, 0.9303] | 0.9691 [0.9676, 0.9707] | 0.9279 / 0.9374 / 0.9741 | **passed** |
| B18 (3) | 0.8476 [0.8429, 0.8522] | 0.8679 [0.8640, 0.8719] | 0.9422 [0.9405, 0.9438] | 0.8607 / 0.8791 / 0.9470 | **passed** |
| B21 (6) | 0.7921 [0.7886, 0.7956] | 0.8196 [0.8167, 0.8225] | 0.9182 [0.9170, 0.9194] | 0.8004 / 0.8266 / 0.9210 | **passed** |
| B24 (10) | 0.7053 [0.7017, 0.7089] | 0.7476 [0.7447, 0.7506] | 0.8903 [0.8892, 0.8914] | 0.7150 / 0.7556 / 0.8933 | **passed** |
| B27 (12) | 0.7886 [0.7855, 0.7917] | 0.8153 [0.8127, 0.8179] | 0.9137 [0.9127, 0.9147] | 0.7958 / 0.8214 / 0.9161 | **passed** |

Every cell is admissible: 0 unpaired runs, 0 censored pairs, and every run verified. A/A is exactly 1, so the margin clause
`1 - upper > max |R_aa - 1| = 0` holds trivially. No row is significantly worse.

Holm across the 8 rows cannot change any verdict. The smallest margin is B15 at w = 1, where the pooled ratio of 0.9691
is about 39 linearised standard errors below 1.

The validator's own pricing, written from the raw records without the producer's code, gives the same point ratios as
ecbench to within 5.3e-15. The validator's own cluster bootstrap resamples targets within each session (4,000 resamples,
own seed). Its intervals are no wider than ecbench's, and its largest replicate upper end is 0.9733 (B15, w = 1).

**T3 (secondary, fresh test of `prefix=1` on P22, P24 and B27): failed on all three rows, as the producer claims and as
registered ("unlikely to hold").** All three pooled intervals lie below 1 at every weight.

| row | pooled w = 0 / 0.1 / 1 | replicate upper ends ≥ 1 | verdict |
|:--|:--|:--|:--|
| P22 | 0.9626 / 0.9639 / 0.9685 | w = 1: r1 1.0010, r3 1.0072 | failed (w = 1) |
| P24 | 0.9774 / 0.9788 / 0.9837 | w = 0: r1 1.0069, r2 1.0032; **w = 0.1: r1 1.0095, r2 1.0057**; w = 1: r1 1.0191, r2 1.0148 | failed (every weight) |
| B27 | 0.9750 / 0.9760 / 0.9795 | w = 1: r1 1.0037 | failed (w = 1) |

The README says that a replicate's upper end reaches 1.001–1.019 "at w = 1 (P24 also at w = 0)". **On P24 it also fails at
w = 0.1** (finding F3). The verdict is unchanged.

**Distance to the matched rho (`rho-neg`, the same session, binary and targets; S(arm)/S(rho-neg), §5a band).** Every
number recomputes. ecbench's band order on every row is `cand > rho-neg at every weight`: rho is ahead everywhere.

| row | w = 0: inc → cand | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| P20 | 3.764 → 2.971 | 3.992 → 3.264 | 5.094 → 4.674 |
| P22 | 4.274 → 3.328 | 4.497 → 3.631 | 5.544 → 5.054 |
| P24 | 4.566 → 3.485 | 4.820 → 3.828 | 6.025 → 5.458 |
| B15 | 2.517 → 2.306 | 2.672 → 2.477 | 3.483 → 3.375 |
| B18 | 2.489 → 2.110 | 2.649 → 2.299 | 3.460 → 3.260 |
| B21 | 3.764 → 2.982 | 3.988 → 3.269 | 5.070 → 4.655 |
| B24 | 4.227 → 2.981 | 4.526 → 3.384 | 5.945 → 5.293 |
| B27 | 5.089 → 4.013 | 5.331 → 4.346 | 6.461 → 5.904 |

On every curve, `rho-neg` uses the curve's full automorphism group (`automorphisms_available = 2`, negation only), so it is
the strongest available reference.

**Overall verdict: passed.** The receipt is admissible, and T1 passed on 8/8 rows. T3 failed on 3/3 rows, as claimed.

Three findings bear on how the result may be described:

- **F1. The w = 1 margin is entirely a guard-policy effect.** The guard skip is sound (section 1), but it is the deferred
  guard, a lever already in the repository, applied to the missed abscissae. It is not a sibling-pair effect (section 3).
- **F2. The raw sessions exist only in the scratchpad.** They are not in the repository or the store, so the validation
  gate is not met (section 7).
- **F3. Wording and code nits.** The README's T3 wording omits P24's failure at w = 0.1, and a doc comment and attribute
  are misplaced in the code (section 6). Neither changes a verdict.

## 1. Is the guard skip on a missed abscissa sound? (question 1)

The code read is the full diff of `39d3d20d72`, together with the relation loop around it: the target-source branches, the
guard, the renewal block and the counters of `collect_and_solve_relations_scaled_touching` in
`src/cryptanalysis/ic_boundary.rs`.

### What the guard does under stride-renew(-joint)

`StrideRenew`, `StrideRenewJoint` and `StrideRenewSib` are guarded (`is_guarded`), and their guard is not deferred.

- Every presented target `R` costs `guard_probes += 1` and `seen.insert_duplicate(g.key(R))`.
- On a hit, the target is counted in `repeats_skipped` and the loop `continue`s.
- Because `renews_stride()` is true, a repeat does **not** set `walk_fresh`, so the guard never changes the progression.
- The progression changes only on a relation (stride renewal) or on an identity point (restart). Neither depends on the
  guard's outcome.

The guard's sole function is the one its comment states: a target already in `seen` yields no second relation, so no rho
collision can pin the logarithm. For a target that does not decompose, the guard has no effect on rows, the answer or the
progression. Its only effect there is to save the oracle call on a repeat.

`seen` is exact. Both `FastMap<()>` and `GuardLineSet` store the full 64-bit key, so the guard has no false positives, and
inserting fewer keys cannot change which decomposing target is skipped.

### Equivalence argument

The equivalence needs two facts, both verified in code:

1. **An x-probe miss implies that the full oracle returns no relation.**
   - `KSumOracle::decompose_relations` is `free_rows` (only with `free`, which is off in every spec) plus one `probe`.
   - `probe` begins with `self.map.get(&(g.key(target) >> 1))?`.
   - `probe_x(x)` is `self.map.contains_key(&(x + 1))`.
   - `g.key >> 1` is `x + 1` on both prime (`((x+1) << 1) | sign`) and binary (`FastPoint::pack` = `2(x+1) + s`) curves.
   - So a miss on `x + 1` is exactly the first, early-returning step of the full probe.
   - Conversely, a key hit is a full abscissa match: the stored sum is `±R`, and the full probe confirms it and returns the
     signed summands.
2. **The progression is the incumbent's.**
   - Given the same relations, `C ± [i]S` with `C = A + [m+1]S` presents `A + S, …, A + [2m+1]S` in order.
   - After a relation on `R` the anchor is `R` and the multiples are rebuilt for `S'`, so the next block's first target is
     `R + S'`.
   - An identity restart draws the start exactly as the incumbent does. The stride is unchanged, so the multiples stay
     valid.

From these, by induction over the trials:

- At every trial the candidate's `seen` is a subset of the incumbent's. It holds every presented target except the missed
  abscissae.
- A target that decomposes has every earlier copy decomposing too, because the oracle is a pure function of the point and
  the fixed table.
- A decomposing earlier copy was either formed in full or lifted after an x-hit, and in both cases was inserted.
- So a decomposing target is a repeat in the candidate exactly when it is a repeat in the incumbent. The two runs produce
  the same relations in the same order, and therefore the same renewals, progression, rows and answer.

### How a missed (never-lifted) point can be a repeat

- **The missed point repeats an earlier target.** The incumbent skips it at the guard. The candidate probes `x + 1`, which
  misses. Neither run produces a relation, and each spends one priced native operation.
- **A later full-formed target repeats an earlier missed point.** The incumbent skips it at the guard. The candidate's
  guard passes it, because the missed point was never inserted, and then the oracle misses (one counted lookup). This is
  why the candidate skips fewer repeats (3,484 against 12,272 over 4,608 triples). The extra lookups are counted, and the
  identity `lookups = pairs + guard_probes - repeats` holds on every record.
- **A decomposing repeat of a missed point.** This is impossible: if the repeat decomposed, the missed point would have
  decomposed too, so it would have been a hit.

The guard is still applied wherever a relation can arise: to every start, centre and plus point, every lifted minus point
and every minus point of a full pair. Relation independence is unchanged. `repeated_column_rows` and
`pinned_by_repeated_row` are 0 on all 4,608 cand records (and on inc, off and inc-prefix), and the linear-algebra phase is
identical to the incumbent's on every triple.

### Is any work performed but not counted?

None was found.

| work | how it is charged | check |
|:--|:--|:--|
| Sibling pair: `P + J` full and `x(P - J)` from one inversion | 1 group add, plus 1 `sibling_second_points` (priced at w), plus 1 x-probe counted in `lookups` | On the 1,728 prime triples, the instrumented field-op tally of the relation phase is exactly the charged adds' and doubles' field work (1I + 2M + 1S per add, 1I + 5M + 2S per doubling) **plus one M and one S per pair**. No hidden inversion, so no hidden addition. The 4 exceptions are full pairs (`C = ±[i]S`), where one charged add is a doubling and the other is `P + (-P)`. The residuals there (inv -1, mul +1, sqr 0 against the incumbent's -1, -2, -1 for its `P + (-P)`) are exactly the group law's special cases. |
| Minus abscissa key (`x + 1`) | integer increment, part of the lookup | the same in the incumbent's probe |
| Lift after an x-hit | one full `g.add` (conservative: λ₋ was already known) | `lifts` term in the identity |
| Multiples `[2]S..[m+1]S` | 1 doubling plus m-1 adds, lazily, once per stride that starts a block | `doubles_cand = doubles_inc + multiple_ops/m` and `multiple_ops ≡ 0 (mod m)` on every triple |
| Centre `C = A + [m+1]S` | 1 add a block | `centres` term |
| Wasted plus points (block cut by a relation) | charged when formed, as pairs | `walk_sibling_unused_plus` is informational |
| Coefficient bookkeeping (`addmod`) | `walk_modr_adds` (bitop/v1 only, as on the incumbent) | not part of GAE on either arm |
| Binary curves' sibling field work | not tallied (binary `field_ops` is "unknown" on both arms) | covered by the code reading and the group-op identity |

The sibling formulas themselves were checked with the validator's own arithmetic:

- prime: `λ₋ = -(y_J + y_P)/(x_J - x_P)`;
- binary: `λ₋ = λ₊ + x_J/(x_P + x_J)` and `x = λ₋² + λ₋ + x_P + x_J + a`.

On 2,000 nonce-seeded point pairs on each of the 8 curves, the abscissa of `P - J` matches a full addition, with 0
mismatches.

## 2. Same targets, relations, rows and answer, run for run (question 3)

The validator wrote its own record checker, which shares no code with `analysis/correctness.py`. It ran over all 4,608
(session, round, target) triples, warm-ups included.

| check | mismatches |
|:--|:--|
| `off` = `inc` in every field except identifiers, `role`, method params, `fbm_id`, the label and wall/host timing (cost, counters, every phase, outcome, `work` bitops included) | 0 |
| `cand` = `inc`: target, target index and seed, `algorithm_seed`, recovered log, `counters` (targets_tried, relations_found, matrix rank/rows/work/dependent) | 0 |
| `cand` = `inc`: the factor-base, oracle-setup, linear-algebra and verify phases in full (wall excluded) | 0 |
| `cand` = `inc`: relation-phase trials, relations, stride steps/restarts/renewals, direct skips, prefix entries, scalar mults, rng draws | 0 |
| Group-op identity `G_cand = G_inc - stride_steps + pairs + 2 full_pairs + centres + lifts + multiple_ops` | 0 |
| `walk_x_only_probes = sibling_second_points = pairs`; the `counted:sibling_second_points` marker is present | 0 |
| `guard_cand = trials - (pairs - lifts)`; `guard_inc = trials`; `lookups_inc = trials - repeats_inc`; `repeats_cand ≤ repeats_inc` | 0 |
| Native priced-at-w delta `N_cand - N_inc = 2 lifts + repeats_inc - repeats_cand` | 0 |
| Every record verified with `matches_target`; every arm recovers the same `d` for a target | 0 |

The records do not carry row contents. Row equality rests on three things: the code argument of section 1, identical
linear-algebra work counters (row ops, Gauss entries scanned, pivot steps, row-build words and mods, column-part inserts) on
every triple, and identical answers.

Totals over the 4,608 triples:

| count | total |
|:--|--:|
| pairs | 9,525,000 |
| full pairs | 22 |
| lifts | 21,892 |
| centres | 1,025,705 |
| multiple ops | 368,384 |

## 3. Where the w = 1 gain comes from (finding F1)

From the records:

- At w = 0 the saving is the group additions the pairs avoid.
- At w = 1 the native priced work is almost unchanged (ΔN = +3,039 to +6,244 per 432 pairs).
  - The sibling point priced at 1 GAE cancels the addition each pair saves.
  - The guard probes skipped on missed abscissae (also priced at 1 GAE, as lookups) offset that charge.
- So **the w = 1 margin exists only because missed abscissae are not guarded.**

The validator's arithmetic below is not a measured arm. If every minus point were guarded, the w = 1 ratio on the same
records would be above 1 on every row, and that figure is still a lower bound, because guarding needs the point's `y`.

| row | measured w = 1 | if every minus point were guarded | guard probes as a share of inc's w = 1 price |
|:--|:--|:--|:--|
| P20 | 0.9175 | 1.0089 | 0.196 |
| P22 | 0.9116 | 1.0068 | 0.201 |
| P24 | 0.9058 | 1.0057 | 0.206 |
| B15 | 0.9691 | 1.0228 | 0.132 |
| B18 | 0.9422 | 1.0226 | 0.185 |
| B21 | 0.9182 | 1.0090 | 0.195 |
| B24 | 0.8903 | 1.0088 | 0.247 |
| B27 | 0.9137 | 1.0042 | 0.187 |

The skip is sound (section 1), it removes work that is really performed, and wall time agrees.

- Relation-phase wall `cand/inc` is 0.54–0.83 and solve wall is 0.69–0.95 (L0, practicality note only).
- The candidate's relation phase is faster in 348–430 of each row's 432 pairs.
- So this is **not a relabelling or an accounting change**, and T1 holds as registered.

Skipping the guard for targets that do not decompose is the **deferred guard**, though. `TargetSource::StrideRenewDeferredGuard`
was validated as its own lever in `research/ic_srenew_deferred_guard_20261029` (dg/srenew 1.000 at w = 0, about 0.76 at
w = 1), and here it is applied only to the missed half of each block.

- That lever exists only for non-joint `stride-renew`, so no `stride-renew-joint` arm with a deferred guard could be run.
- On these records the incumbent's guard probes are 13–25 % of its w = 1 price, so a deferred guard on every target would
  plausibly save more at w = 1 than `walk_sib` does. This is an estimate, not a measurement.

Recommended description: `walk_sib` is a sibling-pair lever at w ≤ 0.1 plus a partial deferred guard at w = 1. Any
leaderboard component line should name both effects. A joint deferred-guard source is the open control.

The README does say that "the missed abscissa also skips its guard probe … so the native work is unchanged". It does not
say that without that skip the lever loses at w = 1, or that the skip is the existing deferred-guard lever.

## 4. Pilot, m-selection and registration (question 4)

- **Seeds.** The pilot uses `target_seed` 20261120500–507 and `seed` 20261120550–557 (64 targets, 1 round, arms `sib4`,
  `sib8` and `sib16`). The confirmatory runs use 2026112030x–37x and 2026112040x–47x, matching the formula
  `idx = 10·row + rep` on 24/24 specs. The two sets are disjoint, and `git grep` finds no other spec or benchmark record
  that uses them.
- **Rule reproduced.** The validator reran the registered `analysis/model.py` (sha256 `9e79c12f…b8db`, which is in
  `REGISTRATION.sha256`) on the validator's own extract of the pilot sessions. The output is identical to the committed
  `pilot/model_pilot.json`.
  - The model reads only `inc` records.
  - Its argmin at w = 0.1 over the grid {1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24, 32} gives exactly `M.json` on 8/8 rows:
    P20 6, P22 8, P24 12, B15 2, B18 3, B21 6, B24 10, B27 12.
  - The registered predictions are reproduced to 3 decimals.
- **Order.** The order is as claimed, though every timestamp is self-reported.

  | event | time (UTC) |
  |:--|:--|
  | pilot sessions | 03:42:40–03:43:22 |
  | registration commit (m per row in `M.json` and in the 24 specs) | 03:46:43 |
  | first confirmatory session | 03:46:53 |
  | last confirmatory session | 03:50:07 |
  | results commit | 03:57:58 |
- **Registration hashes.** `REGISTRATION.sha256` checks 32/32. Registered files changed after `1189d9f96a`: none (PROTOCOL,
  M.json, specs, evaluate/correctness/model, run_all, pilot).
- **Predictions against measurement.** The largest miss is B27 at w = 0, measured 0.7886 against a predicted 0.779 (+0.0096).
  Every cell is within the registered ±0.02.
- **Caveat.** The selection weight (w = 0.1) and the model were fixed after the pilot. Because the pilot is disjoint and
  only selects m, this is a valid two-stage design.

## 5. Recomputation, replay, digests, answers and controls (question 5)

- **ecbench compare.** The validator reran 228 comparisons with its own invocation (`--weight 0/0.1/1 --resamples 4000
  --seed 20261120`): T1 and T3, pooled and per replicate, plus gaps, A/A and off. They are identical to the producer's
  `results.json.gz` (maximum |difference| 0.0). The registered rule applied to the rerun gives T1 holding on 8/8 rows and
  T3 holding on 0/3, with none significantly worse.
- **Own pricing.** For IC arms, `S·√r = Σ_phases (adds + doubles) + w · (calibration-pinned PRICED_NATIVE counts +
  counted table_insert_attempts + counted sibling_second_points)`. For rho, it is `total_gae + w · canonicalisations_uncharged`.
  The point ratios agree with ecbench to 5.3e-15. `sibling_second_points` is priced at w: without that charge the w = 1
  ratio would not reproduce.
- **Replay.** `ecbench verify --replay-all --exit-code` ran on a nonce-drawn sample of 8 sessions (P20-r2, P22-r3,
  P24-r2, B15-r3, B18-r2, B21-r3, B24-r1, B27-r2), then on the other 16, all under the CPU lock. Result: 24/24 rc 0, 24,768
  records, 18,576 replays reproduced, 0 problems. This is a same-binary replay, so it shows determinism, not independent
  implementation.
- **Digests.**

  | artifact | sha256 | result |
  |:--|:--|:--|
  | `sib-confirm-sessions.tar.gz` | `56caf4b3fe39acc9282f2d6f3665e2d98853fc34e1c440d67a2a5453eea55410` | matches the README and the bench record |
  | `sib-pilot-sessions.tar.gz` | `21adfcce0932a506f730ba502a65b7781ff0ec1ce2128bfa7071df98062f9cb5` | matches |
  | `data/sessions.sha256` | | 72/72 |
  | `pilot/sessions.sha256` | | 24/24 |
  | session `spec.json` = committed specs | | 24/24 confirmatory, 8/8 pilot |
  | binary | `ff3172218ab6aa4f3d1f19d02b5cc3d2401cf1d737c06b5c0cc11202a50af844` | matches `BINARY.sha256`, every `session.json` (24 + 8), every record and the auditor; `strings` shows the `walk_sib` parameter text, `stride_renew_joint_sib` and `walk_sibling_multiple_ops` |
- **Independent `[d]G = Q`.** The validator used its own prime and GF(2^n) arithmetic, with no producer code, on **all**
  1,152 targets of all 24 sessions (a superset of a nonce sample; prime and binary rows).
  - 1,152/1,152 satisfy `[d]G = Q`, with `Q` on the curve and `[r]Q = O`.
  - `G` lies on the curve with order `r` on all 8 curves.
  - Every arm agrees on `d`.
- **Controls.**
  - **A/A.** `rho-neg-aa` is record-for-record identical to `rho-neg` (all fields but role and identifiers), and the ratio
    is 1.0000 at every weight. The control is deterministic, so it is uninformative about noise.
  - **Default-off.** `off` = `inc` on 4,608/4,608 triples (above), and `off/inc` = 1.000000 at every weight. In code,
    `walk_sib = 0` leaves `source` unchanged, `xprobe = None` and `sr_m = 0`, and `joint_gq` is unchanged for
    `StrideRenewJoint`.

## 6. Other findings

- **F3a (README).** The T3 line omits P24's per-replicate failure at w = 0.1 (r1 1.0095, r2 1.0057). The verdict is
  unchanged.
- **F3b (code nit).** In `39d3d20d72` the new `fn sr_plus_unused` was inserted between the doc comment plus
  `#[allow(clippy::too_many_arguments)]` of `pub fn collect_and_solve_relations_touching_check` and that function.
  - Both now attach to the private helper, and the public function has lost its doc comment.
  - There is no behavioural or CI effect, because `too_many_arguments = "allow"` is set crate-wide in `Cargo.toml`.
  - It should be fixed by moving the helper.
- **The incumbent reproduces.** The incumbent's gap at w = 0 (2.49–5.09 on these targets) matches the prefix-joint study's
  2.48–4.99 on its own targets within the target-sample spread.

## 7. What this does and does not support; gate status

- **It supports** a registered, constant-factor reduction of the k-sum Table B arm's counted cold single-target cost.
  - `cand / inc` is 0.705–0.916 at w = 0, 0.748–0.927 at w = 0.1 and 0.890–0.969 at w = 1.
  - It holds on every row, pooled and per replicate, for the same targets, relations and answers as the incumbent.
  - It is not a relabelling. The w = 1 part is the guard-skip effect described in F1.
- **It does not support** any change of exponent, any claim against rho, or any transfer.
  - Rho remains 2.11–4.01× cheaper at w = 0 and 3.26–5.90× at w = 1.
  - The track is generic decomposition, bounded by √r.
- **Validation gate: not met (F2).**
  - The raw sessions exist only as scratch tarballs (`uploaded: false` in both bench records), and the scratchpad can be
    wiped. The gate requires the validator's recompute to be from raw data in the repository or store.
  - There is no `VALIDATION.json`, replay-certificate file or JSON validator report.
  - The archive must be landed in the store (AGENTS.md §13/§13a) before a `VALIDATION.json` can cite this report. Until
    then, no leaderboard or `benchmarks/README.md` row.

## Limitations

- **Binary not rebuilt, unit test not run.** The disk had 2.7 GB free, and a release build in a private target directory
  needs about that. Binary binding rests on four things: hashes, `strings`, source unchanged since `39d3d20d72`, and
  behaviour that matches the code on every identity above.
- **Replay used the producer's binary.** The independent checks are the EC arithmetic, the sibling formulas, the pricing
  and the record identities.
- **L0, local, counted cost only.** The route was not isolab-eks; that deviation is disclosed. Wall time is a practicality
  note only.
- **Deterministic A/A.**
- **Self-reported timestamps.**
- **Validator scripts are not committed.** They remain in the validator's scratch directory (sha256 in the YAML below).
- **The w = 1 attribution in F1** is arithmetic on the measured records, not a measured arm.

```yaml
validation_report:
  id: VAL-20261120-7085ad
  task_id: "independent validation of research/ic_ksum_sibwalk_20261120 (dispatched by the main session; no TASK id given)"
  snapshot: {code: 39d3d20d72, protocol: 1189d9f96a, results: c1b719d9ce, tree: clean, src_changed_after_code_commit: false}
  nonce: {value: 5b4c82dad8e2095a19f807852a7c4312, drawn: "2026-11-20T04:01:22Z"}
  run_ids: ["24 ecbench sessions {P20,P22,P24,B15,B18,B21,B24,B27} x r1..r3 (sib-confirm-sessions.tar.gz 56caf4b3...5410); pilot 8 sessions (sib-pilot-sessions.tar.gz 21adfcce...9cb5), not data"]
  artifact_checks:
    - registration_before_data: pass (1189d9f96a 03:46:43Z; first confirm session 03:46:53Z; pilot 03:42:40-03:43:22Z; self-reported)
    - registered_hashes: pass (32/32); registered files unchanged after 1189d9f96a
    - session_digests: pass (tarballs 2/2; data/sessions.sha256 72/72; pilot/sessions.sha256 24/24)
    - specs_equal_committed: pass (24/24 confirm, 8/8 pilot); seeds as registered; pilot seeds disjoint; seeds unused elsewhere
    - binary_sha256: pass (ff3172218ab6...f844 in BINARY.sha256, 32 session.json, records, auditor); strings check pass; not rebuilt
    - coverage: pass (24 sessions complete; every record verified; 0 unpaired, 0 censored)
    - replay: pass (nonce sample of 8, then all 24: 24,768 records, 18,576 replays reproduced, 0 problems)
    - raw_data_durable: FAIL (scratch tarballs only, uploaded false)
  metric_recomputations:
    - "ecbench compare, 228 comparisons (w 0/0.1/1, 4000 resamples, seed 20261120): identical to results.json.gz (max diff 0.0)"
    - "own pricing from raw records: point ratios agree with ecbench to 5.3e-15; own cluster bootstrap and delta-method intervals consistent"
    - "T1: holds 8/8 rows at w = 0, 0.1, 1, pooled and per replicate; max upper 0.9741 (B15, w = 1); none significantly worse"
    - "T3: fails P22 (w=1), P24 (w=0, 0.1, 1), B27 (w=1); pooled < 1 everywhere; none significantly worse"
    - "gap to rho-neg w=0 2.49-5.09 -> 2.11-4.01, w=0.1 2.65-5.33 -> 2.30-4.35, w=1 3.46-6.46 -> 3.26-5.90; band: cand > rho-neg at every weight on every row"
    - "model rerun on pilot inc records: identical to pilot/model_pilot.json; argmin at w=0.1 = M.json on 8/8; measured - predicted within 0.0096"
  control_checks:
    - aa: exactly 1, record-identical (deterministic; uninformative about noise)
    - default_off: off == inc on 4,608/4,608 triples in every deterministic field; off/inc = 1.000000
    - run_for_run_identity: cand == inc on 4,608/4,608 triples in targets, seeds, answers, counters, setup/LA/verify phases, trials, relations, stride counters, scalar mults
    - count_identity: G_cand = G_inc - stride_steps + pairs + 2 full_pairs + centres + lifts + multiple_ops, 0 mismatches; doubles identity, guard/lookup/native identities 0 mismatches
    - prime_field_tally: 1,728 prime triples, field work = charged ops + 1M + 1S per pair; 4 full-pair cases explained by group-law special cases
    - answers_independent_ec: 1,152/1,152 [d]G = Q (all targets), Q of order r, G of order r on 8/8 curves
    - sibling_formulas_independent: 16,000 pairs (2,000 x 8 curves), 0 mismatches
    - relation_independence: repeated_column_rows = pinned_by_repeated_row = 0 on every IC record
  heuristic_validation_checks:
    - "predictions registered before data: pass (M.json in 1189d9f96a; within +-0.02 on 24/24 cells)"
  cost_model_checks:
    - unit: ecbench.gae per sqrt(r), cold single target, every phase charged; band 0/0.1/1 on every arm (rho canonicalisations, IC lookups, guard probes, table inserts, sibling second points)
    - sibling pair: 1 add + 1 sibling_second_points at w (repository convention); w = 0 prices the second point (1M + 1S) at 0, w = 1 at a full add
    - guard skip on missed abscissae: sound and real (section 1); carries the whole w = 1 margin (F1; guarded counterfactual 1.004-1.023)
    - no uncounted work found (field tally on prime rows; code audit on binary)
    - memory not reported (m+1 stored points; negligible)
  proof_architecture_checks: []
  verdict: passed
  claim_verdicts:
    T1_walk_sib: "passed on 8/8 rows: P20 passed, P22 passed, P24 passed, B15 passed, B18 passed, B21 passed, B24 passed, B27 passed"
    T3_prefix_fresh: "failed on P22, P24, B27, as claimed (README omits P24 at w = 0.1)"
    gap_to_rho_neg: passed as recomputed; rho ahead on every row at every weight
    w1_attribution: "F1: the w = 1 margin is a partial deferred guard, not a sibling effect; describe both"
    gate: not met (raw data not durable; no VALIDATION.json / replay certificate / JSON validator report)
  limitations: [binary not rebuilt, unit test not run (disk), same-binary replay, L0 local counted cost only, A/A deterministic, self-reported timestamps, raw data scratch-only, validator scripts not committed, F1 counterfactual is arithmetic not a measured arm]
  validator_scripts_sha256:
    val_records.py: 1b9ec4c517bb40f98a33aa3f073fe0659b9e8692904c84791d269a568478f8cf
    val_compare.py: 3839df378ec453afbbad94630facb8cb0c61164df173b5e98049b2bb0f7f01c4
    val_diff.py: a1ba88f5eef08f6111bddea1b2f81ee19bac085b25ccf1a59dfb923c11106103
    val_ec.py: bd0f14bd4bbf9f0b1b5c4947b83121cd73c0a0ab20a326119eeb7b9c1269b96f
    val_attrib.py: 2bea6625ede4410d35eba1162e29e277d9856016d13c3f2ee1c627d8ad8de400
  artifact_paths:
    - research/ic_ksum_sibwalk_20261120/VALIDATOR_REPORT.md
review_attestation:
  joints_owned: [whole-study validation as dispatched; specific questions 1-5]
  sources_read:
    - research/ic_ksum_sibwalk_20261120/{README.md, PROTOCOL.md, M.json, registered-at.txt, BINARY.sha256, REGISTRATION.sha256, run_all.sh, run_pilot.sh, analysis/evaluate.py, analysis/model.py, pilot/analyse.py (head), pilot/pilot_table.txt, pilot/runlog.tsv, pilot/verify.txt, data/evaluate.txt, data/correctness.txt, data/verify_summary.txt (head), data/runlog.tsv (head), data/sessions.sha256, pilot/sessions.sha256, specs/*, pilot/specs/*, results.json.gz}
    - "raw sessions (validator scratch extracts of both tarballs): records.jsonl, session.json, spec.json, host.json (head)"
    - git objects 39d3d20d72 (full diff), 1189d9f96a and c1b719d9ce (logs, stats)
    - src/cryptanalysis/ic_boundary.rs (CountedGroup sibling/key docs, PrimeCurve add/sibling_pair/sibling_lift/x_sibling_pair_counted/key, BinaryGroup key/x_sibling_pair, FIELD_OPS_PER_ADD/DOUBLE, KSumTable insert/prefix/probe/probe_x, GuardLineSet, RelationGuard, collect_and_solve_relations_scaled_touching set-up, target sources, guard, renewal, counters)
    - src/cryptanalysis/koblitz_fast.rs (FastPoint::pack); src/cryptanalysis/ic_framework/plugins.rs (KSumOracle diff and prepare/decompose/probe_x/decompose_relations, new unit test); src/cryptanalysis/ic_framework/mod.rs (walk_sib wiring)
    - src/cryptanalysis/ecbench/band.rs (TABLE/SIBLING_WORK_COUNTERS, field class, priced_gae); methods.rs PRICED_NATIVE; Cargo.toml lints (grep)
    - benchmarks/records/20261120T0346Z-ksum-sibwalk-tableb.json and 20261120T0342Z-ksum-sibwalk-pilot.json (top-level keys)
    - research/ic_ksum_prefix_joint_20261118/{specs/*-r1.json, README.md (grep), VALIDATOR_REPORT.md (head, tail, grep; prior round, format)}
    - research/koblitz_k1_sibwalk_20261118/VALIDATOR_REPORT.md (head; prior round, named in the task as context)
    - research/ic_srenew_deferred_guard_20261029/README.md (git show; prior validated lever)
    - docs/VALIDATION_GATE.md; AGENTS.md (section 5a, section 6, IC measurement rules); crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md
  read_sibling_reports: false
  verdict: passed (receipt admissible; T1 passed on 8/8 rows; T3 failed on 3/3 as claimed; gate not met)
```
