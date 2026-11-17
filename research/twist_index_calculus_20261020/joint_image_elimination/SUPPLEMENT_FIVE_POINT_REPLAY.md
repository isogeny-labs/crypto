# Independent point-keyed replay of all five-factor zero cases

Frozen after the scalar-keyed census completed, before computing any of
the point-keyed sets below. It is the independent completeness obligation
already required by `SUPPLEMENT_FIVE_FACTOR_ZERO.md` when nonzero rank
appears. Retain every original result and every case, including zero rank.

Use only the 13-key public input to construct the actual source/image
points. Read the prior five-factor physical sets and headline counts for
comparison, but do not read any factor-log vector, recovered scalar,
scalar-keyed pair table or BSGS output. Sharing the public curve arithmetic
and orbit construction is explicit; independence concerns the enumeration,
keys, partition, row reconstruction and matrix algorithm, not a new field
arithmetic implementation or formal completeness proof.

Build sorted signed ordinary triples keyed by their complete group point,
with the first sign positive. Query both a point and its negative to
restore both global signs. Pure five-tuples use their two smallest IDs as
the query pair; mixed tuples use their smallest ordinary ID and image as
the query pair. Remaining IDs must be larger. Thus every signed physical
tuple has one canonical partition, different from the earlier scalar-keyed
ten/six-partition join. All actual coordinates enter an injective radix-p
key; infinity has its own sentinel. Verify every returned five-point sum.

Reconstruct anchored rows directly from the public action coefficients.
Check them against the existing row serializer, but compute rank with a
separate unnormalized cross-multiplication elimination and back substitution.
Use extended Euclid for division, separately from the earlier Fermat-based
RREF. Compare complete signed sets, zero rows, projective rows, rank and
full-factor point verification. No target column may acquire a nonzero
coefficient; no target recovery is attempted.

Run all 21 original cases and all three arms, one Rust worker, 8 GiB and
1,200 seconds per case after separate compilation. Keep timeouts, incomplete
sets and missing cases; they are not zero yield. The shared busy reservation
continues to serialize heavy work. Unit controls compare the new canonical
partition with direct five-term enumeration in a separate small group and
exercise dependent, full-rank and inconsistent matrices.

The run is an independent oracle replay, not a polynomial collector or an
admitted precomputation/IC price. A passing rank difference would still need
actual polynomial collection, charged setup and target descent, matched
generic-image and IC controls, A/A, complete prices and strongest rho before
an end-to-end gain can be claimed.
