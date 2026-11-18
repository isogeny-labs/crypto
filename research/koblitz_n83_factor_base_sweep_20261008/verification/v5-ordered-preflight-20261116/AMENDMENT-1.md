# Write-once amendment after attempt 1

Attempt 1 started at `2026-11-16T15:40:18Z` under the original
`PROTOCOL.md` and `run-preflight.sh`. The worker exited zero and wrote
`PREFLIGHT_ONLY` with `ordered-orbit-chain-s3`, zero trials, zero
relations, and zero SAT calls. The shell's `EXIT` trap then ran a second
`docker inspect` after `docker rm` and overwrote the first successful
inspection with `[]`. The first inspection cannot be recovered. Attempt
1 is therefore incomplete for the explicit Docker OOM/exit receipt
gate, despite its successful worker result. Preserve it unchanged.

Attempt 2 keeps the same source, binary, image, S3 object, fixture,
limits, and zero-trial command. `run-preflight-attempt-2.sh` only changes
the write-once output/container names and prevents the trap from
overwriting an existing inspection. It also requires exactly one
inspection record before the OOM assertion. Its output remains on
SSD990-2 because the existing writable boot mount is reserved by a
separate Conductor task. Both the boot volume and SSD990-2 must be at
least 2 GiB available at launch and during execution; the disk watcher
stops the container if either falls below that floor. If the headroom
is insufficient, preserve a stopped attempt and do not relax the floor.

The original pass gate and reporting limits remain in force. This
amendment makes no performance or factor-base selection claim.
