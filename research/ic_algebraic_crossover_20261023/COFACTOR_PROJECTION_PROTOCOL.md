# Whole-curve points and cofactor-projected relations

2026-10-25 18:40 UTC, before this audit's output. The strict all-summands-in-G
audit is not the only possible IC interpretation. A verified full-curve relation
P1+P2+P3=R projects to hP1+hP2+hP3=hR, even when the original summands are
outside the prime subgroup. This possible method version must remain visible.

For all seven original fixtures, on both previously fixed GF(2^22) E0/twist
models, enumerate every full-curve point above V. Verify h-projected membership,
retain zero projections and multiplicities, count distinct nonzero projected
points, and hash the exact inventory. For a liftable target choose the least
encoded y, compute hR, and record whether it is identity or a direct projected
base hit. For each original SAT draw use exactly the first previously defined
quadratic-chain witness, test all signed point choices and verify every full-
curve relation after projection. Keep strict-G and projected relations distinct.

Use the existing exact models, prime/cofactor/generator metadata and the
versioned canonical UID adapter. Native Rust BinaryCurve arithmetic only;
2-GiB address-space and 300 seconds, one thread, shared-host correctness
diagnostic. Freeze source/binary before output and independently rerun byte
for byte. Preserve any failure and render an inspected report.

The audit does not know factor-base logarithms or prove an ordinary query
distribution. A complete projected-IC version still needs a declared public
target/preimage/cofactor-component law, projected-base logs and relations,
scalar recovery and verification, and all setup/online/lifting/projection costs.
It does not change any earlier strict-G receipt, Boolean root count or failure.
