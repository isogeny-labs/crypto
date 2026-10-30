# Prime m2 first-fall certificates and strict filtered witnesses

Status: proposed argument and implementation contract, before affected production
data. All six retained prime m2 original/baseline/spike40 planted/natural inputs
are required. No target, equation, point manifest or selector changes. This stage
addresses a remaining prime-field gap in the full n83 three-family m2–9 goal;
it does not replace the pending binary quartic frontier or higher-m obligations.

## Primary sources checked

- Caminata–Gorla, https://arxiv.org/html/2112.05579, Definitions 1.3 and 1.5,
  Section 2 and Section 4: first fall is nontrivial highest-form syzygy degree;
  last fall requires filtered-space membership. A highest-form syzygy alone
  does not prove a strict affine fall. Keep both certificates separate.
- Phuong–Tran, https://arxiv.org/pdf/2211.13548, Theorem 2.7 and its proof:
  multiplication by powers of the sum of variables has maximal rank in the
  square-zero algebra in characteristic greater than its variable count.
- HILBERT_DESIGN.md and frozen src/bin/hilbert_v2.rs: complete source validation,
  invertible diagonal normalization, primality and inclusion-matrix rank
  certificates for these exact six saved systems. Reuse unchanged v2 donor bytes, including its required planted-witness suffix.
  HILBERT_AMENDMENT_V2.md records why the earlier v1 template is insufficient.

## Graded certificate

Let ell=2r+1, with ell=27 and r=13 in production. Set z_i=w_i*v_i and
u_i=w_i*v_(ell+i), with all w_i nonzero in Fp. Then normalized selector
equations are z_i^2-w_i*z_i and u_i^2-w_i*u_i. Put X=sum z_i and Y=sum u_i.
The remaining source equation is f=S3(X,Y,t), with curve coefficients a,b.
Its degree-four part is F=X^2*Y^2. Its degree-three part is
-2t*(X^2*Y+X*Y^2). All other terms have degree at most two.

In A=Fp[z]/(z_i^2), multiplication by X^2 from A_k to A_(k+2) is injective
for k<=r-1: maximal rank and C(ell,k)<=C(ell,k+2). The same holds in the u
block. Therefore multiplication by F is injective in every total multiplier
degree <=r-1 in A tensor B, using its direct bidegree decomposition.

At multiplier degree r, take

    H(z) = product_(j=0..r-1) (z_(2j)-z_(2j+1)),
    q=z_(2r).

In the square-zero algebra each paired sum kills its paired difference, hence
H*X=H*q and H*X^2=0. H is nonzero: its 2^r squarefree monomials are distinct.
This is a nontrivial annihilator at total relation degree D=r+4.

To pass from this annihilator to first-fall degree, use that all variable
squares form a regular sequence. Modulo Koszul syzygies, a syzygy of these
squares together with F is determined by its F-coordinate in Ann_(A tensor B)(F).
If that coordinate is zero modulo the square ideal, subtract Koszul syzygies
with F; the remaining syzygy of the squares is Koszul. Conversely every
annihilator lifts to such a syzygy. Thus no nontrivial syzygy precedes D and H
supplies one at D. The characteristic-p quotient in Definition 1.3 has no
effect in degrees <=D<p; its additional Frobenius trivial syzygies also have
higher degree. The predicted exact highest-form FFD is D, not a global LFD.

## A separate strict affine witness

Let NF denote reduction by the normalized selector equations and put

    A0=H, A1=NF(H*X), A2=NF(H*X^2), g=NF(H*f).

These are ordinary polynomial consequences with derivations of degree <=D.
Exact multiplication shows deg A0=r, deg A1<=r+1, deg A2<=r+1. Therefore
deg g<=r+3=D-1. Store g as the following exact sum of tensor products:

    A2 * (Y^2 - 2tY + t^2)
  + A1 * (-2tY^2 + (-2t^2-2a)Y + (-2at-4b))
  + A0 * (t^2Y^2 + (-2at-4b)Y + (a^2-4bt)).

The Y polynomials also use their selector normal forms. This is an exact
reconstructible polynomial representation; no term is dropped or sampled.
Retain the fully expanded A0,A1,A2 factors and the three small B factors.

Let S=sum w_i. In the square-zero z algebra, coefficient pairing with H gives

    [z_0...z_(ell-1)] H*(A1)_(r+1) = (-2)^r,
    [z_0...z_(ell-1)] H*(A2)_(r+1) = (-2)^r*S.

For the second identity, write P_j=z_(2j)+z_(2j+1), H_j=z_(2j)-z_(2j+1).
NF(H_j*P_j)=w_(2j)z_(2j)-w_(2j+1)z_(2j+1). In the pairing, the two-variable
term from NF(H_j*P_j^2) vanishes when multiplied by H_j. Cross terms between
distinct pairs have degree r and do not contribute. The unpaired square gives
w_(2r)*q*H; each paired/unpaired cross term gives its pair's weight sum.
Together they give S*(-2)^r. Verify both pairings directly from the saved
sparse A factors, separately from this closed-form evaluation.

Define lambda on degree-(D-1) normal forms as the coefficient of
z_0...z_(ell-1)*u_0*u_1 in H times that homogeneous part. Then

    lambda(g_(D-1)) = 2*(-2)^r*(S-2t).

Lambda vanishes on F times every homogeneous multiplier of degree r-1,
because H*X^2=0 in the square-zero algebra. The factor 2 is the coefficient
of u_0*u_1 in Y^2. If S-2t is nonzero, the displayed coefficient certifies
deg g=D-1 and nonmembership in the degree-(D-1) leading image.

For completeness of the lower filtered space, injectivity above implies that
NF(q*f) has degree deg(q)+4 for every nonzero squarefree q of degree <=r-1.
Thus the span of those products with deg q<=d-4, together with the selector
ideal truncated at d, is closed under every permitted degree-bounded product
for d<=D-1. It contains the original generators and equals V_d. This uses the
monic selector Groebner basis to reduce without increasing degree. Hence the
nonzero pairing proves g in V_D but g not in V_(D-1), and yields LFD>=D.

When S-2t=0, record the zero pairing and mark this strict-witness test
inconclusive. Do not turn it into a lower bound or conclude that no other
witness exists. The graded FFD certificate is independent of this exception.
Global LFD upper and exact LFD remain null without another certificate.

## Native implementation and falsification controls

Use Rust for all arithmetic and polynomial computations. Sparse factors use
u64 squarefree masks and canonical u128 prime-field coefficients; multiplication
reduces z_i^2=w_i*z_i. Direct coefficient pairing uses complementary masks.
Reuse the frozen donor's primality, full-source and rank checks without editing
it. Replay must reconstruct every coefficient and reject altered sources,
weights, masks, coefficients, rank certificates and verdicts.

Before production, independently expand the full two-block S3 polynomial and
H*f for small odd ell, compare against the compact tensor reconstruction, and
perform dense filtered V_(D-1)/V_D closure checks where feasible. Include varied
nonzero weights, both signs, varied a,b,t, exceptional t=S/2, small-characteristic
or zero-weight rejection, wrong source templates and deliberately corrupted
saved factors. Compare the rank-based injectivity against explicit matrices.
Controls must find the positive strict falls and leave the exceptional-target
test inconclusive. A matching zero pairing alone is not a positive control.

Freeze source, protocol, six input/source/base hashes, controls and budgets
before production. Fresh-process and S3-restored replay are required. Archive
all factors, circuits, inputs, outputs, errors and partial attempts; document
byte layout and reconstruction. No performance, elite-base, independent-review
or complete algebraic-formalization claim follows from this producer dataset.
