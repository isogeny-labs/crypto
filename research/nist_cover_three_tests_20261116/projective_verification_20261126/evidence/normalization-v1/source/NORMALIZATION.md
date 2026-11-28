# Rational KM infinity normalization

Frozen before implementation/call. Same user waiver, PR #5255, resource
limits and unchanged previous evidence. This is a Class-to-Class map API,
with no new PCC registrations or performance comparison.

The endpoint/transport phases found the line at infinity to be one
irreducible degree-four closed point Hinf. Thus every Fp-rational divisor's
infinity support is an integer multiple of the complete orbit. The verified
principal relation Hinf-Href permits replacing it by Href.

Implement normalization of any valid effective-degree-12 KM representation
Class W=L(9Href-D), using the original 34 monomials with x exponent <=3.
Reconstruct the unique numerator coefficients of all 22 rows against the
85-point V3 evaluation basis. Check every reconstructed evaluation exactly.
Independently verify the infinity quartic is irreducible with GMP Frobenius
tests and its x^4 coefficient is nonzero. Since its root has degree four and
the reduced numerator's x exponent is <=3, a nonzero highest homogeneous
part cannot vanish there. With simple infinity intersection, its vanishing
order is therefore 9 minus numerator total degree. The common minimum over
W gives infinity multiplicity m=9-max(degrees), in 0..3.

Return the class space obtained by multiplying each evaluated function by
(x-56)^m. Its divisor is D-m*Hinf+m*Href and its class is unchanged; the
principal-function witness is ((X-56Z)/Z)^m. Reconstruct again and require
infinity multiplicity zero. This operates on the input space, without its
generating scalar or a stored point-support decomposition.

Test m=0,1,2,3 on D=m*Hinf+(3-m)*Href. Also test m=1,2 on the nonzero class
represented by D=m*Hinf+(3-m)*Href-Q0+P, with Q0 the first reference point
and P the first point of the second split fiber. Construct their actual
spaces from the unchanged target point class divided by (x-56)^m. Require
detected multiplicity m, normalized canonical space equal to the expected
finite class, zero remaining infinity multiplicity and actual KM equality
before/after. Reject a changed evaluation entry outside V3.

Input contract: valid 22-dimensional Riemann--Roch class spaces in the
fixed KM model; this is not general adversarial subspace validation. The
test covers all possible infinity multiplicities for degree12, plus nonzero
classes. A general projective individual-point transpose remains separate.

Use unchanged GMP KM arithmetic with new coefficient reconstruction and
normalization code. Fresh source/input/binary digests and directory; strict
build <=300s and worker <=600s, one CPU and1GiB each. Retain exact failures,
exits, native diagnostics and complete wrapper/build/worker clocks. Sources:
the independently checked infinity factorization, TRANSPORT.md, actual
quartic_km_v3 source and its monomial/RR input conventions; no novelty claim.
