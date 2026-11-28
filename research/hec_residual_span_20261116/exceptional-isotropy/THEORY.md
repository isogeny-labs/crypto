# Exceptional-prime stable isotropy after the cyclotomic screen

[pcc:unverified] Conditional on cyclotomic-self-quotients/THEORY.md premises and canonical pairing. No quotient models or speed result.

For l!=p,7, the integral cyclotomic Tate module reduces to the regular O/l module. Frobenius pi=P(sigma) acts semisimply, even when repeated eigenvalues appear, because O/l is a product of fields. Its adjoint is p/pi. Therefore eigenspaces with values a,b pair only if ab=p. An irreducible self-reciprocal simple block is nondegenerate and cannot be partly selected by a Frobenius-stable subspace.

At l=251,p113 and l=14489,p421, l=-1 mod7. O/l has3 quadratic fields, each preserved by conjugation. A full degree2 Frobenius factor acts irreducibly on its two-dimensional component, which is nondegenerate. If the exact polynomial has one repeated linear factor and two other distinct irreducible quadratics, any stable isotropic subspace has dimension at most1, from the scalar two-dimensional component. Hence no maximal isotropic dimension3 exists. Degree-gcd alone misses this obstruction.

At l=659,p421, l=1 mod7, so O/l is split. If all6 Frobenius eigenvalues (with multiplicity) lie in F_l:
- a non-self-reciprocal pair of eigenspaces of equal dimension d is paired perfectly. A stable Lagrangian in their sum chooses any subspace U in the first, and its annihilator in the second. Count sum(k=0..d) GaussianBinomial(d,k;l).
- a self-reciprocal eigenspace (a²=p) is symplectic and has even dimension2m. Its Lagrangian count is product(i=1..m)(l^i+1).
Multiply independent reciprocal-block counts. O-stable choices number8 by selection of one in each conjugate pair of6 cyclotomic lines. Subtracting8 counts stable kernels not O-stable, not quotient isomorphism classes.

If additionally End_Fp(J)=O, a non-O-stable kernel cannot yield a rational principally polarized quotient isomorphic to J: composing quotient with such an isomorphism would give a rational endomorphism, whose kernel is O-stable. This equality of rational endomorphism rings follows here from Q[pi]=K (nonzero powerbasis index), Tate faithfulness/centralizer dimension, and the integral maximal order O already embedded in End_Fp(J). This distinguishes new rational polarized quotients from selfquotients, but does not show non-hyperellipticity, distinctness between quotients, low construction cost, or a faster attack. Geometric isomorphisms over extensions are a different question.

The gate computes eigenvalue/multiplicity data exactly before applying these formulas. It does not enumerate subspaces or build a Weil pairing in coordinates. Proof of the canonical pairing and independent validation remain dependencies.

Sources: prior complete exception arithmetic; Milne AV sections10,13,14 as linked/read in cyclotomic-self-quotients/THEORY.md; standard finite symplectic-space counting derived by the subspace/annihilator argument above. Own specialization, no novelty claim.
