# PROTOCOL: Cheon's attack on curves shaped like P-224

Written and committed before any measured run (2026-10-22). Everything below is
fixed in advance; deviations are listed in `REPORT.md` under "Deviations", never
edited in here.

## 1. Question

For NIST P-224 (group order `n`, 224 bits):

* `n - 1 = 2^2 * 3^6 * 5 * 17 * 2153 * q195`, smooth part `d- = 533,642,580 = 2^28.991`;
* `n + 1 = 2 * 7 * 19 * 641707280681 * q177`, smooth part `d+ = 170,694,136,661,146 = 2^47.278`.

A static-DH (Brown-Gallant) or q-SDH-style leak of `[x^i]G` lets Cheon's
algorithm recover `x` in

* `n - 1` variant: `sqrt(n/d) + sqrt(d)` exponentiations, `= 2^97.50` for `d-`;
* `n + 1` variant: `sqrt(n/d) + d` exponentiations with about `2d` leaked powers,
  `= 2^88.36` for `d+`;
* matched Pollard rho (negation map, `0.886 sqrt(n)`): `2^111.83`.

These are formulas. This study turns them into **measured scaling laws** on
scaled-down prime-order curves whose `n -/+ 1` has P-224's shape, fits the
exponents, and extrapolates (labelled extrapolation) to 224 bits.

Shape ratios (frozen, computed from P-224's published `n`):

    rho_minus = log2(d-) / log2(n) = 28.9913 / 224.0000 = 0.12943
    rho_plus  = log2(d+) / log2(n) = 47.2784 / 224.0000 = 0.21106

Scope. This is the DLP with auxiliary inputs (DLPwAI), not the plain ECDLP. The
Cheon arms are handed `[alpha^i]G` that a rho arm cannot use; every Cheon figure
carries its leaked-power count as its own column. No row here is an ECDLP
advance and none will be described as one.

## 2. Frozen inputs

### 2.1 Sizes

`b in {32, 36, 40, 44, 48, 52, 56}` (target: `n` has exactly `b` bits). Minimum
for a fit: four sizes. Order of work: ascending.

### 2.2 Curve construction (random coefficients + point counting)

Native Rust (`examples/cheon_p224_shape.rs`, `gen`), no CM. For each size `b`
and each family `F in {minus, plus}`:

1. `p` = the least prime `>= floor(1.5 * 2^(b-1))` (same `p` for both families).
2. Coefficients `(a, c)` for `y^2 = x^3 + a x + c` over `F_p` come from a
   SplitMix64 stream seeded with `(0x43686561 << 32) ^ (b << 8) ^ F` (`F` = 0
   for minus, 1 for plus), drawn in order, `a, c in [1, p-1]`. Skip singular
   curves.
3. `#E` by Mestre-style baby-step giant-step over the Hasse interval from a
   fixed point (no randomness beyond the stream). Accept only if `#E = n` is
   prime and `n != p`. This is rigorous: a prime `n` equal to the unique
   multiple of `ord(P)` in the Hasse interval; the order is re-checked after
   freezing by `[n]G = O` with `G != O` (see 2.4).
4. Shape test. `family minus`: there is an integer `d` with `d | n-1`,
   `(n-1)/d` prime, and `|log2(d)/log2(n) - rho_minus| <= 0.015`. `family
   plus`: the same with `n+1` and `rho_plus`. If several `d` qualify, take the
   one closest to the ratio. The first accepted curve in stream order is frozen.
   (So `d` is the whole smooth part: the cofactor `(n -/+ 1)/d` is a single
   large prime, as for P-224's `q195`, `q177`. `d` itself is not required to be
   smooth. Only its size and the primality of the cofactor are matched.)
5. Generator `G`: the point with the least `x >= 1` on the curve, smaller `y`.
   The subgroup is the whole group (cofactor 1).

The frozen curves are written to `curves.json` and committed before the first
measured run; the measured binaries read only that file.

### 2.3 Secrets and targets

ecbench's planted-target law: `target_seed = 20261022`, six targets per curve
(index 0 to 5). `Workload::build` of the `prime_explicit` curve gives `Q = [k]G`
and the planted `k`. Cheon arms and the matched rho arm use the same targets.

### 2.4 Independent curve check (before any run)

`curves.json` is re-verified by `check`: `p` and `n` prime; the curve is
non-singular; `G` on curve; `[n]G = O`; `n` in the Hasse interval; shape
condition recomputed from `n` alone with a trial-division factoring of `d`.

## 3. Arms

| arm | what | inputs the arm uses | leaked powers |
|:--|:--|:--|--:|
| `rho.negation` | ecbench matched reference, run by `ecbench run` | `G`, `[alpha]G` | 0 beyond the target |
| `cheon.pminus1_comb` | Cheon theorem 1, BSGS in the exponent, fixed-base comb tables (tables charged) | `G`, `[alpha]G`, `[alpha^d]G` | 2 (`alpha`, `alpha^d`) |
| `cheon.pminus1_naive` | same, every step a double-and-add. This is the algorithm of `src/cryptanalysis/cheon_attack.rs`, re-counted in group operations | same | 2 |
| `cheon.pplus1_comb` | Cheon theorem 2 via the norm-one torus of `F_{n^2}`, comb tables | `[alpha^i]G`, `1 <= i <= 2d` | `2d` |

Family `minus` runs rho and the two `pminus1` arms; family `plus` runs rho and
`pplus1_comb`. (`pplus1` on the minus curves and vice versa are not run: `d` is
matched to one side only.) The kangaroo variant is not run (memory trade, not
a different exponent; out of scope and said so).

`cheon.*` are new, native, in `src/cryptanalysis/cheon_counted.rs`, over the
repository's `CountedGroup` (`PrimeCurve`, `GroupOps`). They take the target
`[alpha]G` and the leaked points; they never take `alpha`. The driver supplies
the leaked points computed from the planted `alpha` as the problem statement
gives them (not charged), and verifies the recovered value against `alpha` in
its own step.

Existing code: `src/cryptanalysis/cheon_attack.rs` implements the `n - 1`
variant only, over `BigUint` points, counting exponentiations. It has no `n + 1`
variant (the `n + 1` implementation is the legacy Python
`research/torsion_auxiliary_inputs_20260918/cheon.py`, which this study does
not run, per AGENTS.md "no Python"; its algorithm is ported). As a link to the
existing code the study runs `cheon_attack` on the three smallest `minus`
curves and requires the same `alpha` and the same exponentiation count as
`cheon.pminus1_naive`.

ecbench. Its registered methods take a target and no auxiliary points, and its
workload identity has no field for leaked powers; adding one changes the
hashed workload record that CI replays for every committed session. Not done
here (listed under Unfinished in the report). The rho arm runs through the real
`ecbench run` (sealed session, `verify --replay`); the Cheon arms run in the
study's driver, on workloads rebuilt from the same spec, and the driver checks
that its workload ids equal the session's.

## 4. Unit and accounting

* Unit: one affine addition or doubling = 1 group operation (the repository's
  GAE). `S = ops / sqrt(n)`. Ratio columns: `S/S_rho` (same curve, same target)
  and `ops / F(n,d)` with `F = sqrt(n/d) + sqrt(d)` (minus) or `sqrt(n/d) + d`
  (plus).
* Charged for Cheon: every point addition and doubling in table construction,
  baby steps, giant steps, the shift `[zeta^-k0] [alpha]G`, the polynomial
  multi-scalar sums of the plus variant (naive double-and-add per coefficient),
  and the final verification `[alpha]G = Q`. Additions with the identity are
  not counted (rho's walk never has them).
* Not charged but counted, per run, as `*_uncharged` (AGENTS.md section 5a):
  hash-table inserts and lookups, scalar-field multiplications mod `n`
  (exponent sequences, binomial coefficients), `F_{n^2}` multiplications.
  rho's unpriced counters are read from the ecbench record, every one listed.
  Every cross-arm comparison is stated at 0, 0.1 and 1 GAE per unpriced
  operation, applied alike to every arm, and an order between two arms is
  claimed only if it holds at all three weights; otherwise "unresolved".
* Leaked-power count is a column of every Cheon row; the leak itself is free.
* Phase columns: tables, baby, giant, shift, polynomial, verify.

## 5. Pre-registered predictions (formula-based, written before data)

Notation: `s_X` = fitted slope of `log2(ops)` against `log2(n)`.

* H1 (minus). `d = n^0.129`, so `sqrt(n/d) = n^0.435`. With comb tables and the
  log factor of an exponentiation, `s_cheon-minus in [0.40, 0.50]`.
* H2 (plus). `sqrt(n/d) = n^0.395`; the polynomial phase is `~ 9 d log n`
  (`n^0.211` times a log). At the sizes run the polynomial phase is expected to
  be a large share, so `s_cheon-plus in [0.30, 0.46]`.
* H3. Matched rho: `s_rho in [0.45, 0.55]`, `S_rho` between 0.6 and 2.0.
* H4. `ops/F(n,d)` is flat in `n` for the search phase (slope of
  `log2(ops/F)` on `log2 n` within +-0.08).
* H5 (distance to rho, a report not a gate). At every size run, both Cheon
  variants cost more than rho (`S/S_rho > 1`) because the constants (about 20x
  and 125x) exceed what `n^0.065` and `n^0.105` have recovered by 56 bits; the
  fitted lines are predicted to cross at a size between 55 and 130 bits.
* Extrapolation to 224 bits, predicted before data: minus `2^101` to `2^103`
  (formula `2^97.5` times a constant `c in [10, 40]`); plus `2^94` to `2^97`
  (formula `2^88.4` times `c in [60, 200]`).

## 6. Success and stop conditions

* **A row is a result only if** the recovered log equals the planted `alpha`
  (checked by the driver) and `[alpha]G = Q` holds. Failures, timeouts and
  wrong answers are kept in the tables as rows with their reason. They are
  excluded from fits and counted.
* **The scaling study is a result if** at least four sizes have at least four
  verified targets per arm.
* **Pre-registered readings (no thresholds on the sign of an improvement; the
  Success criterion in AGENTS.md reports distance, not pass or fail):**
  - each measured exponent is reported with its interval against its
    prediction in section 5 (inside or outside);
  - rho's `S` and exponent against H3 is the sanity gate for the unit: if
    `s_rho` falls outside `[0.40, 0.60]` or any rho `S` is outside `[0.3, 4]`,
    the rho arm is declared unreliable and nothing is ranked against it;
  - the control: `cheon_attack.rs` cross-check (section 3) must agree, or the
    `pminus1_naive` arm is declared mis-counted and the minus rows are not
    reported;
  - negative control: `cheon.pminus1_comb` and `cheon.pplus1_comb` fed a wrong
    `[alpha^d]G` (resp. a wrong power list) must not return the planted `alpha`
    (unit tests).
* **Stop.** Per run: 1800 s wall; a run that exceeds it is recorded as a
  timeout and the size's remaining runs of that arm are skipped. Per size: peak
  memory above 6 GB or a failed `check` stops growth at the previous size. Total
  measured wall: about 3 hours; sizes not reached are listed as not run, and the
  fit uses the sizes that did run.
* Not allowed: changing sizes, `d`, curves, seeds, targets, unit or accounting
  after seeing a result; dropping a failed target; choosing among targets.

## 7. Analysis plan (fixed)

1. Per arm, per size: median, geometric mean, min and max of ops over verified
   targets; `S`; `S/S_rho` by target (paired on the same target); leaked powers.
2. Fit (A): ordinary least squares of `mean_t log2(ops)` on `log2 n` over the
   sizes. Report slope and intercept, a t-interval (dof = sizes - 2) and a
   percentile bootstrap interval (2000 resamples of targets inside every size).
3. Fit (B): the same with `log2(ops / F(n,d))`, which isolates the constant and
   its trend; extrapolate as `log2 F(224, d_P224) + fit_B(224)`.
4. Fit (C), `plus` only: separate power-law fits of the polynomial phase on
   `log2 d` and the search phases on `log2 sqrt(n/d)`; extrapolate phase by
   phase to the P-224 shape, then add.
5. Extrapolation to `log2 n = 224` with the 95% prediction interval of the OLS
   line, and the same for rho. Labelled **extrapolation** wherever it appears;
   the 224-bit figure carries the distance from the largest measured size.
6. Compare with the formulas `97.50`, `88.36`, `111.83`; report the gap in bits.

## 8. Deliverables

`curves.json`, per-run JSONL, ecbench session (rho), `REPORT.md` (with
observation, calculation and interpretation in separate sections), SVG graphs +
PNG renders + PDF, `benchmarks/20261022-cheon-p224-shape/` copy with a row in
`benchmarks/README.md`. Host and isolation level recorded; operation counts
stand at any level, wall time is descriptive only.
