# Line-layout tournament terminal observation (2026-11-01)

Status: **inconclusive; no end-to-end ECDLP speedup is established**.

The source-linked failed-run report, flow diagram and PDF live under
[`reports/run-20261101T0121Z/`](reports/run-20261101T0121Z/REPORT.md). The named
obstruction and its exit criteria are tracked in
[`problems/ic-line-layout-eks-failure-cause.md`](../../problems/ic-line-layout-eks-failure-cause.md).

The implementation and frozen protocol merged through PR #3116 at merge commit
`b4d751706eb2244314ebc9b84ba9545485ba4ac7`. The two preregistered, concurrent
EKS sessions were dispatched against that exact commit. Both passed their
contract and credential-free build jobs and then terminated in the EKS
experiment job with remote-result exit code 23 (`state: failed`); the GitHub
launcher wrapper consequently exited 26. No successful pod UID or authenticated
result URI was returned by either launcher.

| Workflow run | EKS job | Job UID | Request SHA-256 | Terminal result |
|---|---|---|---|---|
| [41260778029](https://github.com/isogeny-labs/crypto/actions/runs/41260778029) | `gh-41260778029-1` | `09392132-5307-4557-bf4c-4cda79e9aaa5` | `3732f132b4b04e600270af891435633872358cc3c77890f7aa7729855dd53bcc` | failed, result 23 / wrapper 26 |
| [41260778856](https://github.com/isogeny-labs/crypto/actions/runs/41260778856) | `gh-41260778856-1` | `f786770c-3cc3-4cb2-9b0c-e486d621ab95` | `0fc352b643f362f84fdc69c7d1062a1502dd228ae25b435ad72879c6b254ef55` | failed, result 23 / wrapper 26 |

The public workflow logs bind both runs to native binary SHA-256
`0784fe47212d64dd2ee81510d3c28d2f604782086500135a3114c61738f854b7`
and the exact merge commit. Detailed Kubernetes status, pod logs and object
inventory remain in each private launcher prefix. They were not available in
this workspace, and the public receipt does not identify the payload exit code.
Consequently, the failure cannot be assigned to the candidate, isolation gate,
owner instrumentation, A/A gate or infrastructure from the public receipt
alone. No repair is justified without that evidence.

## Excluded diagnostic

The exact binary was recovered from immutable GitHub Actions build artifact
`13064949140`, its SHA-256 was verified, and the frozen spec was exercised
locally. The deterministic plan contained 360 executions. With the explicitly
non-admitted `--allow-busy` diagnostic mode, all 360 outcomes verified and all
320 replay-eligible records reproduced. The A/A and candidate operation ratios
were exactly 1.0.

Every local record graded L0 because this workspace cannot supply PSI-based
quiet-host evidence. Therefore its wall comparison is descriptive only. It
showed candidate/incumbent wall ratio 1.1345 and median paired ratio 1.0495;
these values neither establish a regression nor support a speedup. They must
not be substituted for the missing EKS result.

## Stopping rule and next evidence version

Both planned trials reached a terminal state. No scientific retry, alternate
candidate or threshold change was launched after observing the failures. This
preserves the preregistered stopping rule: the current evidence version is
inconclusive.

An authorized lab operator must inspect `status.json`, `logs.json`,
`objects.json`, and any uploaded job artifacts under the two private launcher
prefixes. If the evidence identifies an implementation-caused failure, repair
it without weakening the L2, replay, exact-operation, A/A or confidence-bound
requirements. Any further performance trial requires a new frozen evidence
version and preregistration; it cannot retroactively complete this one.

The reviewed follow-up is `.github/workflows/isolab-failure-diagnosis.yml`.
It is fixed to the two recorded identities, verifies request and accessible
artifact hashes, and retains only allowlisted failure metadata. It cannot submit
or retry a Job, and raw measurements and stderr remain private.

Its first dispatch, [run 41266993333](https://github.com/isogeny-labs/crypto/actions/runs/41266993333)
on merge commit `0b915934de7f57b814507bfe32208175b4fd3a94`, authenticated
successfully and read the four fixed launcher receipts. Both payload-prefix
reads returned 403. A reader defect then treated the failed manifest copy as
success and attempted to verify a nonexistent file, so both matrix jobs failed
and no diagnosis artifact was uploaded. The follow-up correction distinguishes
not-listed, read-unavailable and hash-verified payload states, and reports only
an identity-checked Kubernetes status classification when payload reads are
unavailable. No experiment retry or broader IAM grant is introduced.

The relation-yield pruning instrumentation merged separately through PR #3091.
That instrumentation and this line-layout implementation are engineering
results; neither establishes the still-open end-to-end ECDLP index-calculus
speedup claim.
