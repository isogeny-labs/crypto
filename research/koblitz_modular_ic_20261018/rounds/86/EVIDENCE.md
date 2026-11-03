# R86 checked artifacts

The native attempt `runs/affine86-01` is committed directly in this
directory because its raw files are small. `runs.sha256` binds every raw
file. Its source revision is
`a7837ba7b98f3a9b32e1b279e9b93695d0899b19`;
`source.sha256` was checked from inside the raw directory, and the
driver checked all six source bytes against that historical Git
revision plus all four donors. The checkpoint gate recorded its
S3 URI in `gate.log`. The native exit is zero, the exact pass marker
is present, and there are no Singular `?` errors. Both deliberately
corrupted controls were rejected.

The Lean module is `lean/Affine86.lean`. It was compiled with Lean
4.34.1 and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`.
The checked declarations are:

| Mathematical assertion | Lean declaration | Scope |
|---|---|---|
| `k=(u+1)c²` from `b=uc²+c` | `Affine86.k_simplification` | any commutative characteristic-two ring |
| cleared lambda cross-product | `Affine86.lambda_cross_product` | same |

The audit reported two theorems, zero failing and zero trusted.
`lean4checker --fresh` was unavailable in this environment. The
congruences of the explicit `b,c` polynomials modulo the inverse-base
equation and the upper relative-field equation are not formalized by
these two Lean lemmas. The one-page PDF and its SVG source were
visually inspected.

Replay the native source in a fresh attempt name only if independent
reproduction is required. To check retained raw bytes without
repeating the producer, run `sha256sum -c runs.sha256` from this
directory. The raw record is immutable; a correction gets a new
attempt ID or round.
