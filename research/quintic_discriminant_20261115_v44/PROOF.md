# GCD-free squarefreeness and adaptive support for binary quintics

Let H=X^5+aX^4+bX^3+cX^2+dX+e over any field of characteristic two. Its derivative is P=X^4+bX^2+d. Division gives H mod P=C X^2+E, C=ab+c, E=ad+e. Define Delta=E^2+bEC+dC^2.

If C is nonzero, a common root satisfies x^2=E/C. Substitution into P is equivalent to Delta=0; every algebraic closure contains such x. If C=0 and E is nonzero, H and P have no common root and Delta=E^2 is nonzero. If C=E=0, P divides H, so H is repeated and Delta=0. Therefore H is squarefree exactly when Delta is nonzero, with no inversion, square root or generic GCD required. This is a specialized resultant identity, not a cleared novelty claim.

For squarefree H, support in a finite binary subspace V is equivalent to L_V mod H=0. For repeated quintics, (L_V mod H)^8=0 mod H retains every root multiplicity. Computing Delta chooses between those exact predicates for any binary field and any target. This removes the target-specific certificate requirement for the fast branch; repeated-root cases remain complete. Complexity and storage match the existing cached remainder kernel plus a constant number of field operations. Five field multiplications and two field squarings compute Delta as written (including the two products defining C,E).

Unlike the preceding target-specific variant, this guard does not reject repeated polynomials: it routes them to the generic fallback. The field must actually have characteristic two; prime odd fields and odd extensions need a separately derived predicate. The mathematical identity covers binary extensions of every degree, but measured performance coverage is only the declared diagnostic panel.
