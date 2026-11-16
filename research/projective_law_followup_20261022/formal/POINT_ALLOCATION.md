# Common 2+2 affine point allocation

Status: conditional pointwise theorem. It is not a divisor or `Pic⁰` proof.

The source-style common 2+2 polynomial identity yields, at every affine
point `(x,y)` satisfying `y²=f(x)`, the evaluated norm equation

`(n(x)-b y)(n(x)+b y)=u₁(x)u₂(x)s(x)`.

When the right side is zero, `b ≠ 0`, `2 ≠ 0`, and `y ≠ 0`, exactly one of
the conjugate factors vanishes. At `y=0`, both vanish. This gives a precise
pointwise exception at a rational branch point; it does not count the
scheme-theoretic multiplicity there.

| Statement | Lean declaration | Status | Conditions |
| --- | --- | --- | --- |
| Evaluated norm | `Common22PointAllocation.eval_norm` | proved | Curve point and polynomial norm identity |
| Conjugate overlap implies branch | `Common22PointAllocation.overlap_implies_branch` | proved | `2≠0`, `b≠0` |
| Exactly one nonbranch zero | `Common22PointAllocation.exactly_one_nonbranch` | proved | Norm root, curve point, `2≠0`, `b≠0`, `y≠0` |
| Both factors at branch root | `Common22PointAllocation.branch_overlap` | proved | Norm root, curve point, `y=0` |
| Source-style evaluated norm | `Common22PointAllocation.source_style_eval_norm` | proved | Valid Mumford congruences, nonzero determinant, normalized quintic |
| `C=0` degree drop | `Common22PointAllocation.cubic_degree_le_two_of_c_zero`, `support_degree_le_one_of_c_zero` | proved | Source-style cubic coefficient `C=0` |
| `B≠0,C=0` coefficient witness | `Common22PointAllocation.b_ne_zero_c_zero_witness` | proved | Algebraic source coefficient values; no smooth-curve instance claimed |
| Zero-divisor multiplicity / residual output | none | open | Local valuations and output Mumford reduction |
| Principal divisor / `Pic⁰` | none | open | Smooth projective completion and pole order at infinity |

The [machine-readable status rows](point-allocation-status.json), Lean source,
and `point-allocation-build.log` are the proof receipt. These
results assume the proof-side source-style coefficients from
`Common22SourceHead.source_curve_eq`; a Rust-to-Lean semantic translation is
still open. The theorem does not assert that a repeated support root is
allocated with the required multiplicity. The `C=0` degree lemmas correct the
previous unconditional pole-order-six statement: pole order five for this
branch remains a written geometric claim until the infinity valuation is
formalized. No cost or cohort result changes.
