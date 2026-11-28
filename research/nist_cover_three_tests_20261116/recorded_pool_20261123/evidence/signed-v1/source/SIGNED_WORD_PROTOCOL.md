# Signed short-word successor audit

This is a separate, frozen static/development audit of the same recorded
P-192 pool;the original protocol and results remain unchanged. No new curve
points, random search or performance campaign. Sources:original PROTOCOL.md,
its exact pair method and target Newton-jet construction;frozen target and
projection sources. KB unavailable as recorded in the original protocol.

H-C: there is a nonzero signed coefficient row on the225 recorded canonical
elliptic images, of coefficient L1 norm at most4, whose elliptic sum is zero.
Predicted useful outcome:at least one new row or additional coefficient rank.
Falsification is only for this finite alphabet and length bound.

Use alphabet O,+B0,-B0,...,+B224,-B224 and all unordered pairs with diagonals.
Compute full ordered fingerprints using OpenSSL and direct GMP arithmetic.
Join equal keys twice,by grouped buckets and by independently sorted entries.
Canonicalize each equality to a sparse coefficient row before applying the
4096 nonzero-row cap. Drop formal cancellation;orient the first nonzero
coefficient positive. Compare complete row sets and verify every retained
row by elliptic addition. Any signed word of length<=4 can be partitioned
into two pairs,negating one side and padding with O. This covers the stated
domain,including repeated symbols. Do not enlarge the domain after results.

Known-answer control is separate from the recorded pool:use G and2G to
require the nonzero row 2G-(2G),and require formal cancellation aliases to
collapse. Reject a missing pair,an off-curve point and a mutated known-answer
row. Record all control outcomes and every candidate;no scalar recovery.

Separately test the canonical half-trace alias. Sum the reference images
to kappa;compute T=[(n+1)/2]kappa by both backends,require 2T=kappa and
compare T and -T with the recorded canonical base. If T=s*Bj,retain the
row 2*s*ej-Krow and check its elliptic sum and union rank. This row is a
separate structured test,not part of H-C's length bound. The conditional
geometric motivation is:for a rational bitangent contact divisor D with
2D~K,its norm obeys2f_*(D)=kappa;since n is odd,whole contact divisors have
the same aggregate norm T. This does not assert bitangent data is present.

For every retained signed/half-trace row,replace each canonical base symbol
by its recorded curve point with the recorded projection sign. Subtract
the total divisor degree at the recorded point with image O. Split into
positive and negative divisors;pad both to degree12 using the same first
reference point. Classify their actual target classes with the generalized
Newton-jet encoder. Run the same encoding on all six baseline principal
rows and require a coefficient-mutated baseline row to be nonprincipal.
No norm-only row is relabelled principal. Report rank separately for all
norm rows and target-principal rows.

Caps:4096 nonzero signed rows,256 actual target checks,120seconds/1GiB for
the arithmetic worker,600seconds/1GiB for target checks,300seconds/1GiB per
strict native build. Exceeding a cap is a retained bounded failure. Fresh
write-once evidence directories pin sources,inputs,binaries and exits before
workers execute. No performance or global curve-absence claim. [pcc:unverified]
