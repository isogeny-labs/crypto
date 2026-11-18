# Mechanical task card CARD-XP-20261029-stride-renew-04

author_role: experiment-director
executor_role: executor-mechanical
experiment: stride-renew-prime (one job ic-srenew-p32r2-20261029, the rerun of size b=32; protocol backlog/XP-20261029-stride-renew-prime/PROTOCOL.md frozen at commit 4d4593aabda667a681fab4a67e351dc5bcb7c1f4; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (the launcher waits for its remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the request. This card adds no authority.
precondition (checked by the main session before dispatch, not by you): the launcher builds of the two jobs launched by card CARD-XP-20261029-stride-renew-03 (P20 and P24) have finished. Step 5 checks the project's build count again.

## Frozen values

- REPO = /home/user/crypto (checkout commit bc970ff9a)
- REMOTE = /home/user/crypto/taskq/deploy/eks-karpenter/remote
- One job, size b=32, instance c8a.2xlarge, execution deadline 3600 s: job_name ic-srenew-p32r2-20261029; REQUEST_P32R2 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32r2.json; RECEIPTS_P32R2 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32r2 (must not exist before launch); LOG_P32R2 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32r2.log (outside RECEIPTS; must not exist before launch)
- Why this card exists: job ic-srenew-p32-20261029 (pod 871976a3-bb93-4450-93cd-e22ed581bc61) completed, but its L2 gate rejected 1 of 1680 executions (seq 1605, arm rho-neg, "L2: 51.0 preemptions per second", job-exit 42). Protocol section 6.1 requires that every execution be admitted, and protocol section 4 reruns a size's job whole under a new name on host failure. The request here is byte-identical to request-P32.json apart from job_name (ic-srenew-p32r2-20261029 instead of ic-srenew-p32-20261029); the spec is identical. The original job's data (receipts-P32, launch-P32.log, S3 objects) is kept, not deleted; this card touches none of it. This is a new job name, so it is a first launch of this request, not a resubmission.
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (already verified by the main session with `aws sts get-caller-identity`). AWS credentials are inherited from the process environment; never write, print or copy them.
- env: AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin; ISOLAB_CLIENT_OUTPUT equals RECEIPTS_P32R2
- Checkpoint worktree for step 1: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride
- Pinned sha256:
  - REQUEST_P32R2 31cb6e156271d168663ff32c225d69112967cb7a4a04c07ba839d7e0f9f8c17b
  - REMOTE/validate-request f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7
  - REMOTE/launch-remote c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d

## Procedure (use a single Bash call per step; change nothing). Steps 1 to 5 are checks; if any of them fails, launch nothing.

Step 1. Checkpoint gate. Run:
    cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride && PATH=/tmp/claude-0/awsvenv/bin:$PATH scripts/checkpoint.sh gate
  The exit status must be 0.

Step 2. Hashes (the request and both remote scripts). Run:
    sha256sum -c - <<'HASHES'
    31cb6e156271d168663ff32c225d69112967cb7a4a04c07ba839d7e0f9f8c17b  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32r2.json
    f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7  /home/user/crypto/taskq/deploy/eks-karpenter/remote/validate-request
    c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d  /home/user/crypto/taskq/deploy/eks-karpenter/remote/launch-remote
    HASHES
  (Remove the four leading spaces from every line of this step's here-document, including the line HASHES that ends it; the here-document ends only at a line that is exactly HASHES.) The exit status must be 0 and all 3 output lines must end with ": OK".

Step 3. Receipts directory and log absent. Run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32r2 && echo RECEIPTS_ABSENT
  It must print RECEIPTS_ABSENT. Also run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32r2.log && echo LOG_ABSENT
  It must print LOG_ABSENT.

Step 4. Validate the request. Run:
    cd /home/user/crypto && AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32r2.json && echo VALID_P32R2
  The exit status must be 0 and the output must contain the line VALID_P32R2.

Step 5. At most one build in progress (one Bash call, exactly these two lines in this order):
    export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
    aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w
  The only line printed must be 0 or 1, with nothing on stderr. (The project allows 2 concurrent builds; this launch needs one slot. The export line is there because a `NAME=value` prefix on the aws line would not reach the aws command inside `$(...)`, which is expanded first. The list command takes no --max-items: that option appends a second line `None`, the pagination token, which batch-get-builds rejects as an invalid id.) If the line is 2, refuse with no launch.

Step 6. Launch the job, wait, read the log (one Bash call, exactly these lines in this order):
    cd /home/user/crypto
    echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32r2 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32r2.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32r2.log 2>&1 < /dev/null &
    echo "PID_P32R2 $!"
    python3 -c "import time; time.sleep(90)"
    echo "==== LOG_P32R2 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32r2.log

## Return (verbatim tool output, nothing else)

The LAUNCH_UTC line, the PID_P32R2 line, and the `==== LOG_P32R2 ====` line with the lines `head -n 20` printed under it in step 6. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for the launcher to finish.

## Write scope

The executor writes nothing itself. The launched launch-remote writes only inside RECEIPTS_P32R2 (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32r2) and uploads its request to the S3 bucket named above, and the shell redirect writes LOG_P32R2. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 does not print RECEIPTS_ABSENT, or does not print LOG_ABSENT.
- Step 4 exits with a status other than 0, or does not print VALID_P32R2.
- Step 5 prints anything other than the single line 0 or the single line 1, or prints anything on stderr (the line 2 means both build slots are taken; refuse, do not wait).
- Step 6 does not print the line beginning with PID_P32R2.
- The file named LOG_P32R2 does not exist after step 6.

Each refusal is a specification_error back to the main session. A refusal in steps 1 to 5 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If the launcher reports an error in its log (for example a failed StartBuild), return the log as printed and run nothing further: an ambiguous submission is never retried automatically.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs a detached launch with the log outside the receipts directory and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 6 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline.
