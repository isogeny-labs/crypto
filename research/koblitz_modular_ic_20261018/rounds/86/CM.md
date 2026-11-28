# R86: affine lambda update on the diamond overlap

The retained R49 rational source coordinates give
`u=(b+c)/c²` and `v=(bu³+c)/((u+1)c)` on their registered
open. Solving the first equation gives `b=uc²+c`; substituting
into the second gives `c=[v(u+1)+u³+1]/u⁴`. In the R85 inverse
chart `z=1/u`, `V=z²v`, with `E=V²+z²V+z+z³=0`, set
`g=1+z+z²+V`. The candidate expressions are

`c=z(1+z)g`,
`b=z⁷+z⁶+z⁴+z²+(z⁵+z³+z²+z)V`.

The frozen Singular calculation at source revision
`a7837ba7b98f3a9b32e1b279e9b93695d0899b19` reduced
both denominator-cleared R49 coordinate equations to zero
modulo `(E)`. It also reduced `zk+(1+z)c²` to zero for
`k=b+(1+c)c`. The corrupted `b+1` failed its coordinate check.

The retained R50 diamond formula is
`lambda1=(lambda+k/c)/(c³/k)`. On the same open, the proposed
expression is

`lambda1=[lambda+(1+z)²g]/(z²g)`.

The complete cross-product between these two rational expressions
reduced to zero modulo `(E)`; adding 1 to the proposed numerator
gave a nonzero remainder. The run exited 0, printed its exact
marker, emitted zero Singular `?` errors, and verified the six
committed source bytes and four donor hashes. The two source
coordinates, `k` identity, affine cross-product, and two corrupted
controls are the exact scope of this accepted calculation.

The accompanying Lean proof covers `k=(u+1)c²` from
`b=uc²+c` in any commutative ring of characteristic two, and the
resulting cleared lambda cross-product. These are algebraic
lemmas; the congruences modulo `E`, the coordinate map, and the
degree-16 relative equation are checked natively or remain open.
The actual field rational map also requires the registered
denominators to be units, as recorded by R49/R50. Next transport
the full degree-16 relative equation and then all 32 R50 RR basis
generators in both directions, with complete module lifts.

Sources checked: R49 `COMPACT-NORMAL-PROTOCOL.md`, R50
`NEXT_SOURCE_NORMALIZATION.md`, and the accepted R85 `CM.md`
and `results.json`. This round makes no novelty claim about a
modular transfer.
