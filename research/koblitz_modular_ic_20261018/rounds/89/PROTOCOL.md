# R89 all-32 normalized RR generator transport, frozen before data

Use the accepted R50 PID basis `B` of rank 32, its common
denominator `F(z)` of degree 37, and its saved full source
embedding `B*S=F I`. The source power basis is
`T^i V^j`, `0<=i<16`, `0<=j<2`; normalized generators are
`e_a=sum B_ia T^i V^j/F`. The saved PID bytes have 32 basis
columns, maximum coefficient degree 37, and index 376. Bind
the exact 2.7 MB donor hash and check this degree census before
launch. R50's bound `14+d` is therefore 51.

Build the accepted 32-dimensional source field over `F2(z)`
from R50's two monic equations. For every basis generator,
evaluate the R85/R86 forward map and R88 inverse map inside
the quotient field, including substitution into all coefficient
polynomials and the common denominator. In each direction,
multiply by successive powers of its chart-overlap unit
(`h=z+V` forward, `k=z+V+z²` inverse), stopping at the first
power at most 51 whose 32 coordinates in the normalized `e`
basis are all polynomials in `z`. Extract and independently
reconstruct every power-basis vector; verify the full identity
`B*C=F*W` for each of the 64 column images. Corrupt one
coefficient in every column and require the product to fail.
Export all 64 pole powers, coefficient vectors, and power-basis
vectors to `transport89.sing` for a separate reload check.

Run one native Singular/FLINT process, seed 89, one thread,
4 GiB virtual memory, 1200 seconds. Commit and publish the
source/protocol and donor hashes before execution. Accept only
exit zero, exact marker, no native `?` errors, all 64 columns,
all full products, all corruption rejections, and a complete
certificate export. A failure identifies its direction, column,
and stage where possible; retain raw output, then freeze any
repair under a fresh attempt ID. No timing comparison.

This tests localized module membership for the 32 actual R50
generators. It does not by itself identify exceptional branches
in a proper projective model. Sources checked: R50
`RR-ALGEBRA.md` and `NEXT_SOURCE_NORMALIZATION.md`, retained
R50 PID and relative-equation exports, and R88 `CM.md`.
