# V5 Linux exporter build: one capacity retry

Written before attempt 2. [Attempt 1](attempt-1/RESULT.md) exited 93
at the boot-volume admission floor before copying source or creating a
Docker container. Its raw evidence remains unchanged.

A read-only host inventory identified `/Users/adamburan/go/cache` as
Go's configured, rebuildable build cache (`go env GOCACHE`), occupying
about 9.9 GiB, with no Go build process running. After a Conductor scope
check and claim, `go clean -cache` completed with exit 0. A subsequent
`df -Pk /` showed **11,568,280 KiB** available, above the original
2,097,152 KiB admission floor. `du -sh` showed the cache at 8 KiB.
This cleanup did not alter the N83 source, input, prior receipts, or
the Colima VM. Its effect is a host-capacity observation, not an
algorithmic measurement.

Permit exactly one attempt 2 using the **unchanged** [frozen launcher](run-build.sh)
and its image, source pin, command, cgroup, disk floors, watchdog and
1,800-second outer cap. The launcher source/output paths remained absent
after attempt 1, so this attempt can use its write-once paths. Capture
a fresh prelaunch disk snapshot under `attempt-2/`. If any admission
gate still fails, retain the distinct exit and do not retry under this
amendment. Success still means only a verified Linux exporter build;
no solver work or total IC runtime follows.
