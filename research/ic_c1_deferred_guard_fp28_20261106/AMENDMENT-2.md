# C1 route A: AMENDMENT-2 (procedural, filed before any S2 byte and before any S1 record is read), revision 1

**Status: FILED 2026-11-07 for the Coordinator's sign-off (DEC-20261107-b74e2c ruling 4; design task TASK-20261107-c5a0f4); it is frozen when its pull
request merges.** With the `protocol.json` change in section 6 it is the version-3 freeze. It carries no approval: the Coordinator's approval decision for
the new runbook session is a separate, later record.

Protocol `IC-C1-DEFERRED-GUARD-FP28-20261106`, frozen version 2 (carries C1-AMENDMENT-1): `protocol_sha256`
`9844f78d435951cf9a5f19c9a89ff2a38656e58e94d632f577093db846f72166`, commit `4c44ad2ab582f0cd38b9fcf0dd0dd49df2800dd6`, `frozen_at`
`2026-11-06T11:03:41Z`. Version 1 (`43b1ca918c034433f3584f37db439613d11a618c0eec13d3d7831a1fa96d092f`) stays as provenance of AMENDMENT-1.

`filed_before_affected_data: true`, with this exact scope and disclosure:
- **Not yet existing:** no S2 byte exists. No `launch/request-s2.json` job has been submitted.
- **Not yet read:** no S1 `records.jsonl` content, no arm statistic, no ratio, no `compare` output and no parity field has been read by anyone. The only S1 reads
  beyond the section 9 item 2 set are the integrity counts disclosed as PD-1 in section 4.
- **Existing:** the S1 job ran once and wrote its own output (section 2). Reading that output is exactly what this amendment prescribes, in the order and
  by the checks of section 3 (b), before anything else about it is read.

It changes no N, seed, curve, target count, arm, weight, threshold, margin, accounting rule, decision mapping, request, instance, deadline, budget line or cap,
and no success criterion. **`PROTOCOL.md` is not edited**: its bytes stay as frozen. The pointers to this amendment are this file, `links.amendments`,
`amendments[1]` and `extensions.c1_amendment_2` of `protocol.json`.

## 1. What is amended

Frozen text, `PROTOCOL.md` section 18 (revision 6, with C1-AMENDMENT-1), the "Execution route, revision 6 ruling":
- the layout `S01-probe`, `S02-trials`, `S03-verify`, where `S02-trials` runs T1 (S1) and then T2 (S2);
- "`launch_eks.sh` ... downloads the job prefix", in the `S02-trials` bullet, and the sentence "`launch_eks.sh` exits 0 only when the launcher has returned,
  `job-exit.json` and `SHA256SUMS` are downloaded, and the job's exit code is 0, 42 or 43" (the source of the download is unspecified there and is specified below);
- section 9 item 3 names a launcher defect that affects S1 and S2 equally and says "the fix needs a `protocol_amendment`". This is that amendment.

Nothing else in section 18, section 9, section 10 or any other section is changed. Where this file and the sentences above disagree, this file governs.

## 2. Reason (facts, with the commands or records they come from)

- The S1 job `ic-c1-dg-s1-20261106` (Job uid `8a20d036-efc2-45d2-ada6-6442b93c5a52`, launcher build `47cf1e4f-10a7-4ee3-862d-9e9700e122e3`) was
  submitted once by `launch_eks.sh` v3a at 2026-11-07T01:56:56Z. Its pod `5885ba61-53c2-4e2e-8a47-ff1ff04ee2e7` ran container `task` once and terminated
  with exit code 42 at 02:14:20Z (launcher `status.json`, copied read-only from `s3://isolab-results-104186811093-us-east-2/runs/ic-c1-dg-s1-20261106/launcher/47cf1e4f-10a7-4ee3-862d-9e9700e122e3/`).
  42 is the expected exit of section 9 "Exit codes".
- The pinned launcher `taskq/deploy/eks-karpenter/remote/run-request` (sha256 `c4f75c585ca6c689bdbe3fa4840459df5347696db8901a3e23296239d21e6819`) exits at line 68
  (`[[ "$state" == succeeded ]] || exit 23`) before line 71 sets `pod_uid` and line 73 sets `raw_uri`. So for any failed Job, including a container exit 42, the
  launcher's `result.json` carries `pod_uid ""` and `results_uri ""`. The S1 `result.json` (sha256 `7c3d9954a561a7a0d07e498afe3ecdf5b8c53359cfa28abb578a67dc8a6d60e7`) has
  exactly that shape: `state failed`, `exit_code 23`, `pod_uid ""`, `results_uri ""`.
- `launch_eks.sh` v3a (sha256 `0fcdd67e22f6105c36883bf9ba7edb62e512e6ff135979e0b63084fb2c9b7dc0`), section 3, downloaded only a non-empty `results_uri`. It
  downloaded nothing, recorded `outcome_class infrastructure_candidate` and exited 5. The 42/43 accept branch was unreachable for every expected exit.
  DEC-20261107-b74e2c ruling 1 classifies this as an implementation error in our wrapper, not an infrastructure outcome and not evidence. S2's request uses
  the same pinned launcher and also exits 42, so section 9 item 3 applies and this amendment is its fix.
- No `-r2` is triggered (DEC-20261107-b74e2c ruling 2): the S1 job wrote its own `job-exit.json` and exited 42.

## 3. Replacement text

> (a) **Locating a job's results (section 18, S02-trials / launch wrapper).** The wrapper `implementation/launch_eks.sh` (v3b) locates a job's results with
> `implementation/locate_results.sh`. A non-empty `result.json` `.results_uri` is used as is. When it is empty, the locator reads the launcher receipt at
> `.receipt_uri` (which must be `s3://BUCKET/runs/JOB/launcher/<id>/`): `status.json` `.job.uid` must equal `result.json` `.job_uid`, `.job.job_name` must equal the
> request's job name, and exactly one pod must exist; that pod's uid is `pod_uid` and its terminated `exitCode` is recorded. The listing of `s3://BUCKET/runs/JOB/`
> must hold only `launcher/` and the `pod_uid/` prefix. After the download, `job-exit.json` `.pod_uid`, `.job_name` and `.exit_code` must equal the located pod, the job
> name and the status exit code, and the exit code must be 0, 42 or 43; otherwise the outcome is never `job_completed`. `raw-result.json` records
> `results_locator` (`launcher_result` or `receipt_status_pod_uid`), `job_uid` and `pod_uid`. The class logic, the 42/43 accept branch, the duplicate checks,
> the manifests and the exit codes of `launch_eks.sh` are unchanged.
>
> (b) **S1 is the job already run (section 9 item 2).** S1 is RUN-ICPERF-8b0d47's job, pod `5885ba61-53c2-4e2e-8a47-ff1ff04ee2e7`. It is brought in by a read-only
> reconciliation under its existing run id and is not rerun. `implementation/reconcile_run.sh` starts from the committed launcher receipt
> `runs/RUN-ICPERF-8b0d47/job/launcher/result.json` (sha256 `7c3d9954a561a7a0d07e498afe3ecdf5b8c53359cfa28abb578a67dc8a6d60e7`), applies the locator of (a), downloads the pod
> prefix read-only into the reconciliation step's output directory, checks that `job-exit.json` equals the located pod, the job name and the status exit code, runs the
> unchanged `check_job.py` on the download (it must print `CHECK_JOB_OK`), and writes `reconcile-result.json` naming `run_id RUN-ICPERF-8b0d47` and `trial_id T1`. It contains
> no job-submission, build-start or S3-write call and writes nothing under `runs/RUN-ICPERF-8b0d47/`. Before S2 only the `check_job.py` set is read: `records.jsonl` is hashed
> and its newlines are counted, never parsed. `run-summary.json` is transported as is: `SHA256SUMS` must list it at the sha256 of the empty string
> (`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`) and the file must have 0 bytes; any other result stops the session.
>
> (c) **The new session.** A new runbook session, with a fresh run id for S2, runs `S01-probe`; `S02-reconcile-t1` (b); `S03-trial-s2`, S2 once on v3b and only if
> `S02-reconcile-t1` passed; and `S04-verify`, the frozen `analyze-c1.sh` on the reconciled S1 session directory and the S2 session directory. S2 uses the frozen
> `launch/request-s2.json`, unchanged. The `-r2` requests are not bound. A failure of any step stops the session. The section 10 gate line is recorded before the S2
> launch: `actual_spend(S1)` is taken at HB USD 0.93108 until a billing or cost record exists, never estimated, so `0.93108 + 0.93108 + 0 = 1.86216 <= 5.00`. A
> conditional S2-r2 line would be `2.79324 <= 5.00`.

## 4. Disclosure of procedure deviation PD-1 (DEC-20261107-b74e2c, deviations)

Before S2, and outside the runbook, the top-level session made out-of-band read-only reads of the S1 prefix. It read `audit.stderr` (the line
`audit OK: 10800 records, 10800 verified, 9600 replays reproduced, 0 problems`) and the tail of `run.stderr` (the line `10800 records: {"verified": 10800}`). Section 9 item 2
(revision 6) says that audit and replay are not read before S2. Impact: integrity counts only. No arm statistic, ratio, parity field, `compare` output or `records.jsonl` content
was read. S2's request bytes are frozen and its counts are deterministic given the seeds, and section 9 item 3 runs S2 whatever S1's outcome, so no choice can be selected on
what was read. These reads are not evidence. The `S04-verify` step re-derives the audit. No further pre-S2 read outside the `check_job.py` set is permitted, by anyone.
The disclosure is carried to the evidence review's `review_plan`, in the blindness joint.

## 5. What this does not change

- Seeds, curves, targets, N, arms, weights, thresholds, margins, the accounting rule, the decision mapping, the 3,600 s deadline, the instance type, the four frozen
  requests (sha256 values in `extensions.requests`) and the hard cap USD 5.00.
- The pins in `implementation/remote-pins.sha256` (sha256 of the pin file `7b7531895ea935bf996fb983d837c57fe5a14439e93bb38917d9ee5a7468ee0f`), the
  acceptance set {0, 42, 43}, the duplicate guard, the deadline-hit rule, the rule that a `-r2` is a new session after a Coordinator decision, and `check_job.py`
  (sha256 `d60564ddcfae7fb9607049e5bad6e53984051301d8dad84b979f9094e1c7af2d`).
- `analyze-c1.sh` and every gate in it (sha256 `66aa2ffd91e10f5d1f1a65be086cd17c3bf388b4299157f646b75ea49d994dcc`).
- Nothing about the hypothesis, the claims or the predictions. This amendment makes no statement about any arm.

## 6. How it is filed (format supported by `tools/experiment_protocol.py`, as for C1-AMENDMENT-1)

1. Merge `AMENDMENT-2.md` (this text) to `main`; its sha256 is recorded in `extensions.c1_amendment_2.file_sha256`.
2. `protocol.json` becomes version 3: `version` 3, `links.amendments` (a second entry), the `amendments[1]` entry, `extensions.c1_amendment_2` (with `prior_freeze`, the version 2
   freeze), and `freeze` set back to the draft block. `extensions.execution_route.steps` (`S01-probe`, `S02-trials`, `S03-verify`) is superseded for the re-session by section 3 (c)
   and named in `supersedes_fields`; the field itself is not edited.
3. `experiment_protocol.py validate`, then `freeze ... --filed-before-affected-data`, then merge. The runbook re-binds `inputs/protocol.json`, a byte-identical copy of the frozen
   file, and its `/freeze/protocol_sha256` binding. Only provenance keeps version 1 and version 2.

Receipts are archived verbatim in `amendment-2-receipts.txt`, beside this file: the `validate` output (0 findings), the reproduction of the version 2 hash from the version 3 file
(`jq '.version=2 | .amendments=[.amendments[0]] | .links.amendments=[.links.amendments[0]] | del(.extensions.c1_amendment_2)'`, then `experiment_protocol.py hash`), the freeze output,
the `hash` and `jq -r .freeze.protocol_sha256` agreement, the sha256 of this file, and `git diff --stat` of `PROTOCOL.md` against its freeze commit (expected: empty).

## 7. Open points

- An independent red-team review of the locator joint (`locate_results.sh`, `reconcile_run.sh`) precedes the Coordinator's approval of the new runbook (DEC-20261107-b74e2c T-3).
- The owner is told, before any relay, that S1 ran once and stopped on a wrapper defect, that its spend is at most USD 0.93108, that this amendment is filed, and that S2 runs on v3b.
  This is notice, not a new consent (DEC-20261107-b74e2c ruling 4, `owner_gate`).
- The approval decision for the new runbook session cites the sha256 of this file and the `protocol_sha256` values of versions 1, 2 and 3.
