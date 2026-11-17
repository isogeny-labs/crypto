# r36 fixed-base enumeration submission

This preflight binds r36 to the already-frozen coordinate-variant protocol v9. It records the exact protocol, source index, runner command, wrapper, image digest and resource envelope before launch. The launcher enumerates the full 2^27 selector domain for the three curve families and checks reuse across m=2..9. Output is append-only under this job's S3 prefix.

The job name is `n83-fb-variants-r36-20261031-r1`, matching the frozen attempt binding. The EKS overview showed no existing r36 Job before preparation. The failed r34 attempt remains retained separately.

Files: `protocol.json`, `source-index.json`, `freeze.json`, both command records, `preflight.json`, and `SHA256SUMS.json` bind the prelaunch package.
