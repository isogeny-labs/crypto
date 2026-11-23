# Protocol (frozen before any IC data): `ksum free=1` on the pruned k-sum incumbent, random-binary curves n = 21, 24, 27

2026-11-19. Lane B (generic binary / Koblitz, one target, cold, no weak families, no GHS, no multi-target).
Every number in this lane is `[pcc:unverified]`. Frozen before any index-calculus (IC) record was produced. The only data taken
before this file: one rho-only smoke session (`smoke/`, 6 targets, B27) whose IC arms errored (see "Blocker" below). No prediction
below was edited after the first IC run; deviations go in README.md.

## 1. Incumbent (desk result, no new measurement)

Sources read: `docs/ic/LEADERBOARD.md` (Table B, "Table B random-binary rows re-measured with the k-sum tables", "k-sum table build
without dead-end partial sums", "K1 Koblitz, one target ... pair-table factor base sized for n"), `research/ic_tableb_binary_ksum_20261115/`,
`research/ic_ksum_prune_20261115/`, `research/ic_ksum_sibling_20261118/` (sib is prime-curve only), `research/ksum_m67_pilot_20261115/`
(m = 6, 7 worse than m = 5 on B27), `research/koblitz_ksum_fold_study_20261115/` (Koblitz fold: independent validation FAILED, wrong incumbent),
`research/ic_fb_size_model_20261119/FILTER_ECONOMICS.md`, `research/ic_binary_unsat_cost_20261119/` (negative), `research/lit2026_w4_early_abort_20261011/`,
`research/pdp_ml_sat_ceiling_20261027/`.

The best validated one-target IC arm on random-binary curves with n <= 29 is the pruned negation-folded k-sum table:
`ic.pipeline`, `factor_base = binary-multiplier:size=N,mult=1`, `oracle = ksum:m=4|5,prune=1`, with (m, N) per curve
n21: (4, 10) / (5, 9); n24: (4, 12) / (5, 9); n27: (4, 16) / (5, 12). Its distance to the matched negation-map rho
(`research/ic_ksum_prune_20261115`, LEADERBOARD.md): n21 5.03 / 5.15 / 5.76, n24 5.28 / 5.42 / 6.06, n27 5.65 / 5.82 / 6.59 at w = 0 / 0.1 / 1.
Koblitz one-target: the best row (kf3-c1g-sr, K1 n23) was not validated (VAL-20261115-f7df34 failed; its gain is the generator-orbit base);
the K1 table row is 5.6-5.95x rho at n23. Random binary is therefore the incumbent lane.

## 2. Cost decomposition (for the lever)

Cost S (GAE per sqrt r) = build(E) + (trials until the matrix is solvable) x per-trial cost + probe confirmation adds + elimination.
E = 2^(m-1) C(N, m) table entries. The pruned build is one add per kept partial sum; trials are one walk step plus one probe each;
a probe hits with probability about E/r; the linear system has N base unknowns plus log Q, so the incumbent needs about N + 1 target hits.
Set-up (build) is 0.4-0.6 of S at w = 0 (ksum_m67_pilot, ic_ksum_prune). With k target hits needed, cost is about E + k r / E (units of adds), so
S_min about 2 sqrt(k). The incumbent pays k = N + 1 hits.

## 3. Candidate (one change): `free=1`

`ksum:m=k,prune=1,free=1` verifies the build's key collisions (two distinct signed k-sums with the same abscissa) and feeds them to the
relation loop as target-free rows (once, with the first trial). This is the existing `free` flag of `KSumOracle`. A search of
`research/` for `free=1` finds it measured on no k-sum arm: only the Frobenius pair table (`research/frobenius_table_free_rows_20261106`,
zero non-trivial rows, collisions below 1 expected) and curve25519. The k-sum table is the first table here large enough in E^2/r to expect
collisions: expected number of colliding pairs c = E^2 / (2 r) (abscissa orbits: about r keys). Each independent collision row replaces one
target hit, which costs about r/E trials. In the model of section 2 with c = E^2/(2r) free rows, cost is E/2 + (N+1) r/E, minimised at
E = sqrt(2 (N+1) r) with S about sqrt(2 (N+1)), against 2 sqrt(N+1) without: at best 0.71x, reached at a larger table than the incumbent's.
So the measured question has two parts: (a) free=1 at the incumbent's own (m, N); (b) free=1 with the table re-sized (N + 1, N + 2).
Not a relabelling: same group, same base, same trials rule, same linear algebra; the rows are verified collisions and every add and probe
of the verification is counted by the harness.

## 4. Design

Curves (explicit binary, from `research/ic_ksum_prune_20261115/specs/B<n>-r1.json`): B21 `icv1-f2m21-t2591-de54d72e`, B24 `icv1-f2m24-tm3047-2d0c0e8c`,
B27 `icv1-f2m27-t12543-569dca8b`. Primary rows: all three (n <= 29). 3 replicates per curve, 48 public targets (law `hash_to_subgroup_v1`, seeds in
`make_specs.py`), 3 rounds (algorithm seed per (curve, replicate, target, round) shared by every arm, so every comparison is paired by run).
Arms (every one `ic.pipeline`, `targets=walk`, `max_trials=13528399621`, `binary-multiplier:size=N,mult=1`):
`k{4,5}-p-N{-1,0,+1,+2}` (incumbent family, prune=1) and `k{4,5}-pf-N{-1,0,+1,+2}` (candidate family, prune=1,free=1): 16 arms per curve, N0 = the frozen sizes of section 1.
Reference: `rho.negation` (matched rho, `rho-neg`) and `rho-neg-aa` (control) in an ecbench session on the same targets; priced at w = 0, 0.1, 1 GAE per
unpriced native operation by `ecbench band`. IC arms are priced in the same unit, at the same three weights (section 6).

## 5. Tests

T1 (fixed configuration, paired): for each (m, N) of the incumbent family at N0, R_w = cost(k-pf) / cost(k-p), paired by (replicate, target, round),
cluster bootstrap by target (4000 resamples), 95% interval.
T2 (strictest against strictest, primary): R*_w = min over candidate arms (pooled mean at w) / min over incumbent arms (pooled mean at w); both minima chosen on the
same data, so the winner's-curse advantage is symmetric (8 candidate and 8 incumbent arms). Interval by paired bootstrap of the two chosen arms.
T3 (A/A): rho-neg-aa / rho-neg = 1 exactly (deterministic counts); an incumbent arm run twice (`k5-p-N0` and a duplicate `k5-p-N0-aa` in replicate 1) must give ratio 1.000.
T4 distance to rho: S(strictest candidate) / S(rho-neg) at w = 0, 0.1, 1, per row (never a pass/fail gate).
Improvement is declared for a row only if the upper 95% end of R*_w is below 1 at all three w, pooled and in each of the 3 replicates.
Class if it holds: constant-factor (generic collision-class method, bounded by sqrt(r), cannot beat rho or BSGS). No exponent claim.

## 6. Unit and cross-check

`ecbench` at HEAD cannot run the `ksum` oracle on binary curves (`oracle ksum does not run on a binary curve`: the dispatch line of commit d87eb39548 is not in
`src/cryptanalysis/ecbench/methods.rs` at HEAD; src is frozen for this lane). The IC arms therefore run through
`examples/icbin_ksum_free.rs`, which calls the same library entry points as `solve_ic_binary` (`run_pipeline_with_base`, `BinaryMultiplierBase`, `KSumOracle`,
`Calibration::pin` with the curve slug, `PipelineSpec`), on the exact targets of the ecbench rho session. S = `RunReport.total_gae` / sqrt(r); S(w) = S + w U / sqrt(r),
U = unpriced native operations plus counted table work (`table_insert_attempts`, marker `counted:`), as `ecbench band` does by default since 2026-11-08
(`src/cryptanalysis/ecbench/band.rs`). Not claimed: that this equals an ecbench record bit for bit. Cross-check (registered): the example's pooled mean S for
`k5-p-N0` and `k4-p-N0` at w = 0 / 0.1 / 1 must lie within 8% of the committed `research/ic_ksum_prune_20261115` values (B21: 6.34 / 7.06 / 13.60 for k5-p, B24 k4-p 6.40 / 7.15 / 13.96,
B27 k5-p 6.04 / 6.79 / 13.57). If it does not, the unit is not matched and no ratio to rho is reported (ratios between IC arms, same unit, stay valid).

## 7. Predictions (falsifiable; none edited afterwards)

P1 (hard gate): every run of every arm recovers the planted/verified logarithm (0 failures; free rows never produce a wrong log).
P2: expected collision count c = E^2 / (2 r) is >= 1 at the incumbent sizes of all three curves, and the number of free rows emitted (counter `free_rows_emitted`) has mean >= 0.5 per run at n27 (N0).
P3 (T1): at N0, R_0 of k5-pf over k5-p lies in [0.90, 1.00] on n21, n24 and n27, never above 1.02; the saving grows with n.
P4 (T2): the strictest candidate beats the strictest incumbent with upper 95% end below 1 at all w on at least one of the three curves; point estimate R*_0 in [0.75, 0.95].
P5: the strictest candidate arm's N is above the incumbent's N0 on at least 2 of 3 curves (the optimum table size moves up with free rows).
P6: w-dependence is weak: |R*_1 - R*_0| < 0.06 (free rows save trials, which are priced like adds; the native work they save is table probes).
P7 (T4): the strictest candidate stays above rho at every w and curve: S / S_rho >= 3.5 (no crossing); the distance shrinks by at most 25% from the incumbent's.
P8 (cross-check, section 6): example unit matches committed incumbents within 8% (3 of 3 curves, all three w).
Kill rule: if the upper end of R*_w >= 1 at some w on all three curves, the lever is negative and is reported as such.

## 8. Not claimed

Wall time, any leaderboard/benchmarks row (no validation gate: no VALIDATION.json, no second-host replay, no independent validator), any scaling beyond n = 21-27,
anything about security. Local, single thread (RAYON_NUM_THREADS=1), L0 (counted cost). Blocker disclosed: ecbench at HEAD cannot reproduce the committed binary
ksum records (dispatch lost), a regression for replay of `research/ic_tableb_binary_ksum_20261115` and `research/ic_ksum_prune_20261115` sessions at HEAD.
