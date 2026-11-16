# Qualification attempt, 2026-11-03

## What was done
Published f4fd538dde and verified five GitHub file contents before one launch at19:33:00UTC. Outer elapsed30.240972875seconds; serviceCPU6.903seconds. Captured native receipts, journal, stopped/cgroup/loop state and extracted the qualifier receipt with readonly debugfs under30second/512MiB limits. Diskstat unchanged by extraction. No mount or daemon restart.

## Decisions made
Preserve failed attempt and600worker-second debit; remaining reserved campaign23400seconds. USD1qualification allocation and existingUSD56reservation unchanged; actual dollars not measured. No automaticretry. Original assembly disk digest remains historical; qualification worker records its newdigest.

## Open threads
Independent postreview; diagnose temporary-directory availability before proposing reviewed successor. Full matched cohort not launched. Full release-library check remains unverified after prior resource failure/timeout.

## Errors
Qualifier failed before all cases with FileNotFoundError: no usable temporary directory. Outer worker records commandfailed1, launcher serviceexitedbefore successfulsnapshot. Earlier heartbeat request used epoch instead of fencing_epoch and received409 without changes; corrected heartbeat succeeded before expiry.
