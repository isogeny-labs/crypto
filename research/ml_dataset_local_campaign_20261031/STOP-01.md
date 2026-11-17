# First local-XFS campaign stop

The frozen 64-cell campaign began once at 2026-10-31T00:57:06Z under
Conductor session `dc4e4943` and controller `campaign.py run`, after native
preflight and a fresh readback of the S3 Git checkpoint for source HEAD
`74741a06973d6d317c19df48d4a55551d21a6f10`. The campaign root is
`/var/tmp/ml-dataset-local-campaign-20261031`; its immutable admission and
per-shard receipts retain the exact commands and outcomes.

Indices 0 through 6 each produced one verified BSGS ECDLP cell, native report,
strict cyclic-subgroup export, restricted archive and custody pointer. Both
archive and pointer passed fresh S3 GET byte comparisons for each index. The
seven `checkpoints/shard-XX.json` files give the content-addressed S3 keys and
exact hashes; they are part of this Git checkpoint. These rows cover a subset
of the frozen seed/generator/key/twist population and do not complete the
planned matrix.

The controller then stopped before creating the shard-07 attempt marker:
`less than 4 GiB XFS free; stop before new attempt`. At the subsequent disk
check, `/var/tmp` was XFS with 4,202,561,536 bytes available, below the
frozen 4,294,967,296-byte floor. The controller exit was 1; its Conductor
wrapper also reported that `bash` harness checkpointing is unsupported.
There is no active controller or shard-07 worker. The content-addressed S3
archives and Git receipts preserve all seven attempted cells. A resume must
first reconcile the live process and local/S3 receipts, confirm at least 4 GiB
free and the original 12-hour campaign deadline, then use the same controller;
its exclusive lock and attempt markers prevent a duplicate attempt.

This is an infrastructure-cap stop, not an observed solver failure or a reason
to replace planned cases. No timing comparison or extrapolation is drawn from
these seven cells. The remaining 57 indices have no attempt under this frozen
local-XFS campaign.
