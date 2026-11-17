# Native search checkpoints and genuine durable replay

This branch activates the reviewed native checkpoint producer in eight production source files. Explicit conservative_affine_envelope_checkpoint_v1 under the existing v5 request selects one checkpoint after each committed batch, limited to16 batch-hook attempts across the retained continuation. Existing policies remain unchanged. Existing initial/exclusion/final snapshots still share the global capture limits; the new16 limit applies only to additional batch hooks. The owner retains partial native state, exact publication failure and controller state, using its original Budget and deadline. The borrowed encoder publishes through the real capture sink.

The source is byte-identical to the reviewed component proposals (ASSEMBLED_SOURCE.json) and all eight modules parse. Relevant prior component controls:19kernel,28overlay,21owner/kernel,6policy and16serde/selection tests. The genuine v2 fixture joins actual production map build/import, expansion, real search callback, real durable publication and original-descent/semantic replay:26variables,26generators,12verified rows. It uses one real budget and deadline and prepays readback from retained lengths. Full original-cell/CLI admission, full assembled Rust build and production-policy acceptance remain outstanding. The fixture uses stdsort/supplied normalization cursor and batch_rows128 rather than the production policy's1; its filesystem is controlled, not hostile-concurrent mutation validation.

The historical-checkpoint export work remains research-only: strict prefix parser13tests including all6762 cuts of a6761byte snapshot; selector13tests; joint17tests including4new real-chunk controls; semantic replay14tests. These counts overlap inherited suites and are not a count of unique tests. The parser validates observed bytes and exposes complete binding fields; caller still must bind admitted originals, verify descent/math, and join/revalidate the complete outcome/capture. This branch does not activate fallback export or relax v1 report joins.

## Explicit request

Use the existing exclusive channel request in a future authorized plan:

```json
{"schema":"crypto.ml-dataset.channel-requests/v5","owned_boolean_search":{"mode":"matrix_continuation","pattern":0,"policy":"conservative_affine_envelope_checkpoint_v1"}}
```

This is a source capability, not a cloud or scientific-run authorization. Existing budget/deadline remain expired. The original pending build source70e0b1d8f435 remains unchanged in its separate checkout (1463inputs verified); this new branch has no full-build result.

## Custody

S3_REPLAY_RECEIPT.json binds the14,097,552byte archive and170verified members with fresh byte-equal readback. Full sources, harnesses, raw capture fixtures and failed attempts are in that archive; per-packet manifests bind full archive packets, not these reduced Git review copies. Exact logs stay in S3; Git copies normalize terminal whitespace. Prior receipts chain to the owner/policy/kernel/encoder source evidence archives.

V1 genuine durable fixture is preserved with its readback pre-admission limitation. V2 quotes before readback and tightens readback caps to retained lengths. Both independent reviews record fixture scope precisely. No fall-degree, scalar recovery, full-search completion or instruction-count claim is inferred from these reservation/accounting checks.
