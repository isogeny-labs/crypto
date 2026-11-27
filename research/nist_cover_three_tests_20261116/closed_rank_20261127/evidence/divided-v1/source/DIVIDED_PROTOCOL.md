# Divided aggregate image aliases, frozen 27 November 2026

The original 225-column base, trace expression h for kappa and baseline rank6
remain frozen. Compare labels (kappa-A-B)/m for m=1,2,3, with A,B in the signed
old-point alphabet including O and repeated symbols. A degree-m closed divisor
whose conjugate points all map to a rational Q has aggregate image mQ. These
label families are necessary supersets: a label need not be geometrically
realizable. Inverting 2 or 3 in the prime-order elliptic group does not provide
a rational 3-torsion point or establish a torsion attack.

## Native observation plan

Precompute all 225 divided points and divided kappa using inverses modulo r.
Verify [m]Pdiv=P for every point/reference with both GMP and the named OpenSSL
curve. Reuse the unchanged norm helper on all 101926 signed pairs per family;
every pair and complement must agree between both arithmetic implementations.
The m=1 stream digest must match the frozen old-column norm result.

Represent six times each even-y label by the sparse integer old-base vector
(6/m)*orientation*(h-word(A,B)). At a shared x-coordinate, subtract unequal
vectors; a nonzero expression at O is also a candidate relation. Normalize
integer gcd and sign, deduplicate, and retain source family, pair and
normalizer witnesses. The gcd is a unit modulo the fixed prime order; still
check every primitive candidate directly with both arithmetic implementations.
Cap distinct nonzero candidate rows at4096; exceeding the cap means incomplete.
Retain complete family counts, digests, duplicates and identity counts.

Append candidates to the original six rows over225 columns and, separately,
zero-pad them into the original429-column matrix. Eliminate all204 old
auxiliary columns. A closed rank above6 is a candidate requiring actual
cover/KM realization and separate replay; do not declare goal success from an
elliptic-coordinate alias alone.

## Controls

Use a separate public G,2G fixture with kappa=4G and empty relation baseline.
The same three-family search must find the primitive row (2,-1), rank1, and
exercise identity labels. A changed divided point must fail multiplication
back; a changed candidate coefficient must fail its group sum; omission of a
pair must fail completeness. No synthetic fixture enters the actual matrix.

## Execution and standing

OneCPU,1GiB virtual memory,300s strict build,600s native worker. Preserve
source/input/binary hashes, costs, errors and raw outputs. Conductor gating is
ignored per the user's explicit instruction; mathematical limits are unchanged.
Prior work is the already reviewed trace/norm methods and frozen half-trace
tests; this expands the compared label domain. No new PCC, scalar-recovery,
novelty or speed claim. All producer conclusions remain [pcc:unverified].
