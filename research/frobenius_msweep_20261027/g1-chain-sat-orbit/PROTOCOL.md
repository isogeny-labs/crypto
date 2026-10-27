# G1 protocol (frozen before any data): chained-S3 SAT, plain x-domain against the exact Frobenius-orbit x-domain

Status: prepared, not run. Design frozen by this file; changes after data exist go in an AMENDMENT.md with the reason, never by editing this file.
Driver: `examples/frobenius_msweep_g1_chain_sat_orbit.rs` (auto-discovered example; Cargo.toml needs no entry). Run commands: `RUN_CARD.md`.

## 1. Question

Audit claim C3 (2026-10-26) reads: "sound orbit encodings are >= 0.95x as fast as plain at n = 17, 23, 31", and it is UNSUPPORTED because
H-BINSTD-c72284 Stage 2 never ran. No solver-level Frobenius-symmetry experiment exists at m >= 3. This cell asks, at m = 4, 5, 6 and
n = 17, 23, 31 (Koblitz a = 0, y^2 + xy = x^3 + 1), on identical instances through the same chained-S3 loop:

  Q1. Is the cost per relation of the exact Frobenius-orbit x-domain (arm O) at most 1/0.95 times that of the plain x-domain (arm P)?  (C3, non-inferiority)
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

Design caveat, stated up front (contrarian): arms P and O have the same solution set and the same m x kn search space. Simultaneous rotation of all summands maps a solution of T to a solution of sigma(T), not of T, so no arm breaks a symmetry of a generic fixed target (only sigma-fixed targets have one, and those are not drawn). Arm O is a re-encoding of the domain constraint, so Q1 is a question about encoding size and propagation strength, not about a symmetry reduction. Arm R/Q add a genuine factor-2 symmetry (swap of the first two summands). No n^(m-1) or D(n,m) = C(n+m-1,m)/n discount is applied to any cost in this cell; the exact class count appears only as information (for prime n, as for 17, 23, 31, the number of orbits of k-orbit m-tuples under simultaneous rotation is exactly k^m n^(m-1), since no non-trivial rotation fixes a tuple). Cost is in solver operations, never divided by a class count.

## 4. Instances, controls, matching

Cell = (a = 0, n, m, k, seed). Factor base: k full-length (length n) Frobenius orbits drawn from random subgroup points, both signs, so 2kn points and kn distinct x-values; identical for every arm (`build_cell`). Matching:
- Factor-base size matched by construction (same object). Relation yield matched by construction: every arm solves the same target list and the oracle fixes the true verdict; yield differs only through censoring, which is reported (anytime table at common budgets) and which invalidates the stratum above 20%.
- Targets (indexed, derived from the cell seed, arm-independent): natural (random subgroup multiple), planted (sum of m random factor-base points, always SAT: positive control, proves the harness finds relations), coset (a point outside the order-r subgroup, never a sum of subgroup points: negative control, x-feasible so the solver must refute through the group lift). Natural targets with oracle verdict false are negatives too.
- Oracle: exact meet-in-the-middle sumset over ordered tuples with repetition, no solver. A `found` with oracle false, a `refuted` with oracle true, or a certificate that does not re-add to the target is an `agree = false` row, which makes the stratum invalid.
- Certificate: `verify_cert` re-adds the returned points in the group and checks membership in the factor base; independent of the solver.
- A/A: plain on sigma(T) against plain on T. The solver is deterministic, so rerunning the same CNF with the same seed is not an A/A; the conjugate is a genuinely different CNF of an isomorphic problem, so it measures instance-level noise (conservative for the paired O/P comparison, whose pairs share the target).
- Detector self-check: the analysis injects ratios 1.05 to 2.0 into the A/A log ratios and reports which are detected. Injection is not a real positive-effect control; there is no arm with a known real effect, and this is a limitation.
- Interleaving: jobs are shuffled with a fixed seed across (cell, arm); targets inside a job are in index order for every arm. Arm labels are not masked in the analysis (the driver is mechanical; the decision rule is fixed in section 7).

## 5. Sizes, resolution and power

Cells (k from `suggest_k`: smallest k >= 2 with expected m-sums (2kn)^m / m! >= 0.7 * 2^n):

| n | m | k | fb points | natural-target yield (expected) |
|---|---|---|---|---|
| 17 | 4 | 2 | 68 | 0.999 |
| 17 | 5 | 2 | 68 | 1.000 |
| 17 | 6 | 2 | 68 | 1.000 |
| 23 | 4 | 3 | 138 | 0.835 |
| 23 | 5 | 2 | 92 | 0.999 |
| 23 | 6 | 2 | 92 | 1.000 |
| 31 | 4 | 8 | 496 | 0.691 |
| 31 | 5 | 3 | 186 | 0.578 |
| 31 | 6 | 2 | 124 | 0.905 |

Per cell N = 80 targets (40 natural, 20 planted, 20 coset) x 4 arms + 80 A/A solves. Strata analysed: planted, natural_sat, unsat (coset plus oracle-false natural), natural_all.

Resolution (computed, not assumed): with paired log-ratio sd s over N pairs the 95% half-width is 1.96 s / sqrt(N) on the log scale, MDE = exp(that).

| s (log) | MDE N=20 | N=40 | N=80 | N needed for the C3 margin (half-width <= ln(1/0.95) = 0.0513) |
|---|---|---|---|---|
| 0.5 | 1.24 | 1.17 | 1.12 | 365 |
| 1.0 | 1.55 | 1.36 | 1.24 | 1460 |
| 1.5 | 1.93 | 1.59 | 1.39 | 3285 |

SAT solve costs are heavy-tailed; s >= 1 is the realistic prior. Consequence, stated before the data: at N <= 80 the design can resolve a 1.25x to 1.6x difference but cannot resolve the 0.95x C3 margin unless the paired sd is below about 0.13. The most likely honest outcome for Q1 at a given stratum is "unresolved, MDE reported", not "supported". Natural_sat has about 0.58 x 40 = 23 (31,5) to 40 targets; the unsat stratum has about 20 targets at n = 17 and 23 (coset only), so negative-control comparisons are resolved only for large effects. The achieved s and MDE per stratum come from the A/A rows and are printed in `summary.json`.

## 6. Metrics

Primary: solver `word_ops` (deterministic, host-independent, includes unit propagation and XOR work). Secondary: conflicts, propagations, wall time. Wall time is valid only on an isolated host (4 shared jobs perturb it) and never decides alone. Priced currency for the section 5a band: GAE = `lift_adds` (group additions in the lift, native work) + price * `word_ops`, price in {0, 0.1, 1}. Cost per relation = total cost / relations found over the same target list, reported per arm and per price, plus the anytime table (yield and cost per relation on a geometric budget grid up to the common censoring horizon). Distance to matched rho: the analysis prints an analytic placeholder (pi r / 2)^(1/2) additions with (k+1) relations needed; it is a distance indicator only, not the matched measured rho reference with the section 5a band that the validation gate requires. This cell therefore produces no leaderboard or `benchmarks/README.md` row.

## 7. Decision rule (fixed now)

Per cell, per stratum, per arm X in {O, R, Q} against P, on the primary metric (and each secondary): the geometric mean of the paired ratio cost(X)/cost(P) over targets where both arms finished and agree with the oracle, with a 95% percentile bootstrap interval (2000 resamples, fixed seed). Then, in order:
1. any `agree = false` row, or a tie-out failure -> invalid (analysis refuses on tie-out failure).
2. censored fraction over the two arms > 0.2 -> unresolved_censored. Caps are never raised after seeing data.
3. A/A interval excludes 1 -> invalid_aa_excludes_one (the instrument is biased).
4. upper bound < 1 -> improvement; lower bound > 1 -> regression; otherwise unresolved_no_difference.
5. Section 5a: an improvement or regression is ranked only if the priced-GAE interval has the same side of 1 at all three prices (`section_5a_band`); otherwise the order is "not stable across band" and nothing is ranked.
6. C3 is supported at a cell only if arm O's primary-metric upper bound <= 1/0.95 = 1.0526 (`c3_noninferior_at_0.95`) in natural_all and natural_sat, with no invalid flag. Support at one cell is not support of C3 at all of n = 17, 23, 31; the claim is graded per cell and summarised as k of 9.
No pooling across m or n, no post-hoc strata, no subgroup that is not listed in section 4. Multiplicity: 9 cells x 3 arms x 4 strata; any single "improvement" is a lead for a replicate seed, not a finding.

## 8. Runtime estimate (code reasoning only; nothing has been timed)

- Oracle per cell: level-a sumset of at most (2kn)^a / a! points plus one group addition per such point per target. Largest: (31,5,3) with about 1.1e6 level-3 points, about 10 to 30 s per target, 80 targets, so 15 to 40 min; (31,4,8) and (31,6,2) are smaller. Memory a few hundred MB per process, 4 processes in parallel.
- SAT job (one cell, one arm, 80 targets): at most 80 x cap x t_conf seconds, where t_conf (seconds per conflict) is unknown and could lie between about 5e-5 and 5e-3 for XOR-heavy chained S3 CNF at these sizes (100x band). The pilot measures it. The cap rule (RUN_CARD) fixes cap x t_conf <= 180 s, so each job is at most 4 h (hard timeout 14400 s) and typically far less, since found/refuted solves end before the cap. 5 jobs per cell (P, O, R, Q, A/A), 9 cells, 4 in parallel: worst case 45 x 4 h / 4 = 45 h, realistic 10% to 30% of that, i.e. about 5 to 15 h. The pilot and n = 17 cells come first and decide whether n = 31 is attempted.
- Not a measurement; the first pilot replaces this with a number.

## 9. What does not count

- Any ratio from a stratum with censored fraction > 0.2, an A/A that excludes 1, or an oracle/certificate disagreement.
- The pilot stage (different seed, small N, used only to set the cap); it is excluded from inference.
- wall_s from a loaded or shared host as a deciding metric; conflicts alone (an encoding can change cost per conflict).
- Cost in units divided by n^(m-1) or by D(n,m), or any credit for solutions "represented" by a rotation class.
- A better yield through a larger cap or a different seed chosen after looking.
- Relabelling: a re-encoding that gives the same solutions and fewer clauses is a result about encoding size, not a new attack. An orbit arm that wins only because the plain list encoding is bloated (kn codes) is an accounting improvement of the baseline encoding, reported as such.
- Any statement about ECC2K-130: these cells are dense (expected sums about 0.7 of the group order, yield near 1), the real regime has yield far below 1, and the factor base is 2 to 8 orbits, not the hundreds an index calculus needs. Transfer to m = 10 or to large n is untested.
- Any leaderboard or benchmark row (no matched measured rho reference, no VALIDATION.json).

## 10. Prior (before data)

O/P in word_ops is likely within 0.5x to 2x; I expect the orbit domain to have fewer clauses and similar propagation strength, so a modest gain is plausible but the main risk is a loss from the rotation-choice wires. Probability that C3 is graded "supported" at a cell given the resolution above: below 15%. Probability that at least one cell/stratum shows an unflagged improvement of arm O: about 25%. Arm R/Q: first-pair ordering halves a symmetric search, so improvement on UNSAT strata is most plausible there (about 40%), but UNSAT strata are small. These are priors, not results.
