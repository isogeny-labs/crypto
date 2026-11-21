# Mechanical task card CARD-XP-20261029-stride-renew-03

author_role: experiment-director
executor_role: executor-mechanical
experiment: stride-renew-prime (two jobs ic-srenew-p20-20261029 and ic-srenew-p24-20261029; protocol backlog/XP-20261029-stride-renew-prime/PROTOCOL.md frozen at commit 4d4593aabda667a681fab4a67e351dc5bcb7c1f4; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (each launcher waits for its remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the requests. This card adds no authority.
precondition (checked by the main session before dispatch, not by you): the launcher builds of the two jobs launched by card CARD-XP-20261029-stride-renew-01 (P28 and P32) have finished. Step 4 checks it again.

## Frozen values

- REPO = /home/user/crypto (checkout commit bc970ff9a)
- REMOTE = /home/user/crypto/taskq/deploy/eks-karpenter/remote
- Two jobs, one per size b in {20, 24}; each uses instance c8a.2xlarge and execution deadline 3600 s:
  - b=20: job_name ic-srenew-p20-20261029; REQUEST_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json; RECEIPTS_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20-02 (must not exist before launch); LOG_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20-02.log (outside RECEIPTS; must not exist before launch)
  - b=24: job_name ic-srenew-p24-20261029; REQUEST_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json; RECEIPTS_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24-02 (must not exist before launch); LOG_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24-02.log (outside RECEIPTS; must not exist before launch)
- Why this card exists: card CARD-XP-20261029-stride-renew-02 (same P20 and P24 launches) was refused at its step 5 and launched no job: its step-5 command used `--max-items 5 --query ids`, which prints a second line `None`, passed to batch-get-builds as an invalid id. Only that step 5 differs here. Card -01 launched P28 and P32 only; its P20 and P24 StartBuild calls were refused by the project's concurrency limit (AccountLimitExceededException, 2 concurrent builds), with no build created. The two launches here are therefore first launches, not resubmissions. The paths above keep the suffix -02 (card -02 created nothing at them; step 3 checks that they are absent).
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (already verified by the main session with `aws sts get-caller-identity`). AWS credentials are inherited from the process environment; never write, print or copy them.
- env (common to both jobs): AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin; ISOLAB_CLIENT_OUTPUT is per job and equals that job's RECEIPTS_P{b}
- Checkpoint worktree for step 1: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride
- Pinned sha256:
  - REQUEST_P20 934c04297e1f46b3bc737be234d9c3070071cc8b62252f0a99cfd214103e2f36
  - REQUEST_P24 f3c12341d2b4e736eaba2b6dcb9188c5e35a6eacc370041399271e6eb97f4068
  - REMOTE/validate-request f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7
  - REMOTE/launch-remote c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d

## Procedure (use a single Bash call per step; change nothing). Steps 1 to 5 are checks; if any of them fails, launch neither job.

Step 1. Checkpoint gate (runs once for both jobs). Run:
    cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride && PATH=/tmp/claude-0/awsvenv/bin:$PATH scripts/checkpoint.sh gate
  The exit status must be 0.

Step 2. Hashes (both requests and both remote scripts). Run:
    sha256sum -c - <<'HASHES'
    934c04297e1f46b3bc737be234d9c3070071cc8b62252f0a99cfd214103e2f36  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json
    f3c12341d2b4e736eaba2b6dcb9188c5e35a6eacc370041399271e6eb97f4068  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json
    f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7  /home/user/crypto/taskq/deploy/eks-karpenter/remote/validate-request
    c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d  /home/user/crypto/taskq/deploy/eks-karpenter/remote/launch-remote
    HASHES
  (Remove the four leading spaces from every line of this step's here-document, including the line HASHES that ends it; the here-document ends only at a line that is exactly HASHES.) The exit status must be 0 and all 4 output lines must end with ": OK".

Step 3. Receipts directories and logs absent. Run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20-02 && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24-02 && echo RECEIPTS_ABSENT
  It must print RECEIPTS_ABSENT. Also run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20-02.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24-02.log && echo LOGS_ABSENT
  It must print LOGS_ABSENT.

Step 4. Validate the two requests (one Bash call, exactly these two commands joined by &&, in this order):
    cd /home/user/crypto && AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json && echo VALID_P20 &&
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json && echo VALID_P24
  The exit status must be 0 and the output must contain the two lines VALID_P20 and VALID_P24.

Step 5. No build in progress (one Bash call, exactly these two lines in this order):
    export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
    aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w
  The only line printed must be 0, with nothing on stderr. (The export line is there because a `NAME=value` prefix on the aws line would not reach the aws command inside `$(...)`, which is expanded first. The list command takes no --max-items: that option appends a second line `None`, the pagination token, which batch-get-builds rejects as an invalid id and which caused the refusal of card -02.)

Step 6. Launch the two jobs, wait, read the logs (one Bash call, exactly these lines in this order):
    cd /home/user/crypto
    echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20-02 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20-02.log 2>&1 < /dev/null &
    echo "PID_P20 $!"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24-02 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24-02.log 2>&1 < /dev/null &
    echo "PID_P24 $!"
    python3 -c "import time; time.sleep(90)"
    echo "==== LOG_P20 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20-02.log
    echo "==== LOG_P24 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24-02.log

## Return (verbatim tool output, nothing else)

The LAUNCH_UTC line, the two PID_P lines, and, for each job, the `==== LOG_P<b> ====` line and the lines `head -n 20` printed under it in step 6. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for any launcher to finish.

## Write scope

The executor writes nothing itself. Each launched launch-remote writes only inside its own RECEIPTS_P{b} (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20-02, /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24-02) and uploads its request to the S3 bucket named above, and the shell redirects write LOG_P20 and LOG_P24. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 does not print RECEIPTS_ABSENT, or does not print LOGS_ABSENT.
- Step 4 exits with a status other than 0, or does not print both lines VALID_P20 and VALID_P24.
- Step 5 prints anything other than the single line 0, or prints anything on stderr (an output of 1 or 2 means a build is in progress; refuse, do not wait).
- Step 6 does not print both lines beginning with PID_P20 and PID_P24.
- Either of the files named LOG_P20 and LOG_P24 does not exist after step 6.

Each refusal is a specification_error back to the main session. A refusal in steps 1 to 5 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If either launcher reports an error in its log (for example a failed StartBuild), return both logs as printed and run nothing further: an ambiguous submission is never retried automatically, and the other job is not cancelled or resubmitted.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs two detached launches with logs outside the receipts directories and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 6 waits with python3 for that reason. Each launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. The two launches are independent: one job's failure does not change what the other is asked to do.
