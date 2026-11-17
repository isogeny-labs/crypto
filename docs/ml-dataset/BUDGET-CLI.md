# Dataset budget CLI and retained accounting

This interface implements bounded campaign allocation and request accounting for
`crypto dataset`. Software validation and any actual cloud admission are recorded
separately. A successful local admission does not establish worker IAM access,
actual AWS billing, or a completed dataset cell. The original campaign scope and
solver gaps remain in [REMAINING-COVERAGE.md](REMAINING-COVERAGE.md).

## Prepare one authoritative campaign

The controller supplies a frozen `crypto.ml-dataset.budget-policy/v1` JSON record
and a finite grant request. The policy names the exact allocator path and Linux
host boot identity, original deadline, approved remote prefixes, regional price
and resource evidence, prior obligations and integer caps. The money ceiling is
USD 200 total for the existing shared campaign, not a new allowance. A grant
reserves resources from that balance and binds the exact Job/Pod UID, executing
source commit, binary, plan, protocol, shard and exact worker-journal path. Known prior costs and uncertain
outcomes cannot be silently reset to zero. A checked compute price floor covers
the reserved resource lifetime. A separately identified noncompute component
within prior obligations must cover conservative rounded request, storage and
download costs for the whole campaign caps; it is not charged twice.

Run input preflight with the exact intended executor before preparing the grant.
Its `source_commit` and `binary_sha256` identify the executor; the request's
`protocol_sha256` must use `protocol_file_sha256`, the complete protocol file
digest, rather than `freeze.protocol_sha256`.

Use preflight's `plan_sha256` for the request's plan binding. It hashes the
native compact serialization of the parsed `DatasetPlan`, in the same field
order used by generation and replay. `dataset plan` also reports this field.
Keep `plan_file_sha256` (and the planning command's legacy `sha256`) as the
separate saved-file identity: formatting or key order can change that file hash
without changing the native plan binding. The request's shard index/count must
match the intended invocation. Retain the preflight receipt before issuance.

```sh
crypto dataset budget admit --dry-run \
  --policy policy.json --ledger /durable/campaign.sqlite --request request.json
crypto dataset budget reserve --create-ledger \
  --policy policy.json --ledger /durable/campaign.sqlite --request request.json \
  --out grant.json
crypto dataset budget status \
  --policy policy.json --ledger /durable/campaign.sqlite
```

`admit --dry-run` does not create a directory, ledger, grant or cloud request.
A well-formed rejection still exits successfully as a report: check the JSON
`admitted` field, not only the process exit code, before reserving anything.
It reports hypothetical capacity if the authoritative ledger has never existed.
`reserve` commits the allocation before exporting the grant. Use `--create-ledger`
only for first initialization. Later reservations open the original ledger and
omit that flag. An exclusive authority marker survives an accidentally missing
database; deleting the database does not release any allocation. Retain the
ledger and marker together on a durable local filesystem with SQLite locking and
fsync semantics. Do not copy writable allocators between hosts or use a stale
backup to resume spending. Total loss of authority state requires reconciliation;
there is no automatic reset or quota refund.

A fixed finalization partition protects activation, completion, catalog and
accounting receipts from ordinary work. A separate finite per-cell `Resources`
JSON envelope is required for budgeted generation. Its six fields are unsigned
64-bit integers: `money_micro_usd`, `output_bytes`, `put_attempts`,
`read_attempts`, `download_bytes`, and `local_bytes`. Zero is a zero allowance,
not an unlimited setting. The grant's `resources` includes its `finalization`
partition; the remaining work partition must hold all cell reservations and
other work cumulatively. Reservations are not automatically refunded after
failure or completion. A cell allocation does not itself establish actual
compute cost or a hard filesystem-write limit.

Size finalization for activation and STS, completion objects, catalogs, lab
receipts, and growing SQLite audit checkpoints, including fresh verification
reads and permitted retries. The pre-cell minimum check of 64 KiB output,
two PUT attempts, four read attempts and 256 KiB download is only a floor; it
does not guarantee sufficient finalization capacity for an arbitrary cell.
Actual requests fail closed when the needed allocation is unavailable. A
trace-size setting alone does not bound request counts or metadata bytes.

## Reserve before creation, bind after observing the worker

When a Job has not yet been created, use an ownerless launch reservation. Do not
invent Job or Pod UIDs to obtain a direct grant. The request pins the exact EKS
cluster ARN, namespace and Job name in addition to the same resource, executor,
plan, protocol, shard, journal and expiry fields. See the exact JSON schemas in
[the frozen reservation contract](PRELAUNCH-RESERVATION.md).

```sh
crypto dataset budget reserve-launch --dry-run \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --request launch-request.json
crypto dataset budget reserve-launch --create-ledger \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --request launch-request.json --out launch-reservation.json
# External launcher creates/reconciles the exact reserved Job and retains evidence.
crypto dataset budget bind-launch \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --binding launch-binding.json --out grant.json
```

Use `--create-ledger` only for the first authoritative initialization. On exact
retries, omit it and retain the same request, target and original ledger. The
dry-run command forbids output and initialization options, creates no files,
and requires inspecting `admitted` in its report. A successful dry run neither
reserves resources nor creates a Job.

The committed reservation consumes its complete envelope before submission.
Use the returned `reservation_sha256` in the binding: it identifies the native
typed serialization, independently of the reservation file's formatting. The
binding names the observed target, actual Job/Pod UIDs and SHA-256 of the retained
observation evidence. The trusted launcher must verify those external facts;
the allocator checks their consistency and digest format, not Kubernetes itself.

Binding moves the same allocation into a worker grant in one transaction.
Pending and bound reservations stay in status permanently, including expired
or failed attempts. Exact reserve/bind retries re-export identical records;
changed requests, targets, owners or evidence are rejected. Recovering an expired
receipt does not authorize new submission or activation. A pending reservation
cannot activate a worker, and direct `reserve` cannot reuse its launch ID.
Ambiguous submission must be reconciled under the original Job name.

These commands make no cloud requests and do not submit or deliver a grant.
The [launch adapter](LAUNCH-ADAPTER.md) still requires actual owner/placement
reconciliation, finite resource lifetime and the separately reviewed
[startup delivery mechanism](STARTUP-GRANT-DELIVERY.md). Its proposed Kubernetes
permission and the worker S3 policy remain unapplied.

## Activate exactly one worker journal

Inside the intended worker, use the executing binary named by the grant:

```sh
crypto dataset budget activate \
  --budget-policy policy.json --budget-grant grant.json \
  --budget-owner owner.json --budget-state /work/budget/worker.sqlite
```

The owner document contains the grant's exact `job_uid` and `pod_uid`. The launcher
must reconcile those with the actual Kubernetes objects and distribute a grant
only to its designated worker; comparing the supplied document is not an
independent Kubernetes attestation. See the [launch-adapter contract](LAUNCH-ADAPTER.md)
for the required handoff between controller and worker. Activation
creates the worker journal, conditionally writes its immutable nonce-bearing
control object, freshly downloads and checks that object, then makes the local
allocation spendable. Every network attempt consumes the protected allocation.
If activation is interrupted, use the same journal and `--resume`; an existing
activation object with another journal nonce cannot activate a new balance.
The grant fixes the absolute journal path, and a retained authority marker rejects
reinitialization after database loss before any credential or activation request.
Losing the journal does not authorize recreating its remaining quota.

Same-host retries require the original journal path, authority marker, grant,
owner and Linux boot identity. A replacement host or changed boot identity
requires reconciliation and a separately issued grant from the retained
authoritative allocator. Keep the original allocation charged or reserved;
do not reconstruct or refund its balance from an audit checkpoint. The allocator
is also bound to its original path and host boot identity, so copying its
database is not supported automatic recovery.

Credentials are explicit. EKS IRSA uses `AWS_ROLE_ARN` and
`AWS_WEB_IDENTITY_TOKEN_FILE`, with the exact role admitted by the grant. STS
bootstrap and refresh requests are counted before transmission and have a bounded
response. No ambient default credential chain, IMDS lookup, CLI transfer manager
or listing request is used. Static credentials are also accepted as an explicit
pair; temporary static credentials additionally require a session token and
`CRYPTO_DATASET_AWS_EXPIRATION_UNIX`. Never put credential values or token contents
in a policy, receipt, Git commit or dataset feature. Expired credentials stop
before a data request.

The transport uses fixed us-east-2 S3 and STS endpoints. It rejects mixed static
and IRSA credential tuples, configured store endpoint overrides, nonempty
`HTTP_PROXY`/`HTTPS_PROXY`/`ALL_PROXY` values (including their lowercase forms),
and `AWS_ENDPOINT_URL`, `AWS_ENDPOINT_URL_S3` or `AWS_ENDPOINT_URL_STS` overrides.
This is a deployment compatibility requirement; an environment that requires a
proxy needs a supported, admitted deployment path before launch.

## Generate, publish, fetch and restore

Pass all four `--budget-*` inputs to S3 generation, publication, exact-object fetch
and restore. Ordinary operations open an existing active journal; only explicit
activation creates one. Generation also requires `--budget-cell-envelope` and
reserves that cell's allocation before starting work. See the concrete pilot
commands in [OPERATIONS.md](OPERATIONS.md).

On generation resume, keep the same cell envelope: an existing cell reservation
cannot change. With `--publish`, completed cells receive fresh publication checks
before new computation, and every request still consumes budget. `--max-cases`
counts only new attempts; it does not limit those rereads. Standalone
`dataset publish` instead uses unallocated Work capacity. Unused capacity held
inside a Cell reservation is not automatically available to that command;
admit publication allowance separately when using that route.

`dataset budget fetch` downloads one known object using an exact key, SHA-256 and
byte length. Its key is relative to the supplied remote prefix. It is intended
for portable catalog receipts needed by a fresh restore; it does not discover
unknown keys or list a bucket. A new destination is required, and it is installed
only after verification. Fetch and restore leave local audit checkpoints.

Restore and republish bind the *current executor* to their grant while preserving
the imported producer's original source and binary identity. They still check the
original plan, protocol and shard. Generation resume requires the original
producer build. Budget snapshots live in separate restricted sidecars: changing
an accounting grant cannot change the bytes of an immutable cell receipt.
A fresh-host reader uses its own issued and activated grant, rather than the
terminated producer's journal or a copied checkpoint.

## Interpret the counters precisely

- Request counts are conservative charged attempts. Charging precedes the send;
  a crash between charge and transmission may overcount. Each permitted retry
  consumes another unit. Unknown outcomes remain charged.
- GET and HEAD share the read-attempt ceiling, while status retains their separate
  counts. STS bootstrap/refresh also consumes an explicitly recorded conservative
  read unit and response-byte reservation.
- Reserve output bytes before a conditional PUT. Record verified unique bytes
  only after a matching fresh full GET, length and digest check. A conflict alone
  never proves that the existing object is correct.
- Work cannot consume the finalization partition. Exhaustion stops further work;
  it does not make unpublished or unverified artifacts durable.
- Local audit checkpoints cover a journal prefix. Publishing a checkpoint creates
  later accounting events, retained by a subsequent local checkpoint. Checkpoint
  copies are evidence, not permission to restore an older spendable balance.

The current local trace-size poll is a stopping mechanism, not a strict checked
write limit. Process stdout, later metadata and checkpoint bytes still require
space. Full checked-writer and artifact-count enforcement remains explicit work.
Likewise, native request accounting cannot stop an unrelated AWS principal or
prove an account-wide billing cutoff. Actual resource lifetime, earlier runs,
regional price evidence and retained authority state remain part of admission.
