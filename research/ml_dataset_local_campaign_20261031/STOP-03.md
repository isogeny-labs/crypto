# Third local-XFS admission stop and resume

The same frozen controller advanced through shard34, with each new cell
independently verified, strictly exported and read back from restricted S3.
It stopped before shard35 because XFS free space fell below the frozen 4 GiB
floor. No shard35 attempt marker existed at that stop. A subsequent inspection
found no active old controller/worker and measured 5,515,427,840 bytes free,
so the controller resumed from its immutable receipts without regenerating
indices0..34. The active resumed command is the same `campaign.py run` under
local session5583. The separate tmpfs fallback has no frozen protocol, native
plan, or generated cell and does not overlap this campaign.
