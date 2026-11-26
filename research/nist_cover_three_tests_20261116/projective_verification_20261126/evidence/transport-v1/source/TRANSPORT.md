# Projective hyperplane relation and transport

Frozen after the endpoint panel and before transport implementation/call.
Same user waiver, task T-2, PR #5255 and existing resource limits. No new
PCC registration or performance comparison. Preserve both endpoint attempts.

Use the first retained split vertical fiber H_ref, x=56 with point indices
42,43,44,45 in the fixed primary relation pool. Let H_inf be the complete
quartic intersection with Z=0. The line ratio Z/(X-56Z) has divisor
H_inf-H_ref, so its class must be zero and its elliptic pushforward must sum
to O. This test checks actual projective endpoints and their transport.

Separate OpenSSL worker: rederive the global map identity and complete
infinity factorization; reconstruct each projective endpoint from local
coordinates and sum all Frobenius conjugates with multiplicity. Independently
reconstruct the four finite reference images (including pole handling), check
each against its original recorded image, replay the reference quartic's
entire root product, and require the two elliptic sums equal. Reject a dropped
nonidentity infinity conjugate and a dropped reference intersection factor.

Actual target KM worker: reuse the unchanged GMP target arithmetic and same
85-point evaluation pool. D0=3 H_ref. Encode the degree-12 effective divisor
D=H_inf+2 H_ref inside L(3 D0) by the complete 22-dimensional subspace of
degree-nine homogeneous sections Z*(X-56Z)^2*p6. In affine evaluation these
are (x-56)^2*p6/(x-56)^9. Require dimension 22 and actual km.zero(D)=true.
Add the known nonzero class at the first point of the second split fiber and
require that changed class is nonzero. Retain the complete class space.

The OpenSSL certificate checks the complete two hyperplane intersections;
the GMP worker performs actual target-Jacobian class arithmetic. This does
not implement a general projective Jacobian transpose API.

Snapshot sources, all input bytes, compiler versions and pre/post digests.
One allowed CPU and 1GiB per native invocation; strict builds <=300 seconds,
each worker <=600 seconds. Record both builds, both workers and complete
wrapper costs in a fresh directory. Retain any failure without cap widening.
Sources: preceding PROTOCOL.md, exact quartic_km_v3 arithmetic, original
relation pool/reference convention and standard principal-divisor pushforward.
