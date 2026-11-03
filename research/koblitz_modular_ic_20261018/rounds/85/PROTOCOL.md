# R85 diamond base-overlap products, frozen before data

On the accepted R50 inverse-base quadratic chart use
`E=V²+z²V+z+z³`, `h=z+V`, `H=1+z+z²+zV`, and
`N=V+zV+z²`. The proposed forward transition has
`z1=1/h` and `V1=N/h²`; its inverse-base open is
`h1=z1+V1+z1²`. Check full polynomial identities
`h²+zH=E` and `N+h+1=H` in characteristic two. Clear
all denominators from `E(z1,V1)` by multiplying by `h⁶`,
then check exact ideal membership in `(E)` with a full lift
witness. A corrupted numerator `N+z` must fail membership.
These products imply `h1=1/z` on the `E=0`, `z*h!=0`
overlap and the base-coordinate round trip. They do not yet
transport the 32 upper-field generators.

Run one Singular job, seed 85, 1200 seconds, 4 GiB virtual,
one Singular/FLINT thread. Copy committed input/protocol/driver
bytes and verify donors. Accept only exit 0, exact marker,
zero native `?` errors, full product/lift identities and
corruption rejection. Retain failures and freeze repairs.
Separately compile the pinned Lean statements for the two
principal-open polynomial identities and numerator identity,
then audit axioms. No timing comparison.
