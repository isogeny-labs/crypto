# TR-04 workflow attempt 1: blocked before primary execution

[Run 40973940936](https://github.com/isogeny-labs/crypto/actions/runs/40973940936)
was dispatched on 2026-10-29 at 23:33:25 UTC and completed with **failure**
at 23:41:34 UTC. Source and workflow revision were both
`85fc870c96a3ea707afe11e06cf884eba6f957ce`. There were **zero primary field
attempts, zero completed research rows and all 27 expected rows missing**.
This is an infrastructure failure record, not a null scientific result.

## What ran

Preflight job `123797235834` passed source pinning, the full pinned JSON
Schema, native tests (35 library + 2 legacy binary + 3 CLI), strict Clippy,
formatting, native frozen-protocol/analysis-source checks and presence of
the existing artifact-store credential bindings. Credential presence did
not establish S3 authorization. Preflight used the hosted development
runner; no `crypto-eks-linux` research worker was assigned.

The unmodified checkpoint gate exited **1**. Its retained log states:

> checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/runner/.config/checkpoint/upload-token
> checkpoint: cannot upload, so nothing was checkpointed

The field matrix and remote aggregate jobs were skipped by their dependency
conditions. No field binary, selector distribution, relation record,
independent primary replay, or auxiliary control receipt exists for this
attempt. No job was retried, checkpoint bypassed, alternate backend used,
or credential, runner, capacity, funding or repository setting changed.

## Evidence and reporting

GitHub retained artifact `12941208365`, `tr04-preflight-40973940936-1`:
19,654 bytes, SHA-256
`3005f6c178558119459e3553f28bc7fa842ec7251123fcd6d432c35fb66f0570`.
The connector-downloaded ZIP matched that digest exactly; its 15 original
files include the protocol, execution identity, all development logs and
checkpoint exit/log. The direct CLI download encountered a destination
403; the supported connector download succeeded without changing network
policy or authentication. This is a verified GitHub download, not an
independent artifact-store download.

The raw ZIP, full preflight job log, single check-in snapshot, analysis
commands/exits and routing-validation source are retained together in
`evidence.tar.xz`; [receipt.json](receipt.json) gives its hash and inventory.
No raw evidence was split or truncated around the artifact policy.

The unchanged frozen native analyzer was run locally **only on three absent
field paths**. Summarize and report each returned the expected nonzero exit
1. [Native report](native-report/REPORT.md), [canonical export](native-report/export.json),
[diagram](native-report/recorded-values.svg) and [PDF](native-report/report.pdf)
retain zero evidenced logical attempts and list all 27 missing IDs. The
export's flow `planned=0` counts actual exported rows; the protocol still
expects 27. No missing attempt is fabricated. Local reporting is not a
research rerun or a completed remote aggregate. Both native PDF pages and
the separate [failure report PDF](report.pdf) were visually inspected.

There are fewer than two comparable research values, so
[execution.svg](execution.svg) shows what ran in place of a quantitative
comparison. [report.typ](report.typ) is its editable PDF source. The original
plan PDF, frozen protocols, prior failures and root-validation receipts are
unchanged. The IC leaderboard, performance figures and benchmark tables have
no new scientific value to add and were left unchanged.

## PR defect repaired separately

Reviewer thread `PRRT_kwDOSMdswM6xzHgB`, comment `4419135390`, identified that
`tr04-p*` also selects the preflight artifact. That would add an unwanted
fourth input and fail aggregation even with three valid shards. The workflow
now requires a digit after `tr04-p`. A regression using the actual action's
Minimatch 9.0.3 dependency reproduces the old match, accepts all three
declared field artifacts, excludes preflight/summary/delivery and wrong
run/attempt names, and retains unexpected numeric shards for native rejection.
See [pattern-routing-validation.json](pattern-routing-validation.json).
Actionlint with ShellCheck passed. This routing defect did not cause the
observed checkpoint failure. No scientific code, frozen parameter, threshold,
analysis source or decision rule changed, and no new workflow was dispatched.

The action's primary source is
[download-artifact v4](https://github.com/actions/download-artifact/blob/v4/src/download-artifact.ts),
reviewed blob `5cc6b1704fb5ef0d4487ed63d910d4f8a9f2fbd2`, which uses
`new Minimatch(inputs.pattern)`; its v4 lockfile pins 9.0.3.

## Requirements still open

| Requested work | Verified status |
| --- | --- |
| Bounded twist-guided sampling | Implemented; no primary execution |
| Matched controls | Implemented; no primary execution or six control receipts |
| Common public relation finder | Implemented; no primary relation records |
| Paid work to independent rank | Implemented; no measured comparison or primary replay |
| Durable delivery | No experiment archive upload or fresh-store byte download; remote authorization unverified |

The study remains **generic decomposition (bounded by sqrt(r); cannot beat
rho)** under repository classification. Primitive Work counters are not
independently recomputed even in the implemented verifier. This attempt
supports no physical timing/GAE, relation-yield, scalar-recovery, end-to-end
DLP, security or universal-impossibility conclusion. The primary experiment
remains blocked on the existing checkpoint transport; it is not discharged
by this failure report or the independent development tests.
