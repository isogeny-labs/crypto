# Protocol: how the Joux-Vitse cover cost S/rho scales with p (frozen 2026-10-24, before the main run)

Scope. This curve family only: the `jv_cover` genus-3 cover of E(F_{p^6}) over F_{p^2} built by `generate_spec(p, seed)` (full rational 2-torsion, weak class), Nagao decompositions, `f4_fp` + sparse-FGLM linear stage (default `JV_LIN=sparse`). It is a statement about how this class scales, not about prime-order curves. Scaling measurement only; no speedup or "beats rho" claim is made or implied.

Status labels used everywhere: **measured** (read from a run record), **derived** (computed from measured values, formula shown), **model** (extrapolation or cost-model prediction).

## Hypothesis (registered before any run of this study)

H1 (log-linear): log(S/rho) is, to a first approximation, linear and decreasing in log p over p = 53 ... 1009 for this family.

Cost model (derived, not fitted). Relation collection is ~99.99% of S. One relation needs ~720 Nagao tests (decomposition rate 1/720), and the relation count is about the factor-base size |F| ~ p/2 (the 2026-10-24 probe at p = 251 had |F| = 129, 130 relations, 82263 residuals). With C_cov the mean cost of one test in F_p multiplications and c_e the cost of one E(F_{p^3}) addition in F_p multiplications (both counted by the binary):

    S      ~ 720 (p/2) C_cov / (c_e sqrt(l))                 (units of sqrt(l) additions)
    rho    ~ S_rho  (the binary's rho S, steps/sqrt(l), about 1.2 and independent of p)
    l      ~ p^6 / 4  (probe: l = 2^45.8 at p = 251, p^6 = 2^47.8), so sqrt(l) ~ p^3 / 2
    S/rho  ~ 720 C_cov / (c_e S_rho p^2)

Check at the probe point (p = 251, seed 1, measured C_cov = 9.705e5, c_e = 331, rho S = 1.153): the formula gives 720*9.705e5/(331*1.153*251^2) = 29.1; the binary's own `predicted_s_over_rho` is 29.07 and the measured S/rho is 26.5.

Predicted slope of log(S/rho) against log p: **-2** plus d ln(C_cov/c_e)/d ln p, which the model takes as ~0 (the F4 system size does not depend on p; only the factor base does).

Decision rules (frozen):
1. H1 is supported if the 95% interval of the fitted slope lies below 0 and the two half-slopes (primes <= 151 versus primes >= 251, each fitted on its points) differ by less than 0.7. H1 is falsified if the interval contains 0 or the half-slopes differ by 0.7 or more (curvature).
2. The cost model's slope -2 is "consistent" if the point estimate lies in [-2.4, -1.6]. It is **falsified** if the 95% interval lies entirely outside [-2.5, -1.5] (for example the earlier "about p^-1.2" reading, or a slope steeper than -2.5).
3. p*, the p at which the fitted line meets S/rho = 1, is computed as exp(-a/b) from the fit y = a + b x. It is a **model** extrapolation. If measured points straddle S/rho = 1 it is an interpolation and said so. It is the distance remaining on this class, not a pass/fail gate.

## Instrument and controls (all provided by the binary)

Binary: `cargo build --release --example jv_cover` at the working tree of 2026-10-24 (sha256 in `results/probe/bin.sha`, also copied to `results/versions.txt`). Host: 4 vCPU shared VM; counted multiplications do not depend on load; wall times are practicality notes only.

Per `dlp` run, recorded in the JSON (`CoverDlpReport`):
- `solved` and `correct` must both be true: the final logarithm is checked against the planted secret `spec.d` and `ec.mul(g, d) == q`.
- Every relation is verified by `verify_dec` against its residual before it enters the linear system (a failing decomposition is dropped); `mismatches` (oracle cross-check against the meet-in-the-middle oracle every 64th residual, available only when |F| <= 40, so p = 53 only) must be 0; `incomplete` and `timed_out` are recorded and reported, not hidden.
- `relations`, `residual_ratio` (residuals / (720 |F|)) and `decomposition_rate` against 1/720 are reported to show the relation yield is on model.

Per prime, one `ccov` run (60 tests = 40 random + 20 constructed): planted decompositions found must be 20/20, oracle mismatches 0, unverified 0, incomplete 0. This is the detector-can-see-the-effect control (planted) and the oracle control.

A `dlp` row with `correct = false` or `solved = false`, or a `ccov` row with planted < 20 or mismatches > 0, is a **failure** that is reported and excluded from the fit with the reason; nothing is rerun to replace it silently.

## Rho reference

rho is the binary's own `rho_e` (r-adding walk with exhaustive storage on <G,Q> in E(F_{p^3}), every group operation counted, S = ops / sqrt(l)), run `--rho-runs 16` per (p, seed) at p <= 251 (l <= 2^46; the table does not fit memory above about 2^48, per the binary's header). At p = 503 and 1009 (l ~ 2^52, 2^58) rho cannot be run, so those rows use `--rho-ref`.

Primary reference: the **pooled mean of all rho S samples at p <= 251** (every run of every seed), used for every point, so the ratio is not moved by rho's own sampling noise (single-run sd about 0.5, per-point mean of 16 has about 11% error). The fit tool recomputes S/rho = `total_muls / (rho_ref * sqrt(l) * c_add_e)` from the recorded `total_muls`, `l`, `c_add_e`; for points with their own rho runs it equals the binary's `s_over_rho` when the same reference is used. The main command passes the default `--rho-ref 1.3` for 503/1009 only so the binary finishes; that value is not used in the analysis. Secondary analysis: each point's own rho mean where available. The tool also reports the regression of rho S on log p, to check the pooling assumption (rho S independent of p); if its 95% interval excludes 0 that is reported as a model-assumption violation affecting the 503 / 1009 rows.

## Section 5a band (AGENTS.md)

The counted unit is F_p multiplications priced into group additions (`c_add_e`). Unpriced native work exists on both arms: for rho a hash computation, a table lookup and a table insert per step (3 native operations per step, from `rho_e`; `steps` is recorded, so this count is derived); for the cover IC the F4 additions and reductions, row memory traffic, the Weil-form and Mumford bookkeeping, and the Wiedemann vector operations beyond the 16-multiplication-per-op charge. **The binary does not emit the cover arm's unpriced counts.** Under section 5a an arm that does not emit its counts cannot be ranked, and this is recorded as an implementation gap, not worked around. Consequences stated here, in advance:
- Every S/rho in this study is a lower-bound comparison at weight 0 for both arms. No ranking of cover versus rho is made.
- As a sensitivity only, the tool reprices the rho side at 0, 0.1 and 1 GAE per native operation (rho cost x (1 + 3w)) and prints S/rho at each w. Because the cover arm's unpriced work stays at 0, these are NOT a valid band comparison and are labelled so; they show only how far the rho denominator can move. The slope of log(S/rho) vs log p is unaffected by a p-independent w, since (1 + 3w) is a constant factor (shift of the intercept only).

## Plan, commands, order (sequential, one job at a time)

Define B = `target/release/examples/jv_cover`, R = `research/jv_cover_p_scaling_20261024/results`.

1. Controls, per prime p in {53, 101, 151, 251, 503, 1009}:
   `B --exp ccov --sizes p --seeds 1 --residuals 40 --constructed 20 --max-degree 24 --budget 300 --json R/ccov_p<p>.json`
2. Main, per prime and per seed s in {1, 2, 3}, one invocation each:
   `B --exp dlp --sizes p --first-seed s --seeds 1 --rho-runs 16 --check-every 64 --json R/dlp_p<p>_s<s>.json` for p in {53, 101, 151, 251};
   `B --exp dlp --sizes p --first-seed s --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json R/dlp_p<p>_s<s>.json` for p in {503, 1009}.
   The probe run (p = 251, seed 1, same command as above) was made before this freeze to time one run and is reused as that data point (`results/probe/`, same binary).
3. Cost gate for p = 1009: seed 1 is run under `timeout 1500` (25 min). If it does not finish, 1009 is dropped and the failure recorded. If it finishes, seeds 2 and 3 run only while the cumulative compute (all runs including the probe) stays under 2.5 h; otherwise 1009 has fewer seeds and the table says how many.
4. Analysis: `fit/` (standalone Rust, no Python) reads the JSON and writes `results/fit.json` and `results/fit.txt`: pooled rho reference, per-point S/rho, OLS of ln(S/rho) on ln p over all per-seed points (slope, intercept, t-interval with n-2 df), a cluster bootstrap over seeds within each prime (10000 resamples, fixed seed) for the slope and p*, half-slopes, p* with interval, the rho-S trend, the repricing sensitivity, and the same fit with each point's own rho. The same tool writes the SVG (log-log S/rho vs p, per-seed points, fit line, S/rho = 1 line, model slope -2 reference). PNG via ImageMagick `convert` from that SVG. Python is not used for any number.
5. Report: `README.md`, `REPORT.pdf` (pandoc to ODT, then LibreOffice headless to PDF, because no TeX or typst engine is installed on this host).

Freeze record: sha256 of this file at freeze time is in `results/FROZEN.sha`.
