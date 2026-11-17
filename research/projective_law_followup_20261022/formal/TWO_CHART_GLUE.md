# Two-chart glued scheme: checked construction

Pinned Lean now constructs a `Scheme.GlueData` from the separate affine and infinity-chart schemes, their localized overlap spectra, the checked overlap isomorphism, and the two checked open immersions. The resulting `gluedCurve` is a scheme. Both chart maps into it are open immersions and jointly cover its underlying topological space. The actual source is `Common22TwoChartGlue.lean`; `two-chart-glue-build.log` records a successful replay and axiom audit. All five audited new declarations depend only on `propext`, `Classical.choice`, and `Quot.sound`.

The construction uses the pinned `CategoryTheory.GlueData'` interface with index type `Bool`. Its diagonal overlap data is manufactured by `GlueData.ofGlueData'`. The required `t'`, factorization, and cocycle obligations in `GlueData'` quantify three pairwise-distinct Boolean indices, so the Lean proof closes each by contradiction. The two off-diagonal transitions are the inverse maps of `principalOpenSpecIso`.

| Gate | Status | Evidence or remaining theorem |
| --- | --- | --- |
| Two chart schemes, overlaps, embeddings and transitions | Checked | `chart`, `overlap`, `embed`, `transition` |
| Full two-chart gluing datum | Checked | `preData`, `glueData` |
| Glued scheme | Checked | `gluedCurve` |
| Chart maps open and jointly covering | Checked | `chartIntoGlued_open`, `chartIntoGlued_jointly_surjective` |
| Isomorphism with a chosen smooth projective genus-2 completion | Open | Define the completion and its chart maps; prove chart coverage and transition agreement, then an isomorphism |
| Infinity stalk, principal divisor and Pic⁰ transport | Open | Establish comparison within the chosen completion and its divisor theory |
| Rust complete-law correspondence and rho cost | Open / unmeasured | Independent semantic proof and native timing |

The glued object is mathematically a scheme built from the two explicit affine charts. This packet does not prove properness, smoothness of every fiber, connectedness, or that it is the designated projective completion of the curve. It does not transport the infinity valuation or divisor class and makes no operation-cost claim. The standardized gate data and graph are `two-chart-glue.csv` and `two-chart-glue.svg`.

Replay with pinned dependencies from this directory: compile `Common22PrincipalOpenSpec.lean` to `/dev/shm/Common22PrincipalOpenSpec.olean`, then run `TMPDIR=/dev/shm LEAN_PATH=/dev/shm lake env lean Common22TwoChartGlue.lean`. The proof source uses no admitted theorem.
