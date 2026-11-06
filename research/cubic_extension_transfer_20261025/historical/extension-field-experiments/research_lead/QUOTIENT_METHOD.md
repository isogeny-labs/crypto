# From the one-step torsion screen to explicit quartic targets

**Historical construction-stage note:** see [the current results](NEXT_RESULTS.md).
The prepared-curve sufficient condition below now has a counterexample, and
generic polynomial divisor conversion plus basic target index calculus have
been completed on two small-field examples.

This continues the earlier eligibility experiments. The implemented new stage
constructs quotients for newly eligible inputs, rather than replaying Tian's
supplied numerical map. See [the generated results](QUOTIENT_RESULTS.md) for
counts, equations, certificates, and the remaining boundary.

## Scope of the mathematical claim

Let p >= 5 be prime, K=F_(p^3), and let E/K be ordinary of prime order r>3.
The earlier arithmetic argument shows that p=1 mod 3 and r=4 mod 9 are exactly
the gate for reaching scalar -I Frobenius on E[3] with at most one rational
3-isogeny. This gate concerns the specified auxiliary-3 transverse kernels.
It is not a criterion for all possible ECDLP attacks.

For the prepared curve, the exact F_3 module enumeration supplies 12 rational
maximal-isotropic transverse kernels Lambda in its cubic Weil restriction W.
Set A=W/Lambda with its descended principal polarization. In the experiments,
the theta data and odd gradients reconstruct the geometric curve associated
with A, and transport its canonical equation directly into base-field
coordinates. A separate obstruction check is essential: A can be the
quadratic twist of that curve's Jacobian over F_p.

The elliptic preprocessing isogeny is injective on the r-subgroup, and the
quotient isogeny W->A has kernel order 27, coprime to r. Thus a usable quotient
preserves the target subgroup. These degree arguments do not replace the
missing implementation that converts general quotient images into divisors
on the constructed quartic.

## A small exact representation of the quotient theta section

For a three-torsion point P on y^2=x^3+a4*x+a6, write

    s_P = (3*x_P^2+a4)/(2*y_P),
    l_P(x,y) = y + y_P + s_P*(x-x_P).

The tangent line at -P has divisor 3[-P]-3[O]. Normalize these nine sections
using Tian's normal-Weil-set algorithm so their cocycle is e_3(P,Q)^2.
The code verifies all 81 cocycle identities at independent evaluation points.

The three elliptic factors each have H^0(3O) basis (1,x,y). Therefore their
product has a 27-dimensional section space. After shifting by the chosen
two-torsion Q3 on each factor, the desired invariant theta section is a tensor
in this space. Its coefficients come from the group-trace projector

    sum_(P in Lambda) rho(P).

The invariant subspace has dimension one. Averaging the constant section can
give zero. The implementation tries columns of the projector until it finds a
nonzero one; a zero average is not classified as a bad quotient.

For level-two normal functions g_P, the quotient functions are

    h_P(R) = g_P(R)^3 * theta(R+P)^2/theta(R)^2.

They are already normal because the new cocycle is d_2^3=d_2. This avoids
another normalization of 64 functions. Exact pole/zero cancellation is used
where the evaluation point maps to O on an elliptic factor.

## Odd gradients give the quartic in the right differential coordinates

At each of the 28 odd two-torsion characteristics, differentiate the local
regularized theta section. At a finite two-torsion x-coordinate e, the
invariant differential dx/(2y) gives

    D(1,x,y) = (0,0,3*e^2+a4).

At O, with t=-x/y, the regularized basis has value (0,0,-1) and derivative
(0,1,0). Tensor contraction supplies every odd gradient exactly.

For a non-hyperelliptic genus-three Jacobian these gradients are its canonical
bitangent lines. They lie in the cotangent space of W via the separable
quotient map. An Aronhold system with characteristic indices

    63, 38, 52, 41, 13, 26, 19

gives a quartic by Riemann's formula. Every one of the 28 lines is independently
checked: its quartic intersection polynomial must have even root
multiplicities, including the point at infinity. Global smoothness is also
checked. The output is a plane quartic, hence genus three and non-hyperelliptic.

The cotangent coordinates are useful for descent. If z generates K/F_p, form

    B_(i,j) = (z^j)^(p^i),  0<=i,j<=2.

This is the change from an F_p basis of the Weil-restriction tangent space to
the three conjugate elliptic coordinates. Substitute Z=B*Z_base into the
reconstructed equation and normalize by a nonzero coefficient. The code
checks that every resulting coefficient is fixed by p-Frobenius, then coerces
it to F_p. This implementation does not reconstruct from Dixmier-Ohno
invariants or search for a projective isomorphism after reconstruction.

This is a derivation from the standard Gauss-map/odd-theta description. No
priority claim is made for this differential-coordinate descent technique.

## A direct obstruction screen using the same tensor

The known chi_18 criterion distinguishes a non-hyperelliptic Jacobian from its
obstructed quadratic twist. Jacobi's derivative formula on the special
fundamental system

    odd:  63,38,52
    even: 0,4,32,55,62

expresses the determinant of the three odd gradients as pi^3 times the product
of the five even theta constants, up to sign. In a rational differential basis,
the resulting expression is, up to a fixed nonzero square factor,

    X = (product of the 36 even theta constants)
        * (det(three odd gradients) / product(five even theta constants))^18.

The usual modular-form conventions give the square factor 2^26. Its value is
irrelevant to the square class. The construction eliminates square-root
choices algebraically: each single-factor normalizing constant appears an
even number of times across the 36 even characteristics. If the three elliptic
level-two constants are a,b,c, their contribution is a^15*b^15*c^9 per factor.
The gradient-determinant and denominator powers likewise involve integer
powers only. The rational differential-basis change contributes det(B)^18.

The code verifies X is nonzero and lies in F_p. Its quadratic character predicts
whether A is the base-field Jacobian. For validation, every predicted result in
the original 23-input sweep is matched to its independently certified quartic
by the normalized equation, not by an arbitrary torsion-basis label.

Algebraic evaluation of Serre's obstruction is established prior work. The
potential practical contribution here is this specialization to the small
cyclic Weil-restriction construction, together with the closed eligibility
screen and automatic 12-kernel search.

## Independent Jacobian-order certificates on the small-field sweep

For each reconstructed curve, count points over F_p and F_(p^2). The observed
counts N1=p+1 and N2=p^2+1 force

    L(T) = 1 + a3*T^3 + p^3*T^6,
    #J(F_p) = p^3+1+a3,
    |a3| <= 2*sqrt(p^3).

Compute exact orders of rational-point divisor differences in the actual
Jacobian, then take their least common multiple d. If the interval above
contains exactly one multiple of d, that multiple is the certified group order.
It is r in the usable case and 2*(p^3+1)-r in the obstructed case. The
certificates save the point counts, interval, and witness divisor orders.

SageMath 10.6 Hess points have a representation-sensitive generic zero test in
some cases. The implementation uses divisor-class equality with G.zero(). A
standalone regression independently counts points in three extensions and
records the discrepancy; this recovered implementation issue is not an
arithmetic counterexample to the construction.

## What the point-map audit establishes

`theta_point_probe.py` evaluates the newly constructed quotient in the 64
projective level-two Weil-function coordinates. These coordinates identify
opposite points. On the first p=7 input, all 27 kernel translations are checked,
and all r=373 subgroup points are enumerated. The expected 187 distinct images
establish faithfulness modulo sign in that toy example.

This is not a conversion to divisor classes on the reconstructed quartic.
Tian's general image-inversion step, sign recovery, and an actual target
index-calculus solver remain to be implemented and benchmarked. No new
end-to-end ECDLP speedup is claimed.

## Prior work and the plausible research contribution

- [Tian, 2020 preprint / 2023 publication](https://arxiv.org/abs/2012.07173)
  supplies the normal-Weil-set quotient and image-inversion framework. The
  present implementation specializes its elliptic gluing to ell=3.
- [Galbraith-Hess-Smart, 2002](https://eprint.iacr.org/2001/054) already uses
  isogenies before descent. Preprocessing by an isogeny is not new.
- [Ritzenthaler, 2010](https://arxiv.org/abs/0901.2920) and
  [Lachaud-Ritzenthaler-Zykin, 2010](https://gaati.org/zykin/zykin/2010_Klein.pdf)
  establish and compute Serre's obstruction via modular forms.
- [Kirschmer-Narbonne-Ritzenthaler-Robert, 2022](https://arxiv.org/abs/2004.08315)
  gives algebraic theta/modular-form evaluation and Serre-obstruction
  computation for polarized varieties isogenous to powers of an elliptic
  curve. This is substantial overlap with any broad novelty claim about
  algebraic obstruction screening.
- [Grushevsky-Salvati Manni, Gradients of odd theta functions](https://arxiv.org/abs/math/0310085)
  is background for gradient descriptions; the bitangent/Gauss-map connection
  is classical, not a newly discovered principle here.

The defensible prospective contribution is a concrete, reproducible automatic
pipeline and coverage study for prime-order cubic inputs, with a cheap one-step
screen, exactly 12 candidates, direct differential-coordinate descent, and
explicit accounting for the obstruction. Novelty of this exact combination
still needs expert/literature review; the full 2023 journal text of Tian was
not available in the access checked earlier.

The sharper open question, within the sampled scope of ordinary prime-order
curves with j not in F_p, is whether an eligible prepared curve must have at
least one unobstructed member among the 12, or whether additional isogeny steps
are sometimes necessary. The bounded coverage search tests that question but
does not prove a universal statement. Special j-invariants were excluded and
are not covered by the experimental evidence.

The follow-up separates two interpretations of that conjecture:

1. Every curve already having scalar -I three-torsion immediately has a usable
   kernel among the twelve. This is **false**, by the p=109 example in
   [COUNTEREXAMPLE.md](COUNTEREXAMPLE.md).
2. Under the stated prime-order congruences, there exists some freely chosen
   isogeny of degree one or three reaching a prepared model with a usable
   kernel. This weaker existential statement remains open here. The bad
   prepared model is repaired by one step and does not refute this version.

No priority claim is made for either formulation. The arithmetic gate still
correctly characterizes the specified torsion eligibility; it is not by itself
a sufficient stopping condition for the full construction.
