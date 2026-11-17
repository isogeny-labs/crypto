# Prime-IC line-layout tournament: terminal run report

**Run start:** 2026-11-01 01:21 UTC\
**Source:** `b4d751706eb2244314ebc9b84ba9545485ba4ac7`\
**Native SHA-256:** `0784fe47212d64dd2ee81510d3c28d2f604782086500135a3114c61738f854b7`\
**Outcome:** both preregistered EKS jobs terminated failed; performance decision unavailable.

![Failure evidence and bounded diagnostic flow](failure-flow.png)

The diagram is descriptive. It contains no timing or inferred speed result.
Its editable source is [`failure-flow.dot`](failure-flow.dot), with an SVG copy
at [`failure-flow.svg`](failure-flow.svg).

## Requested scope and evidence status

- **Two concurrent EKS sessions — verified complete.** Workflow runs
  41260778029 and 41260778856 bind the exact source and native binary.
- **Unchanged scientific gates — preserved but not completed.** Both Jobs
  failed before a successful payload was authenticated; no threshold changed.
- **End-to-end speed decision — blocked.** Neither run returned an
  authenticated session directory or comparison.
- **Zero outcome-driven retries — verified complete.** Exactly the two frozen
  trials ran and both failures are retained.
- **Privacy-preserving diagnosis — complete at status-evidence level.** The
  first read exposed a payload-access and copy-status failure. Corrected run
  41267915424 classified both failed Pods at the L2 gate without publishing raw
  data. The payload manifests remain unverified and exact L2 predicates absent.

## Observed execution

Both workflows passed their contract and credential-free build jobs. EKS Jobs
`gh-41260778029-1` and `gh-41260778856-1` then returned launcher state
`failed`, remote-result exit 23 and GitHub wrapper exit 26. The public receipts
contain Job UIDs but empty successful Pod UID and results URI fields.

The original public evidence did not distinguish payload execution, native
audit, L2, A/A, owner instrumentation or infrastructure failure. Corrected
read-only diagnosis now locates both failures at the L2 isolation gate using
identity-checked Kubernetes status. Raw records, timing values and stderr
remain private.

## Excluded local diagnostic

The exact retained binary planned 360 executions and, under explicit
`--allow-busy`, verified all 360 outcomes and reproduced all 320 replay-eligible
records. Every execution graded L0 because this host lacks the required PSI
evidence. Those observations are correctness diagnostics only; their wall
values are not admitted and are not graphed or promoted.

\newpage

## Evidence-preserving diagnosis

The follow-up workflow is read-only and fixed to the two recorded workflow,
Job, launcher, request, source and binary identities. It validates the normalized
request hash, downloads only allowlisted diagnostic objects, checks each object
against the retained `SHA256SUMS`, and publishes a compact classification. It
does not run the binary, submit or retry a Job, print raw logs, or upload raw
measurements.

The first merged dispatch, workflow run 41266993333, authenticated successfully
and read the four launcher receipts for both trials. Payload-object reads were
forbidden by the existing role. The reader incorrectly treated that copy failure
as success and then attempted to verify a nonexistent manifest; both matrix jobs
failed and uploaded nothing. The correction records the payload access state and
uses only identity-checked Kubernetes status when payload objects are unreadable.
It does not widen IAM permissions or elevate that status evidence to a
hash-verified payload receipt.

The corrected source merged at
`15b58954b965a44b24a80f8b493e945d91076803`. Its single dispatch, workflow run
41267915424, completed both fixed matrix jobs. Each retained launcher prefix
contains 19 objects and one failed Pod. Both Pods exited 42 with classification
`l2_isolation_gate`:

- **Frozen workflow 41260778029:** Pod
  `458712fd-1251-4d62-b943-b35f44a51e44`; retained artifact
  [`diagnosis/41260778029.json`](diagnosis/41260778029.json); payload read
  unavailable and manifest not verified.
- **Frozen workflow 41260778856:** Pod
  `bfe971ea-45d0-4f50-a851-8afbd61f61f6`; retained artifact
  [`diagnosis/41260778856.json`](diagnosis/41260778856.json); payload read
  unavailable and manifest not verified.

The records contain no below-L2 detail: `l2`, `audit` and `control` are null,
owner run/replay checks are false, and private stderr hashes and content are
absent. They establish the terminal gate at status-evidence level only. Exact
predicate attribution requires narrowly authorized, manifest-bound access to
the retained `l2-gate.json`; no candidate change or scientific retry follows
from these records.

## Sources checked

- [`taskq/deploy/eks-karpenter/HARNESS.md`](../../../../taskq/deploy/eks-karpenter/HARNESS.md): request identity, failure states and private-result boundary.
- [`taskq/deploy/eks-karpenter/ACCESS.md`](../../../../taskq/deploy/eks-karpenter/ACCESS.md): existing GitHub OIDC role and scoped S3 read route.
- [`taskq/deploy/eks-karpenter/remote/run-request`](../../../../taskq/deploy/eks-karpenter/remote/run-request): exact launcher receipt and artifact locations.
- [GitHub OIDC with AWS](https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws): short-lived workflow identity; no static key is introduced.

## Claim and graph boundaries

This failed run supplies no admitted quantitative comparison. The index-calculus
scoreboard, leaderboard, progress timeline and lab browser were checked and left
unchanged because there is no verified numeric result to add. No `benchmarks/`
entry is created because no performance measurement was admitted.

The relation-yield pruning instrumentation from PR #3091 is a separate merged
engineering deliverable. This run report does not convert that instrumentation,
the line-layout implementation, or workflow success into an end-to-end speedup.
