# Protocol: why the k = 4 solve loses 6 solutions on a conjugate-pair line, and a solver that recovers them (frozen before data)

2026-11-06. This follows
[`lift_dense_conjugate_pair_decomp_20261106`](../lift_dense_conjugate_pair_decomp_20261106/README.md).
On curve 1 of that study, both conjugate-pair targets that decompose gave
quotient dimension 4,090 instead of 4,096 at Macaulay degree 29–32, and
`gaudry_quartic`'s solver returned nothing. All other solves were generic.

This file is committed before any new code or computation. No count of
short decompositions has been looked at.

## Hypotheses (fixed before data)

The Weil-restricted symmetrised S₅ system is 4 equations of degree 8 in
e = (e₁, …, e₄) over F_p.

- **H1 (solutions at infinity).** Bézout gives 8⁴ = 4,096 solutions in
  projective space. A quotient dimension that stays at 4,090 at every
  Macaulay degree means 6 of them lie on the hyperplane at infinity. After a
  random projective change of coordinates, every such point is in the
  affine chart, so the quotient dimension becomes 4,096.
- **H2 (what they are).** A solution at infinity in e(t)-space sends some
  t_i, and so x_i = x₀ + u·t_i, to ∞, which is the point O. The leading
  coefficient of S₅ in x₁ is S₄(x₂, x₃, x₄, x_R)², so these points are
  **3-term decompositions** R = ±P₂ ± P₃ ± P₄ over the line, or shorter
  ones. So the dimension deficit should be nonzero exactly when R has a
  decomposition into at most 3 points of the line's factor base. That
  makes the effect generic, not specific to conjugate-pair lines; those
  lines would hit it more often only because they carry many more short
  relations.
- **H3 (the fix).** With the random projective change of coordinates, the
  unchanged `solve_system4_with` returns every oracle 4-term decomposition,
  on the failing targets as well. The 3-term ones show up as solutions with
  e₀ = 0, so they are recovered as well.

## Instrument

This extends `examples/conjpair_decomp_cost.rs` (study
`lift_dense_conjugate_pair_decomp_20261106`). `gaudry_quartic` stays
unchanged. The extension adds:

- `st.outcome` recorded on every solve.
- A **short-decomposition oracle**: the number of 1-, 2- and 3-point
  multisets of the line's factor base summing to R, by exact
  meet-in-the-middle.
- A **projective transform**:
  - homogenise each Weil component to degree 8 with e₀;
  - draw a uniformly random A ∈ GL₅(F_p) (seeded) and substitute
    (e₀, …, e₄) = A·(y₀, …, y₄);
  - dehomogenise at y₀ = 1, then solve in y₁..y₄;
  - map each rational solution back by (e₀, …, e₄) = A·(1, y);
  - e₀ ≠ 0 gives an affine solution e/e₀, and e₀ = 0 is a point at
    infinity.

  The transform's F_p multiplications are counted and added to
  total_muls.
- **New targets**, drawn after the existing ones from a separate seeded
  stream so the main run's curves and targets are unchanged. Per arm and
  curve there are 2 targets planted as **3-point** sums of that arm's
  factor base.

## Runs

Same p = 29, same 2 curves, same lines and H as the main run.

| set | targets | solves |
|:--|:--|:--|
| R1 | the 12 main-run targets, plain (no transform) | 12. These repeat the main run and must reproduce its total_muls and dim exactly (A/A against `reports/run-20261106T0643Z/rows.jsonl`), now with the outcome recorded |
| R2 | the same 12 targets, transformed | 12 |
| R3 | the 8 new 3-point-planted targets (2 per arm × curve), plain and transformed | 16 |

Every target also gets the short-decomposition counts.

## Predictions (decision rules)

**Q1-a (H2, exact).** On every plain solve (R1 and R3), dim < 4,096 if
and only if the short-decomposition count (1, 2 or 3 points) is
non-zero. 0 exceptions.

**Q1-b (H1, exact).** On every transformed solve (R2 and R3), dim = 4,096.

**Q1-c (both arms).** On R3, the plain solve has dim < 4,096 on **both**
arms, std included.

**Q2 (H3, exact).** On every transformed solve, the affine solutions
(e₀ ≠ 0), after the quartic split and the lift filter, equal the oracle's
4-point decompositions. That includes curve 1's two targets that failed
before.

**Q2-cost (registered band).** The transformed total_muls on the 12 R2
targets lies within [0.9, 1.1] × the main run's generic plain cost on the
same curve and arm.

**Q1-d (report only).** The number of points at infinity among the
transformed rational solutions, against the short-decomposition counts.

If Q1-a fails, the deficit has another cause, and the run reports what the
recorded `st.outcome` strings say.

## Host, analyst

- Local container, as before. The metric is a deterministic count.
- The analyst is the producer, using coded rules computed by a Rust
  `--summarize-inf` mode.

## Amendments

(none)
