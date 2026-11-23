# Root v4 permit repair — prospective source only

Preserve v3 failed/sealed attempt and all costs. No automatic retry. Unchanged root validation: four exact controls then unfiltered suite, 1800 seconds, 2 CPUs, 16 GiB, 10-second permit, 15-second lease safety and original absolute deadline. Current ledger/lease/publication/cache bindings are unset; admission is false.

Worker repair: only an expired first snapshot permits one immediate second read, with strict invocation/complete lease identity, finite numeric expiry, 250 ms monotonic reread bound and final unexpired check. Malformed/identity errors never retry. No sleep or grace. Anomalies retain both parsed snapshots and raw-byte digests/timestamps; at most32 records are retained. Oversized permit files (>4096bytes) fail closed. A fresh second permit is authority only at its final check; no inference from eventual supervisor state.

Supervisor order: heartbeat validates the lease and records its result in memory; cumulative receipt flush is deferred. show captures and validates current InvocationID and ControlGroup, then atomically/fsynchronously publishes a permit before expensive cumulative checkpoint/log flush. The permit itself durably records owner identity, sequence and issuance/expiry. No permit is minted while CLEANING/interrupted or when current unit is terminal. Changed owner/cgroup aborts before publication. Existing crash cleanup still uses captured owner; owner permit provides another durable identity record if the later checkpoint fails. The subsequent checkpoint remains mandatory, and its failure still stops/cleans the attempt. Neither lease nor permit deadlines are extended. This ordering change requires independent review, particularly first ownership capture and cleanup behavior.

Publication diagnostics retain the last32 issue/expiry/write-duration records. The old growing observation receipt is not truncated. This is a bounded repair/diagnostic; the source of v3 expiry remains unproved.

One proposed qualification invocation uses the actual extracted worker read/check functions and launcher show/permit functions against fixed in-memory inputs.20 cases cover fresh/stale/expired, wrong identity, malformed/oversize/nonfinite expiry, bounded reread, signal/deadline, publication-before-checkpoint, lease margin, cleanup/interruption/terminal and owner change. One structural case verifies heartbeat no longer flushes before owner validation. These software fixtures do not establish scheduler/filesystem latency or live daemon correctness. No network, compiler or native payload is started.

After independent review and exact source publication, root may reserve60 seconds for this single command (50s internal alarm;55s TERM plus5s kill;512MiB address limit and1MiB per-file ceiling):

```sh
( ulimit -v 524288
  ulimit -f 1024
  /usr/bin/timeout --signal=TERM --kill-after=5s 55s /usr/bin/python3 -B /home/ec2-user/crypto-pencil-evidence-recovery-20261105/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-library-validation-v4-20261109/qualify-permit.py --output /var/tmp/pencil-root-v4-permit-qualification-20261109
)
```

V3 compilation mutated the private cache. A NEW60-second bounded snapshot using unchanged snapshot-cache.py is required at a serial boundary after all other payloads end, with new output path /home/ec2-user/pencil-validation-receipts/root-v4-cache-snapshot-20261109. Reuse prior snapshot command's1CPU/1GiB/55sRuntimeMax+3sstop and caller-owned logs, bind actual new current-cache.json/result/terminal. Do not reuse v3 cache inventory or regenerate immutable inputs. Retain initial materialized manifestc5178... and exact source/toolchain hashes.

Only then finalize actual current ledger, fresh owner lease, reviewer and qualification/cache receipts, publication/readback and serial terminal into a new admission file. Parent decides one manual1800-second attempt; this draft allocates none. Qualification60+snapshot60+attempt1800 are conservative reservations, not authorizations or renewed campaign time.
