# Fused residual step: preregistered protocol (written before any data)

Date: 2026-10-31. Branch `claude/ic-gap-20261031`. Goal: lower the counted cost of the
current best index-calculus pipeline on prime curves (the residual collector, `targets=residual`,
res2/res4 in `research/ic_planner_exhaustive_20261026`) and report the distance to matched rho.

## Claim classification (controlled-experiment skill)
- C1 (engineering, confirmatory): the fused step (`targets=residual-fused`: one group addition per
  walk step from a precomputed table J_j - P_i, 16*|F| setup additions counted) lowers counted S
  against the baseline collector (`targets=residual`) of the same size B, same instances.
- C2 (descriptive): distance of each arm to matched rho at weights w = 0, 0.1, 1 (section 5a band).
- C3 (negative control, exploratory): the fused collector's relation yield on a curve-free null equals
  the baseline's, i.e. the gain is generic collision search; it is NOT a structural IC result and is
  recorded as such (same classification as `ic_prime_residual_lp_20261026`).
Relabelling/accounting is excluded: both arms are charged in ecbench's counted unit by the same
harness; every answer is verified by the harness against the known discrete log.

## Design
- Curves: p16, p18, p36, p37 of `ic_planner_exhaustive_20261026/specs/R-*.json` (same group data).
- Arms per curve: `rho` (rho.negation reference); baseline `res2`, `res4`, `res8`; candidate `res2f`,
  `res4f`, `res8f` (identical to the baseline arm except `targets`); `res4aa` = byte-identical copy of `res4`
  (A/A: noise floor / harness symmetry).
- 16 targets per curve, target seed 20261201 (fresh, never used before), measurement seed 20261202,
  3 rounds + 1 warm-up, `alternate_measured` order, isolation_required L2.
- Counted units are deterministic given (spec, seeds); wall clock is not a claim here.
- Positive control: the harness verifies every recovered log (`ecbench verify --replay 12`).

## Decision rule (frozen)
For each pair (res{B}, res{B}f), paired by instance, with the harness's paired bootstrap 95% interval:
- C1 holds for B at weight w iff the whole interval of S_fused/S_baseline is < 1 at w = 0 AND at w = 0.1 and 1.
  Expectation (not a threshold): about 0.55-0.6 at w = 0 (2 adds -> 1 add per step, plus setup).
- A/A: the interval of res4aa/res4 must contain 1; otherwise the session is void and rerun.
- Null (C3): `examples/ic_residual_curve_free_null.rs`-style comparison; any difference in relations per
  step beyond sampling noise is reported, not hidden.
- Negative results (fused >= baseline at any weight, or A/A failing) are reported as such.

## Measures
`ecbench band --weights 0,0.01,0.03,0.1,0.3,1`, `ecbench compare --a res{B} --b res{B}f [--weight w] --save`,
`ecbench compare --a rho --b res{B}f`, `ecbench verify --replay 12`. `scripts/analyse.sh` regenerates all.

## Execution
Counts are exact and machine-independent; the route (isolab-eks vs this container) affects only isolation
metadata. If L2 admission is unavailable the sessions are run here at the lowest isolation the harness
accepts, the isolation tier is recorded, and no wall-clock claim is made.

## Amendment 1 (2026-10-31, before any study data)
The isolab-eks route could not be used from this container (no `aws` CLI; GitHub workflow dispatch
returned 403 earlier in this session). The study is therefore run here with `specs-local/F-*.json`:
byte-identical to `specs/F-*.json` except `measurement.isolation_required = "L0"` (`--allow-busy`).
Counted S is deterministic given the seeds and independent of host/isolation, so C1-C3 are unaffected; no
wall-clock statement is made. The L2 specs stay committed for a later isolab-eks replay. The only prior
data are the 4-target, 1-round smoke session (scratchpad, not part of the study, used to check the harness accepts the arms).
