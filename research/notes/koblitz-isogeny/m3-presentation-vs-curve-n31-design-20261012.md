# m = 3 curve-vs-presentation at n = 31 (design, 2026-10-12)

This is an additive amendment to `presentation-vs-curve-design-20261003.md`. The model,
the estimator and the thresholds are unchanged:
- **H0:** the 95 % upper bound on ICC_curve is < 0.05 on every metric.
- **H1:** any cost metric has ICC ≥ 0.10 with its lower bound > 0.
- Otherwise: **inconclusive**.

It follows the n = 19 m = 3 arm (`m3-presentation-vs-curve-design-20261011.md`, which
was inconclusive). This arm switches to the `PinX3` solver and uses 5× the probes, to
tighten the yield bound.

## Arm

| item | value |
|---|---|
| class | n = 31, a₂ = 0, from the walk (the same class as the m = 2 n = 31 arm) |
| sample | 32 curves (the sweep's seeded hash, K_a included) |
| subspaces | geometric V_s, s = 1..4 |
| decomposition | m = 3, l = 10, `M3Solver::PinX3` |
| probes | 2000 full-group probes per cell |
| order | cells shuffled with a fixed seed |
| command | `KOBLITZ_PRES_M3=pin KOBLITZ_PRES_GEOMETRIC=1 KOBLITZ_PRES_FFD_TARGETS=0 koblitz_presentation_sweep 31 0 10 2000 32 1,2,3,4` |
| output | `experiments/koblitz_presentation/pres_31_0_l10_geom_m3pin.jsonl` |
| analysis | `examples/koblitz_presentation_icc.rs` |

## Metrics

- **yield per |F|³.**
- **ln µs per call.**
- **work per call:** (s, p) roots per call. PinX3 records them in `sym_roots`.
- **ln projected solve.**

First fall is not measured.

## Expected precision

The pilot full solves at n = 31 gave about 0.09 relations per probe, so a cell has
about 180 relations.

## Budget

About 128 × 2000 × 7 ms ≈ 0.5 core-h.

## Results (2026-10-12)

The run completed all 128 cells in 872 s on 4 cores.
- **Soundness:** every cell matched BSGS, with 0 inconsistent relations.
- **Yield:** 20661 relations from 256000 probes; median 160 relations per cell.
- **Cost:** median 16.6 ms per call.

Analysis output: `experiments/koblitz_presentation/icc_n31_m3_geom.json`.

### Pre-registered metrics

| metric | ICC_curve | 95 % CI | subspace share |
|---|---|---|---|
| yield per \|F\|³ | 0.000 | [0, 0.116] | 0.000 |
| ln µs per call | 0.048 | [0, 0.216] | 0.011 |
| **work per call** ((s, p) roots) | **0.228** | **[0.032, 0.403]** | 0.009 |
| ln projected solve | 0.067 | [0, 0.219] | 0.017 |

**Verdict by the pre-registered rule: H1, on one cost metric (work per call).** Its ICC
is ≥ 0.10 and its lower bound is above 0. The other three metrics are consistent with
no curve effect, but they do not clear the 0.05 H0 bound.

### Mechanism (post hoc, not pre-registered)

`KOBLITZ_ICC_DIAG=1`, output in `icc_n31_m3_geom_diag.json`:

| diagnostic | ICC_curve | 95 % CI |
|---|---|---|
| factor-base size \|F\| | 0.239 | [0.025, 0.437] |
| PinX3 pins per call | 0.230 | [0.035, 0.406] |
| roots per pin | 0.000 | [0, 0.138] |
| work per call ÷ \|F\| | 0.086 | [0, 0.297] |

- PinX3 tries every x₃ in the factor base, so its work per call is proportional to
  |F|.
- **The curve effect on work is the curve effect on |F|.** Per pinned x₃, the linear
  algebra shows none (roots per pin: 0.000).
- Yield normalised by |F|³ shows none either.

|F| is curve-dependent in this class, consistently across the four V. The same
diagnostic on the earlier data:

| data | ICC(\|F\|) |
|---|---|
| n = 19, m = 3, a₂ = 1 | 0.000 [0, 0.000] |
| n = 23, m = 2, a₂ = 1 | 0.011 [0, 0.085] |
| n = 19, m = 2, a₂ = 1 | 0.000 [0, 0.016] |
| n = 31, m = 2, l = 16, a₂ = 0 | 0.087 [0, 0.202] |

The a₂ = 1 classes have cofactor 2. This class has cofactor 1492 = 4·373, and
factor-base entries are folded by the cofactor projection. A plausible cause is that
the cofactor-torsion structure differs across the isogeny class; the sample spans
volcano depths 0 and 1. That is consistent with the cofactor-balance yield spread in
`yield-spread-results-20261003.md`. **It is not tested here.**

### Reading

At n = 31, the curve changes the size of the factor base its x-coordinates produce
in V, and so the work an attacker spends per probe and the relations they must
collect. It does not change the per-element decomposition algebra, or the yield per
|F|³.

Scope: m = 3, l = 10, geometric V, this class (a₂ = 0, large cofactor) and this
solver. The effect is a factor-base-size effect, not a hardness difference in the
decomposition problem.

**Next test:** ICC of |F| against volcano depth and cofactor-torsion type, and the same
arm on an n = 31 class with a small cofactor, if one exists.

## Follow-up (2026-10-18): the |F| effect and the work H1 do not replicate

This is an additive amendment. The results above stand as recorded; this section
records two replications that were run after them.

**Grouping is uninformative.** In the sample, only K_a sits at volcano depth 0, and all
32 curves have Tr(a₆) = 1. So neither depth nor trace can explain a curve effect here
(`KOBLITZ_ICC_DIAG=1`, field `diag_F_by_group`).

**|F| replication.** The same 32 curves were crossed with 32 fresh geometric V (seeds
101–132). Each cell has 0 probes, so only the factor base is measured.

| class | ICC(\|F\|) | data |
|---|---|---|
| n = 31, a₂ = 0, l = 10 | 0.000 [0, 0.007] | `pres_31_0_l10_geom_F_rep_s101-132.jsonl` |
| n = 19, a₂ = 1, l = 6 | 0.000 [0, 0.009] | `pres_19_1_l6_geom_F_rep_s101-132.jsonl` |

At n = 31, the residual variance of |F| is about 282, which is roughly binomial.

**Full-arm replication.** The pre-registered arm was re-run unchanged except for the
seeds: 32 curves × geometric V with seeds 5–8, 2000 probes per cell, PinX3. Data is in
`pres_31_0_l10_geom_m3pin_rep_s5-8.jsonl`, and the ICCs are in
`icc_n31_m3_geom_rep_s5-8.json`.

| metric | original (seeds 1–4) | replication (seeds 5–8) |
|---|---|---|
| work_per_call | 0.228 [0.032, 0.403] | 0.000 [0, 0.043] |
| yield per \|F\|³ | 0.000 [0, 0.116] | 0.075 [0, 0.243] |
| log µs/call | 0.048 [0, 0.216] | 0.017 [0, 0.170] |
| log projected solve | 0.067 [0, 0.219] | 0.094 [0, 0.253] |
| \|F\| (diag) | 0.239 [0.025, 0.437] | 0.000 [0, 0.033] |

**Reading.** The H1 on work_per_call, and the |F| effect that explained it, do not
replicate on fresh subspaces. With only four V per curve, the original estimate most
likely picked up chance alignment between particular curves and particular V; the
bootstrap over curves does not capture that. The m = 3, n = 31 result is therefore
**not H1**. It is inconclusive: no point estimate reaches 0.10 with a positive lower
bound, but the upper bounds on yield and projected solve stay above 0.05. The earlier
"factor-base-size effect" reading is withdrawn. The plausible cause discussed above
(cofactor-torsion structure) is moot.

Lesson for the design: with four subspaces, a curve × V interaction can masquerade as
a curve effect. Future arms should either use more V per curve or require
replication on fresh seeds before calling H1.
