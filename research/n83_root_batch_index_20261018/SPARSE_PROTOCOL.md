# N83 K512 index-stage pilot with the high-K canonical selector

The high-K candidate manifests enable `KIC_SPARSE_CANONICAL=1`. This second
pilot isolates that flag while retaining the same K512 base, scalar/batch
ABBA order, root11 indexed-table layout, index-only boundary, and 180-second
process cap in [PROTOCOL.md](PROTOCOL.md). The native producer source remains
SHA-256 `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The fixed [runner](run_sparse_pilot.sh) is SHA-256
`e2bae3599d1a8d255ec0091976db216435995c0994beebd2c26a86ea8238d628`.
Its only experimental flag change from the first runner is
`KIC_SPARSE_CANONICAL=1` in every process.

Before timing interpretation, compare the full compact-state and root-table
digests, base hash, counts, source and binary hashes, exit statuses, and all
four process receipts. Report both paired scalar/batch index ratios and the
observed host load. This local stage pilot retains `candidate_id: null` and
cannot select an IC pipeline.
