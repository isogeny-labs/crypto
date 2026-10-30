# Direct Hess genus-four control for order 4126

27 October 2026. Conductor T-5516. Literature checked first:
Hess (2004), Theorems 11--14 and Section 6, already describes this
generalized Artin--Schreier construction and subgroup transport.
Menezes--Teske (2004) analyze the cubic field case. Magma's current
WeilDescent documentation specifies the returned curve and divisor map.
This is a known construction and cannot establish a new attack method.

For the exact residual source
`E: y^2+xy=x^3+K.from_integer(32)*x^2+K.from_integer(63)`
over `K=GF(4096)`, with `k=GF(16)` and fixed order-2063 pair
`G=(62,1499)`, `Q=(2633,1798)=[7]G`, choose descent parameter
`c=1`. The Hess factor orders are `(1,3)` and the genus formula gives
`8-4-1+1=4`. Require Magma to confirm the genus and produce a
nonzero image of `G` whose 2063 multiple is zero and whose target
image equals seven times the generator image. Retain raw input,
transport receipt, and separate timings. If the divisor-map output is
directly a hyperelliptic Jacobian point, verify the group relation
there. Do not describe this as a genus-three residual map.

Only after the map checks pass, attempt a full target-dependent
index-calculus solve and a same-source rho baseline, charging cover
construction, map evaluation, relation collection, linear algebra,
target recovery, and source verification. If the available solver does
not execute those stages, record the precise stopping stage and leave
the full cost unknown. No vulnerability claim follows from map
existence alone.
