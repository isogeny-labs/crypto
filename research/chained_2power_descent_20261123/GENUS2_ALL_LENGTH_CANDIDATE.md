# Characteristic layers: a candidate all-length genus-two reduction

Standing: a written derivation over standard finite étale torsion and
polarized quotient facts. No shared Lean assembly, geometric morphism
implementation, independent signed verdict or computational advantage has
been earned. It extends the narrower irreducibility argument in the retained
CT331 note; neither that note nor its handoff is changed. [pcc:unverified]

## Statement and hypotheses

Let k have characteristic different from2. Let A,B be principally polarized
abelian surfaces over k, with A(k)[2]=0. Suppose f:A→B is k-defined and
f*lambda_B=2^n lambda_A, n>=0. Then the geometric kernel has abstract type
(C_(2^a))^2 x (C_(2^b))^2, with a>=b>=0 and a+b=n. It contains A[2^b], so
f=h composed with [2^b], where h is a k-defined 2^(a-b)-polarized isogeny
with kernel (C_(2^(a-b)))^2. The proposed conclusion is that h factors into
a-b rational (2,2) principally polarized quotients. These intermediates may
be products or Weil restrictions; a chain wholly of genus-two Jacobians
needs the usual indecomposability check at each step. [pcc:unverified]

## Layer argument, independently derived

Write K=ker f on separable-closure points. Since char(k)!=2, all relevant
torsion is finite étale and the polarized degree gives |K|=2^(2n). Define
W_j=(2^(j-1)K)[2] inside A[2]. These are characteristic, hence Galois-stable,
F2 subspaces. The two-Weil pairing is perfect and Galois-invariant: its
values lie in mu2, on which Galois acts trivially. [pcc:unverified]

A stable line in A[2] would have its unique nonzero vector k-rational,
contradicting A(k)[2]=0. A stable hyperplane would have a stable perpendicular
line and is excluded too. In dimension4 this leaves dim W_j in {0,2,4}.
For invariant factors C_(2^e_i), dim W_j counts the e_i>=j; successive
differences count the multiplicity of exponent j. Every positive exponent
therefore has even multiplicity. Pad by zero exponents to four factors:
(e1,e2,e3,e4)=(a,a,b,b), and |K| gives a+b=n. [pcc:unverified]

This uses arbitrary Galois stability, not a cyclic finite-field Frobenius
assumption. It does not equate absence of rational two-torsion with
irreducibility. In particular it allows the reducible3+3 and six-cycle
sectors to retain rational first Richelot kernels. [pcc:unverified]

## Scalar factor and rational quotient flag

The polarized kernel is maximal isotropic for e_(2^n), hence K=K^perp by
perfectness and its size. Its exponent is2^a, so K lies in A[2^a]. The
orthogonal of A[2^a] inside A[2^n] is A[2^(n-a)]=A[2^b]; consequently
A[2^b] lies in K. This avoids assuming an arbitrary abstract group
isomorphism respects its embedding in the ambient torsion. [pcc:unverified]

The universal property of [2^b] factors f uniquely as h composed with[2^b]
over k. Pullback of polarizations by [2^b] multiplies their classes by2^(2b),
so h is 2^(n-2b)=2^(a-b)-polarized. Its kernel is [2^b]K, of type
(C_(2^(a-b)))^2. This step uses standard quotient/polarization geometry;
it is not formalized by the earlier custom Bool operations. [pcc:unverified]

For m=a-b>0, the primitive free kernel K' of h has characteristic two-torsion
K'[2]=2^(m-1)K', a stable rank2 subgroup of A[2]. Weil-pairing compatibility
makes it two-isotropic: e2(2^(m-1)x,2^(m-1)y)=e_(2^m)(x,y)^(2^(m-1))=1
for x,y in K'. Quotient by this rational maximal isotropic subgroup gives a
principally polarized surface; the residual isogeny is2^(m-1)-polarized
with kernel (C_(2^(m-1)))^2. Induction yields the proposed rational flag.
If m=0, h has degree1 and is an isomorphism. [pcc:unverified]

## Independent check of the retained irreducible sector

The CT331 induction is consistent with a shorter exponent induction.
For any nonzero finite stable2-primary H, irreducibility forces H[2]=A[2].
Then H=[2]^-1([2]H): both inclusions follow from the full kernel of[2]
being inside H. Induct on the exponent of H, not an unspecified ambient
bound, to get H=A[2^r]. The finite étale quotient then gives
f=u composed with[2^r] with u a k-isomorphism. This argument does not need
finite k or a polarization until the Richelot-chain degree comparison.
In genus2,4^h=16^r gives h=2r for h geometric(2,2) steps. No destination
model evaluation or same-isomorphism-class speed claim follows.
[pcc:unverified]

## Boundaries and next discriminating unit

The dimension4 perpendicular-line step does not exclude stable3-spaces in
dimension6. Even if a particular genus-three Frobenius has only even-
dimensional stable subspaces, a maximal4-isotropic group can still have
invariant factors(4,4,2,2), padded by two zero exponents. It need not contain
full A[2] or admit a rational rank3 first kernel. The separately frozen
Rust unit investigates one explicit product-PPAV module of this form.
It must not be reported as an indecomposable genus-three Jacobian transfer.
[pcc:unverified]

Sources/scope: Magma Richelot handbook text1615 paragraphs31-32 and55-57
(state the scalar/product qualifications explicitly); Bruin--Doerksen
arXiv0902.3480 for polarization/kernel context; pinned Mathlib orthogonal
source cfc55fc for the linear-algebra interface; CT331 at75ab470f; two-hop
source/qualified results in4992. Smith arXiv0806.2995v2 §9 distinguishes a
rational polarized quotient from a genus-three curve/isogeny model over
the base field, including its possible quadratic descent obstruction.
No complete novelty search or signed theorem acceptance is claimed.
[pcc:unverified]
