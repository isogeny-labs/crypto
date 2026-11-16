# AWS ARM64 Spot build host: launch rejection

Status: **blocked before allocation**. The
[frozen host protocol](AWS_ARM64_SPOT_BUILD_SMOKE_PROTOCOL.md) and
[exact user-data script](aws-arm64-spot-build-smoke-user-data.sh)
were committed at `fa6f5abcc4c7`. The script SHA-256 was
`1c67bba8399cd0ec6ed41bad08ab0af8ab718515bd271fa113f52f8347e61fe5`.
An otherwise identical `run-instances --dry-run` returned
`DryRunOperation` before launch, showing only that the IAM action
was allowed.

At 2026-11-15 17:31:10 UTC, the actual one-instance Spot
`run-instances` request used `ecc-challenges-new` in
`us-west-2` and the exact parameters and user-data file in the
protocol. AWS returned:

> Blocked: This account is currently blocked and not recognized as
> a valid account.

The operation exited 254. Fresh read-only `describe-instances`
and `describe-spot-instance-requests` queries filtered by the
unique `n83-k600-arm64-build-smoke-20261115` tag both returned
empty arrays. No build host was allocated, the user-data script
did not run, no source was fetched, and no S3 action was attempted.
This is an account-state/access failure, not a compiler, solver,
factor-base, or mathematical result. The requested larger-memory
build remains open.

Exit condition: an authorized account that can launch the exact
bounded ARM64 host (or a local Linux ARM64 host with more than 6 GiB
usable cgroup memory), followed by a clean-source, fresh-target build
that produces and hashes all five executables under a newly frozen
protocol. A dry-run success alone does not satisfy this condition.
No IC performance graph changed: there was no host run or IC value.
