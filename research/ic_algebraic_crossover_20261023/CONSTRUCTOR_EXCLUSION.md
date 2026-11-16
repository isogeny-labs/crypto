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

## Checked algebraic core

The dependency-free [Lean proof map](lean/README.md) checks the root-difference
identity and a stronger conditional algebraic core. For an additive/multiplicative
f with f^4=id fixing c1 and all cube roots of unity, if alpha and f(alpha)
are roots then f^2(alpha)=alpha. Write u=f(alpha)+alpha. If u!=0, u^3=c1,
and k=f(u)/u has k^3=1. Since f fixes k, f^3(u)=u. Together with f^4(u)=u,
this fixes u; the characteristic-two translation then has order at most two.
The u=0 case is included. `quartic_root_two_cycle` is kernel checked with
only the repository's allowed foundational axioms. The specialization to
q-Frobenius, its cube-root property for even m, and the irreducibility bridge
remain readable finite-field facts rather than checked Lean dependencies.
The accompanying identity certificate certifies only the integer polynomial
expansion, with a faithful Lean proof pointer; it does not certify that bridge.
