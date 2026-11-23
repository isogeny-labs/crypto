# PROTOCOL (frozen before any data) - d_NS at m = 3, Kummer towers

Date 2026-11-15. Status of every number produced: [pcc:unverified].

## Question
d_NS = least c with 1 in span_{F_p}{ mu*F_i : mu = y^S u^e normal-form monomial,
popcount(S)+e+deg F_i <= c, i = 1,2 } in R (m = 3 chain: F1 = S3(x1,x2,u), F2 = S3(u,x3,x_R)).
deg F_i is measured from the group-algebra expansion (expected 4 and 3), not assumed.

## Preregistered prediction
H0: d_NS = 3t+5 for every instance (D9 tight for no-fall certificates).
H1: d_NS < 3t+5 on at least one instance.
Decision rule: any instance whose exact computed d_NS (under the stated u-degree cap) is < 3t+5
=> H1 reported for that cap; H0 "held" only if all instances give exactly 3t+5.
Cap caveat fixed in advance: the full definition allows u-degree e up to c - deg F_i. Dense
elimination at that size is infeasible at t >= 3 under 2 cores / 4 GB, so d_NS is computed as
d_NS(E) = same definition with u-degree of mu <= E. d_NS(E) is non-increasing in E and
d_NS(full) <= d_NS(E). Only d_NS(E) for the stated E is claimed. E = full at t = 2; E in {1,2,3(,4)} at t = 3;
E = 1 only at t = 4, 5.
For E = 1 the certificate is unique pointwise (4x4 Sylvester system per grid point, nonsingular iff gcd = 1
when x_R is not in g*mu_K), so d_NS(1) is computed exactly by pointwise Bezout solve + inverse DFT over (Z/K)^3
(popcount degree of coefficient functions), and cross-checked against the rank method at t = 2, 3.

## Instances
Primes p in {40961, 12289}; t in {2,3,4}; t = 5 only for the E=1 unique route (cheap); 3 instances per (p,t).
Seed = int(sha256(f"kummer_m3|{p}|{t}|{idx}")[:8],16); all draws from random.Random(seed) in order:
a, b (disc 4a^3+27b^2 != 0), g in [1,p-1], x_R in [0,p-1]. Reject (redraw from the same stream) if
x_R in g*mu_K, or any grid point of V^3 gives a common root of f1(u), f2(u) (det of the 4x4 Sylvester-type matrix = 0).
K-th root of unity: omega = h^((p-1)/K) for the least primitive root h.

## Controls
C1 (positive, m = 2): F = S3(x1,x2,x_R), no u, t = 2..5, same primes, 3 instances each: rank-method d_NS must equal
2 + popdeg(f^-1) (pointwise inverse + inverse DFT) and be 2t+2 (D4). Failure => stop and report.
C2 (negative): planted instance where some grid point has a common root of f1,f2 (found by scanning F_p);
1 must not lie in the span for c = 3t+5 (E=2) at t = 2,3; the gate must flag it; at t=4,5 E=1 route reports singular point.
C3 (representation): t = 2, explicit group-algebra multiplication (dict polynomials mod z_i^K-1, symbolic S3 expansion) of
one mu*F column, then grid evaluation, equals the pointwise column.

## Out of scope / optional
True d_F by the Rust engine: only if build < 15 min; otherwise skipped and stated.
No cloud, no commits, files only in this directory.
