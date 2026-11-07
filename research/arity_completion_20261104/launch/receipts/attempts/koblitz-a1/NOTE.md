# Koblitz attempt 1 (Job arity-koblitz-m56-20261104): Spot interruption

- Launched by CARD-arity-launch-koblitz-r2 (launcher 672afd98); Job bfa5cf9f, pod
  a6009182 on c8a.2xlarge ip-10-84-90-176, started 16:02:01Z.
- 16:07:43Z "Pod was terminated in response to imminent node shutdown"
  (Spot interruption); container exit 143; Job failed (backoffLimit 0).
- The exit handler uploaded 25 files; `sha256sum -c SHA256SUMS` passes locally.
  Six m = 2 negative-control records are complete (k0n23, k1n29, k0n31 × both
  engines); the seventh (k0n37, chain-s3-orbit) was in flight (empty stubs).
- Defect: `job-exit.json` says exit_code 0 because job-lib.sh traps EXIT but not
  TERM; Kubernetes records the true 143. Not fixed in the frozen image; the
  retry's records are read with the pod status beside them.
- Retry (protocol: infrastructure retries only, at most 3 attempts) resumes
  from this pod's S3 prefix; run_shard.sh skips the six existing records.
