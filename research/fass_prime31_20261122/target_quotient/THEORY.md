# Target-dependent quotient rank of the first S3 norm

Tracking: crypto PR #4952. Reviewed base: `f0ca664eca0081a30adfcc79dcf62a32de40d2af`.
Status: written mathematical derivation and independent mathematical audit; no
new code execution, benchmark, formal proof, or signed execution is asserted
by this file. [pcc:unverified]

This component concerns the coefficient rank after reduction by a
target-dependent right message. It does not implement the concurrently active
signed three-summand extractor or the four-term pair-spectrum work. No novelty
claim is made.

## 1. Result and exact domain

The proposed extension is valid: for each fixed nonempty subset W of the
selector roots, with k elements, reduction by its degree-2k right message has
generic coefficient separation rank min(d,2k). For the full selector, the
reduction changes nothing whenever its leading coefficient is nonzero.

Let p > 2d be prime. Work initially over a field F of characteristic p that
contains all d-th roots of unity, and put

$$
V=\mu_d,\qquad L(u)=u^d-1.
$$

For the intended ground-field application, take F=F_p and additionally require
d dividing p-1. Without this divisibility, the algebraic result can be read
over a splitting field; an arbitrary subset W need not define a polynomial
over F_p. The assumption p > 2d implies that L is squarefree.

Let A,B,X be independent indeterminates, and use the standard short-Weierstrass
third summation polynomial

$$
S_{A,B}(x,z,t)=(x-z)^2t^2
-2((x+z)(xz+A)+2B)t+(xz-A)^2-4B(x+z).
$$

Define the unchanged first norm and its canonical x-remainder by

$$
N_{A,B}(x,t)=\operatorname{Res}_z(L(z),S_{A,B}(x,z,t)),\qquad
H_{A,B}(x,t)=N_{A,B}(x,t)\bmod L(x).
$$

The prior [coupled report](../coupled/REPORT.md), sections 3 and 4, establishes
the coefficient identities used below. In particular,

$$
\deg_x H<d,\qquad \deg_t H\le 2d-1.
\tag{1}
$$

Fix W contained in V, with 1 <= k=|W| <= d, before varying A,B,X. Define

$$
P_W(A,B,X;t)=\prod_{z\in W}S_{A,B}(t,z,X),\qquad
\lambda_W(X)=\prod_{z\in W}(X-z)^2.
$$

The coefficient of t^(2k) in P_W is exactly lambda_W. On the locus
lambda_W(X) != 0, or equivalently X not in W, set

$$
R_W(A,B,X;t)=\lambda_W(X)^{-1}P_W(A,B,X;t),\qquad
K_W(x,t)=H_{A,B}(x,t)\bmod R_W(t).
\tag{2}
$$

Here R_W is monic of degree 2k, and the remainder is taken in t. Consequently,

$$
\deg_x K_W<d,\qquad \deg_t K_W<2k.
$$

Let C_W(A,B,X) be the d-by-2k coefficient matrix of K_W in the monomial bases
x^i and t^j. Its rank is the minimum number of separated polynomial terms
u_i(x)v_i(t) needed for this particular canonical remainder. The claim is
about this matrix, not every function or polynomial with the same zero set.

## 2. Generic-rank proposition

**Proposition 1.** For each fixed W as above,

$$
\operatorname{rank}_{F(A,B,X)} C_W(A,B,X)=\min(d,2k).
\tag{3}
$$

This is a written derivation at the stated algebraic scope. [pcc:unverified]

### Proof

Write r=min(d,2k). The upper bound r follows from the matrix dimensions.

Specialize A=B=X=0 to exhibit a nonzero minor. Every z in W is nonzero, so
lambda_W(0)=product_(z in W) z^2 is nonzero. All coefficients in the monic
division are therefore regular at this specialization. Direct substitution
gives

$$
S_{0,0}(t,z,0)=z^2t^2,\qquad R_W(0,0,0;t)=t^{2k}.
\tag{4}
$$

The specialized quotient is nonreduced. That causes no problem for monic
division or coefficient specialization; no product-of-fields argument is
being applied at this proof witness.

From the prior cusp identity, for 0 <= j < d,

$$
[t^j]H_{0,0}(x,t)=(-1)^j\binom{2d}{j}x^{-j}
\quad\text{in }F[x]/(x^d-1).
\tag{5}
$$

Reduction modulo t^(2k) preserves the columns j=0,...,r-1. The monomials
x^(-j), for these indices, occupy distinct rows, and their binomial
coefficients are nonzero because p > 2d.

For a concrete minor, use columns 0,...,r-1 and the row set

$$
I_r=\{0,d-r+1,d-r+2,\ldots,d-1\},
\tag{6}
$$

in ascending order, with I_1={0}. Its determinant D_W specializes to

$$
D_W(0,0,0)=(-1)^{r-1}
\prod_{j=0}^{r-1}\binom{2d}{j}\ne0.
\tag{7}
$$

The sign combines reversal of the last r-1 rows with the column factors
(-1)^j. If r<d, this certificate generally does not use the upper-left
r-by-r submatrix.

The entries of C_W belong to F[A,B,X,1/lambda_W]. A conservative explicit
denominator clearing is available. By (1), division by a monic degree-2k
polynomial needs at most q=2d-2k leading-term subtraction stages. Each stage
introduces at most one further denominator lambda_W. Thus lambda_W^q times
every entry of C_W is polynomial, and lambda_W^(qr) D_W is polynomial. Its
value at the specialization remains nonzero. This supplies a nonzero
polynomial minor on the open locus lambda_W != 0 and proves (3).

### Full-selector case is an exact identity, not just a generic statement

If W=V, then

$$
\lambda_V(X)=(X^d-1)^2,\qquad \deg R_V=2d
\quad\text{when }X^d\ne1.
$$

Since deg_t H <= 2d-1, one has

$$
\boxed{K_V(x,t)=H_{A,B}(x,t)\quad\text{whenever }X^d\ne1.}
\tag{8}
$$

This equality holds at every such coefficient specialization, including
exceptional curves where H itself has smaller rank. It does not require
R_V to be squarefree. Taking a smaller squarefree rational part is a
different operation and is addressed below.

## 3. Why the singular proof witness is legitimate

The specialization A=B=X=0 is used only to establish that a rational-function
minor is nonzero. It is not an elliptic-curve fixture, a rational-target test,
or an example of a squarefree right message.

For short-Weierstrass geometry, also assume p>3. Multiplying the cleared
minor by the nonzero polynomial 4A^3+27B^2 shows that its nonvanishing locus
meets the nonsingular-curve locus over an algebraic closure. Additional
nonempty open conditions can be imposed in the same manner.

Requiring a target on the curve does not invalidate generic nonvanishing.
Adjoin Y with

$$
Y^2=X^3+AX+B.
\tag{9}
$$

The target incidence coordinate ring is isomorphic to F[A,X,Y] by substituting
B=Y^2-X^3-AX. The resulting map from F[A,B,X] is injective: the highest
B-degree of a nonzero polynomial supplies its nonzero highest even Y-power.
Thus the cleared minor remains nonzero on this incidence variety. Imposing
Y != 0 excludes targets of order two, and still leaves a nonempty open set.

These are generic algebraic statements. They do not assert that every fixed
curve has a good F_p-rational target, or that any prescribed finite-field
liftability pattern has a rank-deficiency probability bound.

## 4. The rational right message: exact discriminant bridge

Now specialize to a nonsingular curve E/F_p with p>3 and a finite rational
target Q=(X,Y), with Y != 0. Assume X not in V. Put

$$
f_E(u)=u^3+Au+B,\qquad q_z(t)=S_{A,B}(t,z,X).
$$

As a quadratic in t, q_z has coefficients

$$
a_z=(X-z)^2,
$$

$$
b_z=-2((X+z)(Xz+A)+2B),\qquad
c_z=(Xz-A)^2-4B(X+z).
$$

Expansion gives the exact identity

$$
\operatorname{disc}_t(q_z)=b_z^2-4a_zc_z
=16f_E(z)f_E(X)=16Y^2f_E(z).
\tag{10}
$$

The leading coefficient a_z is nonzero by X not in V. Therefore:

| Selector coordinate z | Rational roots of q_z |
| --- | --- |
| f_E(z) is a nonzero square in F_p | Two distinct roots in F_p |
| f_E(z) is a nonsquare in F_p | No roots in F_p |
| f_E(z)=0 | One rational root, with multiplicity two |

For a lift P_z=(z,y_z), the two roots are x(Q+P_z) and x(Q-P_z).
Equivalently, away from the excluded infinities,

$$
\frac{q_z(t)}{(X-z)^2}
=(t-x(Q+P_z))(t-x(Q-P_z)).
\tag{11}
$$

This is the short-Weierstrass specialization of the biquadratic
x-coordinate relation in Bernstein, De Feo, Leroux and Smith, Lemma 4.3
([primary paper](https://arxiv.org/pdf/2003.10118)). Identity (10) can also be
checked directly from the displayed coefficients, without a group-law
existence argument.

Suppose first that f_E(z) != 0 for every z in V, and define the actual
liftability subset

$$
W_E=\{z\in V:f_E(z)\text{ is a nonzero square in }F_p\}.
$$

Let P_V be the unnormalized full product. Then, with monic gcd and monic
squarefree part,

$$
\boxed{\gcd(P_V(t),t^p-t)=\operatorname{rad}(R_{W_E}(t)).}
\tag{12}
$$

Indeed, t^p-t has every F_p element as a simple root. The discriminant table
identifies exactly which factors of P_V contribute such roots. Repeated roots
within the total product are retained once by either side.

When the factors q_z for z in W_E have no common roots, R_(W_E) is itself
squarefree. In that case (12) becomes

$$
\gcd(P_V(t),t^p-t)=R_{W_E}(t),\qquad
\deg R_{W_E}=2|W_E|.
\tag{13}
$$

Thus the fixed-W proposition is the relevant symbolic family for the actual
rational-part quotient on this collision-free arithmetic stratum. The
specialized rank can still be smaller than its generic value; the theorem
does not certify every point of that stratum.

## 5. Squarefreeness, collisions and exceptional branches

For a generic nonsingular E, the selector coordinates have nonzero f_E(z).
For each such z, choose a lift P_z over an algebraic extension. A generic
target Q then gives two distinct finite values x(Q+P_z), x(Q-P_z).

For different z,w, an equality

$$
x(Q+\epsilon P_z)=x(Q+\eta P_w),\qquad
\epsilon,\eta\in\{1,-1\},
$$

would imply either epsilon P_z=eta P_w, impossible because z!=w, or

$$
2Q=-\epsilon P_z-\eta P_w.
\tag{14}
$$

The latter constrains Q to finitely many points. For example, on Y!=0 the
doubling formula is

$$
x(2Q)=\frac{X^4-2AX^2-8BX+A^2}{4(X^3+AX+B)}.
$$

For a fixed finite value x(2Q), clearing this denominator gives a nonzero
quartic with leading coefficient one. The case 2Q=O is also finite, being
the 2-torsion. Thus (14) cannot hold for the generic target. Hence R_W is
generically squarefree. Its discriminant is a
nonzero rational function, so squarefreeness and the nonzero rank minor hold
on a common nonempty open set. The repeated-root witness in (4) is compatible
with this conclusion.

At an actual finite-field target, the following cases must be distinguished.

| Condition | Effect on the bridge or quotient |
| --- | --- |
| Q is the point at infinity | There is no finite target X; use a separate group-law branch |
| Y=0 | The target is 2-torsion; (10) vanishes even for nonliftable z, so the square/nonsquare bridge above does not apply |
| X in V | At least one quadratic leading coefficient vanishes; its missing branch is at infinity, and the degree-2k normalization must not be used |
| A selector root is 2-torsion, f_E(z)=0 | Its factor contributes one distinct rational root, not two |
| Different selector fibers collide as in (14) | The rational squarefree part has degree below 2|W_E|; retain witness multiplicities separately if needed |
| W_E is empty | The rational part is 1, so there is no finite right-message candidate |
| Factor base is restricted to a proper subgroup | Liftability alone is insufficient; filter subgroup membership and use the corresponding selector subset |

The last row is important: gcd(P_V,t^p-t) finds rational x-coordinate
candidates on E, not necessarily points in a required prime-order subgroup.
An odd-prime-order factor base excludes 2-torsion, but the unfiltered full
product can still contain those coordinates. A cofactor operation can change
x-coordinates and should not be silently substituted for this subset filter.

Equation (14) describes collision targets; it is not a new solver for their
scalars. Recovering eliminated coordinates, choosing consistent signs,
checking subgroup membership, and verifying the exact point sum remain
separate obligations.

## 6. Fixed subsets versus arithmetic liftability patterns

Proposition 1 fixes W as a set of constants while A,B,X vary. The actual
W_E depends on quadratic characters of f_E(z). Requiring precisely a chosen
square/nonsquare pattern is a finite-field arithmetic condition, not the
same assertion as a nonempty Zariski-open condition over an algebraic
closure.

Consequently, none of the following follows from the proposition alone:

- every fixed nonsingular curve or rational target attains rank min(d,2k);
- every fixed liftability pattern has a good F_p-rational specialization;
- a failure-probability estimate under uniform rational targets on one curve;
- a probability bound conditional on fixed A, ordinary curves, a prescribed
  group order, or a prime-order subgroup;
- a rank formula for a collided squarefree quotient of degree below 2k.

The generic-rank theorem for each fixed W is still useful. It shows that
selecting fewer right fibers does not, by itself, force small separation
rank: if k>=ceil(d/2), the generic rank remains d; if k<ceil(d/2), the generic
rank reaches the full available column dimension 2k.

## 7. Basis changes and the remaining useful structure

At a specialization where L and R_W split into distinct ground-field roots,
let V_x and V_t be their square Vandermonde evaluation matrices. The grid of
evaluated values satisfies

$$
\bigl(K_W(\alpha,\tau)\bigr)_{\alpha\in V,\ R_W(\tau)=0}
=V_x C_W V_t^{\mathsf T}.
\tag{15}
$$

Both evaluation matrices are invertible. Thus moving from coefficient bases
to CRT/evaluation bases preserves this separation rank; it cannot turn a
full-rank canonical representative into a low-rank separated sum by an
invertible change of either univariate basis alone.

This is not a time lower bound. Full-rank matrices may still have short
circuits or structured operations. A useful next mathematical target remains
an exact nonunit/common-root query that uses the surviving norm circuit
without materializing every coordinate of the product algebra, with separate
accounting for arithmetic, reduction, setup reuse and witness extraction.
Another zero-set representative is outside this rank proposition and would
require its own equivalence proof.

The primary comparison remains elliptic-resultant evaluation in Bernstein,
De Feo, Leroux and Smith, Lemma 4.9 and Theorem 4.11. Their index-system
hypotheses supply compatible elliptic-group structure; a multiplicative set
of x-coordinates does not automatically satisfy them. Example 4.13 also
records the cost dependence when evaluating over a larger coefficient
algebra. No faster common-root method is asserted here.

## 8. Proof and implementation obligations for this scope

No native research code or benchmark was executed for this audit. The following are prospective exact
checks, not measurements recorded by this file:

1. Check monic normalization and canonical remainder against exact
   polynomial division, including the full-selector identity (8).
2. Check the rectangular certificate rows (6), which differ from the first
   r rows when r<d, against the specialized minor (7).
3. Check (10) and the rational-gcd identities (12)-(13), with explicit
   records for square roots, nonsquares, 2-torsion and collisions.
4. Record the actual quotient degree and coefficient rank for each chosen
   fixture, rather than importing min(d,2k) as its measured rank.
5. Keep group-sum recovery and complete ECDLP claims outside a rank-only
   qualifier. They belong to the separately owned signed-extraction work.

The universal statements here remain written derivations. A fixed evaluation
certificate or a finite native fixture suite would not by itself turn them
into a kernel-checked proof. [pcc:unverified]

## Sources and access record

- [Prior coupled-norm report](../coupled/REPORT.md), at the reviewed base
  above: the cusp identity, first-d-column determinant and degree_t bound.
  The present argument extends the canonical coefficient-rank question to a
  fixed-subset target quotient.
- Petit, Kosters and Messeng,
  [Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 2.4 and 3.1: composed factor-base selectors and polynomial-system
  motivation. This is not used as evidence for a fast solver.
- Bernstein, De Feo, Leroux and Smith,
  [Faster computation of isogenies of large prime degree](https://arxiv.org/pdf/2003.10118),
  Lemma 4.3: the biquadratic relation for x(Q+P), x(Q-P); Lemma 4.9,
  Theorem 4.11 and Example 4.13: elliptic resultants and their evaluation
  hypotheses. These primary sections were read during the audit.
- The original Semaev ePrint 2004/031 was located bibliographically, but its
  PDF fetch returned HTTP 403. A Diem author-hosted preprint URL returned
  HTTP 404. Neither unavailable text is represented as newly read evidence.
- KB unavailable in this session: no loaded MCP capability; the parent
  reports that the repository wrapper has no executable. The accepted
  current-main report and the accessible primary sections above were used.
