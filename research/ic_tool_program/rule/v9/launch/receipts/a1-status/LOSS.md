# Attempt a1: lost to a Spot interruption

- Job `rule-v9-20261106-a1` (UID 5a545fed-4434-4e26-bda4-ce55bee21c5e), pod
  33053ca8-de9a-4639-bea4-a129ce6d5461 on ip-10-84-68-133 (AMD EPYC 9R45), started 22:11:18 UTC.
- The EKS audit log (`/aws/eks/isolab/cluster`, pod uid filter, 22:15–22:30 UTC) records
  "Pod was terminated in response to imminent node shutdown" (TerminationByKubelet), exit code 137,
  then `BackoffLimitExceeded` at 22:22:14 UTC (retries 0). The node no longer exists in EC2.
- The last S3 sync was 22:21:34 UTC: v9 at k1n47 holds rows T01–T29 (R1); no v9c size row was started.
  `job-exit.json` and `SHA256SUMS` were not written (the pod was killed before the exit trap's upload).
- Status read 2026-11-11 00:20 UTC through `remote/launch-remote-command` (status request); receipt files here.
- Resumed as attempt a2 from the a1 prefix (`request-a2.json`); v9's k1n47 is split across the two pods.
