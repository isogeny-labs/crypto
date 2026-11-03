# R88: explicit inverse of the rational diamond source map

Write the transformed quadratic-base coordinates as `p,q` and
the transformed upper coordinate as `t`. Set
`k=p+q+p²` and `G=p k²+p+k²`. The inverse formulas on the
registered open are

`z=1/k`, `V=1/k+1/p`,
`T=G[t+(k+1)²]/(p k⁴)`.

The first two invert the accepted R85 base map. The last is
obtained by solving R86's affine update for the original
`T=lambda`. The frozen R88 Singular run checked that `p,k,G`
are nonzero in the target quadratic function field. Its complete
target-side `q` return product is

`p[(p+k)(k+1)+p]+q k² = k(q²+p²q+p+p³)`.

The run checked the zero remainder and full ideal lift, as well
as exact `p` and affine `t` cancellations. In the original
`z,V,T` chart, it checked the two R85 base products and the
coefficient, offset, and complete `T` composition products against
`E=V²+z²V+z+z³`, with three full ideal lifts. Corrupting the
inverse `T` numerator and the source-side returned `T` each
produced a nonzero residual. The single attempt exited zero,
printed its exact marker, emitted zero Singular `?` errors, and
matched six historical source-byte pins and four donor hashes.

Together with R87's preservation of the complete monic degree-16
relative equation, this gives a rational two-way diamond map of
the accepted source field on the registered open. It does not
establish regularity of the normalized affine charts at their
boundary branches. The next exact task is transport of all 32
R50 RR PID basis generators in both directions with full module
lifts and explicit pole clearing.

The Lean module proves the target base product, its zero consequence
under the target equation, and the affine `t` cancellation in any
characteristic-two commutative ring. The three source-side ideal
lifts remain native certificates. Sources checked: R50
`NEXT_SOURCE_NORMALIZATION.md` and R85–R87 `CM.md`.
