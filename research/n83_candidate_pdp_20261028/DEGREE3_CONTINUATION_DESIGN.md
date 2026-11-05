# Continuing the retained n83 m3 closure prefixes

This phase extends the twelve immutable prefixes from
`n83-degree3-m3-20261029-r1`: original, paired baseline and spike40, each with
Koblitz/general-binary and planted/natural targets. It preserves the existing
n83, m=3, ell=27 presentations and point-base identities. It generates no new
point, target or equation. The wider objective still includes all three curve
families, m=2–9, exact FFD/LFD and factor bases with lower PDP regularity.

The previous phase already certifies LFD >= 3 on all twelve identities. A
replayed witness inherited from that prefix is not a newly discovered fall.
This phase seeks complete degree-three spaces or further certified rows, and
checks separately whether a global upper certificate becomes available.
The results do not compare runtimes, rank base quality or establish a universal
FFD/LFD assertion. The current equal m2 regularities and higher-m interval
certificates do not rank the baseline and spike40 bases; a held-out base/target
comparison remains separate work.

Literature checked before implementation: Caminata–Gorla, *Solving degree,
last fall degree, and related invariants*, Definitions 1.3–1.5, the Macaulay
closure construction and Example 2.4,
https://arxiv.org/html/2112.05579. The ordinary polynomial degree limits which
products enter V3. A complete V3 is not a global LFD upper certificate, and
stabilization over a finite range can precede a later fall. The implementation
references are the frozen `src/bin/degree3_m3.rs`, its raw traces and the
previous `DEGREE3_M3_DESIGN.md`. The new binary is `degree3_continue`.

## Identity and continuation

Before creating an output directory, replay the original complete V2 and the
parent V3 prefix. Reconstruct the exact queue, inserted row order and operation
count, and recompute the complete saved parent verdict. Verify every parent
operation stops before its recorded first row/operation cap. Reject completed
parents and budgets that do not extend the prefix. Hash the parent's four
essential files with a streaming SHA256 reader: dense rows, operations, closure
state and verdict. The frozen protocol binds these hashes and the input/base
identities before production.

Retain all parent bytes in their existing content-addressed archive. Write only
new dense rows and operations to a fresh suffix directory. Row IDs and operation
steps remain global across prefix and suffix. Reconstruct the pivot map from
the verified parent, then pop the next saved queue source. The continuation
uses the same highest-pivot reduction rule and ordinary-degree multiplication
guard as the parent. A new low-degree row enqueues exactly its n variable
multiples. Degree-three rows are never multiplied within this degree budget.

A fresh verifier replays the parent, checks its hash binding, reconstructs each
new operation canonically, compares every new row byte, and verifies the exact
queue and first stopping boundary. It rejects altered parent references,
offsets, operation traces, extra row bytes and false completion markers. This
version accepts the original full-prefix format plus one suffix; it is not a
general arbitrary-generation recovery framework or a CRIU restore claim.

## Bounds and controls

Stop at queue exhaustion, 65,536 total rows or 262,144 total operations. These
are cumulative caps including the retained prefix. Every case starts with
32,768 rows. The budget retains 8 GiB virtual memory and one native process;
allow 1,200 seconds for each production or fresh replay, plus 5 seconds for
termination, and 30,000 seconds for the complete launcher. No automatic retries.
All twelve cases are attempted even if another case times out. Incomplete
external terminations retain their raw suffix and logs but acquire no claims
without a later separately specified verification.

Complete V2 plus an admitted quadratic outside V2 proves lower bound 3. A unit
or exhaustive affine-completion certificate gives a separate global upper
bound 3. Both together give exact LFD 3. Otherwise exact and upper bounds stay
null. Full V3 queue exhaustion certifies the V3 space, not global stabilization.
The result labels whether its strict-fall witness came from the parent or the
new suffix. No degree of regularity or solving degree is inferred from a rank.

Ten native tests pass before freeze. The existing seven checks cover all
735,295 monomial indices, cross-word products, ordinary-degree guards, field
and equation IDs, capped traces, LFD controls and tampering. New tests verify
that row-cap and operation-cap resumptions concatenate byte-for-byte to an
uninterrupted toy run, preserve the parent hashes, stop at both new cap kinds,
and reject wrong parent bindings, offsets, operations, extra bytes and false
completion. Runtime controls execute the same paths and require exact LFD 3
on the positive continuation fixture. The xy-only control gives no global
upper certificate. All control and rejected-trace files are preserved.

## Execution and preservation

Source, protocol, binary and parent identities are committed, uploaded and
checkpointed before production. The existing public synthetic input panel is
fixed; there are no fresh random draws or independent-sample claims. Original
and paired planted targets differ; repeated natural inputs are provenance
identities, not independent observations.

The new EKS storage probe `n83-cont-storage-20261029` terminated with exit 1.
Its MCP log endpoint failed by addressing a node name as a pod; the local
kubectl route cannot assume the launcher role. Thus the probe's application
failure cause is unconfirmed. Preserve these receipts alongside the earlier
recorded shared-bucket denial. Use the existing host fallback, with no IAM or
capacity change and no timing claim. This is application-level trace
continuation, not process-memory or infrastructure recovery.

Flush rows and operations every 128 new operations. Upload growing raw outputs
periodically, preserve all statuses and immutable cumulative summaries, and
seal the finished run with a manifest. Parent data are referenced by their
immutable archive, not rewritten or duplicated in the suffix archive. Archive
all suffixes, controls, source, executable, inputs, V2 traces and metadata.
Download the new archive, verify every file, replay all accepted cases against
the downloaded parent archive, and verify every file again. Produce a validated
unified export, exact format map and inspected report after publishing data.
All earlier runs and all publication/development failures remain available.
