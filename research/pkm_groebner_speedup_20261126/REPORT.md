# PKM Gröbner speedup continuation

All empirical measurements are `[pcc:unverified]`.

## Radix-16 qualifier: rejected

The frozen development cell used the same two `m=2`, `t=8` Kummer
refutations under unit, radix-8, and radix-16 gradings.  Both draws were
bit-identical within each arm and all three arms refuted the systems.

| grading | solving degree in that grading | max columns | max dense entries | F_p multiply-adds |
|:--|--:|--:|--:|--:|
| unit / radix 2 | 5 | 4,789 | 3,588,407 | 5,680,719,625 |
| radix 8 | 11 | 3,818 | 2,474,085 | 1,818,707,118 |
| radix 16 | 20 | 7,660 | 5,132,616 | 2,653,868,754 |

Radix 16 is **0.685x** as fast as radix 8 in counted work (1.459x more
multiply-adds), and only **2.14x** faster than unit grading.  It therefore
fails both preregistered gates (at least 2x over radix 8 and at least 10x over
unit) on the development cell.  Confirmation cells were not run after this
directional failure.  Weighted degrees are not ordinary total degrees and are
not compared across rows.

Raw rows: `data/dev/`.

## Audited existing drastic result selected for integration

The repository's just-merged degree-32 five-Vélu-step PKM study supplies the
qualifying mechanism that radix ordering does not:

- equal system dimensions and equation-degree vector;
- ordinary F4 solving degree **6 -> 5**;
- maximum nonzeros about **6x** smaller;
- aggregate F4 multiply-adds about **32x** smaller on the reported matched
  13/14-bit systems;
- three source-pinned equal-18, 13-bit end-to-end component instances all
  recovered every planted scalar, with combined charged GAE ratio 0.3045.

Primary evidence:
`research/pkm_adaptive_sweep_20261123/ANALYSIS_F4_DEGREE32_20261125.md` and
`RESULT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md`.  This is a
small-instance, selected-instance relation-extraction result, not a rho
crossover or an asymptotic improvement.

This continuation integrates a native Rust demonstration of that exact
matched strategy so the degree and operation-count comparison can be rerun
without the prior Python orchestration.

## Native Rust reproduction

`cargo run --release --example pkm_groebner_drastic` completed twice with
bit-identical counted results (`RAYON_NUM_THREADS=4`, binary SHA-256
`4f915bc92b7d32afd081c7badd5b5ccfd4c3ad1e1e129218420bb61a41bef332`):

| arm | base abscissae | shape | solving degree | F4 multiply-adds | scalar |
|:--|--:|:--|--:|--:|--:|
| interval bits | 18 | 10 vars, 11 equations, `[4,2x10]` | 6 | 14,012,270,724 | 2261, verified |
| degree-32 five-Vélu | 18 | same | 5 | 512,636,229 | 2261, verified |

The native reproduction is **27.33x** lower in counted F4 work.  The example
asserts the matched cardinality and shape, both scalar checks, the exact 6->5
ordinary-degree change, and a conservative >=20x operation gate before it
prints success.  Receipt: `NATIVE_RESULT.json`.

## Complete native pipeline continuation

The 2026-11-27 frozen continuation ran two serialized balanced pairs in the
order interval / degree-32 Velu / degree-32 Velu / interval. Each observation
rebuilt its arm and ran from factor-base construction through verified scalar
recovery. All eight runs recovered scalar 2261, had no cost qualification,
and reconciled their five phase charges exactly to `total_gae`.

| arm | observations | summed total GAE | summed cold wall | per-run F4 work | ordinary solving degree |
|:--|--:|--:|--:|--:|--:|
| interval | 4 | 283,364,531.60 | 22.613 s | 14,012,270,724 | 6 |
| degree-32 five-Velu | 4 | 94,799,783.67 | 7.575 s | 512,636,229 | 5 |

The candidate used **0.33455x** the complete charged work, a **2.989x**
selected-fixture end-to-end reduction, and passed the preregistered <=0.5x
gate. Its factor-base construction added 1,143 GAE per run versus zero for
the interval, while the relations-and-solver phase fell from 69.65--71.89M
GAE to 23.02--24.39M GAE. Linear algebra and verification were retained.

Summed cold wall was **0.33500x**, or **2.985x** faster. That wall result is
descriptive: the shared host was serialized for this run but not formally
load-isolated. The F4 unit in `total_gae` is also host-measured because this
generated curve has no pinned `ns_per_f4_muladd`; every receipt records
`solver_priced_by = measured`.

This promotes the result from a relation-extraction counter win to a complete
native-pipeline win only on the selected 13-bit fixture. It remains neither a
random-instance result, an asymptotic improvement nor a Pollard-rho crossover.
Frozen receipt: `NATIVE_E2E_RESULT.json`; protocol commit `49f652707f`;
release binary SHA-256 `5be9a8ce836f33d1a7d20683e7753565e5669ae963226cd5273148b3c5ef07e7`.
