# Compose quotient classes before projection

Let p be a caller-certified prime with p-1=q*m, q in {2,4}, m odd, and pi(z)=z^m. For nonzero z define its class modulo ker(pi); adjoin a zero element. Two nonzero representatives a,b are equivalent precisely when pi(a)=pi(b). Multiplication, squaring and negation respect this equivalence. Zero testing is representative-independent, and pi(-1)=-1. No addition of classes is defined.

The Euclidean accumulator already supplies a representative A satisfying pi(A)=pi(Res(F,G)): each leading-coefficient exponent is reduced modulo q, while all polynomial remainder calculations stay in F_p. Its terminal exponentiation need not happen inside the accumulator. ResultantClass owns private modulus and representative fields, offers combine/square/negate/is_zero/project, exposes no raw representative accessor or addition, and rejects combining different moduli. Scalar embedding is created through the existing modulus-bound Poly context. Constructor checks projection order; primality is still caller-certified. Its scalar storage is a representative, not a canonical q-element encoding.

For main and optional tail representatives A,B (B=1 if absent), the geometry requires pi(A)*pi(B). It now combines their classes and projects once. For q=4 the previous square-class correction was chi_p(H)=H^((p-1)/2). Since 2m=(p-1)/2, chi_p(H)=pi(H^2). The same class therefore incorporates the square of the H representative before projection. The outer geometry sign is retained; Euclidean reciprocity signs are already included in the resultant representatives.

For q=2 the earlier even-power factors vanished, so no H correction is included. All coordinate-difference and resultant-zero checks remain. No arbitrary projected value is accepted as a full field resultant; compatibility projected_resultant/root_projected_resultant explicitly finalize their classes when used directly.

## Exact call accounting at the geometry boundary

The preceding Euclidean geometry performs 1+[tail]+[q=4] large exponentiations: one main projection, one optional tail projection, and one optional quadratic-character correction. The successor performs one terminal projection after class composition. At q=4 the correction uses a representative square and multiplication. This is a derived local call count, not elapsed-time evidence or a total source-cost comparison. Test-only thread-local instrumentation confirms exactly one projection on each of the 1349 valid synthetic geometry cases and zero on the 1151 early rejections. Full paired validation and other source exponents still cost work.

Euclidean divisions, leading-coefficient inversions, geometry lists and decoder construction are unchanged. The prior audit's explicit sqrt(r)-scale geometry and r-input decoder compiler remain material. Next: batch Euclidean transitions using half-GCD with exact degree-drop/sign tracking, then investigate implicit orbit-product evaluation and compact nonlinear decoder construction.

## Sources and scope

Read the preceding Euclidean derivation and implementation, the geometry residual and synthetic checker. The standard resultant recurrence was reviewed from FLINT: https://flintlib.org/doc/algorithms/polynomials/gcd.html . The quotient composition follows directly from the power homomorphism above. KB unavailable in this session (no loaded tools; preceding wrapper check lacked executable). No novelty claim. Prime fields only; projection orders above four remain rejected. No cloud or recovery comparison; original cohort unchanged.
