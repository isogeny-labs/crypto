#set page(paper:"a4",margin:(x:16mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
= Koblitz cover transfers — R29
*The double image, exact base cancellations and a full-level cubic bridge* \
2026-10-28 client date / 2026-10-29 UTC publication.

At degree 12 both first adjugates vanish. R29 computes epsilon² coefficients
of N(F+epsilon H), epsilon³=0, for H=1, x and 1+x. All original norms match
M15 and all first derivatives match R28. Every one of 1,024 coefficients of
all three second-order functions is recovered and checked.

With nonzero unweighted coefficient C, the weighted ratios recover the pair's
product and sum. At the registered F2^12 support the image polynomial is
X² + beta¹¹ + beta⁸ + beta⁵ + beta³ + beta. It is a square, selecting exactly
one actual curve point, of degree 12 and multiplicity two. This completes
the ordinary paired-image data of degrees 460 and 367; it does not identify
every original source branch or its colored scalar-C5 lift.

#image("round29.svg",width:100%)

The complete Frobenius/deck checks use actual image fields, ordinary cusp/SS
avoidance, a corrupted-ordinate detector, exact rational 2-torsion translation
identities and all five coefficient-descent curve/product conjugates.

#pagebreak()
= What cancels, what survives, and the independent model
#table(columns:(1fr,1fr,1fr,1fr,1fr,1fr),inset:4pt,stroke:.4pt+gray,
 [*Degree*],[*Mult.*],[*Frob.*],[*Neg.*],[*T*],[*Neg.+T*],
 [436],[1],[218],[-1],[-1],[-1],
 [12],[2],[6],[-1],[-1],[-1],
 [284],[1],[-1],[-1],[-1],[-1],
 [83],[1],[-1],[-1],[-1],[-1])

Here −1 means no match in the complete Frobenius orbit. The ordinary degree-460
base difference cancels exactly as closed divisors on E49. The asymmetric
367 supports do not coincide under these tested base actions. Colored C5
deck offsets and modular-factor columns are still required. Neither a zero
nor a positive IC relation rank follows from this table.

CUBIC29 separately uses the actual degree-14 lambda-zero point. The QR line
cuts an irreducible, separable cubic. Exact inverse/lift certificates and
Tate group laws produce an actual order-77 point over F2^42, with CRT
projectors 56/22 and diamond action 23. Monic/DVR specialization then proves
a generic degree-three extension. MODULI_BRIDGE.md states the written proper
identification with X1(77) and its quotient; native genus arithmetic alone is
not that certificate.

The independent #link("https://math.mit.edu/~drew/X1/X1opt77new.txt")[Sutherland/van Hoeij model]
is retrieved and archived with its hash. Its global rational-function match
to the incidence coordinates has not been checked. A shared level, genus or
finite point is explicitly not substituted for that identity.

All four native runs and the bounded compiler pass. Lean compiles and audits
two universal pair-sum/polynomial theorems, zero failures and zero trusted
native. Across R27–R29: 40 named theorems, 41 current audited declarations
including one generated congruence theorem. The retained-record checker
verifies source hashes and 21 accepted native runs among 32 attempts. This
is record integrity, not an independent mathematical replay.

The next construction is the colored lift and complete signed divisor,
followed by actual modular-factor evaluation. Full old coupled saturation,
explicit determinant-19 theta sections and the capped map solve remain open.
