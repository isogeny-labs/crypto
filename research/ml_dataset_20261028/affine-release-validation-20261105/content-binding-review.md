# Root independent review

Reviewed the complete V2 derivation source and the V3 parser delta. V2 failed closed on nested checkpoint child libtest output. V3 uses explicit nested suite scopes, preserves both child suites under their parents, requires each parent terminal, and reconciles outer counts independently. Root reran all13 Python controls successfully against the genuine completed log.

The adapter checks unchanged recorded source hashes against immutable commit blobs, all resolved root inputs, six local dependency source sets, dep-info and Cargo fingerprint linkage. Supplemental dependency provenance remains explicitly a postcompile inference using immutable content equality and predating ctime/mtime; it is not retroactively described as a historical prebuild snapshot.

Executed the reviewed adapter on actual terminal receipts, binding commit802444fa9dc94194b2ae525d9cd3ad14fe1b9b52. Derivation passed with outer counts6237passed/0failed/125ignored/0measured/0filtered. No native tests were repeated. Attestations are external in affine-tested-content-attestations-20261105.

Scope: software test/source provenance. No scientific fall-degree or corpus-completeness conclusion follows.
