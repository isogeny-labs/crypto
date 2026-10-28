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
JSON envelope is required for budgeted generation. The request, envelope and
policy must agree on finite counts and bytes; a trace-size setting alone does not
bound the number of publication requests or metadata bytes.

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
independent Kubernetes attestation. Activation
creates the worker journal, conditionally writes its immutable nonce-bearing
control object, freshly downloads and checks that object, then makes the local
allocation spendable. Every network attempt consumes the protected allocation.
If activation is interrupted, use the same journal and `--resume`; an existing
activation object with another journal nonce cannot activate a new balance.
The grant fixes the absolute journal path, and a retained authority marker rejects
reinitialization after database loss before any credential or activation request.
Losing the journal does not authorize recreating its remaining quota.

Credentials are explicit. EKS IRSA uses `AWS_ROLE_ARN` and
`AWS_WEB_IDENTITY_TOKEN_FILE`, with the exact role admitted by the grant. STS
bootstrap and refresh requests are counted before transmission and have a bounded
response. No ambient default credential chain, IMDS lookup, CLI transfer manager
or listing request is used. Static credentials are also accepted as an explicit
pair; temporary static credentials additionally require a session token and
`CRYPTO_DATASET_AWS_EXPIRATION_UNIX`. Never put credential values or token contents
in a policy, receipt, Git commit or dataset feature. Expired credentials stop
before a data request.

## Generate, publish, fetch and restore

Pass all four `--budget-*` inputs to S3 generation, publication, exact-object fetch
and restore. Ordinary operations open an existing active journal; only explicit
activation creates one. Generation also requires `--budget-cell-envelope` and
reserves that cell's allocation before starting work. See the concrete pilot
commands in [OPERATIONS.md](OPERATIONS.md).

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
