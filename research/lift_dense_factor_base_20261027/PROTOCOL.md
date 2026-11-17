# PROTOCOL — lift-dense factor-base subspaces (twist avoidance through the trace)

Frozen 2026-10-27 before the confirmatory `full` run. Harness `src/main.rs`,
`Cargo.toml`, `c2pnb/c2pnb_lines.py`, `c2pnb/c2pnb_delta.py` and the
unchanged upstream cost model `c2pnb/c1_model_upstream.py` (copied from
crypto-autoresearcher `351c9c02`,
`analysis/ecdlp2m-iteration-20261009/c1-gaudry-n4-c2pnb/c1_model.py`,
sha256 `10d2b729…0ce05`). The sha256 of each file is recorded in
`FREEZE.sha256` in the commit that freezes this protocol.

## 0. Disclosure: what was seen before freezing

Exploratory runs (quick mode, and full mode for n ≤ 13 used to time the
exact search) were run under master seed `0x11F7_2026_1027`:
- the m = 2 yield law;
- the composite-degree twisted lines and the u-count dichotomy;
- the c2pnb line check and the cost-model delta;
- a density-normalised deficit of all-lift 4-dim subspaces at n = 9, 11.

Hypotheses H-C and H-D below were formed after seeing that data. The
confirmatory run therefore uses a **new master seed `0x11F7_2026_1027_C`**.
The new seed gives new random curves, nulls, planted sets, targets and line
choices. The c2pnb computations are deterministic up to sampling and are
rerun unchanged. The exploratory outputs are not committed and are not
scored.

## 1. Setting and the claims each part licenses

The setting is E_{a,b} : y² + xy = x³ + a x² + b over F_{2^N}. A nonzero x
lifts to E(F_q) iff Tr(x + a + b/x²) = 0; otherwise its points lie on the
quadratic twist. The lift set is L. For an F_2-subspace V, the factor base
is F = {P : x(P) ∈ V}, with |F| = 2|V ∩ L| (+1 for x = 0). Every ordered
m-tuple of F sums to exactly one point, so:

> mean over R ∈ E(F_q) of #{ordered (P₁..P_m) ∈ F^m : ΣP_i = R} = |F|^m / #E.

So the relation yield of V, against a random subspace of the same dimension
(ρ ≈ 1/2), is (2ρ)^m, where ρ = |V ∩ L| / (|V| − 1). This is a counting
identity (class: theorem). The run checks it by point arithmetic as an
instrument check.

**Twisted-line theorem (composite N = d n).** Write T = Tr_{N/d}, c = √b,
and x = u y with y ∈ F_{2^d}. Then
- Tr_N(c/x) = Tr_d(T(c/u)/y);
- Tr_N(x) = Tr_d(T(u) y).

Hence:
- if T(u) = T(c/u) = 0 and Tr_N(a) = 0, every nonzero x ∈ uF_{2^d} lifts;
- if T(u) = T(c/u) = 0 and Tr_N(a) = 1, none does.

The line is an F_{2^d}-subspace, so Gaudry–Diem Weil descent applies
unchanged in shape. The existence of u is a projective condition on
P(ker T):
- n = 2: ker T = F_{2^d}, so u exists iff c ∈ F_{2^d};
- n = 3: a binary quadratic form over F_{2^d}, so the u-count is in
  {0, 2(2^d − 1)};
- n = 4: a plane cubic, so the u-count is (2^d − 1)·P, with P the number of
  its points.

Class: theorem; the run verifies it exactly on every cell.

**Prime degree (ECC2K-130 relevance).** Prime N has no subfield, so the
twisted-line construction does not exist. Two quantities are measured:
- the largest all-lift subspace dimension kmax;
- the exact number of all-lift k-dim subspaces, normalised by its exact
  expectation for a uniformly random set of the same size,
  E_k = [N,k]₂ · Π_{i<2^k−1} (|L| − i)/(2^N − 1 − i).

Class: comparative/structural claim; needs nulls (§3).

## 2. Populations

| part | cells |
|:--|:--|
| kmax / counts | N ∈ {9, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23}. Curves: koblitz a=0,b=1; koblitz a=1,b=1; random (a = i mod 2, b uniform ≠ 0, ≠ 1): 24 random curves at N ≤ 11, 4 otherwise |
| nulls | one per curve, paired: a uniform random subset of F_q* with exactly that curve's |L| |
| planted | 3 per N: a koblitz-density null with a planted random subspace of dim kp = min(kmax_null_pred + 3, N − 2), density restored outside the plant |
| lines | (N, d) ∈ {(12,6), (16,8), (20,10), (18,9), (15,5), (21,7), (18,6), (16,4), (20,5)}. 8 curves per cell: 4 with Tr(a) = 0, 4 with Tr(a) = 1, b uniform. Targets: 4000 per curve (300 when m = 4) |
| c2pnb | the five ANSI X9.62 c2pnb curves, parameters from crypto-autoresearcher `analysis/binstd-curve-audit/binary-curve-params.txt` |

## 3. Arms and instruments (same instrument on every arm)

- **Exact count (N ≤ 13).** An unpruned exhaustive DFS over reduced-echelon
  bases. It yields counts[k] for every k and the exact kmax. The planted arm
  is its positive control: the plant must be recovered (kmax ≥ kp).
- **Budgeted kmax (all N).** Random prefix while |C| > 2048, then an
  exhaustive tail, restarted until 2^31 candidate-filter operations. It is
  the same on curves, nulls and planted sets. Its planted detection rate is
  reported. A miss on the planted arm means the budgeted instrument cannot
  see needles at that N; budgeted kmax is then reported as "found under
  budget", never as a maximum.
- **Structured control.** The subfield construction at composite N, verified
  element by element.
- **Density above kmax.** A hill climb (1500 moves) from the best all-lift
  basis, at l = kmax+1 … kmax+6 (l ≤ 16), against the mean of 32 random
  subspaces.
- **Lines.** Exact enumeration of both lines (standard F_{2^d}, twisted
  uF_{2^d}). The ordered m = n yield is counted by point arithmetic on common
  targets. The u-count is exact over ker T.
- **c2pnb.**
  - Tr(a), and membership of a and b in F_{2^16}.
  - The exact count of u ∈ F_{2^16} with T(u) = T(c/u) = 0
    (T = Tr_{F_{2^N}/F_{2^{4k}}}).
  - Direct lift tests on 40 sampled line elements, and on 40 standard-base
    elements.
- **Cost-model delta.** The upstream C1 model is rerun with two inputs
  changed, everything else at its defaults:
  - fb_factor = 2;
  - p_dec = 16/24.

  Margins are reported for c_trial = 2^27 and 2^30, for both 2LP and
  basic Gaudry. c_trial is held fixed, an assumption: the descended system
  S₅(u y₁, …, u y₄, x_R) has the same monomial support in y.

## 4. Decision rules

- **H-A (identity check).** On every line cell and curve with a non-empty
  base, the measured mean ordered yield must be within 4 Poisson standard
  errors of |F|^m/#E. Here SE = √(pred/targets), a conservative bound for a
  sum of counts. Any failure flags the instrument.
- **H-B (theorem check).** On every curve where u exists:
  - Tr(a) = 0 gives lifting = 2^d − 1 on the twisted line;
  - Tr(a) = 1 gives lifting = 0.

  The u-count must be 0 or 2^d − 1 at n = 2 (the latter iff c ∈ F_{2^d}),
  and in {0, 2(2^d − 1)} at n = 3; at n = 4 it must be divisible by 2^d − 1.
  A single violation falsifies the harness or the argument; nothing is
  claimed until it is resolved.
- **H-C (prime-degree repulsion, confirmatory).** The statistic is the
  ratio r₄ = counts[4]/E₄. At N = 11, the curve ratios are compared with
  the null ratios (26 vs 26) by a one-sided Mann–Whitney U test
  (H₁: curves < nulls).
  - "Repulsion confirmed at N = 11" iff p < 0.01.
  - At N = 9 (26 vs 26) and N = 13 (6 vs 6), effect sizes and medians are
    reported; at N = 9 the same test is reported but not decisive.
  - N = 12 (composite) is a structural control. There, koblitz-type curves
    with c ∈ F_{2^6} are expected at kmax = 6, against a null median of 5.
- **H-D (no structure found at prime N ≥ 17, descriptive).** Budgeted kmax
  of curves against paired nulls, reported as "curve median − null median".
  Together with the planted detection rate, this only licenses "no all-lift
  subspace beyond the null was found under this budget".
- **Density.** Report the best ρ*(l) of curves against nulls, and the
  implied gain (2ρ*)^m at m = 3. Descriptive.
- **c2pnb.**
  - "Applicable" iff Tr(a) = 0, the u-count > 0, and 40/40 sampled twisted
    elements lift.
  - "Inapplicable by the trace obstruction" iff Tr(a) = 1, and then 0/40
    lift.
  - The cost-model delta is a model output, not a measurement. It is
    labelled as such and adds no leaderboard or benchmark row.

## 5. Not claimed

- No ECDLP was solved and no ECDLP cost was measured, so there is no
  leaderboard, benchmark or rho row.
- The c2pnb margin change is a cost-model figure resting on H0–H3 of the
  upstream model, with c_trial assumed unchanged. Turning it into a
  measured improvement needs the upstream C1 toy ladder run with both
  bases.
- Nothing about N = 131 follows from N ≤ 23 except by the theorems. The
  Weil cap (all-lift dim ≤ N/2 + 1) and the absence of subfields are
  theorems; the repulsion is toy-size evidence.

## Amendment 1 (2026-10-27, after the confirmatory run, before any rerun)

**What failed.** H-A failed as registered: 25 of 87 rows were outside 4
Poisson SE. The pooled measured/predicted yield over the 87 rows is 0.993
and the mean row ratio is 1.009, so there is no bias. The registered rule
assumed Poisson counts. Ordered counts are clustered:
- every decomposition contributes m! orderings and its sign family;
- distinct x-tuples per target are correlated.

So the Poisson SE understates the spread. The as-registered H-A verdict
stays "FAIL (variance model)" in every report.

**Instrument change.** The harness records the per-target sum of squares
of the ordered count (`yield_ordered_m2`). This adds an output and changes
no computed value. A `lines` mode runs only the line cells.

**Amended rule H-A′.** For each row, z = (mean − pred)/(s/√targets), with
s the empirical per-target SD.
- Pass iff |z| ≤ 4 on every row.
- Pass iff, in addition, the pooled per-cell ratio Σ measured / Σ predicted
  lies within its own 4-SE band.

**Rerun.** Line cells only, with the same seed, so the targets are
identical to those of the confirmatory run. The output goes to
`reports/run-20261027T0356Z/lines_amend1/rows.jsonl`. The kmax, count and
c2pnb parts are not rerun, and their verdicts stand as scored.

## Amendment 2 (2026-10-27, after the run): rustfmt only

The CI job `rustfmt (changed files)` requires formatted sources, so
`cargo fmt` was applied to `src/main.rs`. Formatting changes no logic.
Equivalence was checked by running quick mode before and after the change:
the rows (173 lines, `secs` fields removed) are byte-identical, sha256
`591f531e…0ff0` both times. Every result was produced by the pre-format
binary (sha256 of its source in `AMEND1.sha256`).

## Amendment 3 (2026-10-27, after review findings on PR #2686, before any rerun)

Three review findings (Codex, PR #2686) were verified. Each is a defect in
this study.

1. **n = 3 u-count (theorem error).** The set of valid [u] ∈ P(ker T) ≅ P¹
   is the zero set of a binary quadratic form over F_q. It has 0, 1 or 2
   points, or all q + 1 points if the form vanishes. So the u-count is in
   {0, q−1, 2(q−1), q²−1}, not {0, 2(q−1)}.
   - Verified counterexample: N = 15, d = 5, b = 4 gives 31.
   - H-B was scored with the wrong set. It passed only because no sampled
     curve was degenerate.
   - **H-B′:** the corrected sets, n = 2: {0, q−1}; n = 3: as above;
     n = 4: multiple of q−1.
   - Plus an exhaustive census over every b ∈ F_q^* at (12,6), (15,5),
     (18,6), (18,9), (16,4), and 4096 sampled b at (21,7), (20,5)
     (`census` mode). Pass iff zero violations.
2. **Yield expectation vs sampled population.** The sampler draws only
   points with x ≠ 0, so O and T = (0, √b) are never targets, while
   |F|^m/#E averages over all of E. The matching expectation is
   (|F|^m − C_O − C_T)/(#E − 2), where C_O and C_T are the exact numbers of
   ordered m-tuples summing to O and to T.
   - **H-A″:** H-A′ against this expectation. The line cells are rerun with
     the same seed and identical targets, adding C_O and C_T, into
     `reports/run-20261027T0356Z/lines_amend3/`.
   - The run must reproduce the confirmatory means exactly.
3. **No Python execution path (AGENTS.md).**
   - The c2pnb check is replaced by the native `src/bin/c2pnb.rs`. It must
     reproduce every deterministic field of the historical Python output
     exactly: Tr(a), F₁₆ membership, u_kernel_T and u_all_lift. Sampled
     lifts use 200 samples.
   - Scoring is replaced by the native `summarize/` crate. The historical
     `summarize.py`, `c2pnb_lines.py` and `c2pnb_delta.py` and their outputs
     are kept unchanged, as labelled Python-provenance evidence. They are no
     longer executed.
   - The cost-model delta is reported natively only for the 2LP time
     metric, in closed form:
     Δ = ½(log₂ p′/p − 3 log₂ N′/N) = ½(4 − 3) = +0.5 bits.
     The multi-metric table came from the upstream Python model
     (crypto-autoresearcher) and is quoted as historical evidence, not
     regenerated.
   - `plot.py` stays: it is layout only and reads `summary.json`.

The kmax, count, H-C and density results are not rerun; their verdicts
stand.
