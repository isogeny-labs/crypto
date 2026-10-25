# Spot migration reconciliation

Observed at `2026-10-24T23:02:38Z`. This note records continuation state; it
does not add an experimental result.

## Preserved work

- The approved handoff receipt hashes to
  `a16037f61139810dcb13bea313349a2f9d4f1a43471c7ffc04ea3e1828dac0c1`.
  It records a verified workspace and a stopped source Codex.
- The managed Diem owner reconciled the native generator and exited
  successfully. Its runtime receipt reports `deadline-expired`, 50 durable
  records out of 52, and no pending process. The original deadline is past,
  so this continuation does not relaunch the generator.
- The quiesced source-side pencil follow-up is present in the migrated
  `crypto-pr2304-external` snapshot at
  `b5ca4fe417990d81c459f0f5d7ec6f6bae5ac183`. Its saved cap runner, replay
  runner, status, and JSONL hashes are recorded in the machine-readable
  companion file. The preserved target is a completed
  `sparse_matrix_cap` outcome with a verified rho control and no verified
  pencil candidate. The local exact-sparse continuation remains outside the
  authorized lane, so the saved process is not repeated.

## External jobs

The `isolab` EKS control plane is active. The migrated worker identity has
scoped access and cannot list Kubernetes jobs or pods. The six most recent
`isolab-experiment-launcher` builds are all terminal (five succeeded and one
failed); the latest run object observed in the results bucket was written at
`2026-10-24T22:25:35Z`. This is evidence about the launcher and durable
objects, not a claim that every Kubernetes object has terminated.

N6, N7, and N8 use local, checksum-bound inputs. They do not submit, replace,
or alter EKS jobs. This avoids duplicating preserved external work while the
scoped job-submitter lane remains the only mutation path.

The exact fields, identities, hashes, counts, and decisions are in
[`MIGRATION_RECONCILIATION.json`](MIGRATION_RECONCILIATION.json).
