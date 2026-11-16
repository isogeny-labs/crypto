# Second local-XFS campaign stop and resume

After the first storage stop, XFS availability recovered above the frozen
4 GiB floor. The same controller resumed from its immutable local/S3 receipts
and produced verified, strictly exported, freshly read-back cells for indices
7 through 17. It then stopped before the shard-18 attempt marker because
available XFS space had fallen below the frozen floor. A post-stop check
measured 3,713,769,472 available bytes. This was a storage-admission stop;
no shard-18 solver attempt was made in that invocation.

Space later recovered to 9,308,643,328 bytes. With no active controller or
shard-18 worker and no shard-18 attempt marker, the same frozen controller
resumed under Conductor session `c95d0cd4`. The controller's exclusive lock
and one-attempt markers keep indices0..17 from being regenerated. Subsequent
per-cell custody is recorded in `checkpoints/shard-XX.json` as each cell
passes independent replay, strict export, and fresh S3 archive/pointer GET.
The XFS run remains the sole generator for this frozen 64-cell plan. A
separate tmpfs continuation is only provisional design; no tmpfs generation
has been launched.
