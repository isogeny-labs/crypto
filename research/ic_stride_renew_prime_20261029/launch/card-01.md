# Mechanical task card CARD-XP-20261029-stride-renew-01

author_role: experiment-director
executor_role: executor-mechanical
experiment: stride-renew-prime (four jobs ic-srenew-p20-20261029, ic-srenew-p24-20261029, ic-srenew-p28-20261029, ic-srenew-p32-20261029; protocol backlog/XP-20261029-stride-renew-prime/PROTOCOL.md frozen at commit 4d4593aabda667a681fab4a67e351dc5bcb7c1f4; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (each launcher waits for its remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the requests. This card adds no authority.

## Frozen values

- REPO = /home/user/crypto (checkout commit bc970ff9a)
- REMOTE = /home/user/crypto/taskq/deploy/eks-karpenter/remote
- Four jobs, one per size b in {20, 24, 28, 32}; each uses instance c8a.2xlarge and execution deadline 3600 s:
  - b=20: job_name ic-srenew-p20-20261029; REQUEST_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json; RECEIPTS_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20 (must not exist before launch); LOG_P20 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20.log (outside RECEIPTS; must not exist before launch)
  - b=24: job_name ic-srenew-p24-20261029; REQUEST_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json; RECEIPTS_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24 (must not exist before launch); LOG_P24 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24.log (outside RECEIPTS; must not exist before launch)
  - b=28: job_name ic-srenew-p28-20261029; REQUEST_P28 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P28.json; RECEIPTS_P28 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P28 (must not exist before launch); LOG_P28 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P28.log (outside RECEIPTS; must not exist before launch)
  - b=32: job_name ic-srenew-p32-20261029; REQUEST_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32.json; RECEIPTS_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32 (must not exist before launch); LOG_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32.log (outside RECEIPTS; must not exist before launch)
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (already verified by the main session with `aws sts get-caller-identity`). AWS credentials are inherited from the process environment; never write, print or copy them.
- env (common to all four jobs): AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION; an earlier card failed client-side on start-build for want of it, no build created), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin; ISOLAB_CLIENT_OUTPUT is per job and equals that job's RECEIPTS_P{b}
- Checkpoint worktree for step 1: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride
- Pinned sha256:
  - REQUEST_P20 934c04297e1f46b3bc737be234d9c3070071cc8b62252f0a99cfd214103e2f36
  - REQUEST_P24 f3c12341d2b4e736eaba2b6dcb9188c5e35a6eacc370041399271e6eb97f4068
  - REQUEST_P28 3d7602bf2472691b915b4776a0b5cf5f07fd01a4ce5b205a98399e354a4aa98a
  - REQUEST_P32 07e5ef0e73e45c46a61dd067dbd33fc63196937be4983485288c7b1464cd9f62
  - REMOTE/validate-request f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7
  - REMOTE/launch-remote c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d

## Procedure (use a single Bash call per step; change nothing). Steps 1 to 4 are checks; if any of them fails, launch none of the four jobs.

Step 1. Checkpoint gate (runs once for all four jobs). Run:
    cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-stride && PATH=/tmp/claude-0/awsvenv/bin:$PATH scripts/checkpoint.sh gate
  The exit status must be 0.

Step 2. Hashes (all four requests and both remote scripts). Run:
    sha256sum -c - <<'HASHES'
    934c04297e1f46b3bc737be234d9c3070071cc8b62252f0a99cfd214103e2f36  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json
    f3c12341d2b4e736eaba2b6dcb9188c5e35a6eacc370041399271e6eb97f4068  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json
    3d7602bf2472691b915b4776a0b5cf5f07fd01a4ce5b205a98399e354a4aa98a  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P28.json
    07e5ef0e73e45c46a61dd067dbd33fc63196937be4983485288c7b1464cd9f62  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32.json
    f0c06c827dca7aa38a913e787ee4a92be015555e800e5d808c7293b76201cec7  /home/user/crypto/taskq/deploy/eks-karpenter/remote/validate-request
    c5a4e41bda7297a6429196ad4bed3d7bb7232d792d0d40ffcc38c8a73463124d  /home/user/crypto/taskq/deploy/eks-karpenter/remote/launch-remote
    HASHES
  (Remove the four leading spaces from every line of this step's here-document, including the line HASHES that ends it; the here-document ends only at a line that is exactly HASHES.) The exit status must be 0 and all 6 output lines must end with ": OK".

Step 3. Receipts directories and logs absent. Run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20 && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24 && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P28 && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32 && echo RECEIPTS_ABSENT
  It must print RECEIPTS_ABSENT. Also run:
    test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P28.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32.log && echo LOGS_ABSENT
  It must print LOGS_ABSENT.

Step 4. Validate the four requests (one Bash call, exactly these four commands joined by &&, in this order):
    cd /home/user/crypto && AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json && echo VALID_P20 &&
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json && echo VALID_P24 &&
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P28.json && echo VALID_P28 &&
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32.json && echo VALID_P32
  The exit status must be 0 and the output must contain the four lines VALID_P20, VALID_P24, VALID_P28, VALID_P32.

Step 5. Launch the four jobs, wait, read the logs (one Bash call, exactly these lines in this order):
    cd /home/user/crypto
    echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P20.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20.log 2>&1 < /dev/null &
    echo "PID_P20 $!"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P24.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24.log 2>&1 < /dev/null &
    echo "PID_P24 $!"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P28 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P28.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P28.log 2>&1 < /dev/null &
    echo "PID_P28 $!"
    AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/request-P32.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32.log 2>&1 < /dev/null &
    echo "PID_P32 $!"
    python3 -c "import time; time.sleep(90)"
    echo "==== LOG_P20 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P20.log
    echo "==== LOG_P24 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P24.log
    echo "==== LOG_P28 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P28.log
    echo "==== LOG_P32 ===="
    head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/launch-P32.log

## Return (verbatim tool output, nothing else)

The LAUNCH_UTC line, the four PID_P lines, and, for each job, the `==== LOG_P<b> ====` line and the lines `head -n 20` printed under it in step 5. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for any launcher to finish.

## Write scope

The executor writes nothing itself. Each launched launch-remote writes only inside its own RECEIPTS_P{b} (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P20, /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P24, /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P28, /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/stride/receipts-P32) and uploads its request to the S3 bucket named above, and the shell redirects write LOG_P20, LOG_P24, LOG_P28 and LOG_P32. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 does not print RECEIPTS_ABSENT, or does not print LOGS_ABSENT.
- Step 4 exits with a status other than 0, or does not print all four lines VALID_P20, VALID_P24, VALID_P28, VALID_P32.
- Step 5 does not print all four lines beginning with PID_P20, PID_P24, PID_P28 and PID_P32.
- Any of the four files named LOG_P20, LOG_P24, LOG_P28, LOG_P32 does not exist after step 5.

Each refusal is a specification_error back to the main session. A refusal in steps 1 to 4 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If any launcher reports an error in its log (for example a failed StartBuild), return all four logs as printed and run nothing further: an ambiguous submission is never retried automatically, and the other jobs are not cancelled or resubmitted.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs four detached launches with logs outside the receipts directories and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 5 waits with python3 for that reason. Each launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. The four launches are independent: one job's failure does not change what the others are asked to do.
