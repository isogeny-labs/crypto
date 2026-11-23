# RR5 four-summand extraction: exact scope and proof obligations

Date: 2026-11-22 Pacific. Parent tracking PR: #4938.

This note gives a three-parameter algebraic chart for **four finite factor-base
points with distinct abscissae** whose sum is a supplied public point. It is the
degree-four specialization of the repository's existing RR9 construction. It
does not claim a new Riemann--Roch method, a fast root-extraction algorithm, a
complete Prime31 solve, or a complexity improvement. The arguments below are
written mathematical derivations; they are not Lean-kernel or signed-PCC
certificates. [pcc:unverified]

## 1. Sources and non-duplication

The source snapshot inspected for this note is repository commit
`5514f88782e977945c940e424eb1a94aef06ba48`.

- [Existing RR9 projective identity and degree-nine certificate](../prime31_ic_20261115/rr9/RR9_PROJECTIVE_D9_THEOREM_20261117.md),
  Git blob `b7c7bc8db6cdd2680635b1873d608398b2a2b02d`: supplies the
  source-compatible norm identity and the no-solutions-at-infinity argument.
- [RR9 seven-of-eight split diagnostic](../prime31_ic_20261115/native/examples/prime31_rr9_seven_quotient_split.rs):
  supplies the previous signed-leaf identity. Its sign enumeration is a
  diagnostic reference, not an extraction algorithm reused here.
- [Trace--skew / Newton membership study](../trace_skew_prime31_20261122/README.md),
  Git blob `3adf118f52953ac7dc0825c578d0d0e983572e33`: records existing
  pair-symmetric and four-summand input presentations and their limitations.
- Semaev, [IACR ePrint 2004/031](https://eprint.iacr.org/2004/031), and
  Faugere--Huot--Joux--Renault--Vitse,
  [EUROCRYPT 2014 symmetrized summation polynomials](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf),
  DOI `10.1007/978-3-642-55220-5_3`, are established prior-art pointers.
  Fresh full-PDF retrieval returned HTTP 403 for both in this continuation;
  this note does not represent those failed retrievals as a new full-paper
  review. Its directly inspected mathematical source is the committed RR9 note.

Sources checked (KB): the parent session found no loaded research-KB tools and
the repository wrapper exited 127. This is a bounded named-source fallback,
not a claim that the literature or the sibling repositories contain no other
version of this construction. In particular, neither divisor encoding nor
symmetrization is claimed novel.

## 2. Assumptions and the public input

Let `K = F_p` have odd characteristic. Let

\[
 E: y^2=f(x)=x^3+A x+B
\]

be nonsingular, with identity `O`. Let the public target be the finite point
`Q=(r,s)`. Fix distinct `x_i` in `K`, each with a chosen nonzero ordinate `y_i`
satisfying `y_i^2=f(x_i)`. The signed factor base contains both
`P_i=(x_i,y_i)` and `-P_i=(x_i,-y_i)`. Write

\[
 h(X)=\prod_{i=1}^{B_{\rm FB}}(X-x_i),\qquad h(r)\ne0.
\]

The notation `B_FB` is the number of distinct abscissae, not the curve
coefficient `B`. The assumptions exclude identity targets, target abscissae
already in the factor base, and rational two-torsion leaves. The unchanged
ordinary Prime31 fixture can instantiate these assumptions; each executable
caller must check them rather than assuming them from this note.

The desired output is a signed row with exactly four distinct supported
abscissae, together with its four actual points and a literal group-law check
that their sum equals `Q`. No scalar of `Q`, hidden leaf index, supplied
four-tuple, or precomputed pair-sum dictionary is part of the candidate input.

## 3. Three unknowns and four polynomial equations

Introduce three unknown field elements `b,c,d`, and set

\[
 V(X)=X+b,\quad L(X)=cX+d,\quad
 D_Q(X)=X^2+rX+r^2+A.
\]

Define the function on `E`

\[
 \ell(X,y)=(X-r)L(X)+(s+y)V(X).
\]

It vanishes at `-Q=(r,-s)` by construction. Its coefficient of `Xy` is one,
so it has exactly a pole of order five at `O`. Its norm gives the identity

\[
 f(X)V(X)^2-\big((X-r)L(X)+sV(X)\big)^2=(X-r)U(X),
\]

where

\[
 \boxed{U(X)=D_Q(X)V(X)^2-(X-r)L(X)^2-2sL(X)V(X).}
\]

Here `U` is monic of degree four; each of its four lower coefficients has
total degree at most two in `b,c,d`. Compute the unique monic remainder

\[
 h(X)\bmod U(X)=R_0(b,c,d)+R_1(b,c,d)X+
 R_2(b,c,d)X^2+R_3(b,c,d)X^3.
\]

The candidate algebraic problem is precisely

\[
 R_0=R_1=R_2=R_3=0.
\]

Constructing these equations is not the extraction step. A qualifying
implementation must actually solve them over the field, recover their
coefficient roots without a tuple search, factor the resulting quartic, and
produce signed rows. A cap that prevents the required algebraic root cover is
**inconclusive**. Unfinished critical pairs alone need not prevent a complete
answer if derived univariates already cover every branch, all original
generators are preserved, and every terminal assignment is checked against
them; Section 8 states that alternative precisely.

## 4. Soundness, completeness, and the denominator

**Claim 1 (denominator invertibility).** Under the assumptions above, every
solution of `U | h` satisfies `gcd(U,V)=1`.

Proof. If `V(z)=U(z)=0`, the norm identity forces
`(z-r)L(z)+sV(z)=0`. Thus both `V` and the other squared factor in the norm
are divisible by `X-z`, and `(X-z)^2` divides `(X-r)U`. Since `U | h` and `h`
is squarefree, this forces `z=r` and `U(r)=0`, contradicting `h(r) != 0`.
As an implementation check, the displayed formula also gives the exact
identity `U(-b)=(b+r)(d-bc)^2`. No extra saturation variable is necessary
for this stated chart.

**Claim 2 (signed output and converse).** Solutions `(b,c,d)` are in bijection
with unordered signed four-point decompositions of `Q` whose abscissae are
four distinct roots of `h`.

Proof of soundness. A monic divisor `U` of squarefree split `h` has four
distinct roots `z_1,...,z_4` in `K`. By Claim 1 define

\[
 t_j=-s-\frac{(z_j-r)L(z_j)}{V(z_j)}.
\]

The norm identity gives `t_j^2=f(z_j)`. Each point `(z_j,t_j)` is therefore
one of the two signed factor-base lifts. The five distinct finite points
`(z_1,t_1),...,(z_4,t_4),-Q` are zeros of `ell`. The pole divisor has degree
five, so these account for its full zero divisor, with multiplicity one.
A principal divisor on an elliptic curve has zero group sum. Hence the four
leaves sum to `Q`.

Proof of completeness. Given the stated four-point decomposition, the
degree-zero divisor

\[
 P_1+P_2+P_3+P_4+(-Q)-5(O)
\]

has trivial divisor class and is principal. Its function is unique up to
nonzero scalar, lies in `L(5O)=span{1,x,y,x^2,xy}`, and has pole order exactly
five because all five zeros are finite. The `xy` coefficient is therefore
nonzero. Normalize it to one; the resulting function has the form above
because it vanishes at `-Q`. Its monic residual norm is the product of the
four leaf abscissa factors, which divides `h`. The normalized function is
unique, proving injectivity as well.

Consequently, the `xy=1` normalization does not omit a second projective
coefficient chart under these assumptions. The genuine omitted strata are
listed in Section 7.

## 5. Rationality and reducedness of the chart

**Claim 3 (no nonrational coefficient solutions).** Every solution over an
algebraic closure of `K` already has `b,c,d` in `K`.

Proof. The roots of `U | h` are in `K`. The reconstructed ordinates satisfy
`t_j^2=f(z_j)=y_j^2`, so in odd characteristic they are `+y_j` or `-y_j`,
both in `K`. Thus the complete zero and pole divisor of the normalized
function is rational. Frobenius preserves that divisor, so it takes the
function to a scalar multiple of itself. Both functions have `xy`
coefficient one, making the scalar one. All coefficients are fixed by
Frobenius. This justifies omitting enormous field equations; it does not
justify enumerating the field to find the roots.

**Claim 4 (finite radical chart).** The remainder ideal has finitely many
geometric roots and is reduced at each of them.

Proof. Claim 2 bounds the number of roots by
`16 * binomial(B_FB,4)`, hence the solution set is finite. At a solution,
write `h=UW`; squarefreeness gives `gcd(U,W)=1`. An infinitesimal variation
that leaves all four remainder equations zero obeys
`delta U * W + U * delta W = 0`, so reduction modulo `U` forces
`delta U=0` (it has degree at most three).

Differentiate the norm identity. At each signed leaf, its derivative is
`2 t_j V(z_j) delta ell(P_j)`. The first two factors are nonzero by the
non-two-torsion and denominator assumptions, so `delta ell` vanishes at all
four leaves. It also vanishes at `-Q` because that condition was built into
the parameterization. But `delta ell` lies in `L(4O)`: the normalized `xy`
coefficient cannot vary. A nonzero function in `L(4O)` cannot have five
distinct finite zeros. Therefore `delta ell=0` and
`delta b=delta c=delta d=0`. The four-by-three Jacobian has rank three at
every root. A zero-dimensional ideal whose geometric roots all have zero
tangent space is reduced.

This result supports exact quotient-ring or triangular root extraction.
It supplies no small bound on the quotient dimension, Groebner solving
degree, intermediate support, or execution cost.

## 6. A special exact bound at factor-base size four

If `B_FB=4`, the divisibility constraint is exactly `U=h`, hence four
quadrics in three unknowns. Homogenize them with a variable `t`, taking
`V_t=tX+b` and `L_t=cX+d`. At `t=0`, a common root would satisfy

\[
 D_Q b^2-(X-r)L_t^2-2sbL_t=0.
\]

Multiplication by `X-r` and the identity `(X-r)D_Q+s^2=f` give

\[
 f(X)b^2=((X-r)L_t(X)+sb)^2.
\]

A nonzero `b` would make the squarefree cubic a square in the rational
function field, which is impossible. With `b=0`, the identity forces
`L_t=0`. Thus no nontrivial projective root lies at infinity.

For an empty affine fibre, the four homogeneous quadrics in four variables
therefore have height four and form a regular sequence. Their Hilbert series
is `(1+z)^4`, so the degree-five component of the quotient is zero. The
degree-five homogeneous Macaulay matrix has

\[
 {8\choose3}=56\text{ columns},\qquad
 4{6\choose3}=80\text{ row generators}.
\]

It has full column rank exactly when there is no chart solution: the forward
implication is a unit certificate; in the reverse direction a projective
solution gives a nonzero evaluation nullvector. This is a bounded exact
**membership discriminator**; a rank-deficient result still needs actual
coefficient-root extraction. The bound belongs to `B_FB=4` only. It must
not be applied to the higher-degree remainder equations for larger bases.

## 7. Excluded strata and how to report them

| Case | Required treatment |
|---|---|
| `Q=O` | Separate homogeneous-relation path; this affine target chart is inapplicable. |
| `h(r)=0` | Direct signed-base membership handles the obvious one-point relation; an all-four-summand oracle still needs its separate exceptional chart. Do not call exclusion UNSAT. |
| A leaf has `y=0` | Excluded by the non-two-torsion assumption; the reducedness proof would need modification. |
| Two leaves have the same abscissa | Excluded by `U | h`; repeated equal-sign points and opposite-sign cancellation must be classified separately. |
| A four-leaf multiset contains an inverse pair | Its group sum reduces to a two-summand query, but a complete four-leaf enumerator must retain the multiplicity accounting. |
| F4 cap preventing root coverage, missing eliminant, factorization failure | Inconclusive, with the actual cap and work retained. Unfinished S-pairs alone are not an emptiness proof. |
| Complete chart has no roots | Empty **distinct-abscissa RR5 chart**, not absence of every four-summand decomposition. |

The condition `U | h^4` captures abscissa multiplicities at the polynomial
support level, but it does not inherit the squarefree denominator and
reducedness arguments. It is an explicit follow-up, not an implemented
complete exceptional treatment. A structured extension may instead separate
the multiplicity partitions `2+1+1`, `2+2`, `3+1`, and `4`, and cancellation
branches. Their discovery costs must be counted.

## 8. Honest extraction and downstream integration

An admissible sequence is: construct the public remainder ideal; derive
univariate elimination polynomials or certified multiplication matrices;
factor those polynomials; recover coefficient solutions; factor each
degree-four `U`; lift its four signs using the formula; and replay the group
sum. A complete Groebner basis is one way to justify the elimination step,
but is not a necessary condition for the following exact traversal.

At each recursive node retain its original specialized generators, derive a
nonconstant pure univariate in their ideal, and branch over **all** its field
roots. Every solution must appear on one of those branches. Specialize the
retained system and repeat, checking every final assignment against the
original unspecialized input. A derived nonzero constant closes a node as
empty. Induction on the remaining variables proves exhaustive coverage even
if some unneeded Groebner critical pairs were not processed. A node with no
usable univariate, an interrupted factorization, or an expired budget remains
inconclusive. Such a node may not fall back to a full-field or tuple scan.

Factoring an algebraically obtained univariate polynomial is not the same as
trying every field element or scanning every factor-base pair. Conversely,
substituting every root of `h` into an otherwise multivariate search, scanning
the field when no eliminant exists, or building a pair-sum index changes the
method. Any such fallback must be refused or separately classified and
charged; it must not be hidden under an algebraic API name.

For `Q=[a]G+[b]T`, a signed row `c_i` yields

\[
 \sum_i c_i\log_G(P_i)=a+b\log_G(T)\pmod n.
\]

The signs are determined by actual point equality with the chosen base lifts.
For labelled training points `[k]G`, the RHS is the public `k`. Literal group
replay is necessary for soundness, while incremental rank modulo the subgroup
order is a separate necessary condition for useful independent equations.
Four recovered points do not by themselves add one rank. Reuse the existing
authentic-RHS solver's row/rank/target-quotient handling rather than replacing
it with a count of positive extraction calls.

## 9. Costs that remain to be established

The quartic has constant-size coefficients, but the factor-base membership
constraint does not. Direct symbolic monic remaindering has degree at most
`2(B_FB-3)` in the three parameters when `B_FB>=4`; its dense support can
therefore have cubic growth in `B_FB`. A compact quotient recurrence uses
`O(B_FB)` arithmetic gates, but a symbolic gate is not a unit-cost field
operation and a quadratic circuit lift introduces additional unknowns.
Neither presentation yields a running-time bound for elimination.

Charge the complete calculation as

\[
 C_{\rm cold}=C_{\rm base}+C_h+
 \sum_{\rm attempted\ queries}(C_{\rm build}+C_{\rm GB}+C_{\rm extract}
 +C_{\rm factor}+C_{\rm lift}+C_{\rm replay})+
 C_{\rm rank}+C_{\rm LA}+C_{\rm recovery}+C_{\rm verification}.
\]

Retain empty queries, inconclusive queries, failed attempts, setup reused
between targets, and peak memory of symbolic polynomials, matrices,
factorization, and the relation store. Report cold one-target cost first;
amortized cost has its actual target count and cannot replace it. The number
of output rows is itself a cost when a query has many decompositions.
Any exponent claim needs a parameterized family, a bound for the actual
extraction algorithm, and rank/yield assumptions. The three-variable input,
the degree-two quartic coefficients, and a bounded Prime31 correctness check
do not establish that claim.

## 10. Proof map and formalization status

| Claim | Written argument | Formal target and missing bridge |
|---|---|---|
| Norm identity and quartic monicity | Section 3 | Polynomial-ring identity with the public curve equation `s^2=f(r)`; no Lean declaration has been authored. |
| Automatic denominator invertibility | Claim 1 | Squarefree divisor and valuation/multiplicity lemma for `N=(X-r)U`. |
| Signed soundness and completeness | Claim 2 | Elliptic divisor-class/group-law bridge and `L(5O)` basis with pole orders. |
| Rational coefficient roots | Claim 3 | Frobenius action and uniqueness of the normalized principal function. |
| Finite reduced ideal | Claim 4 | Factor-divisor tangent map plus divisor-zero bound for `L(4O)`; algebraic-geometric reducedness bridge. |
| B4 degree-five empty certificate | Section 6 | Homogeneous no-infinity proof, regular-sequence/Hilbert-series specialization, and Macaulay row-span binding. |

`command -v lean` and `command -v lake` returned no executable in this
runtime's PATH during this continuation. No build, axiom audit, or kernel
verification was performed. No placeholder theorem, `sorry`, custom axiom,
or claimed compiled proof is included. The lowest-cost formalization entry
point is the polynomial norm identity, followed by the denominator lemma;
the divisor and quotient-ring bridges remain explicit obligations.

## Qualification addendum: monic auxiliary cofactor

For B=5 through8 the native extractor now retains the same U and h but introduces
W=X^(B-4)+sum_(j=0)^(B-5) w_j X^j. It enforces all B coefficients below the
leading term in U*W-h. Monic division over F_p[b,c,d] writes h=U*Q+R uniquely,
with deg(R)<4. The top B-4 coefficient equations determine W=Q triangularly
with unit pivots. The final four equations are exactly R=0. Thus adding W
preserves the affine coordinate ring; it needs no parameter inversion or extra
saturation and introduces no additional projected solutions. This is a written
algebraic argument, not a formal-kernel proof. [pcc:unverified]

The existing chart argument for denominators is unchanged. In particular,
U(-b)=(b+r)(d-bc)^2. A zero denominator forces either b=-r, contrary to h(r)!=0,
or d=bc, when V^2 divides U, contrary to squarefree h and U|h. This does not add
the excluded repeated-abscissa or identity-target charts. [pcc:unverified]

This construction has cubic input equations, but its naive homogenization can
have auxiliary directions at infinity (homogenizing variable and b,c,d zero,
nonzero cofactor-coordinate vector). The B4 degree-five Macaulay argument does
not transfer. A complete Grevlex basis need not contain a displayed pure
univariate either. The successful fixed B8 controls therefore remain bounded
observations; they do not give a general solving-degree bound. [pcc:unverified]

The separately encoded norm factorization now has evaluated certificate
[IDC1h16239a628042ecea](IDENTITY.json). The unchanged production checker accepted
32 deterministic certificate points and32 additional points over2^61-1, and
refused the expression with its signed cross term omitted. This certificate
has no formal proof pointer and is not signed independent verification. It
checks that encoded polynomial expression, not this divisor-chart theorem or
all implementation paths. See `evidence/identity-01` and REPORT.md.
