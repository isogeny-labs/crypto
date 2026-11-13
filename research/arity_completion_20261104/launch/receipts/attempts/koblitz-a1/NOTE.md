# Koblitz attempt 1 (Job arity-koblitz-m56-20261104): Spot interruption

- Launched by CARD-arity-launch-koblitz-r2 (launcher 672afd98); Job bfa5cf9f, pod
  a6009182 on c8a.2xlarge ip-10-84-90-176, started 16:02:01Z.
- 16:07:43Z "Pod was terminated in response to imminent node shutdown"
  (Spot interruption); container exit 143; Job failed (backoffLimit 0).
- The exit handler uploaded 25 files; `sha256sum -c SHA256SUMS` passes locally.
  Six m = 2 negative-control records are complete (icv1-f2m23-t5197-69e76b73, icv1-f2m29-tm40309-30c52b96, icv1-f2m31-tm90707-c95f16f5 × both
  engines); the seventh (icv1-f2m37-tm534059-32aad96b, chain-s3-orbit) was in flight (empty stubs).
- Defect: `job-exit.json` says exit_code 0 because job-lib.sh traps EXIT but not
  TERM; Kubernetes records the true 143. Not fixed in the frozen image; the
  retry's records are read with the pod status beside them.
- Retry (protocol: infrastructure retries only, at most 3 attempts) resumes
  from this pod's S3 prefix; run_shard.sh skips the six existing records.
