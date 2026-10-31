# Registry field review, 2026-10-28

This is a read-only comparison of immutable JSON metadata, not a curve,
generator, relation-yield, solver-degree, proof or performance experiment.
It narrows the active registration regression recorded in
[the earlier build review](../backend-build-20261027/README.md).

## Inputs and observations

`review.json` binds both complete input files by Git ref, blob, SHA256 and byte
count: the4493-model registry at `2d1407270` and the20592-model registry at
`79a07b8cb`. Their hashes remain respectively
`87724546ad2e971572c4f62ae0fbc4b2bfed8cd028ae30ded78bc4c4bb9287ca` and
`e7e8cd2ce2570e6e6931674ff653fbaf0cd513b447738364dbb5440a0974a420`.
There are2572 shared slugs,1921 missing study slugs and18020 new slugs.
Duplicate model and per-model EC1 keys are explicitly rejected by the metadata
reader. It neither constructs new models nor evaluates curve arithmetic.

The150 changed shared records split exactly as follows:

| Records | Literal JSON changes |
| ---: | --- |
|103|Top-level source citations only|
|24|Aliases only; no alias removed|
|22|Source citations inside an existing representation only|
|1|Two new representation entries, source citations, removal of the unresolved marker, and the descriptive `params.modulus_rule` label|

Across all2572 shared records, the stored `icv1`, `family`, `model_json`,
`trace`, `order`, `j` and `end` values are identical. No previously recorded
representation is removed. The22 changed existing representation objects differ
only in `sources`; their field/curve/subgroup/generator payloads are unchanged
as JSON values. This is not a fresh mathematical verification of those values.

The single additional case is `icv1-f2m73-tm184271214331-9d25cc67`.
Its literal `a`, `n` and `modulus` parameters are unchanged; `modulus_rule`
changes from the constructor description to `recorded in the run`. The two
added records are `EC1N73Ce0h243f847bccdd` and `EC1N73Ce0h9999c3e90683`, with
their source paths and exact record hashes retained in `review.json`.
Their validity is not checked by this metadata review, and they must not be
discarded by restoring the entire older registry.

## Reconciliation boundary

The metadata comparison supports a lossless input selection: retain every
current20592-model record, including those citation and representation updates,
and retrieve the1921 missing study records from the immutable earlier input.
That would have22513 rows by set cardinality. **No such active registry or
validated22513-model catalog is produced here.** Native identity, field/order,
generator, cover membership, partition, source-binding and byte checks on the
combined inputs remain required. Preserve the strict2287-extension assertion;
do not lower it to1324, fabricate cover records or choose a whole input side.
Rebind and repeat this metadata review if the registry bytes change; these
cardinalities do not describe a later main branch automatically.

The pending checkpoint-only authorization/token and shared coordination gates
still block dependent scientific/catalog replays. The uniform96/896/210 study
and handwritten recurrence draft remain unexecuted/unelaborated. Historical
208 public/269 audited declarations and17 identities gain no new theorem or
certificate from this review. Active registry and all frozen sources/results
are untouched.

## Evidence and check scope

`review.json` contains all150 changed slugs, exact per-row hashes, changed field
sets, and the23 changed representation lists. `runs.tar.xz` retains the exact
metadata-reader source as `.py.txt`, its command/result, coordination failure,
CI snapshot and safe autofix-log excerpt. `RUNS.sha256` checks every restored
file; deterministic repacking is recorded in `archive-receipt.json`.
Python performs JSON/file/hash bookkeeping only, with no cryptographic research
algorithm or correctness oracle. No research workflow/runner is added.

Sources consulted: current main/head AGENTS, newest shared project session log,
active requirements/problem/continuation records, the two immutable registries,
and `docs/curves/ICV1.md` (canonical model/field identity and representation
boundaries). No new ECDLP mechanism or novelty claim is proposed.

The previous exact-head autofix run40601454574 completed successfully and says
`artifacts migrate: no changed data files`; it supplies neither a submitted
review nor the absent required `gate`, `size`, `artifacts` checks. Keep PR2381
open, non-draft and auto-merge disabled. No manual upload, checkpoint invocation,
credential reuse or paid compute occurs. The earlier library attempt remains
incomplete/non-green; it is not rerun for this metadata/documentation-only
change. Other library failures remain untriaged.
