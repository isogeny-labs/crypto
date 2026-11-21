# Value-only torsion validation after seed cancellation

Domain is inherited: caller-certified prime p>3 and odd prime r, r!=p,
gcd(r,p-1)=1, nonsingular short Weierstrass curve and canonical finite point.
The new predicate explicitly checks the modulus/order bounds and coprimality,
then uses ValueDivision::new for curve/point/nonzero-y checks and requires
psi_r(Q)=0. It does not certify the primality assumptions.

For odd r coprime to p, multiplication by r is separable of degree r^2.
The nonzero r-torsion consists of simple division-polynomial roots; quotienting
by sign gives simple x-roots because y!=0. The old differential operator is
D=2y*d/dx on an odd-index division polynomial, so Dpsi_r(Q) is nonzero on this
domain. Thus the old derivative-nonzero check is implied mathematically by the
retained predicate and assumptions. Its phase exponent also exists exactly when
gcd(r^2,p-1)=1, equivalent to the retained gcd(r,p-1)=1 check. [pcc:unverified]

Seed cancellation means no phase value is consumed by projected initialization.
We replace phase() with validate()->Result<(),Error> at the base/direct-window/
short-extension call sites and pass the unused unit seed slot to values(). The
projected transport no longer imports the jet module. Its test-only first call
counter now counts subgroup checks, not phase values. The twice-base anchor is
already a subgroup point by construction. All original full-value validation,
geometry, coordinate differences and active-factor nonzero checks remain.
[pcc:unverified]

This removes derivative propagation, inversion of the derivative and phase
exponentiation from the projected path. Value-only evaluation still has its
2y inverse and recurrence work. The full-value reference still computes phases.
The validation cache is currently discarded before direct-factor evaluation;
reuse is the next concrete optimization, not a measured saving here.

Five selected native methods pass. Acceptance agrees with both the old phase
routine and separate affine scalar multiplication on588 point/order cases:
12 accepted and576 rejected, plus7 malformed/unsupported cases. The fields are
5,7,13,17,19,31 on y^2=x^3+2x+1; candidate prime orders3,5,7,11,13,17,19.
The original source fixtures retain499 subgroup rows,100 splits,1375 advances,
450 skipped signs, factor and closure checks. Each invocation filters60 methods.
Claim PCC-20261107-b5cddb is unsigned producer evidence. [pcc:unverified]

The finite acceptance panel is a local predicate test, not an enlarged curve
cohort or independent review. No scalar recovery, timing comparison or cloud
job ran. The original4563-case cohort remains intact.

Sources checked: native phase/geometry/value-only source, predecessor seed
cancellation and Sage division-polynomial review; Andrew Sutherland, MIT18.783
Lecture6 (2017), https://math.mit.edu/classes/18.783/2017/LectureNotes6.pdf,
read2026-11-07, on isogeny kernels, division polynomials and separability of
multiplication. KB unavailable in current tools; prior wrapper attempt lacked
its executable. Standard facts are reused; no novelty-priority claim.
