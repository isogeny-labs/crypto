# Signed length-six norm search, frozen 27 November 2026

Keep the original225elliptic columns, original108by429matrix and old rank-six
baseline unchanged. Extend the completed signed length-four diagnostic to all
signed words of length at most6on the original base. This is a necessary
elliptic norm search, not a demonstrated geometric relation collector.

Use alphabet O,+P0,-P0,...,+P224,-P224. Enumerate all unordered triples with
repetition, O padding included:15390826triples. Compute each sum using named
OpenSSL and independent GMP affine arithmetic, compare every point and a
whole ordered-stream SHA256. Pack canonical signed sum (25bytes), three
uint16labels (6bytes), orientation (1byte) into a32-byte record. Freeze that
record size with static_assert; table payload492506432bytes. Retain no giant
raw table file: the full deterministic input/source and stream digest bind it.

Sort in place by all32bytes, so canonical-sum groups are contiguous and
witness order deterministic. In every nonidentity group, subtract each triple
from a representative with its actual sum orientation. These rows span all
zero-sum six-word rows in that group; both sign orientations are included.
For the identity group retain each triple's own nonzero coefficient row.
Remove formal coefficient cancellations and deduplicate rows up to sign.
Retain complete signed label witnesses, bound4096distinct nonzero rows, and
check every retained elliptic row directly. Reaching the bound rejects
completeness. Append rows to the frozen225-column baseline and separately to
the429-column original matrix with zero padding. Report both closure ranks.
[pcc:unverified]

Qualification: first64columns (129signed symbols;366145triples), without
changing the declared original225-column rank baseline. The full225run follows
only if it passes. A separate public G,2G fixture must detect a nonzero row
equivalent to(-2,1). Compare grouped representative-row span with brute-force
all matching triple pairs on that fixture. A changed witness coefficient and
an omitted table entry must reject. No fixture row enters the real matrix.

One CPU,1GiB virtual memory,300s strict build,600s per native worker. No full
429-column triple table (it would exceed memory). Retain commands, versions,
source/input/binary hashes, exact exit codes, stderr/stdout and stage/wrapper
costs. Timeout is inconclusive, never a proof that the domain is exhausted.
Any positive norm candidate needs actual cover/KM and independent replay
before the positive-rank objective can be marked complete. No scalar recovery,
new PCC or speedup claim. Prior art: recorded_pool_20261123H-C and its reviewed
signed-word meet-in-the-middle construction; no novelty claim.
