# Pre-call selector-target cancellation-coordinate prediction

Tracking: [crypto PR #4966](https://github.com/isogeny-labs/crypto/pull/4966).
This note precedes the new qualifier calls. It adds a classification of the
already selected grid fixtures, not a new target panel or performance run.
All statements are written derivations or predictions with unverified PCC
standing. [pcc:unverified]

## Exact domain

Let p>3 and E: y^2=x^3+Ax+B be nonsingular. Let d divide p-1, let
V=mu_d, and let Q=(X,Y) be a rational target with X in V and Y nonzero.
Use the full selector F(x)=x^d-1 and the unnormalized messages

$$
H(x,t)=\prod_{z\in V}S_3(x,z,t)\bmod F(x),\qquad
P_X(t)=\prod_{z\in V}S_3(t,z,X),
$$

and set G=gcd(P_X,t^p-t), monic. Its root set T is rational and squarefree;
write e=|T|. The polynomial S3 is symmetric in its three coordinates.

For each t in T, define

$$
C_t=\{x\in V:x=X\ \text{or}\ S_3(x,X,t)=0\}.
\tag{1}
$$

This is the proposed **cancellation-coordinate subset**, a set of affine
coordinate pairs. Its name describes one explanation for those pairs. It
does not classify every signed witness sharing them.

## A forced support subset and lower bound

**Lemma.** Every C_t is contained in the H-zero fibre at t. There is a unique
finite t_star=x(2Q) with S3(X,X,t_star)=0, it belongs to T, and

$$
|Z|\ge 2e-1.
\tag{2}
$$

Here Z={(x,t):x in V,t in T,H(x,t)=0}. [pcc:unverified]

*Derivation.* At x=X, symmetry gives H(X,t)=P_X(t), so all t in T supply
the vertical zero (X,t). If S3(x,X,t)=0, the factor z=X in the defining
product for H vanishes, giving the other inclusion.

For every t in T, at least one z in V satisfies S3(t,z,X)=0 because P_X(t)
is a product of those values. Thus the second set in (1) is nonempty. On
the diagonal, direct substitution gives

$$
S_3(X,X,t)=-4Y^2t+(X^2-A)^2-8BX.
\tag{3}
$$

Its coefficient of t is nonzero. Its unique root is the usual doubling
abscissa x(2Q); Q is not 2-torsion, so 2Q is finite. The z=X factor of
P_X is precisely this linear polynomial, placing that root in T.

For t other than t_star, every z supplied above differs from X, giving at
least two distinct grid coordinates in C_t. At t_star, C_t contains at
least X. Summing these disjoint fibres gives (2).

**Equality consequence.** If |Z|=2e-1, every t other than t_star has exactly
two zero x-coordinates and t_star has exactly one. Every observed zero is
in the subset (1); the second set in (1) contains exactly one selector
coordinate in every fibre. This is an equality argument from the lower
bound, not a random-target or generic-curve claim. [pcc:unverified]

## Prediction on the retained Prime31 selector fixture

The preceding [target-quotient report](../target_quotient/REPORT.md) records,
for its fixed p=2147483647, A=847514846, B=2087444724, d=63 and target X=1,
the observations e=67 and |Z|=133. Those original bytes remain unchanged.
Since 133=2*67-1, the equality consequence predicts the following checks
on the same fixture:

- Every H-zero pair satisfies x=X or S3(x,X,t)=0.
- The unique degree-one gcd fibre is t=x(2Q).
- The sum of deg(q_i) over certificate leaves with deg_x(D_i)=1 is 1.
- The sum of deg(q_i) over certificate leaves with deg_x(D_i)=2 is 66.
- No positive leaf has another gcd degree, and the degree-product support
  count is 1*1+66*2=133.

Dynamic evaluation may refine the degree-two fibres into several q_i.
The prediction is about their **aggregate modulus degrees**, not a demand
that the algorithm emit exactly two blocks. Its split decisions can depend
on intermediate leading coefficients and can legitimately over-refine the
coarsest degree partition.

The reference e and support values above are earlier finite-fixture records,
not a newly executed replay in this note. Every listed item remains a
prospective check until linked to the new immutable-source output.
[pcc:unverified]

## Interpretation limit: coordinates do not determine useful rows

For rationally lifted selector coordinates, cancellation triples such as
Q+P-P=Q and -P+Q+P=Q explain the subset (1). Their first partial sums have
abscissae x(Q+P) or x(Q-P). The case P=Q leaves x(2Q), while the cancelling
partial sum O is outside this affine grid and must be handled separately.

An absence of extra coordinate pairs does **not** establish that every
signed relation above those pairs is a cancellation. Distinct eliminated
coordinates, signs or repeated-leaf witnesses may project to the same pair.
For example, if Q=3P and both abscissae lie in the selector, the pair
(x(P),x(2P)) can support P+P+P=Q and also a cancellation explanation using
-P+Q+P=Q. The coordinate test cannot distinguish those signed rows.

Accordingly, the new classifier records `cancellation_coordinate_pairs`
and `other_coordinate_pairs` (or equivalently named fields), separately
from the certificate's support count. It must not label the first count
trivial relations, set useful relation yield to zero from this predicate,
or discard a projected pair before the signed extractor has recovered all
required witnesses and reconciled duplicates and rank.

No new independence-rank claim or complete signed-relation conclusion is
part of this prediction. [pcc:unverified]
