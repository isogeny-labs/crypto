# Frozen protocol

## Objects

- Field: `k=GF(2^83)), polynomial modulus
  `z^83+z^45+z^2+z+1).
- Source elliptic curve: `E: y^2+xy=x^3+1).
- Source order: `#E(k)=9671406556923184866742756=4r), where
  `r=2417851639230796216685689`.
- Cover:
  `C: v^2+u^2v=u^7+u`,
  `phi(u,v)=(u^3,uv+1)), degree three.
- Jacobian endomorphism: `e=phi^* phi_*`.
- Normalized r-torsion projector: `p=[3^{-1} mod r]e`.

## Predeclared obligations

1. General pushforward must accept every validated reduced Mumford divisor of
   degree at most three, including nonsplit support.
2. The independently constructed 486-class GF(8) presentation must agree
   pointwise with the residue-field norm.
3. Homomorphism, `phi_*phi^*=[3]`, and `e^2=[3]e` must replay.
4. The normalized projector must be idempotent, its image component must pass
   the image equation, and its complement must push to zero.
5. Small-field point counts must determine the Prym Frobenius polynomial
   before any n=83 factor claim is made.
6. Every order intersection with `r` is computed in the actual integer
   orders, not in the ambient field size.

## Controls and invalidation

- Identity, rational split support, repeated support, and irreducible
  quadratic/cubic support are all in scope.
- Off-curve, nonmonic, wrong-width, oversized, and non-torsion inputs must
  fail structurally.
- A disagreement with any GF(8) table entry invalidates the general norm.
- A nontrivial gcd between `r` and either Prym factor invalidates the
  no-shared-r conclusion.
- Equal Frobenius trace between the source and a Prym elliptic factor
  invalidates the no-isogeny-factor conclusion.
- Projected-relation experiments remain inadmissible as speedup evidence
  unless their complete cost beats `r^(1/2)` and predicts a finite crossover
  against the strong source rho baseline.

## Evidence boundary

Exact arithmetic and exhaustive finite-fixture tests are certificates.
Asymptotic relation costs remain analytical models.  No planted logarithm,
partial exponent, or ambient `q^3` order may be reported as a solved source
DLP.
