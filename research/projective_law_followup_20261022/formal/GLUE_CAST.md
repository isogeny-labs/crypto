# Off-diagonal gluing transition: checked cast

`Common22GlueCast.lean` relates the `false → true` transition in the
constructed `Scheme.GlueData` to the previously defined overlap transition.
It proves the two off-diagonal overlap objects reduce to the separately
presented affine and infinity overlap schemes. The transition equality uses
the resulting equality casts. This is an identity inside the **constructed**
two-chart gluing datum.

The exact source is checked with Lean v4.34.1 and pinned Mathlib commit
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The build log and
`glue-cast-validation.json` record the exit result, source hash, and axiom
audit. `glue-cast.csv` lists checked and open gates; `glue-cast.svg` and
`GLUE_CAST.pdf` display the same boundary.

The theorem does not identify the glued scheme with a chosen smooth
projective genus-2 completion. Infinity stalks, divisor or `Pic⁰` transport,
production Rust-law semantics, and operation costs remain separate.

To replay from this directory, build `Common22ExplicitChart.olean`,
`Common22PrincipalOpenEquation.olean`, `Common22SeparateChartMap.olean`,
`Common22PrincipalOpenSpec.olean`, and `Common22TwoChartGlue.olean` from the
pinned source files in that dependency order, then run
`lake env sh -c 'LEAN_PATH="/dev/shm:$LEAN_PATH" lean Common22GlueCast.lean'`.
Lake supplies the pinned dependency paths and the inner shell adds the local
checked modules. The packet's build
log records the exact completed check, including `#print axioms`.
