#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Seed initial candidates with the public character
T-1908 · Optional Rust adapter · 2026-11-08

The recovery adapter already obtains the scalar character from its first public-source query. Initialization now uses that value to restrict phase branches before their polynomials are constructed. Identity handling and final point verification remain intact.

#image("flow.svg",width:100%)

*Identity.* Let h=(r−1)/2, 4m divide r−1, n=(r−1)/(4m), L=2n, and V=(k/b)^m. A phase V^n=u implies u²=χ(k)χ(b). Thus two of the four phases are eligible. In Z=k^m coordinates, seeded support is gcd(G_old, Z^L−s). Nonzero s also excludes zero. The adapter receives points, not k.

*Qualification.* 15,264 exact support comparisons pass, including 7,560 true-image retentions, 7,560 wrong-sign exclusions and 144 zero exclusions. All 2,529 public target/capacity cases recover the correct scalar on the order281 fixture; 280 additional one-call caps hold. The trace differs in 180 routes and 189 call totals, as earlier pruning can change termination. These are correctness controls, not timing results. PCC-20261108-1bc6fc awaits independent signed review.

*Costs.* Initial phase-polynomial attempts drop from four to two. Normalization and scalar comparisons are added; this version repeats b^h for normalization. Initial mode0 construction, public sign extraction, quotient work, allocations, inversions, lifting/replay and verification remain charged. No performance-selection policy is changed.

*Custody.* 117 copied sources and 131 compiler dependencies are hash-bound. Inherited stdout is byte-identical. One metadata path-resolution failure is preserved and corrected; native compilation and checks passed. Full runtime formalization and independent review remain open. No cloud or timing panel ran.

*Sources and next step.* Existing powered-image and signed-reciprocal modules; Ben Lynn, Quadratic Residues (Euler criterion and multiplicativity). KB unavailable; exact-source fallback recorded. See DERIVATION.md and validation01/receipt.json. Initial mode0 construction remains the next representation target.
