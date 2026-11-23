#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Principal functions and normalization over F631
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

Fresh principal functions and normalization coefficients are constructed from both actual F631kernels. No F127function, alpha, image or logarithm table is supplied. Normalization and subsequent local-chart admission were separately frozen and executed once; both statuses0 and stderr empty.

== Function and field-port checks
All52nonzero kernel functions pass exact norm, divisibility and Hensel identities; all52constant-coefficient corruptions reject. The old F127inverse-of-two value64fails all52tested curve lifts over F631. The correct value316passes. Eight fixed triples give432independent split product/determinant-versus-norm evaluations, all equal and no exclusions.

== Root-free normalization
Four regular source multiples1..4give identical fresh normalization across samples, without root extraction. RUN.txt contains all54weights. Exact comparison counts:
#table(columns:2,inset:5pt,[Identity],[Passing comparisons],
 [Cocycle constancy],[4374],
 [Cocycle symmetry],[1458],
 [Associativity],[39366],
 [Normality],[5832],
 [Symmetry values],[216],
 [Frobenius scalars],[54],
 [Theta translation actions],[216])

All sampled theta values are nonzero and descend to the base field. Corrupted weights cause152normality and208action failures. These are finite exact checks, not a blanket implementation or geometric proof.

== Local quotient-chart premise
The first16ell-projected source places generate all four points of the rational2-primary group. Its unique nonzero2-torsion divisor has u=[436,451,7,1],v=0, admitting the degree-three local chart. The first projection has order4; geometry must double it once before applying order2formulas.

== Custody and scope
Normalization sourcef832dd0203f4/checkpoint013358Z; local admission8b627c787346/checkpoint013724Z. No scientific rerun or dropped input. RootRatio.lean checks only the conditional scalar identity: compatible square/cube values a2,a3with a2 nonzero yield a=a3/a2. It does not formalize curve/cocycle geometry or code.

No quotient map, DLP, timing, exponent, novelty or tenfold gain is established by this gate. Construction and controls remain chargeable. Genus two/four and the full objective remain open; the earlier failed charged comparison is unchanged.
