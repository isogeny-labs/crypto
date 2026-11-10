# Coefficient size and curve model in prime-field Semaev decomposition (protocol, frozen before data)

Date: 2026-11-09. Branch `claude/gifted-goodall-r3sb7t`.

## Question (user, 2026-11-09)

Does summation-polynomial (Semaev S3/S4) relation finding over F_p depend on
the curve coefficients? The class of ECCp-131 holds about 2^64 of the 2^131
j-invariants, so curves in the class with coefficients near 34 bits should
exist and an isogeny walk reaches one. Experiment design given by the user:

1. Baseline: S3/S4 decomposition time and success rate on a reduced-size
   analogue of ECCp-131 over a 40 to 60-bit prime with a fixed factor-base
   interval.
2. Treatment: isogenous curves with minimal |a|, |b|; Montgomery/Edwards
   models where they exist.
3. Control: random isogenous curves, to separate model effects from noise.

A flat result closes the door. A change would mean JMV plus subexponential
path finding gives a reduction.

## Preregistered expectation

Null. The success count of a decomposition attempt is a property of the set
F = {P : coordinate(P) in [0, B)} and of the target. For every curve the
counting heuristic predicts the ordered-pair count

    Y(R) = #{P in F : R - P in F},  with E[Y] = |F|^2 / #E

whatever a and b are. Root-finding time per attempt is |F_x| times a fixed
per-element cost. After per-curve precomputation that cost has no
multiplication by a or b (section 4). Prior in-repo evidence:
`research/isogeny_class_ic_yield_20261102`. Its P1 XL profile is identical
across 27 Elkies neighbours and 27 random curves at 131 bits. Its P2 found no
curve share of yield variance in full classes at 24 to 32 bits
[pcc:unverified]. Neither study targeted minimal-coefficient curves or
other curve models.

## 1. Exact facts settled before any run

- E1. ECCp-131 has prime order n (h = 1, `src/ecc/curve.rs`). Isogenous
  curves over F_p have equal order (Tate), so no curve in the class has a
  rational 2-torsion point. A Montgomery model needs 4 | #E; an Edwards or
  twisted Edwards model needs a rational 2-torsion point (4 | #E for
  Edwards). **No Montgomery or Edwards model exists anywhere in the class.**
  The model treatment for the ECCp-131 analogue is void by construction. It
  is studied only in side study M (section 3), on a different class with
  4 | #E, and labelled as such.
- E2. Curves with j = 0 or 1728 (a = 0 or b = 0) have CM by Z[zeta_3] or
  Z[i]. They are not in a class of fundamental discriminant |D| ~ 4p.

## 2. Main cells (ECCp-131 analogues: prime order, conductor of Z[pi] = 1)

Prime p ≡ 3 mod 4. E0: y^2 = x^3 + a0 x + b0 with random a0, b0 of full
size, #E0 = n prime, and t^2 - 4p equal to a fundamental discriminant times
at most 4, so the order Z[pi] is maximal or of conductor 2. Class members are
curves with #E = n (Tate). The tool finds them by a filter that accepts
(a, b) when n·P = O for a random point P. Since n > 4 sqrt(p), that forces
#E = n exactly. PARI `ellcard` re-checks every accepted curve.

Arms (one curve presentation each):

| arm | definition | role |
|---|---|---|
| E0 | baseline curve, base x in [0, B) | baseline |
| E0_AA | E0, independent target seed | A/A noise floor |
| E0_ISO1, E0_ISO2 | (u^4 a0, u^6 b0), random u | isomorphic-model control |
| SMALL1..SMALL4 | class members with smallest max(\|a\|, \|b\|), coefficients as signed integers | treatment (minimal coefficients) |
| SMALL1_ISO, SMALL2_ISO | SMALL1, SMALL2 rescaled by random u (same curve, full-size coefficients) | separates coefficient size from the curve |
| A3_1, A3_2 | class members with a = -3 and smallest \|b\| | treatment (small a only) |
| RAND1..RAND6 (p40); RAND1..3 (p48) | uniformly random class members | control (random isogenous) |
| PLANTED | E0; with probability 0.10 a target is P1 + P2 for random P1, P2 in F | positive control |

Cells:

- C40-S3: p ~ 2^40, B = 2^20, m = 2 (S3), T = 1000 targets per arm, all arms.
- C40-S4: p ~ 2^40, B = 2^13, m = 3 (S4 by S3 chaining:
  S4 = Res_X(S3(x1, xR, X), S3(x2, x3, X))), T = 40 per arm. Arms E0, E0_AA,
  SMALL1, SMALL2, SMALL1_ISO, RAND1, RAND2, PLANTED. This cell is a timing
  cell; its yield test has low power, as stated in advance.
- C48-S3: p ~ 2^48, B = 2^24, m = 2, T = 200 per arm. Arms E0, E0_AA,
  SMALL1, SMALL2, SMALL1_ISO, RAND1..3, PLANTED. Replication at a second size.

Targets are uniformly random points: random x until x^3 + ax + b is a square,
random sign. In index calculus the target aP + bQ is a uniform point, so the
two are equivalent. Seeds derive from 20261109 and the arm name, independently
of the curve.

## 3. Side study M (model effect, NOT an ECCp-131 analogue)

p ~ 2^40, a curve with #E = 4 n' (n' prime) and a rational 2-torsion point
(α, 0) with 3α^2 + a a square, so that Montgomery and Edwards models exist.
B = 2^20, T = 1000. Group law in Weierstrass. Membership is tested on each
model's coordinate:

| arm | factor base |
|---|---|
| W | Weierstrass x in [0, B) |
| W_ISO | Weierstrass x after a random u-rescale, in [0, B) |
| MONT | Montgomery x_M in [0, B) (also timed with the Montgomery S3 root-finder) |
| ED_Y | Edwards y = (x_M - 1)/(x_M + 1) in [0, B) |
| ED_YSYM | Edwards y in [-B/2, B/2) (closed under P -> P + T2: y -> -y) |
| W_AA | W, independent target seed |

## 4. Instruments

- S3 root-finder (Weierstrass): for target R with abscissa xR, for each x1 in
  F_x, form the quadratic S3(x1, X, xR) = c2 X^2 + c1 X + c0 with
  c2 = (x1 - xR)^2, c1 = -2[(x1 + xR)(x1 xR + a) + 2b],
  c0 = (x1 xR - a)^2 - 4b(x1 + xR). Then a Jacobi test of c1^2 - 4 c2 c0,
  sqrt (p ≡ 3 mod 4), batch inversion of 2 c2, and a count of roots in
  [0, B). The only coefficient product per element is b·(x1 + xR). It is computed
  with a general Montgomery multiplication on every arm, so the instrument is
  the same on every arm. Counted per target: Montgomery multiplications,
  Jacobi calls, square roots, batch inversions.
- Montgomery S3 root-finder (side study): c2 = (x1 - xR)^2,
  c1 = -2[(x1 xR + 1)(x1 + xR) + 2A x1 xR], c0 = (x1 xR - 1)^2.
- Group-law cross-check: Y(R) = Σ over x1 in F_x of
  [coord(R - P1) in I] + [coord(R + P1) in I]. On the first 20 targets of
  every arm this must equal the root-finder count exactly. Any mismatch voids
  the cell.
- S4 by chaining: for each x1 in F_x, the two roots x' of S3(x1, xR, X) are
  x(R ∓ P1); then the S3 root-finder count with target x' over F_x.
  Y4 = sum of the counts.

## 5. Metrics and analysis (frozen script `tool/analyze.py`)

- Primary: normalized yield Ynorm = Y · #E / |F|^2 per target, with |F| the
  number of points (2·|F_x|). The heuristic predicts mean 1 for S3. Raw Y is
  also reported.
- Counted cost: Montgomery multiplications per target and per base element,
  exact. Native ns per target is descriptive only: L0 local host, arms
  interleaved in shuffled order over 4 threads.
- Contrasts, each with a 95% bootstrap CI over targets (10000 resamples,
  seed 20261109):
  - P1 (primary): pooled SMALL / pooled RAND ratio of mean Ynorm.
  - P2: each SMALL_k, SMALL_k_ISO, A3_k, E0_ISO_k against pooled RAND
    (Holm-adjusted over the family).
  - P3: SMALL_k vs SMALL_k_ISO (coefficient size on the same curve).
  - A/A: E0_AA / E0.
  - Positive control: PLANTED / E0 must have a CI that excludes 1. If it
    does not, the cell is "not informative".
  - Heterogeneity: across all non-planted arms, a permutation test (10000
    shuffles of targets between arms) of the variance of arm means.
- Decision rule. **Flat**: the P1 CI lies inside [0.90, 1.10] (C40-S3) or
  [0.85, 1.15] (C48-S3), no P2/P3 contrast is significant after Holm at
  0.05, the positive control is detected, and per-element counted cost is
  equal on every arm. **Effect**: the P1 CI excludes 1 and replicates in the
  same direction in C48-S3. Anything else is **inconclusive** at the stated
  power.
- Cost of reaching a small curve (derived, measured at toy size): the
  number of candidates the filter tests per class member found, set against
  p/h(D). This extrapolates the search for a minimal-coefficient curve in
  ECCp-131's class to about 2^67 candidate tests, against rho at about 2^65.
  The extrapolation is derived, not measured at 131 bits.

## 6. Isolation and execution

Local container, 4 vCPU, L0 (shared host). The deciding metrics (yield,
counted multiplications) are exact and host-independent; clocks decide
nothing. The isolab-eks default route was not used: this container has no
`aws` CLI or EKS client, and the decision does not depend on clocks.
Recorded in the session log.

No leaderboard or benchmarks row: this is a characterization study, not an
improvement over the index-calculus baseline (CLAUDE.md success criterion).

## Amendment A1 (2026-11-09, before any analysis was run on study data)

AGENTS.md "Implementation language: no Python" forbids Python for curve
construction, harnesses and result generation. Changes:

- `cmsem setup` (Rust) replaces `tool/setup.py`. It counts points by BSGS
  (`src/nt.rs`; it agrees with PARI `ellcard` on the C40 E0, the 24-bit smoke
  curve and the 24-bit side curve), checks the conductor by squarefree
  trial division, and gives an analytic class-number estimate (descriptive
  only). C48-S3 and C40-SIDE are built with it.
- `cmsem verify` (Rust) independently re-checks every arm of every cell:
  orders, Montgomery data, and equal j-invariants for isomorphic pairs. The
  C40-S3 curves were built by the legacy `setup.py` (PARI) before this
  amendment and are kept, not rebuilt. Native verification of them passes.
- `cmsem analyze` (Rust) replaces `tool/analyze.py` with identical
  statistics. The PRNG is SplitMix64 seeded 20261109 instead of numpy's, so
  bootstrap CIs can differ in the last digits from what the Python script
  would print; point estimates are identical (checked on the smoke data).
  The legacy scripts are kept as provenance, labelled legacy.
- C40-S4 uses the C40-S3 curves (arm subset); its cell file is derived by
  text substitution.
- The C40-S3 run binary is sha256 35b17f20… (FROZEN.sha256). Later cells use
  the rebuilt binary, whose hash is recorded in FROZEN_A1.sha256. The S3/S4
  instrument code (`s3_count`, `gl_count`, `target`, `build_base`, the run
  loop) is unchanged.

## Amendment A2 (2026-11-09, after the T=200 C48-S3 analysis, before any extension data)

Registered verdict at the frozen sample size: C48-S3 at T = 200 is
**inconclusive**. P1 = 0.962 [0.801, 1.156]; the upper bound exceeds the
1.15 margin, and the positive control was not detected (PLANTED/E0 = 1.105
[0.836, 1.447], 13 planted targets). This verdict stands and is reported
first.

The design was underpowered, a fault in the protocol, not in the data: 10%
planting at T = 200 adds about 20 planted targets per arm, which is too few
to move a ratio whose SE is about 0.15. Extension:

- C48-S3 rerun at T = 800 per arm (`runs/C48-S3-T800.cell`). It is identical
  to `runs/C48-S3.cell` except T, with the same binary (FROZEN_A1), arms,
  seeds and order seed. Targets are derived per index, so indices 0–199
  regenerate the T = 200 targets. Their rows must reproduce the original
  y/muls exactly (replay check); any mismatch voids the extension.
- Same analysis, margin [0.85, 1.15], positive-control rule and Holm family.
- Caveat stated in advance: the extension was decided after seeing an
  inconclusive interim, which is optional stopping. The T = 800 verdict is
  reported as "extended (A2)", next to the T = 200 verdict, never in its
  place.
- C40-S4 (T = 40) stays descriptive, as registered in §3. It is not
  extended.
