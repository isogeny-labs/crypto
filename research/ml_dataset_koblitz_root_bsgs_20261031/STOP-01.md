# XFS admission stop after index 10

The second bounded invocation used `campaign.py run --max-new-cells 8` after
indices 0..7 had complete custody. It returned exit 1 after publishing
indices 8, 9, and 10, with `campaign stopped: less than 4 GiB XFS free; stop
before new attempt`. A post-stop `df -B1 /var/tmp` read reported
3,883,970,560 bytes available. Index 11 has no attempt marker, launcher,
runner, or checkpoint. The 16-cell population is unchanged, and the next
invocation must reconcile local space, the three new custody receipts, and a
fresh Conductor reservation before attempting index 11.

An independent read-only audit checked indices 8..10 and confirmed native
verify/report/export success, exact local archive hashes, and producer S3
archive/pointer readback flags for each. It separately streamed and hashed
both S3 objects for index 10. The later free-space reading rose above the
floor; it does not replace the controller's recorded admission decision.
