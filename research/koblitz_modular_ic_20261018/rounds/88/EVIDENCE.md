# R88 checked artifacts and proof map

The fourteen raw files of native attempt `runs/inverse88-01` are
committed directly; `runs.sha256` binds every byte. The frozen
source revision is `946a7eb69c43838e51e20cbde96242afa71f87d2`.
The driver checked six source/protocol/driver/input bytes against
that Git revision and four donor hashes. The checkpoint gate URI
is in `gate.log`. The attempt exited zero, printed the exact pass
marker, and emitted no Singular `?` errors. One target and three
source ideal lifts were checked, along with two corrupted controls.

The Lean module `lean/Inverse88.lean` was compiled with Lean
4.34.1 and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`.
The proof map is:

| Assertion | Lean declaration | Scope |
|---|---|---|
| exact target base product `qResidual=k E` | `Inverse88.target_base_product` | any commutative ring with `2=0` |
| target base return on `E=0` | `Inverse88.target_base_return` | same, conditional on target equation |
| cleared affine lambda return | `Inverse88.lambda_return` | same |

`lean-audit.log` records the axiom screen. The source-side
coefficient and offset products are native ideal certificates,
not Lean theorems. `lean4checker --fresh` is unavailable here.
The one-page PDF and SVG were rendered and visually inspected.

For byte-only replay, run `sha256sum -c runs.sha256` in this
directory. The raw attempt is immutable. A repair or new
scientific calculation receives a fresh attempt ID.
