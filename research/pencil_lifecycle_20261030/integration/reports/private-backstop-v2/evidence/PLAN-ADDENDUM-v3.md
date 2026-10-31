# Tightened orchestration, source-only v3

Use run-reviewed-container-v3.sh for the next execution review. Earlier versions remain preserved and are superseded, not executed.

Failed or ambiguous Docker create now enters exact-name reconciliation. Name and unique owner label are saved before create. Cleanup inspects that name and accepts an ID only when the complete ID format, exact name, pinned image ID and owner label match. Full inspection JSON retains the discovered ID. Identity mismatch or unavailable inspection produces an explicit unresolved-create record and no guessed kill/removal. Subsequent cleanup repeats the exact-ID/name/image/label check.

Cleanup now runs in one owned subprocess under timeout29s with a1s KILL grace: overall30s guard. Internal CLI calls also consume one shared remaining deadline, set conservatively to28s in that subprocess. This outer guard includes blocking CLI and local I/O inside the cleanup worker. Expiry writes a distinct CLEANUP-INCOMPLETE reconciliation note rather than claiming successful removal. Host kernel uninterruptible waits cannot be guaranteed reaped by userspace timeout; pending process/resource custody remains explicit if encountered.

The fixture, deadline source and image recipe remain unchanged. bash -n passed for v3. No Docker, sudo, build, fixture, watchdog, host kill or cloud command has been executed. Readiness is still source-only: private lockfile resolution/static build and actual isolated-container checks require root's next review.
