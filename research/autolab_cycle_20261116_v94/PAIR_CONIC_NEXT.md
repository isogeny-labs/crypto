# Next hypothesis: enumerate source sums instead of source pairs over prime intervals

This is an unexecuted bounded-experiment proposal, not another C94 experiment or a novelty claim. It complements the measured odd pair-to-third-coordinate backend with a different contract: all ordered source pairs for one fixed third coordinate u.

Let p be odd, f(X)=X^3+AX+B, S=x+y and T=xy. Direct expansion of the original S3 gives

    S3 = T^2 - 2(uS+2u^2+A)T + u^2 S^2 - 2uAS - 4BS + A^2 - 4uB.

Consequently, over the field,

    S3=0 iff (T-uS-2u^2-A)^2 = 4 f(u)(S+u).

This identity divides by nothing and remains valid in characteristic three and when f(u)=0. For each fixed sum S, every possible product is therefore

    T = uS+2u^2+A +/- 2 sqrt(f(u)(S+u)).

A nonsquare right-hand radicand supplies no product, zero supplies one distinct product. A zero f(u) supplies the single product uS+2u^2+A for EVERY sum; it is not a deleted torsion branch.

For the original prime-field source interval D={0,...,M}, assume 2M<p and M^2<p. Each actual integer sum and product has a unique field encoding. Enumerate the 2M+1 integer sums and at most two products per sum. Reject a canonical product T>M^2. Set the integer discriminant Dlt=S^2-4T. The product is supported exactly when Dlt is a nonnegative perfect square r^2, S-r is even, and both (S-r)/2 and (S+r)/2 lie in [0,M]. Emit both ordered pairs, deduplicating equality. These tests are necessary and sufficient because they exactly factor the monic INTEGER polynomial Z^2-SZ+T. Re-evaluate original S3 before serialization. Thus no scan of D^2, curve-point rationality assumption or loss of repeated/twist polynomial branches is needed. If the no-wrap bounds fail, return unsupported/unresolved pending a separately proved shared-wrap variant.

The source enumeration changes from (M+1)^2 ordered pairs to 2M+1 sums, with O(M) field square roots and exact integer square tests. Memory can be output-linear with streaming sums. This is a genuine operation-count hypothesis for fixed-target interval pairs, not a solution of the five-source outer search: M can still be astronomically large at requested cryptographic dimensions. The binary prefix solver uses a different linearization, consistent with field-specific backends.

Before proposing this route, C94 read research/nist_pair_norm_fiber_v1/PROOF.md: that packet already uses classical symmetric/norm coordinates and exact INTEGER source support, but treats the three-source quartic norm and bounded cubic support. No claim that symmetric coordinates or this conic are new is justified. The falsifiable new implementation question is whether the fixed-target sum scan beats a strong per-source quadratic scan, after all field setup, integer support, complete ordering recovery, verification and output costs.

Next fresh cycle: prove/check the identity by independent literal S3 on every x,y,u over small odd primes, exercise f(u)=0 and characteristic three, and compare the exact all-pair output with exhaustive interval enumeration. Freeze all five NIST fields, M=31/127/511, three source-independent fixed u controls and deterministic planted targets before timing. Keep every empty/positive case. First 90-second component, 4 GiB, one core; cap sums and return unresolved if exceeded. Compare against direct x scan with constrained quadratic recovery AND matched genuine F4/F5/F6 where supported. This proposal has no timing or memory measurements and does not discharge the hard-instance gate.
