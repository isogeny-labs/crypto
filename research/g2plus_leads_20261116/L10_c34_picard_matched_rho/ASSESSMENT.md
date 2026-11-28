# L10 assessment: C_{3,4} / Picard targets with a matched rho [pcc:unverified]

Assessor: independent of the producer (2026-11-16). Every number below is `[pcc:unverified]`:
one session, a shared container, counts only, no validation-gate certificate. This is not a
benchmark. It adds no leaderboard or `benchmarks/` row and makes no speed claim.

## Verdict

- **H3 (primary, orbit IC): inconclusive under the frozen rule.** The numeric condition is met
  by a wide margin: R_T = 8.984, 9.002, 9.001 at p = 4099, 8209, 16411 (threshold 2.0). Gate T7
  failed, though: T7b had 7 of 10 tiny curves determined against a required 8 of 10, and T7c
  failed on 2 curves. PROTOCOL §6 says "all must pass before any metric is read", and §7 makes
  the verdict inconclusive "if any of T1-T8 ... fails". I apply the rule as written.
- **H1 (accounting, existing quartic rows): supported.** F_Q = 1.7338, 1.7269, 1.7322; every
  value is at least 1.10. R2 reconciles within 0.18%, 0.40% and -0.16%.
- **H2: argmin is S3**, the `<rho>` class walk, at all three primes, with min/S2 = 0.4466.
- **H3': both predictions held** at p = 16411: the trials ratio is 2.984, inside [2.5, 3.5],
  and the LA_w ratio is 9.002, inside [7, 11].

## 1. Integrity checks

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
- **Headline numbers re-derived from the raw files** with my own script, not `analyze.py`:
  - From `raw/rhocost.jsonl` (pooled sums over sums), A, N, R, F_Q and S1/S2/S3/S6 match
    RESULTS.md to the printed digits.
  - The R2 medians from `bound_r.jsonl` are 8899.6, 8908.8 and 8921.1, which match.
  - R_T comes out as the same geometric mean, min and max at all 7 primes.
  - For every P row I checked these identities:
    - `fpm_total = enum + line + la_w`;
    - `fpm_la_w = 9 (la_mul + 10 la_inv)`;
    - `krylov_mul = 2 n (n + w)`.
- **Per-curve spread of F_Q** is 1.714 to 1.734 over the 9 Q curves, with the minimum at
  p = 4099, seed 1. The spread of R_T over the 6 pairs at 16411 is [8.990, 9.018], with
  bootstrap 95% interval [8.994, 9.009].
- **Lab source drift.** The run hashed `plane_quartic_pic.rs` as 3bf45c3a. The current file
  (git HEAD, merge of PR #3093) is e6cf9d53, and the old version cannot be recovered from this
  shallow clone. I rebuilt the crate in the scratchpad against the current lab file and re-ran:
  - rhocost for Q, C and P at p = 2053, all 9 curves;
  - tiny at p = 13;
  - measure at p = 523, 24 rows.

  All three outputs are byte-identical to `raw/`, so the drift does not touch these counts.
- **Replay.** Confirmed from `replay/REPLAY.md`: identical except for the p = 16411 Q_full row,
  which is missing from raw because the run hit the CPU limit. H3 does not use Q.
- **Independent mathematical spot check.** I wrote a stdlib Python script with its own point
  counting over F_p, F_{p^2} and F_{p^3}, its own L-polynomial, an enumeration of all lines of
  P^2(F_p) and the orbit fold. It does not use the producer's code. Tested on the tiny curves
  (p, seed) = (13, 1), (19, 1), (13, 3) and (31, 1):
  - It reproduces N1, #J and r exactly: 3648/19, 5296/331, 3753/139 and 35077/5011.
  - On the two determined curves it finds the same lambda as the producer (42 and 2907). With
    every line relation, the orbit matrix has kernel dimension 1 for the correct lambda and 0
    for the wrong one.
- **Why T7c fails: an infeasible gate, not a bug I could find.** On (13, 1) and (19, 1), the
  set of all line relations over F_p, folded into orbits, has rank n_orb - 1 for both roots,
  both mod r and mod 2^61-1. That means rank n_orb - 1 over Q(zeta). Only 6 and 4 distinct
  orbit rows exist, for 6 and 5 orbit columns. On these curves the wrong-eigenvalue matrix
  therefore has a kernel whatever the instrument does. The T7b shortfall comes from the same
  row starvation, mostly in the full arm (kernel dimension 2 at (13, 1) and (19, 1)). T7a
  holds on all 10 curves: every orbit row annihilates the brute-force logs. So the fold is
  correct wherever it can be tested. The frozen gate did not anticipate row-starved tiny curves.
  This finding cannot be used to waive it after the data.

## 2. Instrument and fairness audit

- **Counters.** Pic costs are deltas of the lab `counter()`, and the instrument adds its own
  rho scaling: 9 own multiplications plus a lab RREF of 3 multiplications and 1 inversion,
  giving R = 22 FPM. The re-canonicalisation is charged.
- **Q baseline negation.** A plus N reconciles with the lab's executed `Q_rho_neg` rows to
  within 0.4%, so H1 reprices what the lab actually ran.
- **H3 pairing is fair.** Both arms use the same curve, the same pair stream, the same stopping
  rule (`n_cols + 10`) and the same counted Wiedemann. P_orbit pays more for enumeration (ratio
  0.883), which works against the treatment.
- **The H3 effect is close to an arithmetic identity.**
  - LA is 98.4% to 99.98% of `fpm_total` in P_orbit, and more in P_full.
  - The Krylov count `2n(n + w)` does not depend on the data. With n and w both down about 3x,
    R_T is about 9 by construction.
  - At the measurement tier, LA runs modulo the proxy prime ell, so no logarithm is solved. The
    measured ratio would therefore be the same even if the orbit system were mathematically
    wrong.
  - Correctness rests entirely on the tiny tier (T3, T4, T7a). That is why the T7 failure is
    not cosmetic under the rule.
  - The mechanism is prior art (Duursma, Gaudry and Morain 1999), as the protocol says.
- **The A/A control is weak.** n_cols does not change between solver seeds. Only w and the
  Berlekamp-Massey counts vary, so a ratio of about 1.000 is expected almost regardless of
  noise.
- **PC tests something else.** It checks that the trials counter tracks the column count. It
  does not test the orbit fold.
- **H1 and H2 are model values.** They are ideal step counts times measured step costs. No
  plain-walk or `<rho>`-walk rho was executed, and fruitless cycles and walk overhead are not
  modelled. H1 says nothing about a cheaper negation formula. One may exist and would shrink
  F_Q.

## 3. Distance to the matched rho

- **Picard targets.** The correctly matched rho is the `<rho>` class walk (S3). It is
  0.585 x S1 and 0.447 x S2.
  - Gap movement G = R_T / 1.709 = 5.27 at every prime: the orbit fold closes about 5.3x of
    the full-base IC/rho distance.
  - The proxy distance T(P_orbit) / (sqrt(pi p^3/4) S3) is 0.453, 0.640 and 0.894 at
    p = 4099, 8209 and 16411. It grows by about 1.41 per doubling of p, which is the p^2
    against p^1.5 exponent gap of a full-base IC.
  - Extrapolated, it reaches about 1.26 at p of about 2^15. The constant improves, but the
    exponent does not.
  - The proxy uses r = p^3, excludes descent and solves no DLP. This arm is not the lab's IC
    baseline: that baseline is Diem's double large prime (2LP), and no 2LP orbit arm was run.
    So no measured improvement over the current IC baseline is shown.
- **Existing generic-quartic rows.** Under H1 the lab's matched rho is overpriced by about
  1.73x in ideal expectation. Their IC/rho ratios move against IC:
  - bound_r 2LP/rho goes from about 0.0089, 0.0086 and 0.013 to about 0.0154, 0.0149 and
    0.0225;
  - genus_ladder at p = 1031 goes from 0.027 to about 0.047.

  IC stays below rho in those rows, but the distance shrinks by 1.73x. This is a correction to
  existing rows, not an improvement. Per PROTOCOL §7 it should be reviewed by the row owners;
  this lead edits no row.

## 4. Concerns (summary)

1. H3 is blocked by T7b and T7c. The failure traces to row starvation at p = 13 and 19, which
   I reproduced independently. The frozen rule still says inconclusive.
2. R_T of about 9 is forced by data-independent Wiedemann counts at a proxy modulus. The
   measurement tier cannot detect a wrong fold.
3. The study measures full-base IC, which has the wrong exponent against rho. The orbit fold
   was not tested on top of the lab's 2LP baseline.
4. H1 and H2 are model values, not executed walks.
5. The A/A control is nearly degenerate by design.
6. Lab source drift (3bf45c3a to e6cf9d53). I checked that it does not change the outputs.

## 5. Single most valuable next step

Freeze a successor protocol before collecting any data. It should do two things:

1. Replace T7 with a gate that is feasible on row-starved curves. For example, check T7b and
   T7c only on curves where the all-lines orbit matrix has rank n_orb - 1 over Q(zeta), or use
   tiny primes of at least 31.
2. Run the orbit fold inside the lab's Diem 2LP relation search on Picard curves.

Charge both against an executed `<rho>` class-walk rho (S3) on the same Jacobians. That
replaces the model ratio with a measured matched rho and tests whether G of about 5 survives at
the real IC baseline.

Paths:

- my check script: `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l10check.py`
- my rebuild: `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/rebuild/`
