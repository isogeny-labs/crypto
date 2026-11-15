# Prospective native auxiliary projective-tower correctness qualification

Freeze before native implementation/calls. Goal remains the complete classical
subexponential prime-field algorithm. This is a fixed software encoding panel,
not a cost/yield experiment, exponent test or general construction benchmark.
PCC-20261115-be28ec remains unverified. [pcc:unverified]

Use main E/F19:y²=x³+2, auxiliary A/F19:y²=x³+2x+2, T=(16,11), G=(12,5).
Planned order(T)=8. Build the three standard Vélu2 steps from successive
order2 kernel points, retaining homogeneous coordinates
N(P,Q)=P²-xi*P*Q+tau*Q², D(P,Q)=P*Q-xi*Q². Proposed constants are
(14,1),(16,5),(17,3), terminal c=15. Verify those values; a failed check stops
the panel, never changes the fixture. Neither8 divides18 nor8 divides20, so
this tests a prime outside the corresponding Kummer/torus divisibility cases.

Reuse canonical native polynomial arithmetic, FLINT roots/quotient extraction,
the norm polynomial helpers, capped backend adapter and independent group
arithmetic. A small deferred norm-builder constructor is permitted to reuse
helpers without constructing irrelevant Kummer equations. Preserve the old
constructor/default CLI semantics and validate their structural selftests.

Use the unit-leading graph substitutions for a0 and monic support U directly.
Keep pairs of numerator/denominator remainders modulo U. Public pre-wrap
layers are substituted exactly; first-wrap quotient degree bounds use their
actual public degrees. Terminal membership is N-cD=0 modulo U. Separate
denominator inverse witnesses are omitted only under the shared projective
coprimality law; native source binding is still checked by the actual circuit.
The proposed m3,t3 shape is14 variables, degree at most3; verify it.

Exactly one qualification001, fresh write-once directory:

1. Native structural controls: curve/point admission, exact order8, each kernel
   and codomain, all fixed auxiliary points under every step, exact8-element
   coset fiber, no repeated roots/poles, trace test2phi(G)!=O, public composed
   numerator-c*denominator equals its monic root product. Retain all values.
   Pole and tau0/invalid-point controls must reject. Reference field scans
   belong only to this finite software harness, never the norm relation oracle.
2. Compare every nonzero rational target on the fixed main E (expected12)
   against the bounded independent complete tuple reference. Every target is
   retained, including empty sets. The candidate sees only the public curve,
   fixed tower and target point, no scalar/assignment/reference model.
3. For each of those targets run the pinned native msolve once and the unchanged
   source-certified PC backend once at D4. Both get the existing20s CPU/wall,
   2GiB address, one thread and quotient128 caps. PC's own row/entry/step/proof
   caps remain unchanged. A timeout/error/failed certificate is UNKNOWN; retain
   it and continue the fixed target census. Every completed set must match the
   full reference exactly, with all original equations, main-curve points,
   tower membership and group sums checked. Stop on any disagreement.

Capture source commit, exact arguments and separate source/build/binary
manifests BEFORE calls. Do not overwrite a manifest. Preserve all inputs,
outputs, certificates, errors, exits, unknowns and build failures. Package each
whole raw round deterministically; large evidence uses the existing verified
artifact-store workflow, not shards. No adaptive target/cap/retry or backend
schedule change, no scientific clocks, no paid job and no cold DLP claim are
authorized by this panel. Independent OpenSSL replay of complete model sets
requires a separate compiled replay interface and must use the same fixture.

Formal evidence: the shared CommRing/IsCoprime step law is being built before
the short AdjoinRoot(F19) endpoint. Its assumptions are actual unit parameters,
not the desired denominator-unit conclusion. Do not report it as a proof of
the native parser, whole isogeny construction or algorithmic complexity.

General-prime auxiliary curve search/point counting, large-prime arithmetic,
admissible growing-family degree/yield and full cold cost remain unimplemented
here. Existing supporting auxiliary probability bounds are reused as stated,
not newly claimed or inferred from these tiny fixtures. [pcc:unverified]
