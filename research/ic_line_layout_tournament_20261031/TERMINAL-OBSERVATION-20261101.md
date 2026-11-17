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
and the exact merge commit. The public receipt alone did not identify the
payload exit code. The later fixed-target diagnosis read the identity-checked
Kubernetes status and object inventory, but not the payload artifacts: both
runs had 19 retained objects and one failed Pod, and both Pods exited 42 at the
`l2_isolation_gate`. This locates each failure before owner replay, A/A and
timed comparison. It does not expose the exact failed L2 predicate, and it does
not justify changing the candidate.

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

An authorized lab operator must provide narrowly scoped access to the retained
`SHA256SUMS` and `l2-gate.json`, or independently qualify the L2 infrastructure
under a new evidence version. The exact L2 predicate must be hash-verified
before a specific infrastructure repair is claimed. The L2, replay,
exact-operation, A/A and confidence-bound requirements remain unchanged. Any
further performance trial requires a new frozen evidence version and
preregistration; it cannot retroactively complete this one.

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

The corrected workflow source merged at
`15b58954b965a44b24a80f8b493e945d91076803`. Its single dispatch,
[run 41267915424](https://github.com/isogeny-labs/crypto/actions/runs/41267915424),
completed both fixed matrix jobs and uploaded allowlisted artifacts
`13068226703` and `13067928403`. The byte-exact retained diagnoses are:

| Frozen workflow | Pod UID | Objects | Classification | Exit evidence | Payload evidence |
|---|---|---:|---|---|---|
| 41260778029 | `458712fd-1251-4d62-b943-b35f44a51e44` | 19 | `l2_isolation_gate`, exit 42 | identity-checked Kubernetes status | read unavailable; manifest not verified |
| 41260778856 | `bfe971ea-45d0-4f50-a851-8afbd61f61f6` | 19 | `l2_isolation_gate`, exit 42 | identity-checked Kubernetes status | read unavailable; manifest not verified |

See [`diagnosis/41260778029.json`](reports/run-20261101T0121Z/diagnosis/41260778029.json)
and [`diagnosis/41260778856.json`](reports/run-20261101T0121Z/diagnosis/41260778856.json).
Both records retain null L2 detail, null audit/control fields, no private stderr,
no timing values and `experiment_retry: false`. They establish the terminal
gate only at status-evidence level, not a hash-verified below-L2 cause.

The relation-yield pruning instrumentation merged separately through PR #3091.
That instrumentation and this line-layout implementation are engineering
results; neither establishes the still-open end-to-end ECDLP index-calculus
speedup claim.
