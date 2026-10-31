# E12b Spot launch: October 27, 2026 Pacific

The user selected one new Spot attempt for E12b. Amendment 15 was committed and
pushed at `020f03193c46d975935a065c966c81941cbdbf6d` before submission. This is a
launch record, not a completed experiment or a scientific reading.

| Field | Value |
| --- | --- |
| Job | `20261028-031317-fd-e12b-spot-20261028-r3-24148b` |
| Instance | `i-03d408f37b22432d1` |
| Host | `m7i.4xlarge`, 16 vCPU, 64 GiB, Spot, `us-east-1` |
| Cell | `3:24:7`, N=45, seed `20261012`, cap 4, four draws |
| Maximum lifetime | 24 hours; no automatic retry |
| Hard deadline | October 28 at 8:13:18 PM Pacific / `2026-10-29T03:13:18Z` |
| Receipts | [`receipts/b-spot/`](receipts/b-spot/) |

## What was done (commands, files, features)

Read the existing Amendments 11–14, frozen `run_par.sh`, prior E12b reference,
and ephemeral launch path. Current EC2 enumeration found only E12a/E12c, while
S3 contained only the two interrupted earlier E12b jobs. Both earlier durable
statuses say `interrupted` / Spot reclaim. No duplicate E12b job was found.

Submitted exactly once with `launch.sh b-spot`, using the unchanged source
archive SHA-256 `4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f`.
The separate r2 reference and post-run completion checker are encoded into the
job environment and SHA-256 checked before the frozen solver script runs.
The launch receipt binds their file hashes and the protocol/source identities.
EC2 reports the instance running. The saved startup log confirms `source.tgz:
OK`, `gate4_reference.jsonl: OK`, and `check-e12b-completion.sh: OK`. New draws
have not completed in this snapshot; the build and original gates are pending.

The original gates 0–3 remain mandatory. The new post-run wrapper enforces the
already registered gate 4, comparing draw 0 with r2 after removing only `secs`
fields, and requires four matching draw identities and successful cell exit.
It does not calculate a scientific verdict. A local fixture check accepted an
unchanged record and changed timing, and rejected changed non-timing data,
three draws, and a nonzero cell exit. Shell syntax and whitespace checks passed.
No cryptographic solver source changed; its original Rust tests run remotely.

## Decisions made (and why)

The explicit user choice supersedes Amendment 12's On-Demand restriction only
for this single attempt. The source, seed, cell, four draws, degree cap, and
mathematical acceptance rules are unchanged. Reclamation, timeout, OOM or a gate
failure leaves the attempt incomplete. The engines have no mid-closure resume.

The original script can exit zero after a failed cell, so launch/runner exit zero
alone is insufficient. The completion checker requires the cell success marker,
all four draws, and reference equality. Times are diagnostic and are not scored.

## Open threads (unfinished, pick up next time)

Inspect this job's `job.log`, raw status, and outputs under
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261028-031317-fd-e12b-spot-20261028-r3-24148b/`.
Require `completion_check.json` with `passed:true`, the original gate records,
raw outputs and binary/source digests before applying the lower-median rule.
At least three of four draws at `>=5` are needed for consistency with H_ell;
partial results do not decide the reading. Independent validation remains a
separate requirement.

E12a and E12c remain their existing jobs. Their logs show original gates passed
and their requested cells started; this authorization does not restart them.
Reconcile live job state before any later action. Conductor task: T-670.
The shared daily session log was reserved by T-668; preserve this handoff here
and link or append it when that reservation is released.

## Errors (what went wrong, how it was handled)

The On-Demand denial is preserved in the earlier record; no IAM policy was
changed. The current runner's multiline command produces unescaped JSON in the
running status reason. Preserve raw status bytes and consult EC2/job logs rather
than treating JSON parsing failure as experiment failure or permission to retry.
