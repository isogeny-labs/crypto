# Joint source-orbit blocks

Freeze before executing this additional representation. The global parametric
construction suite remains its own source snapshot and control.

The existing checked cubic and transported Frobenius actions partition
nonzero L0 abscissae into twelve-element source orbits. The implementation
must verify this partition on every case and independently reconstruct

    g(U) = product_over_source_orbits (U^12 - T).

It must not assume an arbitrary curve has that property. For each checked
orbit, T is nonzero and every root of U^12-T is an allowed source abscissa.
The joint block solves all twelve phases with U still variable. It does
not replace them by a representative and omit transformed residuals.

Write C=B^4 in Fp. Inside this quotient the image coordinates are

    phi: z = -omega^2/(3T) * (T*U + 4B*U^10),
    double: z = [U*(T+8C) - 9B*U^10 + 9B^2*U^7 - 9B^3*U^4]
                / [4*(T-C)].

Check C in Fp and both denominators nonzero, then check each formula against
every actual image point in the source orbit. Pure L0 uses z=U.
Compose the same generic S5 polynomial with these polynomial coordinates,
reducing U^k to T^floor(k/12)*U^(k mod 12) during multiplication.
All three arms consequently have U degree at most eleven in the relation
equations; the block constraint itself has degree twelve. This directly
tests whether the global degree-24 versus degree-32 difference survives a
useful common orbit representation. A smaller global numerator degree is
not itself a solver improvement.

Keep the geometric distinctness guard from SUPPLEMENT_DISTINCTNESS.md and
add U^12-T. For pure L0 it excludes repetition with the fourth factor.
Ordinary cubic roots outside L0 are filtered and charged. Every source
orbit is executed in increasing T order. Physical signed sets are united
and deduplicated across blocks before comparing with the full reference
query. All blocks must finish for a complete query result.

Start complete-root validation with the named p=37, seed=0, query=2
regression in all three arms and both global encodings, then their complete
orbit-block unions. Preserve 8 GiB, one Rayon thread and 1,200 s outer
watchdogs. Native root solving has a cooperative deadline within 1,180 s,
including elapsed setup, to preserve partial field results before the outer
watchdog. A query-level block union shares this deadline across its blocks.
This soft stop is explicit and does not count a partial result as complete.
After correctness, use the preregistered 297-input construction workload
for full comparisons; first-positive controls remain labelled correctness
fixtures and are excluded from population timing claims.

Published invariant methods are prior work. The research question is a
measured complete-cost benefit from this concrete joint implementation;
no factor of twelve in runtime or rank is inferred from orbit size.
