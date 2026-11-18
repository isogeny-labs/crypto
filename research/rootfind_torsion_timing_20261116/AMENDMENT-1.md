# Pre-data amendment 1: restore registry identity binding

Recorded on 2026-11-16 before any timed execution of
`SPEC-K13-LOCAL.json`. The four arms, four targets, eight measured rounds,
warmup, timing fields, analysis, controls and stopping rules in `PROTOCOL.md`
remain frozen.

## Observed preflight failure

The first release binary built from protocol commit `a7469e341c` accepted a
144-execution plan, but its K13 curve facts had `registered=false`,
`ec1=null` and `curve_uid=null`. This contradicts the protocol's registered
curve identity gate. Its plan ID was `ECS1h5840d67387b7`; the pre-repair
binary SHA-256 was
`bc94d5e17fe429902ef5da03bfc63e1ba87696829826fd118450e6299e0ca681`.
The rejected plan is preserved as `PLAN-PRE-INDEX.json`, SHA-256
`a7614b6c920de1418d95888be2a5ae048e0b7cf8f1753599d8093de73f4cf217`.
No timing data were taken with that binary. [pcc:unverified]

The implementation embeds both `docs/curves/registry.json` and the byte-offset
index `src/cryptanalysis/ecbench/registry_index.txt`. The checked-in index
had 22,300 lines and SHA-256
`b251024316e6bdd327ec39d8cc02f83c03bc1100e3c80ed2273ebda97cd09bcf`;
`scripts/curve_id.py:registry_index_text` recomputed 22,278 entries and
SHA-256 `f082729fc1486075f7c0c16d5235535aabbdb704a534456b6e1b2e82b3331779`.
The source-level lookup returns `None` at an index miss, explaining the
unregistered result. The prior broader release library gate had already
reported failures in registry-entry and registered-curve tests; this is a
checkable explanation for those two failures, not a conclusion about the
other six. [pcc:unverified]

## Exact correction and acceptance

Regenerate only `src/cryptanalysis/ecbench/registry_index.txt` from the
committed registry text, using the repository's `registry_index_text` function.
Before any timed run, require:

1. Byte equality of the generated index and committed index; the index test
   and registered-curve test pass from the corrected source.
2. Rebuild release `ecbench` after this source change, record the new binary
   hash, and regenerate `PLAN.json`. Every K13 workload must carry the
   protocol's full curve UID and EC1 alias, registered=true, r=2003,
   cofactor four and identical generator. The four target points, arm methods,
   execution count and seed law must remain those of the original plan.
3. Push a fresh S3 resume checkpoint and pass the checkpoint gate. A current
   Conductor reservation on `.resume/` belongs to T-6342; do not edit that
   path or bypass the gate while it remains held. If admission is still
   unavailable, record the blocker and leave the diagnostic unrun.

The corrected index may change cold-process startup cost for every arm.
No timing row from the pre-repair binary can be compared with a corrected
binary as a matched pair. The spec and analyzer hashes in `PROTOCOL.md`
remain unchanged. This amendment makes no performance claim.
