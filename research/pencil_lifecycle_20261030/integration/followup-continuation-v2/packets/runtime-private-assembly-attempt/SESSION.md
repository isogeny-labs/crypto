# One private assembly attempt

## What was done
Verified published revision2b037c3096 and exact launch/source/review bytes before running the one admitted command. Worker produced image sha256:3a2bb98af4743c888d969c9df00f0769a64e3704e57e97e937769d384e3c8409, unmounted its private filesystem/shim and saved disk custody. Launcher exited1 after33.701950079seconds; journal reports9.246CPU-seconds. Original receipts copied byte-for-byte.

## Decisions made
Preserve original unresolved result. No retry, rebuild or qualification. Keep original 1200-second conservative debit and USD1 allocation. Investigate and independently review read-only cleanup reconciliation. Original source and native receipt remain unchanged.

## Open threads
Correctly parse explicit losetup list output, independently bind stopped service/cgroup, disk custody and detached loop state, then construct qualification bindings as a separate reviewed phase. Disk retained through06:00UTC allocation.

## Errors
Original reconcile called losetup --json --associated without --list. On this host it exits0 with emptybytes, causing JSONDecodeError. Exact explicit --json --list --associated read returns an empty loopdevices list. The followup stop attempt raisedValueError because the service was already gone. Both original errors remain recorded; a new read-only disposition must not overwrite them.

## Read-only disposition

After exact helper publication at80c54bf9aa, one30-second-bounded read-only invocation exited0 at19:13:43UTC (outer0.208919seconds). It observed assembled_stopped_observed, repeated not-found/inactive/dead service and absent cgroup, empty associated and full loop inventories, unchanged original receipt bytes and disk custody. The worker disk digest is inherited, not rehashed. No mutation/rebuild/qualification occurred.
