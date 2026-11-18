#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R73: a degree-70 branch in the normal level-77 source
The complete $f_C=z^5+z^3+z^2+z+1$ fiber is rank 32 over $bb(F)_32$ (rank 160 over $bb(F)_2$). All 32 source square columns define absolute Frobenius; five semilinear iterations give the full $bb(F)_32$-linear relative Frobenius. Scalar linearity, commutation, source unit and stable-image invariance all pass.

#image("fiber73.svg", width: 100%)

The reduced image has dimension 20 over $bb(F)_32$. Fixed dimensions of all 32 powers and integer gcd-degree inversion give four branches of relative degree one, one of degree two and one of degree 14. The last has absolute $bb(F)_2$ residue degree 70. Every fixed dimension is reconstructed from the saved profile.

R65 gives discriminant exponent 12 at $f_C$. The fundamental equality and relative residue sum give $sum f_i(e_i-1)=32-20=12$. Dedekind's different bound $d_i >= e_i-1$, with equality exactly for tame branches, forces all six branches tame. In characteristic two the degree-14 branch must have index one. The Lean theorem checks that last weighted-budget arithmetic under explicit premises; three authored declarations compile and audit with zero failing and zero trusted.

Five terminal attempts are preserved: the first native run had source-ring errors despite a late marker; the repaired second run passes exit, marker and zero-error gates. Two Lean import attempts failed compilation before a final pinned compile/audit passed. The next experiment splits all 63 nonzero absolute idempotents to determine the other local lengths and ramification indices. Uniformizers, valuations and infinity remain separate.

The different inequality is stated in #link("https://research.sabanciuniv.edu/id/eprint/577/1/3011800001170.pdf")[Garcia–Stichtenoth, Section 2].
