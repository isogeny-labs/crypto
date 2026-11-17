# Protocol: factor-base line search for the JV cover (frozen before data)

Date: 2026-11-02. Class: a measured change to an existing attack (the Joux–Vitse
genus-3 cover of `E(F_{p^6})` over `F_{p^2}`, Nagao decompositions), weak class
only (full rational 2-torsion, `#E = 4l`). No claim about prime-order curves.

## Hypothesis and mechanism

The decomposition test asks for the six roots of a sextic to lie in an
`F_p`-line of `F_q`. The baseline uses the line `F_p` itself. Under
`x = a·s + b` (`a = t^2`) the cover `H` is isomorphic over `F_q` to
`H' : y'^2 = h(as+b)/a^7`, and the factor base of `H'` is the line `a·F_p + b`
of `H`. The test is the same six quadrics in six unknowns. Only the base
size `B` changes. Model (derived, `PROTOCOL` constants): decomposition rate
`≈ (2B/p)^6/720`, relations needed `B+1`, so residuals `∝ B^{-5}`.

Evidence before this protocol (measured, crossover records of #3194): the nine
baseline instances' `residual_ratio` (0.79–1.44) tracks `(p/2B)^6` (e.g.
p=1601: B=764/820/824 predicts 1.32/0.87/0.84, measured 1.44/0.86/0.79).

**H1.** On each of the nine instances, the line-search arm needs fewer
residuals and fewer total multiplications than the plain arm, by a factor
predicted by `(B_best/B_plain)^5`.

## Arms

- **A (plain):** the nine existing crossover records
  `research/jv_cover_crossover_20261101/results/dlp_p{1301,1451,1601}_s{1,2,3}.json`
  (`JV_FB_LINE` unset), reused unchanged.
- **B (line search):** `JV_FB_LINE=65536`, same binary family, same nine
  `(p, seed)`; `--exp dlp`, all other options as the crossover's `run.sh`.
  The search (the plain line first, then 65535 random `(t, b)`) is charged in
  `setup_muls`: Horner (2 products per coefficient), norm (3), the square
  table once (`(p−1)/2`), the moves of `G`, `Q`.

## Controls (all must pass before the arm-B numbers are read)

- **C1 (baseline unchanged):** with `JV_FB_LINE` unset, the current binary
  reproduces the p=1301 s3 record bit-for-bit (residuals, relation_digest,
  total_muls).
- **C2 (machinery null):** `JV_FB_LINE=1` (only the plain line) gives the same
  residuals and relation digest as C1; total_muls differ only by the search's
  charged `muls`.
- **C3 (correctness):** every arm-B run `solved` and `correct` (the planted
  logarithm), the moved `G`, `Q` valid on `H'` (asserted).
- **C4 (model check):** per arm-B run, measured rate / `((2B/p)^6/720)` in
  `[0.8, 1.25]` for every run (relative sd of a count of ~800 relations is
  ~3.6%; the band allows the model's approximations).

## Metrics and decision

- Per instance: `S_B/S_A` (U1 total multiplications ratio; harness unit),
  and the predicted `(B_plain/B_best)^5`.
- Primary: geometric mean of `S_B/S_A` over the nine instances, 95% bootstrap
  over instances (20000 resamples). **H1 supported** if the upper end is
  below 1 and every instance has `S_B < S_A`.
- Re-score against the frozen negation-map rho of #3278 (estimator (b),
  pooled: 0.9322 steps/√l U1, 0.3879 U2), per prime: geometric mean over the
  three seeds of `S_B`, 95% bootstrap jointly over seeds and the 126 reference
  runs, verdict above/below/not resolved by the interval against 1, the
  distance remaining, and the §5a band at w = 0, 0.1, 1 (cover native count
  `f4_native_ops`, a lower bound; reference native count as #3278).
- Updated model crossover `p*` in U1 and U2 (slope −2.10 of #3194, not refit).

## Budget and cuts

Local 4-core box, as the baseline arm (same host). Cap 4 h of arm-B wall.
If the cap binds, cut the remaining runs at the largest prime last (run
order: p=1301, 1451, 1601, seeds 1–3), and report the cut.

## Analysis code

`fit/` (Rust) reads the records; no numbers from Python. Labels in the
report: measured / derived / model.
