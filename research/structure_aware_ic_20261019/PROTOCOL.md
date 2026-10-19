# Protocol: two structure-aware index-calculus tests (frozen 2026-10-19, before any timing)

Both tests ask whether a cheap structural restriction moves the cost of a
**complete** index-calculus solve relative to a matched rho.  Each has a ceiling
of about 2×, and any gain is a constant, not an exponent.  Class vocabulary:
advance, engineering, relabelling, accounting, null (AGENTS.md §3).

**Disclosures (before the freeze).**
- Pilot runs on *other* curves (seed 9001 for both tests; 3 targets; not kept as
  evidence) were used only to check that every arm runs and to see the shape of
  the size curve.  They suggested that the A = 6 fold may *not* pay against a
  six-fold rho; the tests below are written to be able to say so.
- Round 6 measured that the coset-restricted MITM beat the default MITM at
  n = 19 and 23 on Koblitz curves; that is the motivation for Test A and is not
  re-used as evidence.
- Host: a cloud VM (`virtual: true`), so isolation is below L2.  Every figure
  here is an operation count `S` (GAE / √r), which needs no isolation level;
  **no wall-time claim is made**.  `isolation_required` is therefore `L0`
  (the frozen round-7 E11b spec said `L2`, which only matters for wall time).

Units: `S` in ecbench's counted unit; the §5a band prices every arm's unpriced
native work, the rho references included, at w ∈ {0, 0.1, 1} GAE per operation
(`ecbench band`).  Two arms are ranked only where the order holds across the
whole band.

## Test A — coset-restricted binary factor bases (E11b)

- **Idea.** On an a = 1, cofactor-2 curve every point of the trace-kernel base
  (`trace_kernel=1`) lies in 2E = the prime-order subgroup, so each relation
  involves half the base points: ceiling ≈ 1/f₂ = 2.
- **Curves (Amendment 2 to round-7 E11b).** Three *new* curves, none used in
  rounds 5–7: `random_binary_instance(n, 7001, max_cofactor = 2)` for
  n ∈ {19, 21, 23}.  (Koblitz a = 1 with prime order exists only for
  n ∈ {5, 7, 11, 17, 19, 23} up to 62; n = 19, 23 are the round-6/7 curves, so
  new a = 1 curves must be non-Koblitz.)  Cofactor 2 forces a = 1.
- **Arms.** `rho.negation` (reference); `rho.plain` (positive control:
  floor ratio plain/negation is √2 by theory); `mitm-default9`
  (`binary-subspace:dimension=9`); `mitm-default9b` (identical parameters:
  determinism/A-A check); `mitm-half9` (`dimension=8,trace_kernel=1`, the half
  base at degree d − 1); `f4-default9`, `f4-half9` (descent-algebraic m = 2 with
  `f4-f2`).  8 targets per curve, 1 round, target seed 20261019.
- **Primary pair.** `mitm-half9` against `mitm-default9`, whole method S.
  The F4 pair is reported with the same rule and is secondary.
- **Verdict per curve.** "Half below default across the band" means
  mean S(half) < mean S(default) at w = 0, 0.1 and 1 (`ecbench band`).
- **Falsified** if that holds on fewer than 2 of the 3 curves.  Effect
  reported: Σ S(half) / Σ S(default) per curve and band weight, with the
  paired `ecbench compare` interval.
- **Controls.** `mitm-default9b` must equal `mitm-default9` exactly (S is a
  count); `rho-plain/rho-neg` must contain √2.

## Test B — automorphism asymmetry on j = 0 prime curves

- **Idea.** Folding by A shrinks the base and the relation count by |A|; rho
  gains √|A|.  On j = 0, |A| = 6.
- **Six-fold rho reference.** `rho.cubic` (walk on {±ζ₃ⁱP}, Wiener–Zuccherato
  look-ahead and cycle escape), the in-repo equivalent of the fixed rho of
  aburan28/cryptanalysis #953 (branch `glv-cycle-escape`, which reports 1.33×
  above the √(πn/(2·#Aut)) floor at 32 bits).  **Equivalence check, stated
  before the run:** pooled `S(rho.cubic)/theory` at w = 0 over the 32-bit test
  curves must be ≤ 1.45 (1.33 plus allowance for 48 walks); if it is not, the
  primary result is still reported but labelled "against an unfixed rho" and
  the ratio to the six-fold floor is reported as the reference.
- **Curves.** p ≡ 1 (mod 3) of exactly `bits` ∈ {20, 24, 28, 32} bits, prime #E (so
  ζ₃ acts on the whole group).  Selected by `examples/structure_aware_curves.rs`:
  j = 0 (`y² = x³ + b`) and random (`y² = x³ + ax + b`, a, b ≠ 0) with the same
  rule on p and #E, so the control differs only in |Aut|.
  Tuning: 2 + 2 curves (seed 5001), 4 targets each.
  Test: 6 j = 0 curves (seed 101); 6 + 6 random curves (seeds 201, 202), 8
  targets each.  Tuning and test curves are disjoint.
- **IC arms.** `ic.pipeline`, `glv-orbit:size=s,group=auto` (A = 6 fold on j = 0)
  and `group=negation` (same points folded by ±1 only), oracle
  `mitm:negation_folded=1`; on random curves `group=negation`.
  `s` is the number of seed abscissae; the base is its closure.
- **Size is tuned, on separate curves, by a frozen rule.** Grid
  s ∈ {8,…,1024}.  For each (bits, arm family) the chosen s minimises the sum
  over the tuning curves of mean S at w = 0.1.  An optimum on the grid edge is
  reported.  Chosen sizes are written to `sizes_<bits>.json` before the test
  specs are generated.
- **Primary statistic** per size and weight w:
  Q = (Σ S_IC,A6 / Σ S_rho.cubic)_{j=0} / (Σ S_IC,A2 / Σ S_rho.neg)_{random},
  sums over curves of per-curve mean S; 95 % bootstrap interval over curves
  (10,000 resamples, families resampled independently, seed 20261019).
  Q < 1 means IC/rho is better on j = 0.
- **Decomposition** (reported, not gating): IC gain Σ IC_A2 / Σ IC_A6 on j = 0;
  rho gain Σ rho.neg / Σ rho.cubic on j = 0; population term
  (IC_A2/rho.neg)_{j=0} / (IC_A2/rho.neg)_{random}.
- **Falsified** (IC/rho on j = 0 is no better than on the random control) unless
  the upper 95 % limit of Q is below 1 at every w ∈ {0, 0.1, 1} at ≥ 3 of the 4
  sizes.  The effect is then Q with its interval.
- **Controls.** A/A at the population level: the same ratio between the two
  disjoint random sets (randA vs randB) must contain 1.  Positive control:
  `rho.plain / rho.negation` on the random curves must contain √2 (the
  instrument sees a known 1.41× effect).
- **Distance to rho.** Every IC row is reported as a ratio to its matched rho.
- **Not claimed.** No exponent, no deployed curve, no wall time.  Complete
  solves: every run's recovered logarithm is verified by ecbench, and sessions
  are re-audited with `ecbench verify --replay 12`.
