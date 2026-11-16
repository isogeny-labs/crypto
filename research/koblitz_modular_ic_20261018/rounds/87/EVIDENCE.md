# R87 checked artifacts and proof map

The 15 raw files of the one native attempt `runs/relative87-01`
are committed directly; `runs.sha256` binds their bytes. Its
frozen source revision is `e1e2359970b9755c232b95822005beb3b5fd7af7`.
The native driver checked seven source/protocol/driver/input bytes
against that Git revision and all four donor hashes. The checkpoint
gate URI is in `gate.log`. The attempt exited zero, printed the exact
pass marker, and emitted no Singular `?` errors. The complete
4,612-term pullback has a checked two-generator ideal lift; the
corrupted lambda shift produced a nonzero remainder.

The Lean module `lean/Relative87.lean` was compiled with Lean 4.34.1
and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`.
`lean-audit.log` records the axiom screen. Its proof map is:

| Assertion | Declaration | Scope |
|---|---|---|
| additive square in characteristic two | `Relative87.square_shift` | any commutative ring with `2=0` |
| additive fourth power | `Relative87.fourth_shift` | same |
| additive eighth power | `Relative87.eighth_shift` | same |
| additive sixteenth power | `Relative87.sixteenth_shift` | same |

The native source polynomial has positive `T` powers 1,4,8,16;
these Lean lemmas justify the algebra used to construct the
shifted-lambda control. They do not formalize the 145-term source
polynomial or its 4,612-term ideal lift. `lean4checker --fresh` is
unavailable on this host. The one-page report and SVG were rendered
and visually inspected.

For byte-only replay, run `sha256sum -c runs.sha256` from this
directory. The raw record is immutable. Any source correction or
scientific rerun gets a fresh attempt ID.
