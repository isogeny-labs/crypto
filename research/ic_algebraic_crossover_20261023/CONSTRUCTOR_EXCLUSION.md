# Why the c3=0 quartic family can be skipped for even m

This is an algebraic exclusion for constructing the coefficient field,
not an ECDLP obstruction or a measured speed result. Let q=2^m with m even
and consider f(X)=X^4+c1*X+c0 over Fq.

If c1=0, perfection of a finite field gives f=(X+d)^4 for some d in Fq,
so f is reducible. This case is excluded for every m.

Suppose c1 is nonzero and f were irreducible of degree four. All four roots
would lie in F(q^4). Differences of distinct roots are the nonzero roots
of X^4+c1*X, hence satisfy u^3=c1. Because q is 1 modulo 3, Fq contains
the cube roots of unity. If c1 is not a cube in Fq, X^3-c1 is irreducible
of degree three. Such a u cannot lie in F(q^4), since its degree would have
to divide both three and four. Thus c1 must be a cube.

Then the entire four-element additive kernel U of X^4+c1*X lies in Fq.
For a root alpha of f, alpha^q=alpha+u for some u in U. Consequently
alpha^(q^2)=alpha+u+u=alpha. Its degree is at most two, contradicting
irreducibility of degree four. Therefore no c3=0 candidate is irreducible
when m is even.

The inherited search orders candidates by (c3,c1,c0). Excluding this whole
family for even m, and excluding c3=c1=0 for every m, preserves the first
irreducible polynomial it would return. The remaining rank/unit test is
unchanged: a quartic is irreducible exactly when it has no degree-one or
degree-two factor, checked by invertibility of z^(q^2)-z modulo it.

Before using the exclusion in the wide core, retain the constructor-only
Callgrind baseline and execute exhaustive native checks at m=4,6, plus
old-backend field/model parity on the declared supported control rungs.
The proof is a mathematical argument, not a formal proof-assistant
certificate; controls establish their finite scope. Instruction attribution
and full-pipeline/native-price measurements remain separate obligations.
