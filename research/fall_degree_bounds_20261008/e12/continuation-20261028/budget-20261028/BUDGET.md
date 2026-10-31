# Total campaign budget and dataset durability

Recorded 2026-10-28 UTC. The user supplied a **USD100–200 total budget through
tomorrow**. This is a cumulative campaign allowance, not a daily allowance or a
per-worker allowance. Aim toward USD100; do not admit work beyond USD200.

This record conservatively includes the full original E12a/E12c authorizations
and the completed E12b run. It changes no existing deadline or frozen input and
launches no new instance. The independent dataset implementation is already owned
by Conductor T-684; this budget/readback record is T-690. Preserve one owner and
one campaign; do not create a duplicate generator or retry an existing run.

## Resource reservation

The retained `aws-rates/*.json` responses are from the AWS Price List API,
queried on 2026-10-28 for Linux, Shared tenancy, us-east-1. On-Demand rates are
used for conservative Spot reservations, not as measurements of Spot billing.
The existing launcher uses one-time Spot requests and finite hard lifetimes.

| Allocation | Calculation | Reserved USD |
|---|---:|---:|
| Existing E12a, r6a.8xlarge | 48 h × 1.8144 | 87.0912 |
| Existing E12c, r7i.4xlarge | 48 h × 1.0584 | 50.8032 |
| Completed E12b, m7i.4xlarge | conservatively 1 h × 0.8064 | 0.8064 |
| Proposed single c6a.4xlarge worker | at most 24 h × 0.612 | 14.6880 |
| S3 storage and requests, first month | reserve | 15.0000 |
| EBS and transfer | reserve | 10.0000 |
| Contingency | reserve | 10.0000 |
| **Total reservation** | | **188.3888** |

Remaining admission headroom is **USD11.6112**. Existing jobs might finish early
and current Spot prices are lower; neither is counted as a saving in this
reservation. The proposed worker is **not launched**. Its next wave is limited
to 20 GiB of unique output, 100,000 PUT requests and 200,000 GET requests. These
are proposed admission limits requiring enforcement in the owning launch path,
not claims that an account-wide billing guard has been installed. Retaining
objects after the first month incurs storage charges outside this reserve;
no dataset deletion or lifecycle change is authorized here.

Run `python3 verify_budget.py` from this directory to recompute the total from
the retained AWS responses. An additional reservation is accepted only if the
sum stays within USD200. The verifier does not create, stop or modify cloud
resources. Before launch the active owner must bind the budget to the actual
instance, concurrency, original deadline, immutable source/protocol, output
caps and durable worker state. New waves do not reset accumulated reservations.
Fresh pricing and an inventory check are required if the launch shape changes.

## Verified S3 state

At 2026-10-28T04:55:50Z, all three selected archives were downloaded and their
SHA-256 digests checked independently against their published snapshot records.
The exact keys and hashes are in `s3-readback.json`.

| Run | Execution state at inspection | Completed new draw rows in selected archive |
|---|---|---:|
| E12a | existing instance running; recurring snapshots active | 0 |
| E12c | existing instance running; recurring snapshots active | 0 |
| E12b | completed and instance terminated | 4 |

E12b has 32 original final output objects. Its raw four-row dataset is 1,686
bytes with SHA-256
`055811b722f441ab681172a605f9a540b174734dbb4f23ff9f03b931d948c81b`.
The E12a/E12c snapshot timestamps have advanced since the original installation;
unchanged archive hashes mean their files have not changed, not an upload
failure. File snapshots preserve outputs and logs, but do not save solver memory.
No degree result or independent scientific verification is asserted here.

The existing E12 artifacts remain in `ephemeral-runs-104186811093-us-east-1`.
New dataset and handoff artifacts use the canonical
`setec-astronomy-artifacts` bucket in us-east-2. Immutable objects precede their
manifests; a successful PUT or matching size alone is insufficient. Require a
separate GET and hash match before recording a shard as durable.

## Sources and session handoff

Sources inspected: retained AWS Price List responses;
[AWS Spot request lifecycle](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-requests.html);
[S3 conditional writes](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html);
the existing `snapshot-to-s3.sh`, `S3_PERSISTENCE.md`, and launcher; current EC2
inventory; and the selected S3 manifests and archive bytes.

### What was done

Reconciled active E12 jobs and the already-owned dataset worktree, queried
current AWS rates, recomputed the conservative reservation, and downloaded and
hashed the latest selected E12 output archives. No experiment was started.

### Decisions made

Treat USD200 as the total ceiling. Preserve active implementation ownership and
existing scientific protocols. Include existing authorized compute in the
reservation and retain funds for persistence before considering a new worker.

### Open threads

The dataset owner must incorporate the actual budget gate and storage caps into
its launch path before a new wave. Cross-chat messaging requires explicit user
authorization. Full generator coverage, scientific replay of E12b, E12a/E12c
completion, and the broader ML corpus remain outstanding.

### Errors

An initial ownership assumption identified the dataset task as this chat's
task. Conductor's conflict and concurrently appearing files exposed the active
owner. Inspection confirmed another user-owned chat; overlapping assignments
were stopped before edits or launches. The shared daily log is reserved by
other active work; this scoped session handoff preserves the required facts
without editing their files.
