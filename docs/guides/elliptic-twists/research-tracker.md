# Elliptic Curve Twists and Discrete Logarithm Research Tracker

Research focus: elliptic curve twists, finite fields and discrete logarithm algorithms, especially whether twist-derived maps can improve index-calculus relation finding over a quadratic extension of a large prime field.

The key criterion is whether a map preserves the target subgroup and a useful factor base, and produces a measurable gain in independent relations or total discrete-log work. A twist isomorphism preserves point order; a weak companion twist does not by itself solve a static target on the original curve.

Last reviewed 25 October 2026.

# Research register

| Topic | Status | Next check |
| --- | --- | --- |
| Quadratic cubic quartic and sextic twists | Established classification for characteristic greater than 3 | Record the base field, power class and minimal isomorphism field. |
| Twist maps and target point order | Established constraint | Track the image of the actual target subgroup through every map. |
| Frobenius fixed and anti-fixed sectors | Established decomposition on the odd-order part | Separate the original rational subgroup from the quadratic-twist subgroup. |
| GLS order-four action | Established construction | Construct the transported Frobenius and verify its action on the chosen subgroup. |
| Factor bases built from endomorphism orbits | Open experiment for the proposed curve | Check invariance, stabilizers, orbit weights and relation-matrix rank. |
| Symmetries in point-decomposition equations | Published methods with model and field conditions | Compare ordinary and invariant-coordinate polynomial systems. |
| Automorphism-assisted Pollard rho | Established heuristic speedup | Include orbit-canonicalization cost, existing negation and useless-cycle handling. |
| Weil descent and GHS | Conditional reduction | Determine the descended genus, target map, kernel and total attack cost. |
| GLS acceleration after descent | Published binary-field case | Keep the demonstrated characteristic-two scope separate from a large-prime quadratic-extension experiment. |
| Isogenies to Frobenius conjugates and Q-curves | Published construction | Investigate cheap endomorphisms beyond curves descending directly to the prime field. |
| Coordinate models and isogeny transfers | Conditional research direction | Preserve the target subgroup and compare decomposition cost on each model. |
| MOV and Frey Rück pairing reductions | Conditional attack | Compute the embedding degree for the actual subgroup order. |
| Sextic twists in pairing groups | Established representation technique | Distinguish twist degree, coordinate field and pairing embedding degree. |
| Twist security invalid inputs and faults | Implementation dependent | State the required input or fault oracle separately from a static ECDLP. |

# Scope and mathematical constraints

The main setting is a finite field of characteristic greater than 3. Distinguish a curve originally defined over the prime field, a general curve over its quadratic extension, and a twist taken over that quadratic extension. Merely allowing larger-field coordinates does not change the original subgroup order or secret scalar.

An isomorphism over an extension transports the same discrete logarithm:

$$
Q=[k]P\quad\Longrightarrow\quad\phi(Q)=[k]\phi(P),\qquad\operatorname{ord}(\phi(P))=\operatorname{ord}(P).
$$

If the base-field point group of a proposed destination lacks the target prime factor, the transported target points cannot lie in that group. They may require a larger field. Twist degree is the degree of an isomorphism field; the isomorphism itself has degree one.

## Frobenius sectors

For a curve defined over the prime field, let π be its p-power Frobenius. On the odd-order part of its quadratic-extension point group, the projections are:

$$
R_+=\frac{R+\pi(R)}{2},\qquad R_-=\frac{R-\pi(R)}{2},\qquad \pi(R_+)=R_+,\quad\pi(R_-)=-R_-.
$$

Division by two here means the inverse of multiplication by two on the odd-order subgroup. The fixed sector is the original prime-field group; the anti-fixed sector corresponds, through a twisting isomorphism, to the prime-field rational points of its quadratic twist. Proposed cross-sector relations must retain information about the target rather than project it to zero.

## GLS maps and their four-point orbits

Start with a prime-field curve, extend its base field quadratically, and take a nontrivial quadratic twist over that extension. The twisting isomorphism is available over a further extension. Conjugating the original p-power Frobenius gives the GLS endomorphism:

$$
\psi=\phi^{-1}\circ\pi_p\circ\phi,\qquad \psi^2(P)=-P\quad\text{on }T(\mathbb F_{p^2}).
$$

A typical orbit is P, ψ(P), −P, −ψ(P), even when the geometric automorphism group has only two elements. On a preserved odd prime-order subgroup, the known scalar action satisfies:

$$
\psi(P)=[\lambda]P,\qquad\lambda^2\equiv-1\pmod r.
$$

For relation finding, orbit representatives can express several factor-base logarithms through one unknown. The factor base must be preserved, or its enlargement must be costed. Once these scalar actions are accounted for, applying the map repeatedly to one relation does not automatically produce independent matrix rows. The proposed benchmark is independent relations per unit of work, with memory and final linear algebra included. [GLS construction](https://eprint.iacr.org/2008/194)

## Group orders over the quadratic extension

If the original prime-field curve has Frobenius trace t, its extension group and its nontrivial quadratic twist over the extension have orders:

$$
\#E(\mathbb F_{p^2})=(p+1-t)(p+1+t),\qquad\#T(\mathbb F_{p^2})=(p-1)^2+t^2.
$$

The first order has a built-in factorization with factors about p. The twist order can have a large prime factor about p² while retaining an efficient transported Frobenius. A quadratic twist taken over the prime field instead becomes isomorphic after extending quadratically. [GLS and related constructions](https://arxiv.org/pdf/1305.5400)

# Twist families and field dependence

For short Weierstrass models, the coefficient changes and roots defining their isomorphisms are:

| Family | Coefficient change | Root used by the map |
| --- | --- | --- |
| Quadratic | A becomes d²A and B becomes d³B | u² = d |
| Cubic at j = 0 | B becomes a²B | u³ = a |
| Quartic at j = 1728 | A becomes cA | u⁴ = c |
| Sextic at j = 0 | B becomes cB | u⁶ = c |

The generic, quartic and sextic families are classified by nonzero field elements modulo squares, fourth powers and sixth powers respectively. Over a finite field, the number of classes modulo n-th powers is gcd(n, q − 1). Cubic and quadratic members sit inside the sextic family; the family name does not fix a member’s minimal extension degree.

At j = 1728 with q congruent to 3 modulo 4, the usual nonsquare quadratic-twist construction can be trivial although a second quartic-family form exists. Characteristics 2 and 3 have nonabelian exceptional automorphism groups at j = 0; degree-eight isomorphism fields can occur in characteristic 2. [Twists and capitulation](https://sigma-journal.com/2017/083/sigma17-083.pdf)

# Verified examples

## Six forms over the field with seven elements

For y² = x³ + B over the field with seven elements, the six coefficients below represent all twist classes. Degrees are relative to B = 1; every count includes the identity. Cₙ denotes a cyclic group of order n.

| B | Point count | Minimal extension degree | Group |
| --- | --- | --- | --- |
| 1 | 12 | 1 | C₂ × C₆ |
| 3 | 13 | 6 | C₁₃ |
| 2 | 9 | 3 | C₃ × C₃ |
| 6 | 4 | 2 | C₂ × C₂ |
| 4 | 3 | 3 | C₃ |
| 5 | 7 | 6 | C₇ |

Quadratic pairs have counts summing to 16. The B = 1 and B = 6 models both have 48 points over the field with 49 elements. These counts and the displayed groups were checked by exhaustive enumeration.

## A prime-order twist group over the field with 121 elements

The curve E: y² = x³ + x + 6 has 13 points over the field with 11 elements and 143 points over its quadratic extension. With u² = −1 and twist parameter 1 + u, its nontrivial quadratic twist over that extension is:

$$
T:\quad y^2=x^3+(2u)x+(10+u),\qquad \#T(\mathbb F_{121})=101.
$$

The counts 13, 143 and 101 were enumerated directly. The original order-13 target cannot be transported into the order-101 rational-point group; its image needs a larger coordinate field.

## Additional arithmetic checks

- Over the field with 101 elements, y² = x³ + 2x + 3 has 96 points and its nonsquare quadratic twist y² = x³ + 8x + 24 has 108 points.

- Over the field with 13 elements, y² = x³ + Ax for A = 1, 2, 4, 8 has 20, 10, 8, 18 points, with minimal extension degrees 1, 4, 2, 4 relative to A = 1.

- Over the field with seven elements, y² = x³ + x and y² = x³ + 3x both have eight points, but their groups are C₈ and C₂ × C₄.

# Attack mechanisms and evidence

## Polynomial relation generation

Published work uses symmetries of twisted Edwards and Jacobi-intersection models to simplify point-decomposition polynomial systems over extension fields. Track the factor-base definition, invariant variables, system degree and solution-lifting cost. A useful coordinate model can preserve the original point group; the word twisted in a model name does not by itself identify a new attack on a different twist class. [Faugère Gaudry Huot and Renault](https://researchportal.ip-paris.fr/en/publications/using-symmetries-in-the-index-calculus-for-elliptic-curves-discre/)

Frobenius-invariant factor bases for subfield curves can reduce the number of systems that need solving and the linear-algebra cost in the analyzed setting. Applying that strategy to a proposed twist-derived map remains conditional on its subgroup and factor-base actions. [On Index Calculus Algorithms for Subfield Curves](https://openresearch.surrey.ac.uk/esploro/outputs/conferenceProceeding/On-Index-Calculus-Algorithms-for-Subfield/99817423502346)

## Pollard rho and efficient arithmetic

Cheap symmetry orbits can improve collision searches by roughly the square root of their size under the relevant heuristic assumptions. Include the cost of choosing representatives and handling useless cycles. If negation is already used, do not count it again as a new gain. Faster scalar multiplication and faster point decomposition are distinct measurements. [Galbraith section 14.4](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch14.pdf)

## Descent and conjugate isogenies

Track Weil restriction, GHS descent and any isogeny-assisted transfer through the actual target subgroup. A homomorphism used to preserve a prime-order ECDLP must not kill that subgroup. For an isogeny, a degree coprime to the target prime is a sufficient condition. The descended genus and relation-generation cost determine whether the transfer helps.

The GLS descent paper discussed here studies generalized binary GLS curves and demonstrates a computation over a characteristic-two field. It does not establish a general large-prime quadratic-extension attack. [Chi Domínguez Rodríguez Henríquez and Smith](https://arxiv.org/abs/2106.09967)

Q-curve constructions provide another source of efficient maps: compose a low-degree isogeny to a Frobenius-conjugate curve with Frobenius. They extend the construction space beyond curves descending directly to the prime field. [Smith on Q-curves](https://arxiv.org/abs/1305.5400)

## Pairings and implementation attacks

For a prime-order subgroup r over a field with q elements, the pairing embedding degree is the multiplicative order of q modulo r. MOV and Frey Rück reductions exploit suitable pairings to transfer to a finite-field multiplicative DLP. Small embedding degree is the relevant condition; merely enlarging the coordinate field does not create it. [Galbraith chapter 26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf)

BLS12-381 represents G₂ on a sextic twist over the quadratic extension, with an untwist into the degree-12 extension. Its twist degree six and embedding degree twelve describe different field relationships. [RFC 9380 section 8.8](https://www.rfc-editor.org/rfc/rfc9380.html#section-8.8)

Twist security, invalid-curve inputs and fault attacks require their stated implementation or oracle conditions. They are separate from solving a static public ECDLP. Montgomery single-coordinate formulas can process points associated with the curve or its quadratic twist; unchecked full-coordinate formulas may reach additional curves. [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748.html)

# Experiments to run

- [ ] Choose one explicit curve over a quadratic extension of a large prime field, its target prime r, generator P and target Q. Record whether it descends to the prime field and the field over which each twist isomorphism exists.

- [x] Reproduce the order-four GLS action on the 101-point toy group and verify the scalar eigenvalue 91 on every point with `tools/verify_examples.c`.

- [ ] Establish comparable Pollard-rho and ordinary point-decomposition baselines, including memory, field-operation and bit-operation costs.

- [ ] Construct a useful factor base preserved by the proposed endomorphism, or measure the cost of taking its orbit closure.

- [ ] Compute orbit sizes and stabilizers. Account for sign symmetry already removed by x-coordinate representations.

- [ ] Compare original and invariant-coordinate decomposition systems: variables, degrees, solving time, memory and actual success probability.

- [ ] Lift polynomial solutions to points, verify the full signed sum and measure independent relation-matrix rank modulo r.

- [ ] Compare the total cost of relation generation and final linear algebra after orbit compression, rather than counting raw symmetry images as new relations.

- [ ] Investigate isogenous models or low-genus covers only after verifying that the target subgroup survives the transfer.

- [ ] Keep binary-field descent reproduction separate from large-prime quadratic-extension experiments.

# Sources and tools

- [Twists of Elliptic Curves](https://sigma-journal.com/2017/083/sigma17-083.pdf) — full characteristic coverage and capitulation.

- [GLS endomorphisms](https://eprint.iacr.org/2008/194) — quadratic-extension construction and efficient point multiplication.

- [Families of fast elliptic curves from Q-curves](https://arxiv.org/abs/1305.5400) — conjugate isogenies, GLS comparison and twist group orders.

- [Using symmetries in the index calculus for elliptic curves discrete logarithm](https://researchportal.ip-paris.fr/en/publications/using-symmetries-in-the-index-calculus-for-elliptic-curves-discre/) — point-decomposition systems and invariant structure.

- [On Index Calculus Algorithms for Subfield Curves](https://openresearch.surrey.ac.uk/esploro/outputs/conferenceProceeding/On-Index-Calculus-Algorithms-for-Subfield/99817423502346) — Frobenius-stable factor bases.

- [Extending the GLS endomorphism to speed up GHS Weil descent](https://arxiv.org/abs/2106.09967) — binary-field descent and its demonstrated scope.

- [Galbraith chapter 14](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch14.pdf) — Pollard rho on equivalence classes.

- [Galbraith chapter 26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf) — pairing reductions and twist representations.

- [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748.html) — Montgomery ladders and twist security.

- [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380.html#section-8.8) — BLS12-381 group equations and Frobenius operations.

- [SageMath finite-field elliptic curves](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html) — enumerate twist classes and inspect point groups.

# Learning materials

- [Illustrated guide PDF](Elliptic_Curve_Twists_Visual_Guide.pdf)
- [Diagram index](diagrams/README.md)
- [Build and verification instructions](README.md)

![Six sextic twist classes over the field with seven elements](diagrams/10_sextic_family_wheel.png)

