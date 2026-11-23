# R94 accepted principal-open unit bridge

The single frozen `overlap94-01` attempt passed with native exit zero,
no `?` errors and the exact `OVERLAP94_ALL_CHECKS_PASS` marker. In the
accepted R50 degree-32 field, the seven functions `1,u,v,h,k,H,J` have
polynomial rank-32 coordinates, all 224 coordinates pass the full
`B*C=F*W` product, and a corrupted coordinate is rejected. The source
also checks polynomial identities

    h²+uH=E,       k²+uJ=E,       h+N+1=H,

where `E=v²+u²v+u+u³`, `h=u+v`, `k=u+v+u²`,
`H=1+u+u²+uv`, `J=H+u³`, and `N=v+uv+u²`.
In the source field the forward image of `k` and reverse image of `h`
both equal `1/u`. A changed forward expression is rejected.

The independent ideal-free F2(u) replay from a **fresh S3 readback**
reconstructs the seven named power vectors, all 224 polynomial
coordinates, the full matrix product and a corrupted coordinate. Its
exact marker is `REPLAY94_ALL_CHECKS_PASS`. The deterministic 18-member
archive SHA-256 is
`79a58d0709cc09d4526cdc7260533bf9e3760e5ea96b6442fd56710ffc6dfccc`.
Checksum-protected upload to `setec-astronomy-artifacts`, fresh GET,
byte comparison and all member hashes passed.

On either principal open, the identities `h²=uH` and `k²=uJ` make `u`
invertible. The mapped opposite overlap unit is `1/u`, hence invertible.
Together with R91's all-generator localized membership and R88's
rational inverse identities, this supplies an explicit overlap-ring
isomorphism for the accepted rank-32 algebra. The identification of
these two charts with a proper normalized source still uses R50's
complete-RR/properness hypotheses and the R27 cusp-support bridge;
this round does not newly certify those boundary hypotheses or a
nonzero order-11 modular-factor action.

R93 is retained as the rejected coefficient-inversion attempt.
Sources: R50 `RR-ALGEBRA.md`, R88 `INVERSE88.sing`, R91 `CM.md`,
[Stacks gluing](https://stacks.math.columbia.edu/tag/01JA),
[Stacks normalization](https://stacks.math.columbia.edu/tag/035E).
