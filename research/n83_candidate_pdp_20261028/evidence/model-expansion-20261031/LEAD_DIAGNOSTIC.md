# Seven-base model lead diagnostic

This note audits the exploratory v4 fit in `run-02/` and asks whether its
inputs contain base-specific signal. It is a post-label diagnostic, not a new
confirmatory analysis. The model predicts only the midpoint of finite top-form
degree intervals; it does not predict exact degree of regularity, FFD, or LFD.

## Result

The v4 dataset contains 336 rows: 48 matched configurations for each of seven
coordinate-bit bases, with 238 finite-interval midpoint labels and 98
positive-dimensional top-form results. All 48 configuration outcomes have the
same status and interval for every base. Grouped leave-one-base-out ridge
regression has aggregate MAE 2.5022, versus 0.0 for the matched-configuration
baseline, and wins 0 of 7 held-out bases. It produces no factor-base lead.

An audit of the saved model rows finds that all 48 configurations repeat the
same `factor_base_membership` tuple across all seven bases. The `point_count`,
`mean_logx`, and `mean_logy` features are constant within each configuration;
only 32 of 48 configurations vary in `binary_terms`. This explains why the
fit cannot rank the bases from point-level features.

The paired planted tuples were deliberately drawn from common support in the
low 26 coordinates. That makes them useful controls for holding the target
fixed, but they do not test how a target sampled from each full 2^27-selector
factor base changes the resulting PDP. Natural controls also need to be
generated and bound to each base explicitly.

## Next lead-generating data

For each curve family and each selected coordinate-bit base, generate matched
planted and natural targets using selectors spanning the full 2^27 domain.
Persist the selector, lifted point(s), target construction, and exact factor
base manifest digest with each system. Keep m=2..9 on the same enumerated base.
Then compute the algebraic labels on those genuinely base-specific systems;
use grouped validation by base and keep top-form intervals, exact regularity,
FFD, and LFD as separate outcomes. The current low-26 common-support panel
should remain as a matched-target control.

The full-domain panel is not yet generated. No existing model result supports
an elite-base ranking or a regularity reduction.
