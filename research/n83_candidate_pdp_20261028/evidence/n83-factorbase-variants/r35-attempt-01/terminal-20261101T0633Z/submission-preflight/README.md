# r35 fixed-base enumeration submission

This preflight package binds r35 to the already-frozen coordinate-variant protocol v9. It records the exact protocol, source index, inner runner command, wrapper, image digest and resource envelope before launch. The inner launcher obtains the 2^27 selector count and r=35 point coordinates, preserves the complete three-family bases, and asserts reuse for m=2..9. The job writes to its own S3 prefix using append-only content-addressed snapshots.

The job name is `n83-fb-variants-r35-20261031-r1`, as required by the frozen runner's attempt-to-prefix check. This r35 slot had no prior Job. The failed r34 Job is retained separately and has no producer outputs.

Files: `protocol.json` is the frozen v9 contract; `source-index.json` lists/hash-binds source objects; `freeze.json` binds bit and attempt; `job-command.sh.txt` runs the protocol-pinned producer; `wrapper-command.sh.txt` stages v9 files from successful r33 into this unique Job root; `preflight.json` records resources and checks.
