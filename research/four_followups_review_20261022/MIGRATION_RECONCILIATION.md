# Spot migration reconciliation

Observed initially at `2026-10-24T23:02:38Z` and refreshed after the next Spot
continuation at `2026-10-25T01:15:12Z`. This note records continuation state; it
does not add an experimental result.

## Preserved work

- The approved handoff receipt hashes to
  `a16037f61139810dcb13bea313349a2f9d4f1a43471c7ffc04ea3e1828dac0c1`.
  It records a verified workspace and a stopped source Codex.
- The managed Diem owner reconciled the native generator and exited
  successfully. Its runtime receipt reports `deadline-expired`, 50 durable
  records out of 52, and no pending process. The original deadline is past,
  so this continuation does not relaunch the generator. The managed service
  refreshed the receipt observation timestamp on 2026-10-25 and exited
  successfully again; the state and durable-record count did not change.
- The quiesced source-side pencil follow-up is present in the migrated
  `crypto-pr2304-external` snapshot at
  `b5ca4fe417990d81c459f0f5d7ec6f6bae5ac183`. Its saved cap runner, replay
  runner, status, and JSONL hashes are recorded in the machine-readable
  companion file and were rechecked byte-for-byte after continuation. The
  preserved target is a completed
  `sparse_matrix_cap` outcome with a verified rho control and no verified
  pencil candidate. The local exact-sparse continuation remains outside the
  authorized lane, so the saved process is not repeated.

## External jobs

The `isolab` EKS control plane is active. At the refreshed observation the
scoped worker identity could list the namespace. All 28 retained jobs were
terminal: 18 succeeded and 10 failed. No job was active or incomplete, no pod
was present, and no job name matched the pencil, Diem, genus, Kummer,
factor-base, projective-law, or four-followup lanes. The six previously
observed `isolab-experiment-launcher` builds were also terminal (five succeeded
and one failed); the latest previously observed durable run object was written
at `2026-10-24T22:25:35Z`.

N6 through N9 use local, checksum-bound inputs. They do not submit, replace,
stop, or alter EKS jobs. No external job was repeated during reconciliation.

The exact fields, identities, hashes, counts, and decisions are in
[`MIGRATION_RECONCILIATION.json`](MIGRATION_RECONCILIATION.json).
