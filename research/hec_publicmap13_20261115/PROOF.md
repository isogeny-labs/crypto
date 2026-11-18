# Extending a difference-surface map to the public prime subgroup

Let psi:J->Jac(Q1) be the constructed isogeny, G=<D1> of prime order
ell=1028893, T the registered rational2torsion point, and epsilon=±1 the
single global sign fixed by the source-jet differential scale lambda10.
For a supplied nonzero public divisor D in G, scan x=[n]D until the existing
section-centered-at-psi(T) decoder succeeds. It returns
Q=epsilon*psi([n]D-T), as an actual oriented divisor class on Q1. Then

M(D)=[(2n)^(-1) mod ell]([2]Q)=epsilon*psi(D).

Indeed2T=0. The scalar2n is known to the mapper and nonzero mod ell for
1<=n<=8192. No logarithm of D relative to D1 occurs in this identity or in
its implementation. One obtains the same signed homomorphism for every
successful input; phi preserves DLP equations since ell is coprime to27,
the degree of psi. Identity maps to identity without scanning. Failure to
find an admissible zero inside a bound is a censored transport, not a zero
image or proof of nonexistence. This gate aborts explicitly on that cap.

This avoids solving two simultaneous difference-surface membership conditions
for a decomposition of an arbitrary public divisor. It does not assert the
scan is constant time or prove a uniform success bound. A random-point heuristic
suggests order q trials, since a divisor inside a threefold has density order1/q;
the actual deterministic cyclic scan may correlate. All trials/exceptional
charts, setup, source arithmetic, target arithmetic and verification must be
charged before a performance comparison. Genus2/4 need their own constructions.

## Why a single endpoint scale orients all cases

The canonical curve reconstructed from the fourth-order Gamma00 jet is embedded
in projectivized source tangent space. Thus the true isogeny differential, relative
to the canonical differentials (x,y,1)dx/F_y of the normalized plane quartic,
is a fixed scalar matrix lambda*I. The defining section's local frame may vary
with x, but its extra Hessian terms have a gradient factor and vanish on tangent
vectors. Therefore, at the decoded ordered pair P,Q,
H(P,P)/[-(n_x F_y(P)-n_y F_x(P))]
=H(Q,Q)/(n_x F_y(Q)-n_y F_x(Q))=lambda.
Reversing the pair negates this ratio. Sourcejet13 observed lambda10 or117
on five different source zeros and their negatives, fixing lambda10 once up
to a common global sign. The public mapper uses that fixed value; it never
fits an output to a known scalar, label or expected group identity.

## Scope of the verification

The independent public divisor B=[2](3,21) is constructed geometrically, without
computing its scalar relative to D1. Map D1,B,D1+B,2B,-B and [1729]D1 in separate
scans. The last control's scalar is available only to the outer checker, not
the mapping routine; no label-dependent branch exists. Compare addition,
doubling, negation and this withheld scalar relation afterward. This verifies
specific outputs and group identities, not independent implementation or a
universal pointwise theorem by exhaustive enumeration. The algebra above is
what extends the restriction to arbitrary successful prime-subgroup inputs.

Prior work: sourcejet13 actual derivatives, difference13 second-jet uniqueness,
geometry13 same-field identification and two-torsion translation. Tian
https://arxiv.org/html/2007.03172 §4.2, Remark4.3 is the established isogeny
transport/sign reference. Scalar division in a known prime-order group is
standard; no novelty assertion. KB query 'isogeny evaluate multiple scalar
inversion restricted divisor map', revision9141e386254340a9bdba3036f2bb706a,
found unrelated elliptic-isogeny notes, not direct evidence about this mechanism.
