# R88 inverse upper diamond coordinates, frozen before data

Use R85's accepted forward base transition `p=1/h`,
`q=N/h²`, with `h=z+V`, `N=V+zV+z²`, and R86's affine
upper update `t=[T+(1+z)²g]/(z²g)`, `g=1+z+z²+V`.
Set `k=p+q+p²`, `G=p k²+p+k²`. The proposed inverse is

`z=1/k`, `V=1/k+1/p`,
`T=G[t+(k+1)²]/(p k⁴)`.

In the target quadratic ring, check `p,k,G` are nonzero,
the complete inverse-then-forward base products modulo the target
equation, a full lift for the `q` product, and the exact affine
`t` cancellation. Reject a corrupted inverse `T` numerator.
In the source quadratic ring, check the R85 base products and
nonzero denominators. Clear all denominators in the proposed
inverse `T` coefficient and offset: with
`H=1+z+z²+zV` and `F=H²+h⁴+hH²`, they are
`F h⁴/H⁴` and `F(H+h²)²/H⁴`. Compare these with the
original R86 coefficient `D=z²g` and offset
`C=(1+z)²g`, require zero remainders, and require complete
ideal lifts for the two coefficient products and the composed
`T` product. Reject a corrupted source-side `T` return.

Run one native Singular job with frozen Git source and donor bytes,
seed 88, one thread, 4 GiB virtual memory, 1200-second cap.
Accept only exit zero, exact marker, zero native `?` errors,
all products/lifts and both negative controls. Preserve failures
and freeze any repair under a fresh attempt ID. This certifies
rational two-way coordinates on the registered open, conditional
on the accepted R50 source field; normalized RR chart regularity
at boundary branches remains a separate task.

Sources checked: R85 `CM.md`, R86 `CM.md`, R87 `CM.md`, and
R50 `NEXT_SOURCE_NORMALIZATION.md`. No timing comparison.
