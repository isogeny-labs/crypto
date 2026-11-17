# Explicit diagonal covers and factor section bases

G16 uses the four actual determinant19 Hermitian matrices from G15, with
w²+w+2=0 and conjugation w↦-1-w. Exact rational Gram–Schmidt
runs through all24 coordinate permutations, clears denominators and makes
each column primitive. Selection prefers an odd degree, then a unit at11,
then the smallest norm inside that finite24-element search. It does not
claim a global optimum over all integral covers.

For each selected integral matrix B the native checker verifies
B†HB=D, det(H)=19 and Norm(det B)=product(D)/19. Generated Lean proves the
actual8x8 integral Riemann-form congruence and section-count identities.
For graphs0/1 use the trace-1 model E0 with w=Frob; for graphs2/3
use the trace+1 model E1 with w=-Frob. B defines a self-isogeny of
the corresponding split elliptic fourth power over F32, with the source
rotation coordinates fixed by G15. Its F2 descent is not constructed.

| graphs | diagonal D | cover degree | differential invertible mod2 |
|---|---|---:|---|
|0,2|3,21,105,285|99225|yes|
|1,3|4,8,112,266|50176|no|

The companions have degree19. Pulling back3Theta yields factor degrees
[9,63,315,855,57] on graphs0/2 and[12,24,336,798,57] on graphs1/3.
The actual Pic2 corrections pulled through the dual CM action give biases
[1,1,1,1] and[0,0,0,0] respectively. For alpha=a+b*w, the dual action on
the unique nonzero geometric2-torsion point T is multiplication by a mod2;
the single-column symmetric theta bias is a*b mod2. The JSON retains
these calculations, the full B and all factor bases.

For O(dO), the explicit basis consists of1, x^i for2i≤d, and x^i*y for
2i+3≤d. At the nonzero2-torsion point T=(0,1) on either Koblitz model,
(y+1)/x has a simple pole at O and T. Thus a basis of
O((d-1)O+T) is a basis of L((d-1)O) plus(y+1)/x. Dimensions are d by
Riemann–Roch. These function/divisor identifications remain written
elliptic geometry, not a full Lean formalization.

The tensor bases have8704314675 or4401589248 elements. They are exported
as exact factor lists plus a lossless Cartesian-product descriptor; billions
of products were not materialized. Each dimension equals87723 times the
cover degree, as required for the pulled-back line bundle.

These are bases on the diagonal finite cover with the stated divisor class.
They still require a rational trivialization against the original mixed
pullback divisor, explicit theta-group kernel actions and descent matrices.
The rational243-dimensional target section space, its evaluation maps and
the saturated proper equations remain unconstructed. Graphs0/2 provide an
odd, separable, unit-at11 cover for this next descent step. Graphs1/3 require
scheme-kernel descent because the differential is singular; geometric
kernel points alone cannot supply their descent.
