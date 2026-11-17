# L4-A amendment 1: exclude global relations from planted targets (2026-10-29)

Registered after the v1 run and before the v2 run. The v1 protocol, its
hash and its data (`data-v1/`) are kept unchanged.

## What v1 showed

The registered falsifier **fired**. At ℓ = 2 on 40 bits, `B = 4` succeeded
on 3 of 4 planted targets (`B = 3`: 2/4), and the reach did not decrease
from 32 bits (`B = 2`) to 40 bits (`B = 4`). The reading under the v1 rule
is "slack beyond §3.2".

## Diagnosis (post hoc, `[measured]` by PARI/GP)

**Every planted success at every cell has `S₄(x; z) = 0` over ℤ.** That
was checked on the symbolic resultant, specialised afterwards
(`zcheck2.gp`). Each success has `z ∈ {1, 2, 3, 25}`, and most are trivial
decompositions: a repeated `x` with `z` equal to the remaining `x`, i.e.
`P − P + Q = Q`.

Cause:
- Small `x_i ∈ [1, B]` repeat often.
- The planting rule took any `F_p` root of `S₄(x; Z)`, and for such `x` the
  quartic has small integer roots.
- A lattice containing `S₄` (whose specialisation at a small integer `z`
  has small integer coefficients) then holds a short vector that vanishes
  at `e*` over ℤ. Success is automatic, and says nothing about modular
  small roots.

This is the F6 global-lifting case, not a lattice reach. The one large-`z`
target at a "successful" bound failed.

## Change

Planted roots with `S₄(x; z) = 0` over ℤ are redrawn. This is tested as
vanishing modulo the independent prime `2^61 − 1`; a false rejection has
probability about `2^{−61}`. If no non-global root appears in 500 draws,
the cell prints `skipped` and counts as having no plantable target (B = 1,
where `x = (1, 1, 1)` is forced). Everything else is unchanged: the cells,
seeds, success rule, reach rule, falsifiers and the binary's lattice code.

Binary `semaev_lattice_reach-v2`; hashes in `BINARY.sha256`.
