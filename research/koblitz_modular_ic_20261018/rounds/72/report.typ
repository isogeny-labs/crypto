#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R72: rational-branch ramification in the normal affine order
The complete 528-product table gives the entire 32-dimensional fiber algebra at each omitted rational prime. All nonzero Frobenius-fixed idempotents are enumerated: three above $z=0$ and 15 above $z=1$. The primitive ones are selected by the complete Boolean order test, checked orthogonal, and checked to sum to the source unit.

#image("ramification72.svg", width: 100%)

At $z=0$, the two residue-degree-two branches have local lengths 28 and 4, hence ramification indices 14 and 2. At $z=1$, four residue-degree-two branches have local lengths 2, 14, 2, 14, hence indices 1, 7, 1, 7. Each local-length partition sums to 32. The passage from local fiber length to $e f$ uses R70 affine normality.

R65's discriminant exponent at $z+1$ is 24. The two index-seven branches are tame in characteristic two, and their different exponents contribute $2(7-1)+2(7-1)=24$, a separate consistency check. The $z=0$ exponent 76 includes wild excess beyond the $e-1$ lower bound.

The sole native attempt passed exit 0, exact marker, and zero-error gates. Six historical source pins and eight donor hashes match. Full square, stable-image, fixed-basis and idempotent matrices are retained. Uniformizers, coordinate and denominator valuations, the degree-five fiber and infinity remain the next geometric calculations.
