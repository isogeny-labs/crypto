# Mechanical task card CARD-XP-20261029-p20-large-n-01

author_role: experiment-director
executor_role: executor-mechanical
experiment: p20-large-n (job_name p20-large-n-20261029; protocol backlog/XP-20261029-p20-large-n/PROTOCOL.md frozen at commit 97321bfc4fb7872d3fd2b89c068b5749d270452e; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (the launcher waits for the remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the request. This card adds no authority.

## Frozen values

- REPO = /home/user/crypto (checkout commit bc970ff9a)
- REMOTE = /home/user/crypto/taskq/deploy/eks-karpenter/remote
- REQUEST = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/request.json
- job_name p20-large-n-20261029 (instance c8a.2xlarge, execution deadline 3600 s)
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (already verified by the main session with `aws sts get-caller-identity`). AWS credentials are inherited from the process environment; never write, print or copy them.
- env: AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION; an earlier card failed client-side on start-build for want of it, no build created), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/receipts (RECEIPTS; must not exist before launch), PATH prefixed with /tmp/claude-0/awsvenv/bin
- LOG = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/launch.log (outside RECEIPTS; must not exist before launch)
- Checkpoint worktree for step 1: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-n64
- Pinned sha256:
  - REQUEST 5f509f8dce2f8f6ff88484239d25bfef01b0ad5e2dc05e77930b53e37c0ca0cb
  - REMOTE/validate-request f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7
  - REMOTE/launch-remote c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d

## Procedure (use a single Bash call per step; change nothing)

Step 1. Checkpoint gate. Run:
    cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-n64 && PATH=/tmp/claude-0/awsvenv/bin:$PATH scripts/checkpoint.sh gate
  The exit status must be 0.

Step 2. Hashes. Run:
    sha256sum -c - <<'HASHES'
    5f509f8dce2f8f6ff88484239d25bfef01b0ad5e2dc05e77930b53e37c0ca0cb  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/request.json
    f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7  /home/user/crypto/taskq/deploy/eks-karpenter/remote/validate-request
    c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d  /home/user/crypto/taskq/deploy/eks-karpenter/remote/launch-remote
    HASHES
  (Remove the four leading spaces from every line of this step's here-document, including the line HASHES that ends it; the here-document ends only at a line that is exactly HASHES.) The exit status must be 0 and all 3 output lines must end with ": OK".

Step 3. Receipts directory and log absent. Run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/receipts && echo RECEIPTS_ABSENT
  It must print RECEIPTS_ABSENT. Also run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/launch.log && echo LOG_ABSENT
  It must print LOG_ABSENT.

Step 4. Validate the request. Run:
    cd /home/user/crypto && AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/request.json
  The exit status must be 0.

Step 5. Launch, wait, read the log (one Bash call, exactly these lines in this order):
    cd /home/user/crypto
    echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/receipts PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/request.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/launch.log 2>&1 < /dev/null &
    echo "PID $!"
    python3 -c "import time; time.sleep(90)"
    head -n 40 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/launch.log

## Return (verbatim tool output, nothing else)

The LAUNCH_UTC line, the PID line, and the lines `head -n 40` printed in step 5. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for the launcher to finish.

## Write scope

The executor writes nothing itself. The launched launch-remote writes only inside RECEIPTS (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/n64/receipts) and uploads the request to the S3 bucket named above, and the shell redirect writes LOG. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 does not print RECEIPTS_ABSENT, or does not print LOG_ABSENT.
- Step 4 exits with a status other than 0.
- Step 5 does not print a line beginning with PID.
- The file named LOG does not exist after step 5.

Each refusal is a specification_error back to the main session. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If the launcher itself reports an error in the log (for example a failed StartBuild), return the log as printed and run nothing further: an ambiguous submission is never retried automatically.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs a detached launch with a log outside the receipts directory and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 5 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline.
