# R86 actual base-coordinate and affine-lambda diamond map, frozen before data

Use the accepted source-base identities from R49:
`u=(b+c)/c²` and `v=(bu³+c)/((u+1)c)` on their registered
open. Solving gives `c=[v(u+1)+u³+1]/u⁴`, `b=uc²+c`.
On R85's inverse chart `z=1/u`, `V=z²v`, define
`g=1+z+z²+V`, `c=z(1+z)g`, and the proposed reduced
polynomial `b=z⁷+z⁶+z⁴+z²+(z⁵+z³+z²+z)V`.
Check the two complete denominator-cleared original-coordinate
identities modulo `E=V²+z²V+z+z³` and reject a corrupted `b`.

For R27's diamond parameters `k=b+(1+c)c`, `s=c³/k`,
`ell=k/c`, check `z k=(1+z)c²` modulo `E`. The proposed
affine upper-field update is

`lambda1=[lambda+(1+z)²g]/(z²g)`.

Cross-multiply this with `(lambda+ell)/s` as a full
polynomial identity modulo `E`, and reject a corrupted
numerator. Passing establishes the rational affine update
on the registered field open. It does not yet check the
degree-16 relative equation under the update or any of the
32 RR generator regularity lifts.

Run one Singular job, seed 86, 1200 seconds, 4 GiB virtual,
one Singular/FLINT thread. Copy committed source/protocol/driver
bytes and verify donors. Accept only exit 0, exact marker,
zero native `?` errors and all asserted identities/controls.
Retain failures and freeze repairs. No timing comparison.
