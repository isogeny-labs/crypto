# R45 coefficient and point encoding
Coordinates are unsigned binary polynomial coefficients in increasing bit order.
Degree7: x^7+x+1 (0x83);9:x^9+x^4+1 (0x211);
11:x^11+x^2+1 (0x805);13:x^13+x^4+x^3+x+1 (0x201b).
Field construction checks every product against independent polynomial
convolution/reduction and verifies all nonzero inverses/no zero divisors.

Each closed-points.txt row: deck-orbit index (1-based), deck-step index
(0-based), b,c,lambda,nu. Each tuple is the lexicographically least Frobenius
conjugate. The physical coefficients are lambda+(1+c)*gamma and nu+b*gamma,
gamma^2+gamma+1=0; they are not obtained by dropping the gamma terms.
All source equations and units are evaluated in that physical quadratic field.
The F2descent tuple residue degree is explicitly measured by squaring all
four coordinates. Sources and runner hashes are recorded per attempt; early
7/9/11runs preserve the pre-amendment source bytes. Source-polynomial exponent
order is physical(nu,lambda,b,c), as in R39. No native output is a Picard image.
