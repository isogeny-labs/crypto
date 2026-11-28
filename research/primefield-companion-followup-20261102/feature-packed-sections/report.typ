#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Packed univariate mode sections
T-1930 · Prime fields · 2026-11-08

The optional adapter replaces explicit convolution over section pairs with a single polynomial quotient power and balanced decoding. Runtime inputs remain public points.

#image("flow.svg",width:100%)

*Derived encoding.* For monic g of degree d, let P(V)=V^d g(V+V⁻¹). Since P has constant coefficient1, V is a unit. The quadratic ring defined by g(W)=0 and V²−WV+1=0 is isomorphic to the quotient by P. Adding X^m=sV gives M(X)=s^(2d) P(s⁻¹X^m). Compute (X−1)^h modulo M, split by exponent residues modulo m, then evaluate each section at sV. DERIVATION.md supplies both inverse maps and decoding uniqueness.

*Multiplicity control.* Retain the raw lift: g=W−2 gives M=(X^m−s)². Removing the repeated factor changes the quotient. The r17,m3,s2 control checks X⁶+13X³+4 exactly; nonsquarefree g is also included.

*Recorded qualification.* Validation02 passes420 kernel cases and1,764 section equalities,23,552 direct-sign state comparisons and3,600 public cases including228 odd-L cases. All3,372 predecessor comparisons match scalar outcomes, routes and five source-statistic fields. All298 single-call guards pass. Deterministic fixtures, unsigned producer evidence; independent signed review pending.

*Custody.* Validation01 kernel and validation02 integration both pass. The earlier driver is snapshotted. Final build uses overflow checks;151 copied sources and168 compiler dependencies are hash-bound. Inherited output matches its reference byte for byte. Build/focused/inherited bounds90/60/60 seconds. No cloud submissions or timing measurements.

*Cost scope.* The modulus degree is2md, and binary powering uses logarithmically many modular products in h. Balanced decoding shares powers across sections. Lift setup, reductions, cache copies, allocations and outputs remain charged. The output cap is not peak RAM. No timing crossover is established.

*Next construction.* Initial phase-domain expansion remains. Apply the oriented relative constraints before dense expansion, with explicit construction costs and completeness.

*Sources.* Predecessor powered/quadratic/homogeneous/oriented modules at754f7d221f02; NIST DLMF18.5.1-2 for reciprocal coordinates. KB unavailable (no MCP or executable). No novelty claim.
