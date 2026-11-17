#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R42: exact larger CM-block recovery
Observations from three native computations and four accepted Lean workflows.
No independent evidence verdict, timing comparison or IC speed claim.

#image("round42.svg", width: 100%)

== Complete finite calculations
The six fixed cases retain the actual target moduli. The positive n11, n83,
n131 cases admit explicit denominator inverses of degrees 9, 81, 129.
The n83 and n131 target moduli have 82 and 130 bits. Large primality remains
a retained input assumption; integer unit identities need no such assumption.
The three controls retain global nonunit obstructions.

For the actual CM exponents, set u=T^i, v=T^j and
D=2(v−u)(2u²+3uv+2v²). The opposite quadratic annihilator factors are
preserved over the integers, for both twist signs. Complete coefficient
certificates give Dred·A=1+Phi·Q+r·H, hence conditional recovery on the
stated primitive-character CM block.

== Exact integral denominator obstruction
#table(columns: (1fr, 1fr, 3fr), inset: 5pt,
  [n], [Matrix dimension], [Least scalar d for this denominator map],
  [5], [4], [22],
  [11], [10], [506],
  [83], [82], [12302938186694],
  [131], [130], [10095126166020618049226])

All original matrix columns, exact Bareiss divisions, adjugate solves and
primitive coordinate witnesses are checked natively. A second norm formula
checks the determinants. Each measured least scalar matches the prospective
Lucas-sequence formula at its fixed n; a universal theorem remains open.
The three positive target cases have explicit unit certificates for d.

#pagebreak()
= What is proved, and what is still missing
The integer matrix M is nonsingular and satisfies M·A=d·e0 with primitive
coordinates (d,A). Any other integral solve M·z=h·e0 has d·z=h·A.
A weighted primitive identity then implies d divides h. This proves minimality
for this fixed denominator multiplication lattice. It says nothing about
minimality among all geometric maps or all quadratic Bezout choices.

== Lean evidence
54 audited declarations across four accepted workflows: 31 base universal and
literal unit facts; 15 actual larger-case polynomial identities and recovery
corollaries; one generic primitive divisibility bridge; seven actual primitive
and target-unit identities. Zero failing or trusted declarations. No native evaluation.
 Pinned Lean 4.34.1 and mathlib d13f23b; one job, 4096 MiB heap,
1200-second compile/audit caps. The concrete matrix determinant is native
rather than formalized. Independent semantic review is pending.

== Retained failures and custody
One large-polynomial attempt stopped before compilation while waiting for the
base audit receipt. One least-scalar compile rejected a missing finite-sum
import; its repair used a fresh attempt. Both are retained. No accepted
producer reran and no cap increased. All nine attempt directories are indexed
in results.json. Raw artifact byte verification is separate from mathematical
replay. Native build/run caps remain 4 GiB virtual memory and 1200 seconds.

== Next construction work
Prove the universal denominator formula with exact exceptional hypotheses.
Compose the compact integral inverse with the opposite-CM annihilator factors,
then bind those operators to actual proper source and target maps.
R41 already evaluates two nonzero source characters on an independent
collector class; R42 adds larger arithmetic certificates.

The proper level77 map, chosen Koblitz model isogeny, full quotient saturation,
polarized embedding, node187 coupled divisors, precision615 source jets,
complete prescribed spaces and degree460/367 divisors remain separate open
obligations. There is no evaluated larger-n point transfer, IC relation rank,
unknown-scalar recovery or speedup here.

The retained R15/R18/R39/R41 notes informed the method. The primary
Molin–Page endpoint timed out; no new modularity theorem is asserted.
Sources, proof maps, all logs, and full coefficients are linked by REPORT.md,
results.json and the immutable artifact manifest.
