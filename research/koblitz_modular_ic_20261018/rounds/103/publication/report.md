# R103: direct derivative norm of the actual degree-112 source

The actual G51 has lambda exponents 16, 8, 4, 1, 0, hence in F2 its
lambda derivative is the l-linear coefficient C(x,y). Since G51 is monic
degree 16, its lambda resultant against C is C^16. R103 directly
computed `Res_y(f51,C)=P123/x^163`. The numerator P123 is exactly the
R100 degree-123 Frobenius root and the sixteenth power reproduces the
R99 full norm `P123^16/x^2608`.

The frozen source-bound Singular run exited zero without native errors,
checked a changed-norm control, and passed fresh replay. An independent
GF(2) checker matched all three source hashes, numerator exponents and
denominators. Lean 4.34.1 checked the exponent and degree arithmetic.

This is an exact shortcut for the displayed monogenic-order discriminant;
normalized local branch valuations remain to be computed. Raw archive
SHA-256:
`1136fb2502eae1d22a4d5b9604e01bd2163d9438725ec7e6d19891c3c1b49e4d`.
