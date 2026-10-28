# Partial artifacts and cumulative S3 quotas

`dataset generate --publish` retains its frozen attempt context before starting a worker and periodically publishes newly acknowledged native events and payloads. A helper process performs uploads; the supervisor continues enforcing the solver's original deadline. `--snapshot-seconds` defaults to 30 and `--snapshot-timeout-seconds` to 120. A failed upload stops the wave. Its local sealed artifacts and earlier verified receipts remain available.

This preserves exposed artifacts. It does not save opaque solver memory, resume matrix elimination or SAT state, establish a complete scientific record, or prove a fall degree. Partial snapshots cannot become completed-cell manifests. Existing final cell publication remains separate.

## Admission before S3 operations

The native dataset CLI requires all three flags for S3 publication or restore:

```text
--budget-policy /absolute/path/policy.json
--budget-allocation /absolute/path/allocation.json
--budget-journal /absolute/path/worker-journal
```

The strict policy uses schema `crypto.ml-dataset.budget-policy/v1`. It includes `campaign_id`, an absolute `authority_dir`, approved `{bucket,prefix}` destinations, `command_timeout_secs`, and six cumulative integer limits: `upload_bytes`, `download_bytes`, `put_attempts`, `get_attempts`, `head_attempts`, `list_attempts`. The allocation has the same six limits and a finalization reserve that is unavailable to bulk snapshots and restore.

Initialize a **new** authority once and issue an envelope before using it:

```sh
crypto dataset budget-init --policy /absolute/path/policy.json
crypto dataset budget-allocate \
  --policy /absolute/path/policy.json \
  --allocation-id worker-attempt-001 \
  --limits /absolute/path/allocation-limits.json \
  --finalization-reserve /absolute/path/finalization-reserve.json \
  --out /absolute/path/allocation.json
```

These commands do not launch workers. Allocation limits must fit the campaign's remaining envelope. Failed, timed-out or uncertain requests stay charged, including failed checksums and conditional-write collisions. The controller authority and worker journal must remain available; a stale or missing established journal fails closed. Do not delete journals to retry. A lost allocation remains charged in full and requires a new, separately affordable envelope.

Independent workers may briefly contend for the controller while opening their
separate allocations. Controller acquisition retries contention and interrupted
calls for at most 30 seconds on a monotonic clock. Each allocation's session
lock remains exclusive and fails immediately on contention. Once the allocation
history has been checked and that session lock acquired, journal replay releases
the shared controller so unrelated workers can proceed. Lock pathname identity
is rechecked across waiting; replacement is an error. This deadline bounds lock
retries, not a filesystem operation stalled on an unavailable mount.

The controller authority must be on durable mounted storage that is not rolled back with a worker snapshot. All replacement hosts must use that same authority. This release does not install the mount or implement an S3-based controller authority. An S3 copy of a worker journal is evidence, not permission to spend its previously recorded remaining balance again.

The counters reserve object-body bytes and explicit AWS CLI command attempts. They do not represent exact wire traffic, service invoices, EC2 runtime, unrelated clients or account-wide dollar enforcement. Dataset compute remains subject to the user's separate **USD100–200 total through tomorrow, maximum USD200** campaign limit and launch admission. No new spending authority follows from a policy example or passing unit test.

## Find and restore retained partial data

The supervisor retains helper requests and results under `partial-snapshots/<cell>/<attempt>/helpers/` and immutable snapshot acknowledgements under the corresponding `snapshots/<ordinal>/` directory. `anchor.json` identifies a verified remote chain. The remote catalog uses a separate `ml-dataset/interrupted-attempts/` namespace. Every acknowledged object is downloaded and checked after publication; merely observing a successful PUT is insufficient.

A helper freezes its exact captured request before network publication. Reusing that request after a failed upload preserves the snapshot's ordinal and content; recapturing a newer event head into an old ordinal is rejected. The supervisor does not automatically retry a failed cloud attempt.

Restore artifacts into a new destination with:

```sh
crypto dataset restore-partial \
  --anchor /absolute/path/anchor.json \
  --out /absolute/path/restored-artifacts \
  --remote s3://setec-astronomy-artifacts/ml-dataset/20261028/store \
  --budget-policy /absolute/path/policy.json \
  --budget-allocation /absolute/path/restore-allocation.json \
  --budget-journal /absolute/path/restore-journal
```

Restore verifies the manifest/receipt chain and every referenced object's size and hash before acknowledging availability. It writes under `interrupted-attempts/`, does not populate `cells/`, returns `solver_resumed:false`, and invokes no solver. Starting a new attempt still requires source, owner, job and budget reconciliation.

Witness material retains its restricted-label role. Public/model feature export must continue using the existing allowlist; the existence of an artifact in a snapshot does not make it a permitted training feature.

## Operational status

The worker-role S3 access change remains separately pending approval. No IAM policy, EKS deployment or new research trial is created by this change. A verified native producer/cloud/independent-consumer roundtrip and a real interruption/replacement exercise remain required deployment evidence. Software fixture tests do not constitute the requested extensive solved corpus.
