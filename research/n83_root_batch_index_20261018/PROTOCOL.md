# Batched N83 S3 root-index construction

The implementation batches the S3 roots for all relative Frobenius shifts of
one representative pair. Montgomery's shared inversion replaces up to 83
separate inversions with one inversion and field multiplications. The builder
retains its pair and shift traversal, root order, canonical-key insertion,
and first-witness policy. `KIC_INDEX_ROOT_BATCH=1` enables this path;
the default uses scalar roots.

## Frozen K512 index-stage pilot

Use the retained N83 K512 base dump at
`research/n83_state28_20261013/runs/control/k512_packed/base_dump.jsonl`,
SHA-256 `7b97fbdb231418a5caccae429b2bb62162865a4a1421399b53573eff32d47b33`.
The source file `examples/koblitz_orbit_dlp_fast.rs` is SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
Run the same release binary four times in fixed scalar, batch, batch, scalar
order. Use an empty target file and `KIC_INDEX_BUILD_ONLY=1`; this mode exits
after base loading, normal-basis self-test, and index construction. Freeze
`KIC_SYMMETRIC_S3=1`, `KIC_PACKED_ROOT_TABLE=1`, `KIC_ROOT_ONLY11=1`,
`KIC_INDEXED_ROOT5=1`, and `KIC_INDEXED5_LOAD75=1` in every process. Set
`KIC_INDEX_ROOT_BATCH` to `0,1,1,0` in that order. Cap each process at 180
seconds and retain the command, stdout, stderr, exit status, resource usage,
source and binary hashes, and host snapshot, including failed attempts.

Compare `base_hash`, column and point counts, state and table counts, the
full compact-state BLAKE3 digest, and the full root-table BLAKE3 digest across
all four runs. A digest mismatch rejects equivalence. Report `index_build_ms`
and `setup_total_ms` for each run and both paired scalar/batch ratios. This
local stage pilot has `candidate_id: null`; it does not select a complete IC
pipeline. A promoted comparison must use exact candidate manifests, fresh
same-point IC/rho pairs, independent scalar replay, and the Tier 1 resource
and calibration gate.

The unit test
`wide::root_bloom_table_tests::symmetric_s3_states_preserve_exact_root_keys_and_first_witness`
checks scalar/batched root and first-witness equality for every retained
state in its deterministic N83 fixture. A separate N73 root-batch test also
checks scalar/batched S3 outputs for 4096 input pairs, including exceptional
cases.
