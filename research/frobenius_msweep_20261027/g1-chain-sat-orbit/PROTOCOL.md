# G1 protocol (frozen before any data): chained-S3 SAT, plain x-domain against the exact Frobenius-orbit x-domain

Status: prepared, not run. Design frozen by this file; changes after data exist go in an AMENDMENT.md with the reason, never by editing this file.
Amendment 1 (2026-10-27, before any data existed, section 11) revised the cells (n = 17 replaced by 19), k, coset count, the deciding statistic, the censoring gates and the cap rule. Sections 1 to 10 below are the amended text; section 11 records what changed and why.
Driver: `examples/frobenius_msweep_g1_chain_sat_orbit.rs` (auto-discovered example; Cargo.toml needs no entry). Run commands: `RUN_CARD.md`.

## 1. Question

Audit claim C3 (2026-10-26) reads: "sound orbit encodings are >= 0.95x as fast as plain at n = 17, 23, 31", and it is UNSUPPORTED because
H-BINSTD-c72284 Stage 2 never ran. No solver-level Frobenius-symmetry experiment exists at m >= 3. This cell asks, at m = 4, 5, 6 and
n = 19, 23, 31 (Koblitz a = 0, y^2 + xy = x^3 + 1; n = 17 is impossible for a = 0, see section 11), on identical instances through the same chained-S3 loop:

  Q1. Is the cost per relation of the exact Frobenius-orbit x-domain (arm O) at most 1/0.95 times that of the plain x-domain (arm P)?  (C3, non-inferiority; C3 as audited names n = 17, 23, 31; n = 17 cannot be built for a = 0, so C3 at n = 17 stays untested here)
  Q2. Is it lower (arm O, or O with first-pair ordering = arm Q, or P with first-pair ordering = arm R)?  (improvement over the baseline, the success criterion)

## 2. Claim classes (controlled-experiment skill)

| Claim | Class | How it is settled here |
|---|---|---|
| Orbit domain is sound: same solution set as plain (C1/C2/C4) | proved for all m (audit); checked here as an exact identity | unit tests and, in every row, agreement with an exact oracle that does not use the SAT solver (`agree`) and an independently re-added certificate (`cert_ok`) |
| The shadow solve loop equals the production loop | exact identity | `--tieout` rows: verdict class, conflicts, patterns, models, solver calls equal for P, O, Q (R has no production entry point; it differs from P by one public call, `order_first_pair`); analysis refuses to run on any tie-out failure |
| Cost ratio O/P, Q/P, R/P on these instances | empirical comparative | paired bootstrap, section 7 |
| Anything about ECC2K-130, a speed-up of a full index calculus, or a gap to rho | not claimed | section 9 |

## 3. Arms (one instrument)

All arms call `shadow_solve`, a copy of the production `sat_decompose_chained_s3_with_domain` built from public API (the original is private and hides propagations and `word_ops`). Same factor base, same targets, same limits, same blocking-clause loop, same lift, same solver build.

| Arm | x-domain | first-pair ordering | production twin |
|---|---|---|---|
| P (baseline) | plain list of all kn x-codes | no | `sat_decompose_chained_s3` |
| O | exact orbit encoding (k representatives, n phases) | no | `..._frobenius_orbit` |
| R | plain | yes | none |
| Q | orbit | yes | `..._frobenius_orbit_ordered` |
| A/A | arm P on the sigma-conjugate target sigma(T) (x -> x^2, the field Frobenius), same cell | no | none |

Design caveat, stated up front (contrarian): arms P and O have the same solution set and the same m x kn search space. Simultaneous rotation of all summands maps a solution of T to a solution of sigma(T), not of T, so no arm breaks a symmetry of a generic fixed target (only sigma-fixed targets have one, and those are not drawn). Arm O is a re-encoding of the domain constraint, so Q1 is a question about encoding size and propagation strength, not about a symmetry reduction. Arm R/Q add a genuine factor-2 symmetry (swap of the first two summands). No n^(m-1) or D(n,m) = C(n+m-1,m)/n discount is applied to any cost in this cell; the exact class count appears only as information (for prime n, as for 19, 23, 31, the number of orbits of k-orbit m-tuples under simultaneous rotation is exactly k^m n^(m-1), since no non-trivial rotation fixes a tuple). Cost is in solver operations, never divided by a class count.

## 4. Instances, controls, matching

Cell = (a = 0, n, m, k, seed). Factor base: k full-length (length n) Frobenius orbits drawn from random subgroup points, both signs, so 2kn points and kn distinct x-values; identical for every arm (`build_cell`). Matching:
- Factor-base size matched by construction (same object). Relation yield matched by construction: every arm solves the same target list and the oracle fixes the true verdict; yield differs only through censoring, which is reported (anytime table at common budgets), is counted per arm and per direction, and makes a stratum unresolved when either arm exceeds 20% (section 7). Every cost is a cost per relation, so unequal yield is priced, not hidden.
- Targets (indexed, derived from the cell seed, arm-independent): natural (random subgroup multiple), planted (sum of m random factor-base points, always SAT: positive control, proves the harness finds relations), coset (a point outside the order-r subgroup, never a sum of subgroup points: negative control). Every x of the chain is restricted to factor-base x-codes, so the chained S3 system for a coset target has no model at the x level and is refuted by SAT alone: no model is produced and the group lift never runs. The coset stratum therefore tests SAT refutation and the harness's absence of false `found`, not the lift. Natural targets with oracle verdict false are negatives too (almost none exist, section 5).
- Oracle: exact meet-in-the-middle sumset over ordered tuples with repetition, no solver. A `found` with oracle false, a `refuted` with oracle true, or a certificate that does not re-add to the target is an `agree = false` row, which makes the stratum invalid.
- Certificate: `verify_cert` re-adds the returned points in the group and checks membership in the factor base; independent of the solver.
- A/A: plain on sigma(T) against plain on T. The solver is deterministic, so rerunning the same CNF with the same seed is not an A/A; the conjugate is a genuinely different CNF of an isomorphic problem, so it measures instance-level noise (conservative for the paired O/P comparison, whose pairs share the target).
- Detector self-check: the analysis injects ratios 1.05 to 2.0 into the A/A log ratios and reports which are detected. Injection is not a real positive-effect control; there is no arm with a known real effect, and this is a limitation.
- Interleaving: jobs are shuffled with a fixed seed across (cell, arm); targets inside a job are in index order for every arm. Arm labels are not masked in the analysis (the driver is mechanical; the decision rule is fixed in section 7).

## 5. Sizes, resolution and power

Group order. Every target, every factor-base point and every sum lies in the subgroup generated by the curve generator, of order r = the largest prime factor of #E (it must exceed the cofactor; `KoblitzCurve::new` enforces this). It is not 2^n and not #E/4: for a = 0, #E(n=19) = 4 * 130873, #E(23) = 4 * 2095853, #E(31) = 4 * 373 * 1439393 (r = 1439393, cofactor 1492), and #E(17) = 4 * 137 * 239 has r = 239 < h = 548, so no usable subgroup exists. The unit test `design_numbers_match_the_protocol_table` asserts these r values (computed by the curve code) and the table below, and that `KoblitzCurve::new(0, 17)` is None. An earlier draft of this table used 2^n and was wrong.

Cells (k from `suggest_k`: smallest k >= 2 with expected m-sums (2kn)^m / m! >= 0.7 r; yield = 1 - exp(-sums / r), a Poisson approximation):

| n | r | m | k | fb points | natural-target yield (expected) |
|---|---|---|---|---|---|
| 19 | 130873 | 4 | 2 | 76 | 1.0000 |
| 19 | 130873 | 5 | 2 | 76 | 1.0000 |
| 19 | 130873 | 6 | 2 | 76 | 1.0000 |
| 23 | 2095853 | 4 | 2 | 92 | 0.7593 |
| 23 | 2095853 | 5 | 2 | 92 | 1.0000 |
| 23 | 2095853 | 6 | 2 | 92 | 1.0000 |
| 31 | 1439393 | 4 | 2 | 124 | 0.9989 |
| 31 | 1439393 | 5 | 2 | 124 | 1.0000 |
| 31 | 1439393 | 6 | 2 | 124 | 1.0000 |

Decision on k, made deliberately: k = 2 is the minimum the orbit arm allows (it needs two representatives), and it already overshoots the 0.7 r design aim in 8 of the 9 cells. The design aim (yield about 0.5) is therefore NOT met; the cells are saturated (many solutions per target) and fewer orbits are not available. We keep k = 2 rather than inflate n or m. Consequences, stated before the data: (i) the regime is dense, as in section 9; (ii) cost per relation here is the cost of finding one of several solutions; (iii) there are almost no natural UNSAT targets (about 24% of the 40 naturals at (23,4), about 0 elsewhere), so the unsat stratum is the coset negative control.

Targets per cell: 40 natural, 20 planted, 8 coset (reduced from 20: refuting one coset target at n = 7, m = 4, k = 2 already needs 162,932 conflicts, so at n >= 19 refutations will almost never finish under any cap in section 8 and 20 of them would only burn a quarter of each job's time). N = 68 per arm, 5 solves per target (4 arms + A/A). Strata: planted (20), natural_sat (about 40; about 30 at (23,4)), unsat (the 8 coset plus oracle-false naturals), natural_all (40). Claims are made on natural_sat, natural_all and planted. The unsat stratum has fewer than 10 targets (`MIN_TARGETS`) in 8 of 9 cells and is expected to be fully censored: refutation cost cannot be measured here, the stratum is a negative control only (no false `found`, certificate and oracle agree), it is not a basis for any cost claim, and in particular the one genuine 2-fold symmetry of arms R and Q (swap of the first two summands), which matters most for refutation, is NOT tested by this cell.

Resolution (computed, not assumed). The deciding statistic is the cost-per-relation ratio with a bootstrap interval; for it, 95% half-width on the log scale = 1.96 * s_boot, MDE = exp(1.96 * s_boot), where s_boot is the bootstrap sd of the log ratio, printed per stratum from the A/A rows (`aa_boot_sd_log_word_ops`, `aa_mde_ratio_95`). To a good approximation s_boot = s / sqrt(N) for a paired per-target log-ratio sd s:

| s (log) | MDE N=20 | N=40 | N=80 | N needed for the C3 margin (half-width <= ln(1/0.95) = 0.0513) |
|---|---|---|---|---|
| 0.5 | 1.24 | 1.17 | 1.12 | 365 |
| 1.0 | 1.55 | 1.36 | 1.24 | 1460 |
| 1.5 | 1.93 | 1.59 | 1.39 | 3285 |

The C3 margin can be resolved only if s <= 0.0262 sqrt(N): 0.117 at N = 20, 0.166 at N = 40, 0.234 at N = 80. The C3 strata (natural_all, natural_sat) have N about 40, so the paired sd must be below about 0.17. SAT solve costs are heavy-tailed; s >= 1 is the realistic prior. The design can resolve about a 1.4x difference at N = 40, not the 5% margin, and "unresolved" is the most likely honest grade for C3. The achieved s_boot and MDE per stratum come from the A/A rows.

Dominant noise source of the priced legs (Amendment 2). GAE = `lift_adds` + price x `word_ops`. At price 0 the cost is `lift_adds` alone. In these dense cells each found model is lifted by a depth-first search over 2^m sign patterns; enumerating that search for m = 4, 5, 6 gives a mean add count of 17, 33.5, 66 with a coefficient of variation of about 0.5, essentially random per target and independent between arms P and O. At N = 40 the 95% interval factor of the price-0 cost-per-relation ratio is about 1.25 to 1.27 even for identical encodings, so an upper bound <= 1.0526 at price 0 occurs for only about 6 to 7% of cells under the null. That leg therefore measures lift luck, not the encoding. Consequence: the price-0 leg does not decide C3 (section 7 rule 9, Amendment 2); it stays in the section 5a band, where it can only block a ranking. For each priced leg the A/A resolution is computed and printed (`aa_priced_gae`, three prices, per stratum), so the achieved resolution of every leg that is used is visible. The C3 legs that remain are `word_ops` and the priced GAE at 0.1 and 1, whose noise is the paired `word_ops` sd above (the lift adds enter at weight 1 against 0.1 or 1 x `word_ops`).

## 6. Metrics

Primary: solver `word_ops` (deterministic, host-independent, includes unit propagation and XOR work). Secondary: conflicts, propagations, wall time. Wall time is valid only on an isolated host (4 shared jobs perturb it) and never decides alone. Priced currency for the section 5a band: GAE = `lift_adds` (group additions in the lift, native work) + price * `word_ops`, price in {0, 0.1, 1}.

Cost per relation (the deciding statistic, all arms, all prices, all four metrics): over a stratum's targets, CPR(arm) = (sum of the arm's cost over ALL its targets, censored solves included at their spend) / (number of relations the arm produced), where a relation is a `found` solve in the satisfiable strata and a `refuted` solve in the unsat stratum. The compared quantity is CPR(X)/CPR(P) with a 95% percentile bootstrap interval (2000 resamples of targets, drawn jointly for both arms, fixed seed). No pair is dropped. Censored spend is a lower bound of the true cost, so it flatters the arm censored more; section 7 therefore bars any claim in favour of the arm censored more and gates censoring per arm. The earlier survivors-only paired geometric mean is still printed (`survivors_only_secondary`, with the dropped pairs counted per direction) but decides nothing. Also printed: the anytime table (yield and cost per relation on a geometric budget grid up to the common censoring horizon) and, per arm and price, the absolute cost per relation (`cost_per_relation`). Distance to matched rho: an analytic placeholder (pi r / 2)^(1/2) additions with (k+1) relations needed, printed only for arm P; it is a distance indicator only, not the matched measured rho reference with the section 5a band (and its unpriced-counter listing) that the validation gate requires. This cell therefore produces no leaderboard or `benchmarks/README.md` row. Exact class count k^m n^(m-1) is printed per cell as `class_count_info_only` (and by `plan`); nothing is divided by it.

## 7. Decision rule (fixed now; implemented in `decide` and `analyze` exactly as written)

Per cell, per stratum, per arm X in {O, R, Q} against P, on the cost-per-relation ratio CPR(X)/CPR(P) (section 6) of the primary metric `word_ops`, then each secondary metric. Let `targets` be the targets shared by X and P, `cens_X` and `cens_P` the censored counts of each arm. Apply in order, the first that fires is the label:
1. any `agree = false` row, or a `found` row without a valid certificate, ANYWHERE in the cell (every arm, A/A included, every stratum) -> `invalid_oracle_disagreement`. Tie-out failure: analysis refuses to run.
2. any (arm, conj) job of the cell lacks a target that another has (a timeout lost rows) -> `invalid_incomplete_rows`.
3. `targets` < 10 -> `unresolved_too_few_targets`.
4. cens_X / targets > 0.2 OR cens_P / targets > 0.2 (per arm, not pooled) -> `unresolved_censored`. Caps are never raised after seeing data.
5. an arm produced no relation, or more than 5% of bootstrap resamples have none -> `unresolved_no_relations`.
6. the A/A interval (plain on sigma(T) against plain on T, same statistic) excludes 1 -> `invalid_aa_excludes_one`. The A/A gate has a 5% false-invalid rate per test by construction (about 36 stratum tests per metric): an invalid flag is a reason to look and replicate, not proof of a biased instrument.
7. upper bound < 1 -> `improvement` if cens_X <= cens_P, else `unresolved_asymmetric_censoring`; lower bound > 1 -> `regression` if cens_P <= cens_X, else `unresolved_asymmetric_censoring` (a claim in favour of the arm censored more is never made); otherwise `unresolved_no_difference`.
8. Section 5a: an improvement or regression on `word_ops` is ranked only if the priced-GAE CPR interval is on the same side of 1 at all three prices (`section_5a_band`); the merged result is `final_grade_word_ops` (`improvement_ranked`, `regression_ranked`, `not_ranked_unstable_across_5a_band`, or the label above). At price 0 only `lift_adds` is priced, so a re-encoding that changes only solver work will normally be "not stable across band": that is the rule working, not an error.
9. C3 is supported at a cell (`c3_grade_O` = supported) only if, for arm O in BOTH natural_all and natural_sat, the upper bound of the `word_ops` CPR ratio is <= 1/0.95 = 1.0526, the label is not invalid, `unresolved_censored`, `unresolved_too_few_targets` or `unresolved_no_relations`, cens_O <= cens_P, and the upper bound of the priced-GAE CPR ratio is <= 1.0526 at BOTH positive prices 0.1 and 1 (price 0, lift additions only, is excluded from C3: Amendment 2). `contradicted` if, in a valid stratum, the label is `regression` (so the lower bound is > 1, the stratum passed the per-arm censoring gate and cens_P <= cens_O) AND the LOWER bound of the `word_ops` CPR ratio is above the margin 1.0526, i.e. O is dearer than 1/0.95 with 95% confidence (an interval such as [1.03, 1.07] is `unresolved`); else `unresolved`. `summary.json` carries the per-cell grade and `c3_cells_supported` of the cells analysed (k of 9). Support at one cell is not support of C3 at all of n = 19, 23, 31; the claim is graded per cell.
No pooling across m or n, no post-hoc strata, no subgroup that is not listed in section 4. Multiplicity: 9 cells x 3 arms x 4 strata; any single "improvement" is a lead for a replicate seed, not a finding.

## 8. Runtime estimate and the cap rule (code reasoning only; nothing has been timed)

- Oracle per cell: level-a sumset of at most (2kn)^a / a! points (k = 2: at most about 3e5 points at level 3 for n = 31), seconds per cell; memory small.
- Cap rule (frozen, executed by the `cap` subcommand, RUN_CARD section 3). The pilot runs ALL nine cells (arms P and O, no A/A, 14 targets each, seed 1027, cap 20000), so every (n, m) has its own measurement; there is no extrapolation from one cell to another. Per cell, t_conf = max over arms P and O of sum(wall_s - build_s) / sum(conflicts) (a ratio of sums over rows with conflicts > 0; the earlier median of wall_s/conflicts was dominated by CNF construction in solves that ended in a handful of conflicts and would have inflated t_conf), b = max build_s over the cell's rows, and CAP = the largest value in {20000, 50000, 100000, 200000, 500000, 1000000, 2000000} with b + CAP * t_conf <= 180 s. If even 20000 exceeds 180 s the cell is recorded as `O-IMPEDIMENT(budget)` and not run. The caps are printed by one command before the main stage, passed to `run_stage.sh` as `n:m:k:CAP` tokens, and never changed afterwards. Caps differ between cells (time per conflict grows with m); nothing is compared or pooled across cells, and within a cell all arms share the cap.
- Job bound: 68 targets x 180 s = 12240 s < the 14400 s job timeout, so a job that obeys the budget finishes; a job that does not loses its tail rows, which `analyze` reports as `invalid_incomplete_rows`, never silently. 5 jobs per cell (P, O, R, Q, A/A) x 9 cells, 4 in parallel: worst case 45 x 3.4 h / 4 = 38 h; most solves end before the cap (planted and natural found), only the 8 coset targets per job run to the cap (about 12% of a job), realistic 10% to 30% of the worst case, about 4 to 12 h. Not a measurement; the pilot replaces this with numbers.

## 9. What does not count

- Any ratio from a stratum with either arm's censored fraction > 0.2, a claim in favour of the arm censored more, an A/A that excludes 1, an incomplete job set, or an oracle/certificate disagreement anywhere in the cell.
- The pilot stage (different seed, small N, used only to set the cap); it is excluded from inference.
- wall_s from a loaded or shared host as a deciding metric; conflicts alone (an encoding can change cost per conflict).
- Cost in units divided by n^(m-1) or by D(n,m), or any credit for solutions "represented" by a rotation class.
- A better yield through a larger cap or a different seed chosen after looking.
- Any cost of refutation, and any statement about arms R and Q on UNSAT problems (the coset stratum is expected to be censored).
- Any statement about the lift (coset targets never reach it).
- Relabelling: a re-encoding that gives the same solutions and fewer clauses is a result about encoding size, not a new attack. An orbit arm that wins only because the plain list encoding is bloated (kn codes) is an accounting improvement of the baseline encoding, reported as such.
- Any statement about ECC2K-130: these cells are dense (expected sums at or above 0.7 of the subgroup order r, yield near 1 in 8 of 9 cells), the real regime has yield far below 1, and the factor base is 2 to 8 orbits, not the hundreds an index calculus needs. Transfer to m = 10 or to large n is untested.
- Any leaderboard or benchmark row (no matched measured rho reference, no VALIDATION.json).

## 10. Prior (before data)

O/P in word_ops cost per relation is likely within 0.5x to 2x; I expect the orbit domain to have fewer clauses and similar propagation strength, so a modest gain is plausible but the main risk is a loss from the rotation-choice wires. Probability that C3 is graded "supported" at a cell given the resolution above: below 15% (and, with the price-0 leg removed from the C3 test, set by the `word_ops` resolution of section 5; had the price-0 leg stayed, it would have been about 6% per cell even for identical encodings). Probability that at least one cell/stratum shows an unflagged, 5a-ranked improvement of arm O: about 20%. Arms R and Q: the first-pair swap is a genuine 2-fold symmetry, but it pays on refutation, which this design cannot measure; on SAT strata (first solution found) I expect about no effect (probability of a ranked improvement about 10%). These are priors, not results.

## 11. Amendment 1 (2026-10-27, before any data; what changed and why)

Reviewer findings, verified in code or by recomputation:
1. Decision rule could grade C3 or an improvement on survivors only (pairs with a censored arm dropped; only a pooled censored fraction guarded it). Changed: the deciding statistic is the cost-per-relation ratio (censored spend counted), the censoring gate is per arm, claims in favour of the more-censored arm are barred, dropped pairs are reported per direction in the secondary statistic. Tests: `censoring_asymmetry_cannot_flatter_the_arm_censored_more`, `analysis_detects_a_known_ratio_and_passes_a_a`.
2. Cap rule could not be executed (pilot ran only n = 17; estimator biased by construction time; one cap for m = 4, 5, 6; 80 x 180 s equalled the job timeout). Changed: all nine cells piloted, ratio-of-sums estimator without build time, per-cell caps via the `cap` subcommand, 68 x 180 s per job. Test: `cap_rule_uses_ratio_of_sums_excluding_build_time_per_cell`.
3. The yield table used 2^n as the group order. Recomputation also showed that the review's replacement (#E/4) is wrong for n = 31 (r = 1439393, not 536893589) and, more seriously, that a = 0 at n = 17 cannot be built (`KoblitzCurve::new` returns None because r = 239 < h = 548), so the three n = 17 cells would have failed at start. Changed: n = 17 replaced by n = 19 (a = 0, r = 130873, #E = 4r, builds with full orbits: test `every_planned_cell_builds_with_full_orbits`); k from the real r (all cells k = 2, saturated; stated, not hidden); coset targets 20 -> 8; the unsat stratum is a negative control only and expected to be censored; the claim about the lift for coset targets corrected (SAT-only refutation). C3 at n = 17 is not tested by this cell.
4. Cost per relation was emitted only for arm P and the decision used per-solve survivors. Changed as in 1: cost per relation per arm, price and metric with bootstrap intervals; priced band uses the same statistic.
Caveats also addressed: resolution arithmetic corrected (section 5); a minimum of 10 shared targets; agreement checked over every row of the cell; job completeness checked; bootstrap seeds now depend on the cell and stratum names, not their lengths; per-cell C3 grade and the k-of-9 count are computed by `analyze` and merged with the 5a band; the exact class count is emitted; `run_stage.sh` writes commit, dirty paths, binary sha256, rustc version, host and arguments to `provenance.txt`. Not addressed: R has no production twin; the A/A injection is still an offline detector, not a real positive control; the 5% false-invalid rate of the A/A gate is stated; tie-out failures are found only at analysis; the rho reference remains an analytic placeholder; n = 31 CNF construction is not preflighted (the pilot does it).

## 12. Amendment 2 (2026-10-27, before any data; what changed and why)

Round-2 review findings, each verified in code, and the change:
1. `contradicted` was graded on the interval UPPER bound (`regression` with ci_hi > margin), so [1.03, 1.07] counted as contradicted. Changed (section 7 rule 9): contradicted iff the label is `regression` in a valid stratum and the LOWER bound of the `word_ops` CPR ratio exceeds 1/0.95. Code: `c3_grade` with `C3Entry.ci_lo`. Test: `contradicted_needs_the_lower_bound_above_the_margin`.
2. The price-0 priced-GAE leg (lift additions only) carried noise (CV about 0.5 per target, independent between arms) that makes `supported` reachable only by luck (about 6 to 7% per cell under the null), and the stated resolution omitted it. Changed: C3 rests on `word_ops` and the priced GAE at 0.1 and 1 (`c3_priced_ok`); price 0 stays in the 5a ranking band only; the A/A resolution of every priced leg is computed (`aa_priced_gae`); sections 5, 7 and 10 and the README state the N = 40 threshold (s <= 0.166) and the lift noise. Tests: `c3_priced_legs_exclude_the_noisy_price_zero_lift_only_leg`, `identical_encodings_with_noisy_lift_still_grade_supported_and_report_priced_aa` (the synthetic rows now carry random `lift_adds`; the former constant 3 hid the noise).
3. Certificates were not persisted (only `cert_ok`). Changed: each row carries `target_xy` and, for `found` rows, `cert_points` (the m points); `oracle` writes `manifest-<cell>.json` (field polynomial, a, b, generator, r, factor-base points and x-codes, signed orbits, representatives); the verifier re-adds every certificate with its own GF(2^n) arithmetic, no solver and no rerun, and `run_stage.sh` runs it and records a failure. A stage whose verification fails supplies no usable relation. Test: `certificates_and_manifest_reach_disk_and_the_independent_native_verifier_accepts_them` (accepts a real n = 7 certificate, rejects a tampered point and a missing certificate).
4. RUN_CARD was not executable block by block (`$BIN`, `$CELLS_N23`, `$CELLS_N31`, `RUN` undefined in fresh shells; empty cell list ran xargs on nothing; `cap` silently dropped a cell with no conflict rows). Changed: `env.sh` defines ROOT, G1, BIN, RUN, R and every block sources it; RUN is minted once into `reports/RUN`; one self-contained block per stage with the cell list taken from `caps.txt` by `grep` in the block; `cap` prints `n:m:k:NODATA` for a cell with pilot rows but no conflicts > 0 (recorded by `run_stage.sh` as `O-IMPEDIMENT(no-pilot-data)`); the card checks that `caps.txt` has exactly 9 lines, one per cell; `run_stage.sh` exits 2 on an empty cell list or when every cell is skipped and uses `xargs -r`. The checkpoint gate is stated as a session-side launcher gate run before submitting, never inside a remote job. Tests: `cap_lists_a_cell_without_conflict_rows_as_nodata`, `run_stage_rejects_an_empty_cell_list`.
Not changed: caveats of the round-2 review (frozen-set binding, n = 19 for n = 17, disclosure rules, censored spend as a lower bound).

## 13. Amendment 3 (2026-10-27, before any data): native certificate verifier

The independent certificate verifier introduced in Amendment 2 was a Python
script. AGENTS.md forbids Python for certificate verifiers. It is now the
standalone Rust crate `verify_certs/` (`g1-verify-certs`, std and serde_json
only), which shares no code with the driver. It makes the same checks:
- each certificate has m points, all on the curve;
- every certificate x-code is in the manifest's factor base;
- the points sum to the target;
- the manifest's factor-base points lie on the curve.

It uses the same exit codes (0 = all pass, 1 = any failure, 2 = unreadable
input). `run_stage.sh` builds it if absent and runs it. The driver test runs
it on a real n = 7 certificate, a tampered point and a missing certificate.
The design and the decision rule are unchanged.
