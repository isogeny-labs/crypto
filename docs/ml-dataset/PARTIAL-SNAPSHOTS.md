# Partial artifacts and cumulative S3 quotas

`dataset generate --publish` retains its frozen attempt context before starting a worker and periodically publishes newly acknowledged native events and payloads. A helper process performs uploads; the supervisor continues enforcing the solver's original deadline. `--snapshot-seconds` defaults to 30 and `--snapshot-timeout-seconds` to 120. A failed upload stops the wave. Its local sealed artifacts and earlier verified receipts remain available.

This preserves exposed artifacts. It does not save opaque solver memory, resume matrix elimination or SAT state, establish a complete scientific record, or prove a fall degree. Partial snapshots cannot become completed-cell manifests. Existing final cell publication remains separate.

## Admission before S3 operations

Live S3 publication and restore use the current campaign interface:

```text
--budget-policy /absolute/path/policy.json
--budget-grant /absolute/path/grant.json
--budget-owner /absolute/path/owner.json
--budget-state /absolute/path/worker.sqlite
```

The policy and grant bind monetary, output, request, download and local-byte
caps, approved destinations, campaign deadline, exact source/binary/plan/
protocol/shard and actual owner. Generation additionally requires a finite
`--budget-cell-envelope`. Use the existing `dataset budget` commands and
[operations guide](OPERATIONS.md) to prepare and activate an admitted grant.
A snapshot helper never issues another grant or creates another worker state.

The supervisor retains the original exclusive worker lease. Each bounded
helper inherits that same open-file-description lease across exec, validates
its actual journal-lock inode, owner, policy, grant, path, boot and activation
nonce, and resumes the same SQLite state. A separately opened owner is refused.
The original monetary and cumulative balances, attempt IDs and unknown-attempt
charges remain shared across parent and child. The real child-process software
control verifies exec inheritance, exclusive parent locking and retained charges;
complete integration/deployment qualification is recorded separately.

Periodic snapshots use the already reserved original Cell scope; terminal
publication uses protected Finalization capacity; restore uses Work. Every
known object is registered by exact digest and length before HEAD/GET. Outputs
are reserved before PUT and independently downloaded and hashed, including
retries and existing objects. All native S3/STS attempts pass current admission;
credential refresh is charged against its policy scope. There is no AWS CLI
fallback or second independently spendable legacy allocation.

The retained v1 `budget-init` and `budget-allocate` commands are offline receipt
compatibility tools only. Their original controller/anti-rollback controls and
unit tests remain, but their type cannot authorize the native live S3 transport.
Historical v1 records retain their original source and protocol attribution.

Worker journals remain bound to their authoritative absolute path and host boot.
Preserve allocation authority and the original journal; a checkpoint or restored
backup is evidence, not a fresh balance. No protection against malicious rollback
of the complete authority is claimed. Failed, timed-out and unknown requests
remain conservatively charged. This cooperative ledger is not an AWS billing
cutoff, launch authorization or additional campaign spending authority. Preserve
the user's existing USD100–200 total campaign limit, maximum USD200, deadline
and separate finite resource-lifetime/worker-access admission.

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
  --budget-grant /absolute/path/restore-grant.json \
  --budget-owner /absolute/path/restore-owner.json \
  --budget-state /absolute/path/restore-worker.sqlite
```

Restore verifies the manifest/receipt chain and every referenced object's size and hash before acknowledging availability. It writes under `interrupted-attempts/`, does not populate `cells/`, returns `solver_resumed:false`, and invokes no solver. Starting a new attempt still requires source, owner, job and budget reconciliation.

Witness material retains its restricted-label role. Public/model feature export must continue using the existing allowlist; the existence of an artifact in a snapshot does not make it a permitted training feature.

## Operational status

The worker-role S3 access change remains separately pending approval. No IAM policy, EKS deployment or new research trial is created by this change. A verified native producer/cloud/independent-consumer roundtrip and a real interruption/replacement exercise remain required deployment evidence. Software fixture tests do not constitute the requested extensive solved corpus.
