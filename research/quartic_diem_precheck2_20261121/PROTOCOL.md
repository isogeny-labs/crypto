# Pre-check 2 for lead #1: Diem line index calculus on prime-order plane quartics, fixed instrument

Status: FROZEN before any measured run (sha256 in `frozen.sha256`). Every number this
study produces is `[pcc:unverified]`.

Parent: `research/quartic_diem_precheck_20261120/` (pre-check 1: STOP under K2, cause
traced to the instrument) and `research/hec_breakthrough_search_20261120/REPORT.md`, rank
#1. The owner approved fixing the instrument and re-running the pre-check under a new
frozen protocol (2026-11-21).

## 1. Question

With the instrument defects of pre-check 1 fixed, how do (a) the counted linear-algebra
work and (b) the total counted cost of Diem's double-large-prime line index calculus scale
with `p` on generic smooth plane quartics over `F_p` whose `Pic^0` has prime order? On the
same curves and solver seeds, how much does each fix change the counted cost relative to
the pre-check-1 instrument?

This is a scaling pre-check. It makes no comparison with rho and cannot produce a
below-rho claim.

## 2. Instrument changes (made and checked before this protocol; development data only)

Source: `src/cryptanalysis/quartic_ladder.rs` and `src/bin/genus_ladder.rs` on top of
`339dfb86af` (the last commit before the change; the frozen pre-check-1 instrument is
`ca72267899`, unchanged in between). The working-tree file hashes at freeze are listed in
`source.sha256`. Binary `genus_ladder.qdiem2`, built from that tree into a separate
`CARGO_TARGET_DIR` in the session scratchpad; sha256 in `binary.sha256`. Every change is
behind an option. The default path (`run_diem`, arm `Q_diem_2lp`) is the frozen
instrument.

1. **Algebraic descent** (`QDescent::Algebraic`, `Ctx::decompose_alg`). Pre-check 1's
   `Ctx::decompose` tests all `~p` rational points against the three conics of the net.
   The new path removes `v^2` with a pivot conic and takes the resultant
   `g = b_1 c_2 - b_2 c_1`, a binary cubic. It finds the distinct rational roots of `g` by
   counted Cantor-Zassenhaus (`proots`), plus the root at infinity. For each root it gets
   the third coordinate by one counted inversion, or, when `b_1 = b_2 = 0`, by the pivot
   quadratic with one counted Tonelli-Shanks square root. Each candidate is checked on all
   three conics (counted), normalised (counted inversion) and looked up in the point
   index. That lookup is the same uncharged identity map that `Ctx::line` uses. A net with
   a common component falls back to the counted scan (`decomp_fallbacks`). The output is
   identical to the scan. This was checked in the unit tests on 16,993 typical classes at 7 primes from 31 to 1033 (2,606 of them decompose) and on 652 constructed triples
   (`decompose_alg_matches_scan`, `decompose_alg_special_nets`) and by identical runs
   (`run_diem_algebraic_descent_same_relations`). The 11 `quartic_ladder` tests pass under
   `cargo test --release --lib quartic_ladder`, with the `crypto` crate at `opt-level = 1`
   because the opt-level-3 test build ran out of memory on this host.
2. **Wiedemann retries: diagnosis.** I added a per-attempt dense rank of the core over
   `F_r` (`--quartic-diag`, uncounted). On development curves at `p = 4093` and `8191`
   (not grid primes), 24 runs had 63 attempts. All 39 failed attempts have the same pattern:
   `rank = rank_no_t`, so the `t` column lies in the span of the others and `t` is not
   determined. The Wiedemann solution passes the residual check but gives a wrong `t`
   (outcome 2). Of the 24 successful attempts, 23 have full rank; the other is rank
   deficient outside `t`. No failure came from the projection, from a degenerate
   recurrence, or from a zero divisor.

   Cause: the line relations are homogeneous. They fix the logarithms only up to
   `alpha v_deg + mu lambda`. Here `v_deg` adds 1 to every `lambda_P` and 4 to
   `lambda_kappa` (a base-point shift), and `lambda` is the true logarithm vector (a
   scale). A decomposition row of `x = [a]G + [b]T` removes only the combination
   `-alpha + mu h a + h b (mu t - tau) = 0`. So `t` is determined only once the rows'
   points `(a_i, b_i)` span the affine plane, or once a line through `P0` enters the core.
   The frozen walk `x + iG` puts every row of one batch on the line `b = const`, so the
   first attempt is singular in `t` by construction. In addition, pre-check 1 collects its
   decomposition rows before the large-prime graph exists, so most of them dangle and
   never reach the cycle matrix. Retries then come from waiting for a second batch, not
   from bad luck. This is an instrument defect. Two fixes:
   - **T kick** (`kick_t`): after every counted decomposition row, the walk steps once by
     `T` (one counted Pic addition), moving later rows off the line.
   - **Descent last, linked rows** (`QSolve::Linked`): decompositions start only after the
     frozen cycle check passes. A decomposition row counts only if it closes a cycle in
     the component of "1" when it is found (counted DSU finds). Before Wiedemann, an
     attempt with no `t` entry in the core is skipped (counted scan). The number of linked
     rows before the first attempt is `--quartic-inhom 4`: three rows in general position
     are needed, plus one. The value 4 was chosen on the development curves: 3 rows gave
     first-attempt failures in 6 of 12 runs, 4 rows in 1 of 12. Failed attempts keep the
     frozen `+4` rows and retry, and every retry is charged.
3. **Binary flags** (no effect without them): `--quartic-arms`, `--quartic-seeds`
   (curve seeds of the quartic cells, same admission path `make_instance(q, s, s, 1)`),
   `--quartic-inhom`, `--quartic-diag` (not used in measured runs).

Not changed: the point enumeration `rational_points` (one-time `O(p log p)`, counted in
`rel_enum`, needed for the factor-base draw and the point index; reported as a share),
the factor base (`nb = ceil(3 sqrt p)`, same shuffled draw), line relations, cycle
construction, peeling, scalar Wiedemann on `AᵀDA`, counters, certification `[t]G = T`.

## 3. Arms (all on the same instance and solver seed, in this order)

| arm | descent | solve policy | kick | role |
|---|---|---|---|---|
| `Q_diem_2lp` | scan | frozen | no | OLD: the pre-check-1 instrument |
| `Q_diem_2lp_alg` | algebraic | frozen | no | isolates the descent fix (same relations as OLD) |
| `Q_diem_2lp_kick` | algebraic | frozen | yes | isolates the T kick under the frozen order |
| `Q_diem_2lp_new` | algebraic | linked, 4 rows | yes | **PRIMARY** (both fixes) |

The full-base arm `Q_diem_full` is not run. Its `p^2` LA was measured in pre-check 1, and
at `p = 32771` it costs about 3 minutes per run. It is not part of the lead.

## 4. Curves, seeds and commands

- Primes 1031, 2053, 4099, 8209, 16411, 32771: the 24 prime-order curves admitted in
  pre-check 1 (curve seeds below), solver seeds 1 and 2. That is 48 paired runs per arm.
- p = 65537 (added under the task rule; a 2LP solve there costs well under 5 minutes): the
  search is run as two processes at 17 s per seed (timing pilot on seeds 1-3, none
  admitted, set-up only). One process takes odd curve seeds 1, 3, ..., 299 and the other
  even seeds 2, 4, ..., 300. Each keeps its first 2 admitted prime-order curves. Solver
  seeds 1 and 2, the same four arms.

Main session (written to `raw/run.jsonl`):

```sh
genus_ladder.qdiem2 run --primes 1031,2053,4099,8209,16411,32771 --genera 2 --arms - \
  --curve-seeds 1031=1,2053=1,4099=1,8209=1,16411=1,32771=1 \
  --quartic --quartic-arms 2lp,2lp-alg,2lp-kick,2lp-new --quartic-inhom 4 \
  --quartic-seeds 1031=54/115/244/290,2053=43/72/73/105,4099=35/205/220/257,8209=10/16/114/165,16411=93/158/252/366,32771=32/163/217/221 \
  --qc 3.0 --max-h 1 --curves 4 --seeds 2 --rho-cap 1000 --threads 2 --out raw/run.jsonl
```

Then, as two processes of one thread each (`raw/run65537_odd.jsonl`,
`raw/run65537_even.jsonl`):

```sh
genus_ladder.qdiem2 run --primes 65537 --genera 2 --arms - --curve-seeds 65537=1 \
  --quartic --quartic-arms 2lp,2lp-alg,2lp-kick,2lp-new --quartic-inhom 4 \
  --quartic-seeds 65537=<odd | even list> --qc 3.0 --max-h 1 --curves 2 --seeds 2 \
  --rho-cap 1000 --threads 1 --out raw/run65537_<odd|even>.jsonl
```

`--genera 2 --arms - --curve-seeds p=1` only makes the binary examine one genus-2 seed per
prime, without running any arm, as in pre-check 1. Rho is skipped (`--rho-cap 1000`).

## 5. Budget (fixed)

At most 2 threads, about 2 hours of compute in total. A genus-4 search owns the other
cores, and `research/genus_ladder_scaling_20261119` is not touched. If the 65537 processes
reach the end of their seed lists with fewer than 2 admitted curves in total, 65537 is
dropped and disclosed. If the session is cut at the budget, complete primes are analysed
and missing cells listed. Nothing is rerun with other parameters under this protocol.

## 6. Metrics (counted units only; wall time recorded, never used)

Unchanged from pre-check 1, per run:

- LA-count (primary for K1): `la_mul`. Sensitivity: `la_mul + la_add`.
- Total (primary for K2): FPM mid = `cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul`.
  Sensitivity: FPM low (`I = 1, kappa = 1`), FPM high (`I = 30, kappa = 9`), and
  `GAE_w = group_ops + w (rel_mul + la_mul)` for `w in {0, 0.1, 1}`.
- Descriptors: `rel_decomp_mul`, `rel_enum_mul`, `decomp_trials`, `attempts`,
  `la_skipped`, `decomp_fallbacks`, `inhom_linked`, `fb_cols`, core size, LA share.

Only runs with `certified = true` and `ok = true` enter a fit. A prime with fewer than 2
certified curves in an arm is dropped from that arm's fit and disclosed.

## 7. Analysis (fixed; `analyze.py`)

- **Exponents** (every arm, primary arm `Q_diem_2lp_new`): curve-level geometric mean over
  seeds, then the OLS slope of `ln metric` on `ln p`. The primary interval is the 95%
  percentile interval of 4000 cluster-bootstrap replicates that resample curves within
  each prime (numpy `default_rng(20261121)`, one generator for the whole script in the
  order of `analyze.py`). The secondary interval is the t-interval of OLS on per-prime
  means. Local slopes are also reported. The fit uses every prime that ran. When 65537
  ran, the six-prime fit (the pre-check-1 range) is a sensitivity reading.
- **Paired comparison (counted ops only).** For every (curve, seed) present in both arms
  and certified in both, take the ratio arm/OLD of FPM mid, `la_mul`, `rel_mul`, GAE w=1,
  GAE w=0, `decomp_trials` and `attempts`. Then take the curve geometric mean over seeds.
  Report the per-prime geometric mean with a cluster-bootstrap 95% interval over curves,
  the pooled geometric mean (bootstrap within prime), and the OLS slope of the
  ln ratio on ln p. The primary contrast is NEW/OLD. The decomposition contrasts ALG/OLD
  (descent), KICK/ALG (kick) and NEW/KICK (linked descent-last) are also reported.
- **Checks.**
  - C0: on all 48 grid pairs, the OLD arm reproduces pre-check 1's
    `Q_diem_2lp` records exactly (`group_ops`, `cantor_*`, `rel_*`, `la_*`, relations,
    `decomp_trials`, `attempts`, core size, `fb_cols`, certified, ok).
  - C1: ALG equals OLD on every field except `rel_mul`, `rel_add` and `rel_decomp_*`.
  - C2: every run is certified, and `decomp_fallbacks` is reported.

  A failed check is disclosed and does not change the frozen rule.

## 8. Frozen kill rule (on the PRIMARY arm `Q_diem_2lp_new`, all primes that ran)

Lead #1 is STOPPED if EITHER

- (K1) the point estimate of the `la_mul` exponent exceeds 1.40, OR
- (K2) the upper end of the primary 95% interval of the FPM mid exponent is >= 1.40.

Otherwise the verdict is PROCEED-TO-LADDER (the full destination measurement with the
L10-corrected matched rho). That is not a success claim. The sensitivity readings
(`la_mul + la_add`, FPM low/high, GAE band, six-prime fit, other arms) are reported. If
any of them would flip the verdict, that is stated as a caveat, but the verdict follows
K1/K2 on the primary metrics.

## 9. What this does not do

- No rho and no IC/rho statement. The L10 corrections are not applied because no rho is
  run.
- No validation gate (replay certificate, VALIDATION.json): pre-check only, no
  leaderboard or `benchmarks/` row.
- The point enumeration stays `O(p log p)` (one-time, uncharged in the descent, charged
  in `rel_enum`). Its share is reported. A sublinear factor-base draw is a later change.
- LA stays scalar Wiedemann on `AᵀDA`, not block Lanczos.
- Development data (`p = 4093, 8191`, curve seeds 45, 89, 20, 106) chose the kick and the
  4-row target. They are not in any fit.
