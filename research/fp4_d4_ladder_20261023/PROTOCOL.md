# Protocol: the cheaper `D₄` decomposition end to end on the `E(F_{p⁴})` ladder

2026-10-23. Frozen before any measured data of this study. Item 4 of the
extension-field programme.

## Question

`research/gaudry_quartic_d4_cost_20261014/` cut the cost of one `k = 4`
decomposition from `C₄ = 2.55·10¹⁰` to `6.89·10⁷` `F_p` multiplications
(weighted Macaulay degree 26, chains, sparsest pivots, F4 replay; `p = 269`),
and measured the new solver end to end only at `p = 29`. The ladder of
`research/ic_next10_20261014/01-fp4-ladder/` (session `ECBS1hf49e5346dc2b`)
still charges the old `C₄`. **What is the end-to-end `S` of the `k = 4` D₄
pipeline with the new solver at the ladder sizes `p = 101, 181, 269, 397`,
and how far is it from negation rho?**

Class (AGENTS.md "Success criterion"): a stage improvement measured into
end-to-end `S`. The method's exponent is unchanged.

## Baseline

The current verified pipeline on these sizes is the ladder's `ic-k4-d4`
arm (`ic.k4_gaudry`, `base = yline-d4`, `c4 = 2.54e10`): mean `S` 1.950·10⁸,
1.495·10⁸, 7.346·10⁷ and 3.835·10⁷ at `p` = 101, 181, 269 and 397. That arm
draws its relations from an enumeration oracle whose work is uncharged,
so those `S` are **lower bounds** on the old pipeline. It ran on
`fp4_translation` curves. This study's curves are `fp4_torsion` (the
construction the real `gaudry-d4` solver runs on), so the comparison with
the baseline is at equal `p`, **not** on the same instances.

A same-instance comparison is not affordable: the committed solver takes
about 8,000 s per solve at `p = 29` (`k4-p29-d4cost`). Its relation stream
equals the new solver's (identical decomposition sets on all 648 closed
solves of the cost study), so a **derived** same-instance baseline is also
reported: the candidate's own record with every solver call re-priced at
the committed `C₄`. It is labelled derived, and it is accounting, not a
measurement.

## Instrument

`ecbench` at the commit that freezes this file. Specs
[`specs/SPEC-A.json`](specs/SPEC-A.json) (`p = 101, 397`) and
[`specs/SPEC-B.json`](specs/SPEC-B.json) (`p = 181, 269`), run as two
concurrent sessions; `fp4_torsion` curves, seed 1, `max_cofactor = 16`, 2
planted targets per curve.

- `ic-d4-new` (candidate): `ic.pipeline`, factor base `fghr-y-line:zero=1`,
  oracle `gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1,charge_unsolved=1`,
  linear algebra `incremental-gauss` (the cost study's configuration;
  `research/sparse_la_timing_20261018` found no sparse solver cheaper end to
  end on this ladder).
- `rho-neg` (reference): `rho.negation`, the strongest generic rho these
  curves carry (`A = 2`: no subfield or extra automorphism; the curves'
  coefficients lie outside `F_{p²}`).
- Audit: `ecbench verify --replay 4` on each session (every IC solve is
  long; the replays cover rho and IC records alike).

## Decision rules (fixed now)

- **Correctness:** every answer verified, and audit 0 problems.
- **Improvement (pipeline):** at a size `p`, "end-to-end `S` lowered" iff
  the candidate's mean `S` is below the baseline's lower bound at the same
  `p` (above) **and** below the derived same-instance committed price. Both
  hold or the size is reported as not shown.
- **Distance to rho:** `S(ic-d4-new) / S(rho-neg)` per size, with the §5a
  band (`ecbench band`, weights 0, 0.1, 1 GAE) on every arm; the gap is
  reported as distance remaining, not as pass or fail.
- **Handover:** least-squares fit of `log S(ic-d4-new)` against `log r` over
  the four sizes, extrapolated to `S = S(rho-neg)`; descriptive.

---

## Deviations log (appended; the text above is unchanged and was committed before any data)

1. **Execution only.** The container was reclaimed at about 07:35-08:18 UTC while the agent session was idle; every process died (uptime reset), the disk survived. Card `CARD-fp4-d4-ladder-20261023-a2` left session `sessions/A` unfinished (never finalised); card `-b2` had not created `sessions/B`. ecbench sessions cannot be resumed, so `sessions/A` is kept as an interrupted, uncited session and both specs are re-run into `sessions/A3` and `sessions/B3`, unchanged.
