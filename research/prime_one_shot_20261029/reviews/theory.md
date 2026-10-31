# Independent theory review of the growing-arity coefficient proposal

Reviewer: `/root/theory_review`, 2026-10-29. Reviewed the root report directly. No scientific solver run or paid compute was performed. This review concerns algebra and the conditional complexity argument, not empirical speed or certified novelty.

## Overall assessment

The norm/Riemann-Roch formulation is mathematically coherent on its stated chart. The odd-arity Graeffe identity is also correct as an equality of root multisets. The proposed system does not, by itself, provide a sub-exponential algorithm: both finite-field rational-solution selection and a uniform solving-cost bound remain substantial missing steps. There are two concrete corrections and one important scheme-level caveat below.

## 1. Correction to the quantitative conditional upper bound

The report assumes base construction cost exp(o(n)) and success probability at least exp(-o(n)), then displays a complete bound exp(O(sqrt(n) log n)). The stronger displayed bound does not follow. For example construction or inverse success probability exp(n^0.9) satisfies the stated assumptions but dominates exp(O(sqrt(n) log n)).

Two valid versions are:

- Keep the current assumptions and conclude only exp(o(n)) complete cost, subject to all rank and extraction assumptions.
- Require base construction, inverse total success probability, rank-producing relation overhead, rational-root extraction, and all other phases to cost at most exp(O(sqrt(n) log n)), then conclude that sharper bound.

This is a real implication error, not a disproof of sub-exponentiality under the weaker assumptions.

## 2. The norm chart and its converse

For odd m=2h-1, a nonzero function with pole divisor exactly (m+1)O=2h O has the form A(x)+C(x)y, with deg A=h and deg C<=h-2. Its highest pole-order term comes from A, so A can be made monic. The norm A^2-f C^2 is monic of degree 2h. The target condition A(r_x)-r_y C(r_x)=0 makes the function vanish at -R, as required.

If U0 is squarefree, U0(r_x)!=0, and gcd(C,U0)=1, norm equality gives one residual zero above each root alpha, with y=-A(alpha)/C(alpha). The sum of those residual points is R by the principal-divisor criterion. Conversely a decomposition with distinct finite abscissas, no x overlap with R, and sum R has a principal divisor sum(P_i)+(-R)-(m+1)O. If the points are Fp-rational, the defining function can be chosen over Fp; this can be seen from the Fp linear constraints on the Riemann-Roch space or descent up to scalar. Thus the selected-chart correspondence is sound when the coefficient field is explicit.

The chart excludes repeated abscissas, opposite pairs, residual target overlaps, infinity, and any lower-order pole degeneration. It should not be claimed to cover a nonvanishing fraction of successful uniform-target decompositions without proof or measurement. Heuristics about independent random tuples do not automatically become conditional-on-sum coverage estimates.

For a large-prime subgroup in a cofactor curve, all recovered points additionally need subgroup checks or a separately analyzed cofactor-handling construction. The report appropriately restricts its initial concrete construction to a whole prime-order curve group.

## 3. Concrete field-of-definition counterexample

Forcing the abscissas into Fp does not force all geometric solutions of the coefficient system to have Fp coefficients or rational y coordinates. This matters when an algebraic solver works over the algebraic closure rather than returning only Fp points.

An explicit example is E/F17: y^2=x^3+2x+2. Its rational abscissas are 0,3,5,6,7,9,10,13,16, each with two y values, so #E(F17)=19. Let z^2=3 in F17^2; 3 is nonsquare in F17. Take

    P=(5,1), Q=(6,3), R=P+Q=(10,6),
    T=(1,8z), 2T=(12,16z), -3T=(15,12z).

The three nonrational points sum to O. For example the tangent slope at T is 12/z, giving x(2T)=12 and y(2T)=16z; the chord from T to 2T has slope 10z and gives 3T=(15,5z), hence the displayed -3T. The rational chord P,Q has slope 2 and gives R=(10,6).

All five residual abscissas {1,12,15,5,6} are distinct, nonzero, and different from x(R)=10. Choose d=16, c=1. Then all are in mu_16. Their residual sum is R, so Riemann-Roch supplies a monic cubic A and linear C over F17^2 satisfying the norm identity, target equation, all four Graeffe layers, and the stated chart conditions.

But A,C cannot both be in F17. An F17 function vanishing at T must also vanish at Frobenius(T)=-T; the norm would then have a repeated factor X-1, contradicting the selected squarefree norm. Thus the bare quadratic system has valid geometric models that are not F17 decompositions.

This does not claim that this R has no rational decomposition; it exhibits a spurious geometric model, which is enough to invalidate any inference that rational x roots automatically solve the rationality problem. It also does not contradict the report's explicitly stated missing rational-solution-selection assumption. The implementation must distinguish these two semantics.

Concrete repair choices include:

- Enforce A,C coefficients w^p-w=0 with binary-exponentiation circuits. This is a polynomial-size quadratic presentation, O(m log p) extra variables, but its solving cost must be included. Merely adding these circuits does not imply a small solving degree.
- Enforce that all roots of U0 are usable curve abscissas, e.g. U0 divides gcd(X^d-c, f(X)^((p-1)/2)-1), with a shared quotient-ring power circuit. This excludes y=0 roots separately and adds real work and variables.
- Use a solver genuinely restricted to Fp assignments, and charge that restriction rather than replacing it with algebraic-closure Gröbner solutions.

The explicit F17 mixed rational/twist tuple should be a regression test if a prototype exports algebraic-closure systems.

## 4. Graeffe formula and coefficient counts

For monic odd-degree U(X)=E(X^2)+X O(X^2),

    T O(T)^2-E(T)^2 = product_i (T-alpha_i^2).

This follows from U(X)U(-X)=E(X^2)^2-X^2 O(X^2)^2 and the sign (-1)^m=-1. The sign in the root report is correct. Iterating and ending at (T-c)^m implies every alpha_i^d=c as a geometric-root statement. Since d|p-1 and c=g^d, X^d-c is separable and splits in Fp, so all alpha_i are indeed Fp. The previous section explains why this does not settle y rationality in geometric coefficient solutions.

There are m coefficient variables per monic layer, plus O(m) norm-function coefficients. The layer equations are quadratic, and O(mt+m) variables/equations before explicit rationality constraints is a valid asymptotic count. Each coefficient convolution has O(m) terms and a full layer O(m^2) terms. Keeping polynomial Bezout witnesses for squarefreeness and coprimality adds O(m) variables and quadratic coefficient identities. Nonvanishing of U0(r_x) is also one inverse-variable quadratic constraint. These counts are input-description sizes, not runtime bounds.

## 5. Important multiplicity caveat (also identified by novelty reviewer)

Terminal Graeffe equality fixes the elementary symmetric functions of the powered roots at a fully collided target (c,...,c). Even when the original alpha_i are distinct and U0 is squarefree, this produces a nonreduced membership scheme. Locally, in characteristic p>m, the power map on nonzero roots is etale; after an etale coordinate change the fiber is the coinvariant algebra

    K[epsilon_1,...,epsilon_m]/(e_1(epsilon),...,e_m(epsilon)).

It has length m! and associated graded top degree m(m-1)/2. Saturating by the discriminant of U0 does not remove this ramification, because the collision occurs at the powered roots, not at the original roots.

This does not by itself prove a global solving-degree lower bound for the extended circuit. Auxiliary variables change filtrations, and m!=exp(O(sqrt(n) log n)) is still sub-exponential at the proposed m. It is nevertheless a concrete reason not to equate quotienting point labels with eliminating their scheme multiplicity.

A required comparator is the radical divisor-membership condition U0 | X^d-c. At a squarefree divisor of separable X^d-c its geometric divisor scheme is reduced: in locally labelled root coordinates the membership derivatives are diagonal nonzero values f'(alpha_i), and the Vieta map is etale at distinct roots. Keeping quotient-ring squaring as

    R_j^2 - Q_j U0 = R_(j+1),  deg R_j<m, deg Q_j<=m-2,
    R_0=X mod U0, R_t=c,

gives an O(mt) quadratic coefficient presentation with the quotient coefficients retained. The complete coefficient/norm solver question remains open, but this comparator avoids the terminal-collision scheme defect. Its existence also makes it inappropriate to present Graeffe as the uniquely justified shared circuit.

## 6. Complexity and counting

With N=O(n) variables and effective degree D=O(sqrt(n)), binomial(N+D,D) has logarithm O(sqrt(n) log n), so a procedure polynomial in that monomial count and n has the claimed sub-exponential field-operation bound. The same conclusion holds for N polynomial in n; the precise O(n) variable claim is stronger than necessary. Polynomial bit cost per Fp operation does not change sub-exponentiality.

However, an upper bound on first fall, last fall, or refutation degree is not by itself a complete root-finding algorithm. SAT extraction, extension-degree filtering, saturation, and solver termination need bounds. The report correctly flags this but should carry the field-of-definition example into its acceptance criteria.

For B actual signed points and m much smaller than sqrt(B), the number of unordered distinct-abscissa tuples is approximately B^m/m!, with opposite pairs excluded. Mean representations across all r targets are that number divided by r. Coverage need not be close to this mean. With m~sqrt(n), a compensating factor O(m) in B preserves log B=O(sqrt(n)); the counting scale is coherent. Independent rank of relation rows and a useful Q coefficient are separate assumptions.

The proposed algebraic norm constraint can be imposed before enumerating membership states, but no efficient way to do so has been supplied. Enumerating binomial(d,m) divisors is exponential at balanced parameters. This is a genuine failure of that naive algorithm, not a proof that every implicit solver is exponential. Likewise x->x^2 is not generally an elliptic group homomorphism, so no free group-law recurrence is preserved through the layers.

## Requested edits to root report

1. Weaken the complete quantitative conclusion to exp(o(n)), or strengthen every overhead assumption to the displayed exp(O(sqrt(n) log n)) bound.
2. Make Fp versus Fpbar solution semantics explicit, add the F17 regression, and count any rationality mechanism.
3. Add terminal-Graeffe scheme multiplicity as a caveat and compare with radical quotient-ring membership.
4. State that polynomially many variables suffice for the conditional monomial argument; do not conceal rationality overhead behind the original O(n) count.
5. Retain the current honest status: no new efficient solver, no sub-exponential theorem, no certified novelty or positive growing-arity scaling evidence.

No issue found here disproves the existence of a sub-exponential prime-field ECDLP algorithm. The missing uniform solver bound is precisely where the requested breakthrough would have to occur.
