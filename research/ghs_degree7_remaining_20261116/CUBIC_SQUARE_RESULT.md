# Cubic-square degree-seven screen: fixed trace +22 carrier

The source at commit `0c362e2ba465` completed both target-orbit representatives
on the fixed genus-five carrier in [PROTOCOL.md](PROTOCOL.md). The final
receipts are [b=68](cubic_square_68_v3.json) and
[b=84](cubic_square_84_v3.json). Both bind to source SHA256
`341780cf619974cb5c160a1f59e9f67cd2018c545e1a79cc2aca203a4e7207be`.

| target mask | systems | zero-dimensional | Frobenius-reduced | constant assignment | parameter solutions | degree-seven, coprime numerators | exact identities | CPU s |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 68 | 243 | 232 | 10 | 1 | 2,785 | 355 | 0 | 7.636379 |
| 84 | 243 | 232 | 10 | 1 | 2,774 | 340 | 0 | 7.837593 |

Both receipts have zero censored systems. The ten exceptional ideals in each
run were solved by the checked `t³+c w³+d` relation and exact enumeration of
all `w∈F243`; the constant branch assignment cannot produce a numerator of
degree seven. The 695 retained degree-seven coprime candidates passed the
necessary divisibility `f | B(A³+A²B+bB³)` but failed its square-polynomial
condition. The result excludes only `B=H²` for monic cubic `H` on this exact
carrier and these two target models. Frobenius conjugation transports the
shape-restricted result to the other eight source models in their two orbits.

The search code is one algebraic implementation; no independent reimplementation
of the elimination has yet been run. The result does not rule out degree-seven
covers through the two remaining denominator classes, different carriers,
or correspondences that are not curve maps. It supplies neither an evaluable
transfer for the trace-+22 class nor an attack-cost measurement. This is a
finite exclusion, not a new vulnerable family. [pcc:unverified]
