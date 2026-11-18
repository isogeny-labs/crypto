# Mechanical task card CARD-XP-20261029-dg-wall-launch

author_role: experiment-director
executor_role: executor-mechanical
experiment: dg-wall (protocol backlog/XP-20261029-dg-wall/PROTOCOL.md; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol (APPROVAL.md in the protocol directory); the main (reasoning-tier) session decided the route and the requests. This card adds no authority.

dispatch values: the dispatch names exactly three values, and the executor substitutes them and changes nothing else.
- MODE: `LAUNCH` or `WAIT`.
- the bit size, 24, 28 or 32. Substitute its digits everywhere the text __B__ appears in this card.
- CARD_SHA256: the sha256 of this card file.

A dispatch with MODE `LAUNCH` runs "Procedure L" and launches the two jobs of one bit size (the wall job and the rho job). A dispatch with MODE `WAIT` runs "Procedure W" and launches nothing.

## What the launcher does, and the slot rule

- Each job is one `launch-remote` process on this machine. It starts one AWS CodeBuild build of the project isolab-experiment-launcher, and that build stays `IN_PROGRESS` for the whole remote job (the build of the previous study lasted 224 s, 337 s and 1060 s). The build is finished when its buildStatus is not `IN_PROGRESS`; the local `launch-remote` process has then exited too.
- The project allows at most 2 concurrent builds. One bit size needs both slots (2 jobs). So the sizes run in this order, one size at a time: 24, then 28, then 32.
- A size is launched only when both launchers of the size before it have exited and the CodeBuild build count `IN_PROGRESS` is 0. Procedure L checks both facts (steps 4 and 7) and refuses otherwise; it never waits. Procedure W is how the main session learns that the previous size has finished.
- The card does not read any outcome. The main session decides, from the report of Procedure W, whether to dispatch the next size.

## Frozen values

- Freeze commit of the protocol: 6e4804417baa031f6a1aee9b9b54c67c90c4dafe; branch claude/ic-dg-wall-20261029 (pushed).
- Resume checkpoint: 20261029T095412Z-6e4804417baa (s3://setec-astronomy-artifacts/checkpoints/crypto/claude__ic-dg-wall-20261029/20261029T095412Z-6e4804417baa/).
- SRC = /dev/shm/src-be2969ce (clean sparse checkout of source revision be2969cebf30a0cfb79088fd10094324c3efb3bb; the launcher scripts there are that revision's own)
- REMOTE = /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote
- Never use the launcher scripts of /home/user/crypto: its launch-remote differs from the one in SRC.
- Image of every job: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c (each request embeds it)
- ecbench sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c; source revision be2969cebf30a0cfb79088fd10094324c3efb3bb (each request embeds both).
- Every job: instance c8a.2xlarge, execution deadline 3600 s, provenance.harness claude-code-local, provenance.run_url https://github.com/isogeny-labs/crypto/blob/6e4804417baa031f6a1aee9b9b54c67c90c4dafe/backlog/XP-20261029-dg-wall/PROTOCOL.md
- DGW = @@DGW@@ (requests, receipts, logs and pid files; nothing larger than a few kilobytes is written there; large outputs, if any, go to /dev/shm)
- Protocol directory PD = @@PD@@
- Protocol worktree WT = @@WT@@
- Backend: isolab-eks AWS launcher, AWS account 104186811093. AWS credentials are inherited from the process environment; never write, print or copy them.
- env of every AWS command: AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin
- Working directory of every launch and validation command: /dev/shm/src-be2969ce
- Checkpoint gate: run in WT with the script of the sibling worktree wt-pbo (WT has no scripts directory).

The two jobs of a bit size (tag = the text after `request-` in the request file name):

| job | tag | job_name | REQUEST | RECEIPTS (must not exist before launch) | LOG (must not exist before launch) | PIDFILE (must not exist before launch) | S3 prefix under s3://isolab-results-104186811093-us-east-2/runs/ (must be empty before launch) |
|:--|:--|:--|:--|:--|:--|:--|:--|
| wall | P__B__ | ic-dgw-p__B__-20261029 | @@DGW@@/request-P__B__.json | @@DGW@@/receipts-P__B__ | @@DGW@@/launch-P__B__.log | @@DGW@@/pid-P__B__ | ic-dgw-p__B__-20261029/ |
| counted | P__B__-rho | ic-dgw-rho-p__B__-20261029 | @@DGW@@/request-P__B__-rho.json | @@DGW@@/receipts-P__B__-rho | @@DGW@@/launch-P__B__-rho.log | @@DGW@@/pid-P__B__-rho | ic-dgw-rho-p__B__-20261029/ |

Pinned sha256 (requests, spec embedded byte for byte as JSON-equal content):
- request-P24.json @@REQ24@@
- request-P24-rho.json @@REQR24@@
- request-P28.json @@REQ28@@
- request-P28-rho.json @@REQR28@@
- request-P32.json @@REQ32@@
- request-P32-rho.json @@REQR32@@

Pinned sha256 (protocol files in PD, as committed at the freeze commit):
- PROTOCOL.md @@PROTOCOL@@
- APPROVAL.md @@APPROVAL@@
- PRELAUNCH-CHECKS.md @@PRELAUNCH@@
- spec-P24.json @@SPEC24@@
- spec-P24-rho.json @@SPECR24@@
- spec-P28.json @@SPEC28@@
- spec-P28-rho.json @@SPECR28@@
- spec-P32.json @@SPEC32@@
- spec-P32-rho.json @@SPECR32@@

Pinned sha256 (scripts):
- REMOTE/validate-request @@VALREQ@@
- REMOTE/launch-remote @@LAUNCHREMOTE@@
- wt-pbo/scripts/checkpoint.sh @@CHECKPOINT@@
- @@TOOLS@@/pidwait.py @@PIDWAIT@@

## Procedure L (MODE LAUNCH). Use a single Bash call per step; change nothing. Steps 0 to 7 are checks; if any of them fails, launch nothing.

Step 0. Card digest. Run (the left side of the comparison is the CARD_SHA256 value stated in the dispatch):

```
sha256sum @@DGW@@/card-launch.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the text __B__ unsubstituted; the substitution is made only in the commands you run.)

Step 1. Checkpoint gate. Run:

```
cd @@WT@@ && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash ../wt-pbo/scripts/checkpoint.sh gate
```

The exit status must be 0.

Step 2. Hashes (the two requests, the protocol files, the launcher scripts, the checkpoint script, the wait script). Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
24) REQ_SHA=@@REQ24@@; REQR_SHA=@@REQR24@@; SPEC_SHA=@@SPEC24@@; SPECR_SHA=@@SPECR24@@;;
28) REQ_SHA=@@REQ28@@; REQR_SHA=@@REQR28@@; SPEC_SHA=@@SPEC28@@; SPECR_SHA=@@SPECR28@@;;
32) REQ_SHA=@@REQ32@@; REQR_SHA=@@REQR32@@; SPEC_SHA=@@SPEC32@@; SPECR_SHA=@@SPECR32@@;;
*) echo BAD_B; exit 1;;
esac
sha256sum -c - <<HASHES
$REQ_SHA  @@DGW@@/request-P__B__.json
$REQR_SHA  @@DGW@@/request-P__B__-rho.json
$SPEC_SHA  @@PD@@/spec-P__B__.json
$SPECR_SHA  @@PD@@/spec-P__B__-rho.json
@@PROTOCOL@@  @@PD@@/PROTOCOL.md
@@APPROVAL@@  @@PD@@/APPROVAL.md
@@PRELAUNCH@@  @@PD@@/PRELAUNCH-CHECKS.md
@@VALREQ@@  @@REMOTE@@/validate-request
@@LAUNCHREMOTE@@  @@REMOTE@@/launch-remote
@@CHECKPOINT@@  @@WT@@/../wt-pbo/scripts/checkpoint.sh
@@PIDWAIT@@  @@TOOLS@@/pidwait.py
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all 11 output lines must end with ": OK".

Step 3. Each request equals its frozen spec and its frozen fields; the freeze commit is in the worktree and pushed; the protocol directory equals the freeze commit. Run, as one Bash call, exactly these lines in this order:

```
python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" @@DGW@@/request-P__B__.json @@PD@@/spec-P__B__.json && echo SPEC_EQUAL_P__B__
python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" @@DGW@@/request-P__B__-rho.json @@PD@@/spec-P__B__-rho.json && echo SPEC_EQUAL_P__B__-rho
diff <(jq -S .job.spec @@DGW@@/request-P__B__.json) <(jq -S . @@PD@@/spec-P__B__.json) && echo JQS_EQUAL_P__B__
diff <(jq -S .job.spec @@DGW@@/request-P__B__-rho.json) <(jq -S . @@PD@@/spec-P__B__-rho.json) && echo JQS_EQUAL_P__B__-rho
jq -e '.job.job_name=="ic-dgw-p__B__-20261029" and .job.instance_type=="c8a.2xlarge" and .job.seconds==3600 and .job.build.image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c" and .job.build.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .job.build.ecbench_sha256=="88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c" and .provenance.harness=="claude-code-local" and .provenance.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.workflow_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.run_url=="https://github.com/isogeny-labs/crypto/blob/6e4804417baa031f6a1aee9b9b54c67c90c4dafe/backlog/XP-20261029-dg-wall/PROTOCOL.md"' @@DGW@@/request-P__B__.json > /dev/null && echo FIELDS_OK_P__B__
jq -e '.job.job_name=="ic-dgw-rho-p__B__-20261029" and .job.instance_type=="c8a.2xlarge" and .job.seconds==3600 and .job.build.image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c" and .job.build.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .job.build.ecbench_sha256=="88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c" and .provenance.harness=="claude-code-local" and .provenance.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.workflow_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.run_url=="https://github.com/isogeny-labs/crypto/blob/6e4804417baa031f6a1aee9b9b54c67c90c4dafe/backlog/XP-20261029-dg-wall/PROTOCOL.md"' @@DGW@@/request-P__B__-rho.json > /dev/null && echo FIELDS_OK_P__B__-rho
cd @@WT@@
git merge-base --is-ancestor 6e4804417baa031f6a1aee9b9b54c67c90c4dafe HEAD && echo HEAD_CONTAINS_COMMIT
git merge-base --is-ancestor 6e4804417baa031f6a1aee9b9b54c67c90c4dafe origin/claude/ic-dg-wall-20261029 && echo COMMIT_PUSHED
git diff --quiet 6e4804417baa031f6a1aee9b9b54c67c90c4dafe -- backlog/XP-20261029-dg-wall && echo PD_EQUALS_COMMIT
```

The output must contain the nine lines SPEC_EQUAL_P__B__, SPEC_EQUAL_P__B__-rho, JQS_EQUAL_P__B__, JQS_EQUAL_P__B__-rho, FIELDS_OK_P__B__, FIELDS_OK_P__B__-rho, HEAD_CONTAINS_COMMIT, COMMIT_PUSHED and PD_EQUALS_COMMIT, and nothing on stderr.

Step 4. Order: the size before this one has finished. Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
24) echo ORDER_NOT_REQUIRED;;
28) python3 -I @@TOOLS@@/pidwait.py 0 @@DGW@@/pid-P24 @@DGW@@/pid-P24-rho && test -f @@DGW@@/receipts-P24/result.json && test -f @@DGW@@/receipts-P24-rho/result.json && echo PREV_RESULTS_PRESENT;;
32) python3 -I @@TOOLS@@/pidwait.py 0 @@DGW@@/pid-P28 @@DGW@@/pid-P28-rho && test -f @@DGW@@/receipts-P28/result.json && test -f @@DGW@@/receipts-P28-rho/result.json && echo PREV_RESULTS_PRESENT;;
*) echo BAD_B; exit 1;;
esac
```

For __B__ = 24 the output must be the line ORDER_NOT_REQUIRED. For __B__ = 28 or 32 the output must contain two `PID_STATE` lines each ending in `exited`, the line PAIR_DONE and the line PREV_RESULTS_PRESENT (a PID_STATE line ending in `alive`, or the line PAIR_RUNNING, means a launcher of the previous size is still running).

Step 5. Receipts directories, logs, pid files and S3 prefixes unused. Run, as one Bash call, exactly these lines in this order:

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
test ! -e @@DGW@@/receipts-P__B__ && test ! -e @@DGW@@/launch-P__B__.log && test ! -e @@DGW@@/pid-P__B__ && echo ABSENT_P__B__
test ! -e @@DGW@@/receipts-P__B__-rho && test ! -e @@DGW@@/launch-P__B__-rho.log && test ! -e @@DGW@@/pid-P__B__-rho && echo ABSENT_P__B__-rho
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dgw-p__B__-20261029/)" && echo S3_EMPTY_P__B__
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dgw-rho-p__B__-20261029/)" && echo S3_EMPTY_P__B__-rho
```

The exit status must be 0 and the output must contain the four lines ABSENT_P__B__, ABSENT_P__B__-rho, S3_EMPTY_P__B__ and S3_EMPTY_P__B__-rho.

Step 6. Validate both requests (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request @@DGW@@/request-P__B__.json > /dev/null && echo VALID_P__B__
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request @@DGW@@/request-P__B__-rho.json > /dev/null && echo VALID_P__B__-rho
```

The exit status must be 0 and the output must contain the lines VALID_P__B__ and VALID_P__B__-rho.

Step 7. Build slots (one Bash call, exactly these two lines in this order):

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w
```

The only line printed must be 0, with nothing on stderr. (The project allows 2 concurrent builds and this launch needs both. An output of 1 or 2 means a build is still running; refuse, do not wait.) (The export line is there because a `NAME=value` prefix on the aws line would not reach the aws command inside `$(...)`, which is expanded first. The list command takes no --max-items: that option appends a second line `None`, which batch-get-builds rejects as an invalid id.)

Step 8. Launch both jobs, wait 90 seconds, read the logs (one Bash call, exactly these lines in this order; run this call once and never again):

```
cd /dev/shm/src-be2969ce
echo "LAUNCH_UTC_P__B__ $(date -u +%Y-%m-%dT%H:%M:%SZ)"
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=@@DGW@@/receipts-P__B__ PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup bash -c 'echo $$ > "$2"; exec taskq/deploy/eks-karpenter/remote/launch-remote "$1"' launcher @@DGW@@/request-P__B__.json @@DGW@@/pid-P__B__ > @@DGW@@/launch-P__B__.log 2>&1 < /dev/null &
echo "LAUNCH_UTC_P__B__-rho $(date -u +%Y-%m-%dT%H:%M:%SZ)"
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=@@DGW@@/receipts-P__B__-rho PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup bash -c 'echo $$ > "$2"; exec taskq/deploy/eks-karpenter/remote/launch-remote "$1"' launcher @@DGW@@/request-P__B__-rho.json @@DGW@@/pid-P__B__-rho > @@DGW@@/launch-P__B__-rho.log 2>&1 < /dev/null &
python3 -c "import time; time.sleep(90)"
echo "PID_P__B__ $(cat @@DGW@@/pid-P__B__)"
echo "PID_P__B__-rho $(cat @@DGW@@/pid-P__B__-rho)"
echo "LAUNCHER_ID_P__B__ $(sed -n 's/^Launcher: isolab-experiment-launcher://p' @@DGW@@/launch-P__B__.log)"
echo "LAUNCHER_ID_P__B__-rho $(sed -n 's/^Launcher: isolab-experiment-launcher://p' @@DGW@@/launch-P__B__-rho.log)"
echo "==== LOG_P__B__ ===="
head -n 20 @@DGW@@/launch-P__B__.log
echo "==== LOG_P__B__-rho ===="
head -n 20 @@DGW@@/launch-P__B__-rho.log
```

(Each launcher writes the pid of its own process into its pid file, so the pid file holds the pid that Procedure W watches.)

## Procedure W (MODE WAIT). Use a single Bash call per step; change nothing. The wait reads only /proc entries of the recorded pids, never `pgrep` or `ps` with a pattern.

Step W0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step W1. Wait by pid. Run, with the Bash tool `timeout` parameter set to 570000, exactly this line:

```
python3 -I @@TOOLS@@/pidwait.py 540 @@DGW@@/pid-P__B__ @@DGW@@/pid-P__B__-rho
```

The script prints one `PID_STATE` line per pid file, then PAIR_DONE (both launchers exited) or PAIR_RUNNING. If the last line is PAIR_RUNNING, run the identical line again; at most 11 runs in total. Stop repeating at the first PAIR_DONE.

Step W2. Read the outcome. Run, as one Bash call, exactly these lines in this order:

```
python3 -I @@TOOLS@@/pidwait.py 0 @@DGW@@/pid-P__B__ @@DGW@@/pid-P__B__-rho
echo "==== RESULT_P__B__ ===="
jq -c '{state,job_name,exit_code,launcher,request_sha256,results_uri,receipt_uri}' @@DGW@@/receipts-P__B__/result.json 2>&1
echo "==== RESULT_P__B__-rho ===="
jq -c '{state,job_name,exit_code,launcher,request_sha256,results_uri,receipt_uri}' @@DGW@@/receipts-P__B__-rho/result.json 2>&1
echo "==== TAIL_P__B__ ===="
tail -n 30 @@DGW@@/launch-P__B__.log
echo "==== TAIL_P__B__-rho ===="
tail -n 30 @@DGW@@/launch-P__B__-rho.log
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
echo "SLOTS_IN_PROGRESS $(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w)"
```

## Return (verbatim tool output, nothing else, then the table)

Procedure L: the output of steps 0 to 7 as printed, then the output of step 8 as printed (the two LAUNCH_UTC lines, the two PID lines, the two LAUNCHER_ID lines, and the two logs). Then this table, each cell copied from a printed line or from this card, nothing computed:

| job | job_name | request sha256 (pinned) | LAUNCH_UTC | PID | LAUNCHER_ID | RECEIPTS | LOG |
|:--|:--|:--|:--|:--|:--|:--|:--|
| wall | ic-dgw-p__B__-20261029 | (the pinned sha256 of request-P__B__.json, listed above) | (LAUNCH_UTC_P__B__) | (PID_P__B__) | (LAUNCHER_ID_P__B__) | @@DGW@@/receipts-P__B__ | @@DGW@@/launch-P__B__.log |
| counted | ic-dgw-rho-p__B__-20261029 | (the pinned sha256 of request-P__B__-rho.json, listed above) | (LAUNCH_UTC_P__B__-rho) | (PID_P__B__-rho) | (LAUNCHER_ID_P__B__-rho) | @@DGW@@/receipts-P__B__-rho | @@DGW@@/launch-P__B__-rho.log |

Procedure W: the output of step W0, the last output of step W1 (with the number of runs made), and the output of step W2 as printed. Then this table, each cell copied from a printed line:

| job | PID_STATE | result.json state | result.json exit_code | launcher | SLOTS_IN_PROGRESS |
|:--|:--|:--|:--|:--|:--|

No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands.

## Write scope

The executor writes nothing itself. The launched launch-remote writes only inside its RECEIPTS directory (a path in the table above) and uploads its request and receipts to the S3 bucket named above. The shell redirects of step 8 write the LOG files and the PIDFILEs. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly one MODE from LAUNCH, WAIT, exactly one bit size from 24, 28, 32, or CARD_SHA256.
- Step 0 or W0 prints a first field different from CARD_SHA256, or exits with a status other than 0.
- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, prints BAD_B, or any of its output lines does not end with ": OK".
- Step 3 exits with a status other than 0, prints anything on stderr, or lacks any of its nine required lines.
- Step 4 exits with a status other than 0, or lacks its required lines (ORDER_NOT_REQUIRED for 24; for 28 and 32: two PID_STATE lines ending in `exited`, PAIR_DONE and PREV_RESULTS_PRESENT).
- Step 5 exits with a status other than 0, or lacks any of its four required lines (a missing S3_EMPTY line means the S3 prefix of that job name is not empty).
- Step 6 exits with a status other than 0, or lacks either VALID line.
- Step 7 prints anything other than the single line 0, or prints anything on stderr.
- Step 8 does not print a line `PID_P__B__ ` followed by digits and a line `PID_P__B__-rho ` followed by digits.
- A file named LOG does not exist after step 8.

Each refusal is a specification_error back to the main session. A refusal in steps 0 to 7 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch.

## Never retry (applies to both procedures)

An ambiguous submission is NEVER retried. This means:
- Once step 8 has started, never run any `launch-remote` command a second time for either request, whatever any output shows: an empty log, a log without a `Launcher:` line, an AWS error, a failed StartBuild, a timeout of the Bash call, an exit of the process, a missing pid file.
- If the launcher reports an error in its log, or step 8 or W1 or W2 prints anything unexpected, return the output as printed and run nothing further. Do not inspect the job, do not list builds beyond the commands of this card, do not delete or move any file, do not resubmit under another name.
- A reported error or a stopped job is not a result for this card. Only the main session decides whether a new card (for example a host-loss rerun under the suffix `-r2`, PROTOCOL.md section 4) is written.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs detached launches with the logs outside the receipts directories, a 90-second partial report and a wait by pid, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 8 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. Each bit size is launched by a separate dispatch of this card; launch and wait are separate dispatches.
