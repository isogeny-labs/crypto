# Two-chart gluing gate: pinned API audit and open obligations

The preceding packet checks an isomorphism of the two overlap spectra and checks their open immersions into the separate affine chart spectra. This packet records the next construction required by pinned Mathlib. `Common22GluingApi.lean` imports the scheme gluing API and checks its named interfaces, plus the already proved overlap isomorphism. It constructs no glued curve.

`Scheme.GlueData` extends `CategoryTheory.GlueData`. In the pinned source, a gluing datum requires chart schemes `U i`, overlap schemes `V (i,j)`, open immersions `f i j`, transition maps `t i j`, and a map `t' i j k` between pullbacks. It also requires diagonal isomorphisms, identity transitions, the factorization `t_fac`, and the triple `cocycle`. The checked two-chart overlap data supplies the off-diagonal candidate `V`, `f`, and `t`; it does not by itself fill all of these fields. A generic two-chart constructor was not found in the pinned `Mathlib/AlgebraicGeometry`, `Mathlib/Geometry`, or `Mathlib/CategoryTheory` tree by the recorded source search, so this packet does not assume one.

| Gate | Status | Exact next proof |
| --- | --- | --- |
| Pinned `Scheme.GlueData` and `glued` API | Checked API | `Common22GluingApi.lean` replay |
| Off-diagonal overlap scheme and open immersions | Checked in preceding packet | `principalOpenSpecIso` and two embedding instances |
| Full two-chart `GlueData` | Open | Define diagonal overlaps and all `t'`, prove `t_fac` and `cocycle` |
| Glued scheme exists for that datum | Open | Instantiate `Scheme.GlueData.glued` after construction |
| Chosen smooth projective genus-2 completion | Open | Construct a target completion and prove chart coverage and transition agreement, then an isomorphism from the glue |
| Infinity stalk / divisor / Pic⁰ | Open | Transport through the completed scheme isomorphism and prove valuation/divisor statements |
| Rust semantics / rho cost | Open / unmeasured | Independent semantic proof and native experiment |

Even a constructed `Scheme.GlueData.glued` would only establish a scheme assembled from these charts. Identifying it with a *chosen smooth projective* curve is a separate theorem requiring the target and its chart embeddings. No smoothness, projectivity, divisor, runtime, or cost claim is made here. The CSV and SVG preserve this boundary for review.

Pinned replay: compile the preceding three source files to oleans in `/dev/shm`, then run `TMPDIR=/dev/shm LEAN_PATH=/dev/shm lake env lean Common22GluingApi.lean`. The exact API and axiom output is in `gluing-api-build.log`.
