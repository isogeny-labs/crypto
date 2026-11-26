# Conditional argument map

All claims below retain their explicit caller assumptions in RESULTS.md.
Native witnesses are producer-unverified; no signed or kernel-checked proof
is asserted. Lean and lake are absent from this environment. The full
formalization remains open rather than silently substituting a weaker theorem.

| Argument | Native evidence | Formal status |
| --- | --- | --- |
| Fixed candidate overhead: spacing divides all retained offset gaps; gcd20 forces zero overhead | check.cpp spacing_gcd; complete constraints.tsv | Written integer argument; Lean unavailable |
| Post-retention monotone retry cost fails on239bit intervals | check.cpp monotone_witness; bounds0–134 and543–727 | Written ordering argument; Lean unavailable |
| Affine nonnegative retry cost fails on192bit intervals | check.cpp affine inequality witness | Written nonnegative linear inequality; Lean unavailable |
| Common initial burn fails under retained-only assumptions | complete startup intervals; max lower5736 exceeds min upper138 | Written interval intersection; Lean unavailable |

The identity-certificate tool certifies polynomial equality over a finite
field; it does not certify divisibility, ordering, integer inequalities,
candidate visitation or historical caller provenance. No IDC verdict is
used to promote these arguments. The model implications and historical
input bridges remain visible, not encoded as claimed axioms.

Raw computational and render failures are retained; the two numbers in a
gap are exact bounds, not uncertain measurements or historical timings.
