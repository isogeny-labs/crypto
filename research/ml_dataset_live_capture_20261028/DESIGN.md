# Incremental dataset artifacts and cumulative store admission

This software change is based on producer commit `0f55f1c333449d4c85f04448da16fbfec15249cb`, which contains additional fixes after the published PR #2756 head `d0334808999e039d95bfea5137d0460db2868aea`. It preserves the producer changes and does not launch a new dataset trial.

The preceding coverage audit identified a concrete gap: native exposed events and complete payloads were fsynced locally, but S3 publication waited until the cell ended. Reclaiming a host during an opaque solver call could lose those local artifacts. The audit is retained at `s3://setec-astronomy-artifacts/ml-dataset/20261028/design/5ab6511bc684387352e1b92805c19f46504430c5fbd59218c4640d677972049f/intermediate-artifact-coverage.md`.

## Fixed contracts

1. An attempt binds the exact retained run, plan, protocol, config and original attempt bytes, source commit, binary digest and cell index. Event records form an ordered hash chain. A worker acknowledges an event only after syncing its complete record and atomically installing its committed watermark. Payloads are sealed before their commit event. Unacknowledged tails and orphan payloads are not claimed as captured.
2. A supervisor helper copies the acknowledged prefix into immutable chunks. The first publication retains frozen context before worker launch. Later helpers retain new context, events and payloads. Network activity occurs in a separate process; the original solver deadline remains enforced. A failed or timed-out publication stops the wave and retains local files and prior remote receipts.
3. Partial snapshots have a separate manifest, receipt, catalog namespace and restore path. Their cell-complete, solver-state-restorable and scientific-result-verified fields remain false. Restore downloads and checks artifacts only; it neither invokes a solver nor creates a completed-cell manifest. Opaque matrix/SAT/root state not exposed by native instrumentation remains absent.
4. The native S3 transport admits every opted-in command before executing it, including internal HEADs, LIST pages, retry attempts and verification reads. Reservations include conservative object-body bytes and command-attempt counts. Failed or uncertain requests stay charged. CLI automatic retries are disabled for guarded calls. Finalization has an explicitly reserved allowance.
5. A durable mounted controller authority issues disjoint per-attempt quota envelopes. A worker mirrors reservations into that authority before its local journal. Missing, stale, malformed or mismatched journals fail closed. Losing a worker's established journal forfeits that allocation; restoration does not mint quota. The same durable authority must be available on a replacement host. This change does not install such a mount in EKS or create an S3 conditional-update authority.

## Budget and launch boundary

The user authorized **USD100–200 total through tomorrow**, with an upper planning ceiling of USD200, not a daily allowance. The earlier conservative reservation of USD188.3888 and USD11.6112 remaining is a planning record, not actual billing. These S3 byte/request limits do not enforce dollar billing, EC2 charges, unrelated AWS activity or service-side traffic. New compute still needs admission against the existing total reservation and current evidence; this change grants no additional spend.

The proposed worker policy remains a separate pending approval. `isolab-job` has not been granted additional permissions here. The only writable bucket is `setec-astronomy-artifacts`, region `us-east-2`. No E12 attempt, deadline, frozen input, measurement or retry rule changes.

## Sources checked before implementation

- Existing producer `src/ml_dataset/{mod,solvers,storage,restore}.rs` and `src/store/backend.rs`; the retained coverage audit above.
- [AWS CLI retry configuration](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-retries.html): the initial request counts toward `max_attempts`; retries must be represented in reservation accounting.
- [S3 GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html): object identity conditions and bounded reads are used by guarded downloads, followed by independent length/hash checks.
- [Rust File persistence semantics](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all): explicit sync reports errors that dropping a file can discard; file and directory durability are separate obligations.

## Validation and remaining scope

Validation will cover interrupted append/payload installation, corrupted chains, cap exhaustion, cumulative retry/restart accounting, separate partial restore, and supervisor lifecycle failure. These are software tests over synthetic fixture artifacts. A real native producer-to-S3-to-independent-consumer roundtrip, cloud interruption/replacement, broad solver coverage, all requested intermediate instrumentation and the extensive corpus remain separate acceptance requirements.

The change cannot establish exact first/last fall degrees, an asymptotic bound, a trained model's benefit, or a performance/security claim.
