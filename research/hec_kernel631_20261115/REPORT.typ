#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Actual primary kernels over F631
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

The unique arithmetic-compatible source from the frozen joint cohort now has two explicitly constructed Frobenius-primary 3-torsion groups. Input p631, constant49, N237185924, ell59296481 and its recovered Weil polynomial. No old kernel, normalization or point table is loaded.

== Exact construction
The first irreducible field modulus is t¹³+t+10. Exact irreducibility and32seeded field-law, inverse and Frobenius controls pass. The extension Jacobian order is computed from the actual L by Newton identities and companion-matrix determinant, with reciprocal and divisibility checks. Its3-adic valuation is6.

After non-3-cofactor projection and reduction to order3, fresh Bezout primary projectors give two groups of27valid points. Construction succeeds at attempt index1, depth0. The inherited code does not record every unsuccessful branch's reason; complete branch diagnostics are not claimed.

#table(columns:2,inset:6pt,[Check],[Result],
 [Group size],[27 each],
 [Pairwise closure],[729 checks each],
 [Order and Frobenius],[All points pass],
 [Intersection],[Only zero],
 [Cross-group sums],[729 distinct valid order3 points],
 [Corrupted divisor],[Rejected])

== Isotropy argument and boundary
The irreducible coprime primaries T³+T²+2 and T³+2T+2 are reciprocal and satisfy T¹³=1. Frobenius-invariant pairings pair a primary only with its reciprocal; hence each three-dimensional primary is isotropic. This is a standard mathematical argument, not an executed Weil-pairing table or formal geometry proof. No pairing values were computed.

== Custody and next step
Sourcee40de6a2b412/checkpoint012553Z preceded the once-only run. Status0, stderr empty; run/KERNELS.txt contains the complete groups and field modulus. An earlier String/char serialization build error is retained separately; a successful build/hash freeze preceded execution.

This is actual larger-field kernel data, not only admission. It does not construct normalization, quotient geometry, transfer maps or a DLP solve. The implementation is fixed to this F631input; no arbitrary-prime arithmetic claim. Next construction must use these groups to rebuild kernel functions, normalization and quotient geometry with independent identity checks. No F127images or logs may be supplied.

No timing, exponent, novelty or tenfold gain is claimed. Construction and verification remain chargeable. Genus two/four and the full mathematical objective remain open; the prior failed cold-cost result is unchanged.
