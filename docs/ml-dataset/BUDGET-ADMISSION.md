# Native cumulative dataset budget admission — design handoff

Prepared 2026-10-28 as a design-first handoff. The design below preceded implementation. Native implementation is now being integrated in `budget.rs`, `s3_budget.rs` and the dataset CLI; see [BUDGET-CLI.md](BUDGET-CLI.md). Software validation and actual cloud enforcement are separate acceptance gates. The completed source `0f55f1c` validation predates this implementation, and no cloud budget-enforcement result is claimed here.

The smallest correct extension is one durable campaign allocator, disjoint worker grants, and a dataset-only S3 backend that accounts before every HTTP attempt. Keep the current immutable publication/readback checks. Do not equate a successful publication, a per-cell trace limit, or a local writer lease with campaign admission.

**Current gaps and integration points.**

- `src/ml_dataset/mod.rs::generate` republishes completed cells before checking `max_cases`; resumed work can therefore consume requests without creating new cells. Its trace limit is polled, can overshoot between polls, and excludes later publication metadata. `Lease` protects one local run directory, not a campaign across hosts.
- `src/ml_dataset/storage.rs::publish_object` calls HEAD and fresh GET itself, then `Store::push` makes additional HEAD/PUT calls, followed by another GET. HEAD-403 fallback, conditional-PUT conflicts, completion objects, catalog receipts, and `mod.rs::publish_lab_receipt` add requests. Count every path, including failures.
- `src/ml_dataset/restore.rs::download` performs HEAD plus a fresh GET for every object on every attempt. Its 1 GiB receipt limit and 32 MiB parsed-JSON limit are per operation, not cumulative download limits.
- `src/store/backend.rs::S3Backend` invokes the AWS CLI with inherited retry settings; downloads use `aws s3 cp`. A backend call is not one HTTP request: CLI retries and transfer-manager downloads can issue multiple requests. [AWS CLI retry behavior](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-retries.html), [S3 transfer configuration](https://docs.aws.amazon.com/cli/latest/topic/s3-config.html).

**Freeze precise cap semantics before admission.**

| Dimension | Proposed hard accounting rule |
|---|---|
| Campaign money | USD 200 total, represented as 200,000,000 integer micro-USD; include existing obligations, active grants, and unresolved attempts. |
| Unique remote output | 21,474,836,480 bytes across campaign output keys. Identity is `(bucket, full key, SHA-256)`; the same CAS key is counted once, while another catalog/lab-receipt key counts again. Include control-ledger objects. |
| Write requests | At most 100,000 charged PUT attempts. Prohibit multipart writes initially; keep literal method counters. |
| Read requests | Conservatively apply the 200,000 cap to GET **plus HEAD** attempts, with separate GET/HEAD counters. Do not silently treat HEAD as free. |
| LIST | Disable in the first implementation; restore starts from a known portable receipt. If discovery is added, freeze a separate limit/cost class and count every page/attempt. |
| Downloads/local output | Separate finite byte reservations are required; neither a unique-output cap nor a GET count bounds transfer or retained failed-attempt disk usage. Preserve the existing per-restore limit too. |

Record these interpretations in the policy; they are stricter definitions proposed for the existing shorthand caps. A key is successful unique output only after fresh GET/length/SHA verification. Reserve its bytes before PUT; an uncertain PUT retains that reservation. An already verified key does not consume unique bytes again, but every new verification consumes requests and download bytes. Restore does not create new remote output bytes.

The existing `shared-budget-2751.md` reserves USD 188.3888, leaving USD 11.6112. Preserve its E12a/E12c original ceilings, E12b allowance, storage, transfer, and contingency. Its proposed USD 14.688 worker allocation is based on a us-east-1 shape; the dataset EKS path is in us-east-2. Reconcile actual resource shape, previous failed waves, deadlines, and pricing before replacing or adding any allocation. Never count the same proposed worker twice; never release it based only on an old “not launched” note. These are conservative reservations, not measured billing.

Historical requests/bytes must be reconciled or covered by a documented conservative initial reservation. Unknown earlier usage is not zero. If no finite upper bound exists, report that cumulative compliance cannot be established; a newly instrumented measurement window must not be presented as the entire campaign.

**Minimal records and API.**

Use `crypto.ml-dataset.budget-policy/v1`, `budget-grant/v1`, and `budget-event/v1`. Policy fields: campaign ID; immutable policy hash; original deadline; money/output/request/download/local-byte caps; accounting semantics; prior obligations with evidence hashes; price snapshot/region/resource IDs; storage horizon; and authorized remote prefixes. Use integers and checked arithmetic throughout.

A grant binds policy hash, grant ID/generation, exact Job and Pod UID, source/binary/plan/protocol hashes, shard, expiry, and finite reservations for compute, requests, output, transfer, local bytes, and finalization. A grant is an allocation from the campaign, not additional money. It must also bind its durable journal identity; missing state cannot create a fresh balance.

An event records a monotonic sequence, previous-event hash, grant/operation/attempt IDs, operation kind, remote key, expected digest/length, charged request class, byte reservation, and outcome. Keep `reserved`, `verified`, `failed`, and `outcome_unknown` distinct. Record observed response status/request IDs without logging credentials. `charged_attempts` is a conservative bound on transmitted attempts, not an exact AWS billing metric.

Proposed Rust boundaries, not implemented APIs:

```rust
CampaignLedger::dry_admit(policy, request) -> AdmissionReport;
CampaignLedger::reserve(request) -> WorkerGrant;
BudgetSession::open(policy, grant, durable_state, owner_identity) -> Session;
Session::reserve_cell(envelope) -> CellReservation;
Session::reserve_object(remote_key, sha256, bytes) -> ObjectReservation;
Session::before_http_attempt(operation) -> AttemptTicket;
Session::finish_attempt(ticket, outcome) -> ();
Session::verify_object(reservation, readback_evidence) -> ();
Session::close_or_reconcile(owner_termination_evidence) -> Reconciliation;
```

Proposed CLI: `dataset budget admit --dry-run` returns requirements, remaining capacity, evidence hashes, and exact rejection reasons without reserving or contacting S3. An explicit allocator command issues the durable grant. S3 `generate`, `publish`, and `restore` require policy/grant/state inputs and reject missing or mismatched bindings. Existing offline file-backend tests remain possible with a finite local test policy. A dry-run report is never launch authority.

**Count at the transport boundary.**

Implement a dataset-only `Backend` using a pinned compatible AWS Rust SDK and individual HeadObject/GetObject/conditional PutObject operations. Add the narrow shared-store constructor needed to inject this backend; leave the default store backend unchanged. All dataset store, catalog, lab-receipt, restore, and budget-checkpoint paths must use the injected backend. No CLI transfer-manager escape path may remain in budgeted operations.

Before each transmit attempt, atomically charge and fsync the journal, then permit the request. The SDK's `modify_before_transmit` hook runs per attempt, including retries; use it or an equivalently tested connector boundary. Budget rejection must terminate that operation without another unaccounted attempt. Pin retry and request timeout settings; verify that the chosen connector does not reissue HTTP requests below the accounting hook. [AWS Rust interceptor contract](https://docs.rs/aws-smithy-runtime-api/latest/aws_smithy_runtime_api/client/interceptors/trait.Intercept.html#method.modify_before_transmit).

Every retry gets a new charged attempt. A crash after journaling but before sending may overcount; do not refund that ambiguous unit. Successful responses and observed transferred bytes are separate metrics. A timeout may have committed the PUT: retain its byte reservation until independent verification. Bound each streamed GET by the admitted expected size and reserve worst-case retry transfer bytes. Disabling CLI retries and replacing `s3 cp` would reduce hidden work, but process-invocation counts alone are insufficient evidence for an exact HTTP-attempt guarantee.

**Cross-worker durability without a new distributed service.**

Use one allocator with a local transactional SQLite ledger (`rusqlite` already exists), full durability, and a single authoritative owner. Do not copy writable ledgers between hosts or put SQLite WAL on a network filesystem. Each grant receives disjoint quota; the sum of outstanding grants plus prior obligations cannot exceed policy limits. The allocator durably records issuance before exposing a grant. Immutable S3 checkpoints/activation records can live under the already proposed `ml-dataset/20261028/` prefix; this design requests no IAM broadening.

Activate a grant once for its exact Pod UID using conditional create and verify any existing activation. A replacement Pod cannot reuse the old balance. Inside that Pod, a local lease and the same durable journal protect restarts. If the journal is lost, stop and conservatively consume the unreconciled grant allocation; do not reconstruct a fresh allowance from receipt count. Old owners must be proven terminated before any unused quota is reclaimed. Expiry alone is insufficient evidence, and controller failover must stop issuance until its authority and latest grant state are reconciled. These choices deliberately sacrifice automatic recovery to keep the first implementation correct.

Reserve a small explicit control/finalization grant for activation, ledger checkpoints, and final receipts. Those requests also count. Per-request journal writes stay local; periodic immutable checkpoints avoid recursively generating an S3 accounting write for every S3 accounting write. On journal loss, charging the full remaining allocation preserves the ceiling even when the exact attempt history cannot be recovered. Concurrent workers may conservatively reserve duplicate CAS bytes; refund duplicates only through authoritative reconciliation, never optimistically before admission.

**Generate, cancellation, and completion.**

Before spawning a cell, reserve its compute envelope and enough bytes/requests to finish its mandatory publication and receipts. Freeze a maximum artifact count and metadata allowance; trace bytes alone do not bound publication requests. Strict local-output limits require checked writes for storage/live sinks and bounded stdout/stderr capture; the current 100 ms size poll is only a stop mechanism. If strict local-byte enforcement is deferred, say so explicitly and enforce remote output at PUT admission without claiming a local cap.

On cancellation, stop new work, terminate/reap the child, persist the partial state, and use the reserved finalization allowance. Failed/incomplete attempts retain charges. Resume loads the same campaign/grant accounting before any repeat upload or download. Budget exhaustion returns a durable `budget_exhausted` state with completed, unpublished, and incomplete work distinguished; it never labels an unverified shard durable.

Compute admission must reserve the actual billable resource lifetime, not just solver CPU seconds. A Job timeout does not prove its EKS node stopped billing. Bind an existing enforceable node/resource lifetime or retain its full conservative obligation. Native request accounting alone cannot truthfully promise an account-wide USD 200 billing cutoff, nor constrain unrelated principals. Price snapshots, first-month storage horizon, allocation scope, and those limits belong in every admission report.

**Acceptance tests before claiming enforcement.**

1. Two concurrent grant reservations cannot exceed a campaign balance; two workers cannot activate the same grant. Restart preserves debits; missing/stale journal and unproven owner termination refuse reuse.
2. Fake HTTP transport exercises retry, HEAD-403 fallback, conditional conflict, denied PUT, and timed-out successful PUT. Every attempted request is charged before send; exhausted quota produces no network request.
3. Inject crashes before/after debit, PUT, readback, and checkpoint. Unknown outcomes remain charged; successful unique bytes appear only after verified readback. Duplicate keys save bytes but never erase request attempts.
4. Completed-cell resume, explicit publish, catalog/lab receipt, and restore all share the same ledger. Failed restore attempts consume cumulative read/transfer capacity. No alternate unbudgeted backend is reachable.
5. Dry admission is read-only and rejects unreconciled prior usage, wrong region/resource identity, insufficient finalization reserve, or USD 200 overflow. Boundary/overflow tests cover every integer cap.
6. The final cloud proof runs only after separate authorization and admission, retaining request/byte journals, grant/checkpoint hashes, raw receipts, and the independent S3 readback. Local tests alone do not establish the cloud budget guarantee.

Original implementation scope: new `ml_dataset/budget.rs` and dataset S3 transport module; narrow backend injection in `store/mod.rs`; budget/session plumbing through dataset generation/publication/restore; controlled writers where a strict local-byte claim is required; and focused failure/concurrency tests. The acceptance checklist remains authoritative; implementation alone does not establish a passing check or a cloud result.
