# R87 degree-16 upper relative equation under the diamond map, frozen before data

R50's accepted `relative-infinity50.sing` gives the monic source
equations `E(V,z)=0` and `P(T,V,z)=0`, where `T` is original
lambda. R85 supplies `z1=1/h`, `V1=N/h²`, with `h=z+V` and
`N=V+zV+z²`. R86 supplies `T1=A/D`, with
`A=T+(1+z)²g`, `D=z²g`, `g=1+z+z²+V`.

For every monomial `T^a V^b z^c` of `P`, construct its exact
contribution after substitution and multiplication by `h^64 D^16`:
`h^(64-c-2b) N^b D^(16-a) A^a`. Check all exponents are
nonnegative, reduce the complete sum modulo the original `(E,P)`,
and require a full polynomial lift from that ideal. A corrupted
update `T1+1` must give a nonzero remainder. Since the only
positive `T` exponents of `P` are 1,4,8,16, the corrupted
difference is computed exactly as the sum of the same monomial
terms with `A^a` replaced by `D^a`.

Acceptance is one native Singular attempt with source-byte/donor
hash checks, exit zero, exact marker, no native `?` errors, zero
positive remainder, full lift equality and a nonzero corruption
remainder. Seed 87, one thread, 4 GiB virtual memory, 1200 seconds,
one attempt; retain any failure and freeze a repair separately. This
tests upper-field equation preservation on the registered rational
open. It does not prove normalized chart regularity at exceptional
branches or transport the rank-32 RR basis.

Sources: R50 `RELATIVE-PROTOCOL.md` and accepted exported
`source-data/relative-infinity50.sing`; R85 `CM.md`; R86 `CM.md`.
No novelty or timing comparison is asserted.
