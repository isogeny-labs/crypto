# R92 complete multiplication table for the R50 rank-32 module

R50 saved 32 normalized RR generators as a free `F2[z]` module with
common denominator `F(z)` of degree 37, an index-376 PID certificate, and
the exact source embedding `B*S=F I`. R91 checked every generator under
both diamond chart transitions. R92 asks the next algebra question:
whether this module contains 1 and is closed under all 528 unordered
products of its 32 basis generators. Multiplication closure is necessary
for the proposed affine chart to be a ring; it does not alone prove
normality or projective completeness.

Work in the accepted 32-dimensional quotient field over `F2(z)`. Form
each generator directly as its R50 basis numerator divided by `F(z)`.
Check the unit's full PID coordinates and polynomiality. For every pair
`1<=a<=b<=32`, multiply the actual functions, reduce in the source
field, extract all 32 power-basis coefficients, reconstruct the element,
and compute coordinates `C=S*W`. Require the complete identity
`B*C=F*W`, every entry of `C` in `F2[z]`, and a failed product after
corrupting one coordinate. Export the unit, pair indexing, all 528
coordinate columns, and power-basis vectors. Fatal exit at the first
failed unit or pair records its index and stage; no partial certificate
is accepted as the full algebra table.

Run one serial native Singular/FLINT process, seed 92, one thread,
4 GiB virtual memory, 1200 seconds. Commit and push source/protocol
and donor hashes before execution. Accept only exit zero, exact marker,
zero `?` diagnostics, 528 pair lines, the unit check, 528 full products,
16,896 polynomial coordinates, 528 corrupted-coordinate rejections, and
a complete export. A separate ideal-free `F2(z)` replay must check the
export after fresh raw-artifact readback. Retain failures and freeze any
repair under a new attempt ID. No timing comparison is made.

The backend is the pinned local Singular installation used for R91.
Moving the larger CAS workload to EKS Spot requires a digest-pinned
Singular image and verified output receipt; the current default image
does not contain this CAS. The result's scope is multiplication closure
of the accepted R50 module, not a proof of integral closure. The
[Stacks Project normalization section](https://stacks.math.columbia.edu/tag/0C44)
identifies the integral closure as the affine normalization and explains
why ring closure is one step toward, not a substitute for, normality.

Sources checked: R50 `RR-ALGEBRA.md` and `NEXT_SOURCE_NORMALIZATION.md`,
the byte-pinned R50 PID and relative-equation exports, and R91 `CM.md`
plus full transport certificate.
