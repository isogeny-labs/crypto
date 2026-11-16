# Resume: exact ECCp-131 representation-filter counted run

## Goal

Run the prospectively frozen exact ECCp-131 coordinate-filter sweep and its
relation, rank, root, and modular-solver controls. Preserve exact counted
evidence; GitHub-hosted clocks are not admissible performance measurements.

## Lead / hypothesis

`RT-REPRESENTATION-FILTER`: a pre-add endpoint-coordinate predicate must show
held-out relation enrichment over independent-null and matched random-base
controls, while admitting output-sensitive qualifying-pair enumeration.

## Spec

`research/eccp131_representation_filter_20261102/protocol.json` and
`PROTOCOL.md`; exact Certicom ECCp-131; B=64/128/256; fixed train/test sizes
and seeds. The triggering commit contains a filed-before-data amendment for
this GitHub-hosted counted-only route.

## State

Source and protocol are committed and pushed. This workflow checkpoints them
before its first data-producing step, then builds and runs the native Rust
instrument. Results are uploaded as one immutable Actions artifact.

## Next steps

1. `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate`.
2. Build the exact trigger revision.
3. Run the sweep and arity-2/3 `ic prime` baselines.
4. Seal and upload every raw file.

## Runs in flight

This GitHub Actions job only; its run ID and artifact ID are the custody
identities.

## Stop and success conditions

One attempt, 180-minute absolute timeout, fixed N, no seed retries. Positive
root/solver controls and A/A replay must pass. A filter requires held-out
lower ratio >1, a pre-add enumerator, and no random-base explanation.

## Open questions

The preferred L2 EKS run remains queued until the USD 25 cost gate records
headroom. F4/F5 remain unavailable at exact 131-bit width.

