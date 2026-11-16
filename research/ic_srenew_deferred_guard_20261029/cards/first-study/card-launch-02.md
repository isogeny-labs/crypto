# Mechanical task card CARD-XP-20261029-srenew-deferred-guard-launch-02

author_role: experiment-director
executor_role: executor-mechanical
experiment: srenew-deferred-guard (jobs for P32; protocol backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md frozen by commit be2969cebf30a0cfb79088fd10094324c3efb3bb, route amended by AMENDMENT-01.md in commit ad24577e; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (each launcher waits for its remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the requests. This card adds no authority.
precondition (checked by the main session before dispatch, not by you): at most 1 launcher build is in progress (the builds of the P24 and P28 jobs launched by CARD-XP-20261029-srenew-deferred-guard-launch-01 may still be running). Step 5 checks it again.

## Frozen values

- SRC = /dev/shm/src-be2969ce (clean sparse checkout of source revision be2969cebf30a0cfb79088fd10094324c3efb3bb; the launcher scripts there are that revision's own)
- REMOTE = /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote
- Never use the launcher scripts of /home/user/crypto: its launch-remote differs from the one in SRC.
- Image of every job: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c (recorded in /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/image.json; each request embeds it)
- One job, instance c8a.2xlarge and execution deadline 3600 s each:
  - b=32: job_name ic-dg-p32-20261029; REQUEST_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/request-P32.json; RECEIPTS_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/receipts-P32 (must not exist before launch); LOG_P32 = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/launch-P32.log (outside RECEIPTS; must not exist before launch)
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (verified by the main session). AWS credentials are inherited from the process environment; never write, print or copy them.
- env (common to all jobs): AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin; ISOLAB_CLIENT_OUTPUT is per job and equals that job's RECEIPTS_P32
- Working directory of every launch and validation command: /dev/shm/src-be2969ce
- Checkpoint gate: run in /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg with the script of the sibling worktree wt-pbo (wt-dg has no scripts directory).
- Pinned sha256:
  - REQUEST_P32 3b2fa05bb278c6a90abcc6f83f5ed8ba071d0c19bd4669be514bd9129118d0b5
  - REMOTE/validate-request dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426
  - REMOTE/launch-remote 076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45
  - checkpoint.sh 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f

## Procedure (use a single Bash call per step; change nothing). Steps 1 to 5 are checks; if any of them fails, launch nothing.

Step 1. Checkpoint gate. Run:

```
cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash ../wt-pbo/scripts/checkpoint.sh gate
```

The exit status must be 0.

Step 2. Hashes (the request and the launcher scripts, and the checkpoint script). Run:

```
sha256sum -c - <<'HASHES'
3b2fa05bb278c6a90abcc6f83f5ed8ba071d0c19bd4669be514bd9129118d0b5  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/request-P32.json
dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/validate-request
076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/launch-remote
56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-pbo/scripts/checkpoint.sh
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all 4 output lines must end with ": OK".

Step 3. Receipts directories, logs and S3 prefixes unused. Run:

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/receipts-P32 && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/launch-P32.log && echo ABSENT_P32
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dg-p32-20261029/)" && echo S3_EMPTY_P32
```

The exit status must be 0 and the output must contain the lines ABSENT_P32 and S3_EMPTY_P32.

Step 4. Validate the request (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/request-P32.json > /dev/null && echo VALID_P32
```

The exit status must be 0 and the output must contain the line VALID_P32.

Step 5. Build slots (one Bash call, exactly these two lines in this order):

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w
```

The only line printed must be 0 or 1, with nothing on stderr. (The project allows 2 concurrent builds and this launch needs one slot. An output of 2 means both slots are taken; refuse, do not wait.) (The export line is there because a `NAME=value` prefix on the aws line would not reach the aws command inside `$(...)`, which is expanded first. The list command takes no --max-items: that option appends a second line `None`, which batch-get-builds rejects as an invalid id.)

Step 6. Launch the job, wait, read the log (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/receipts-P32 PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/request-P32.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/launch-P32.log 2>&1 < /dev/null &
echo "PID_P32 $!"
python3 -c "import time; time.sleep(90)"
echo "==== LOG_P32 ===="
head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/launch-P32.log
```

## Return (verbatim tool output, nothing else)

The output of steps 1 to 5 as printed, then the LAUNCH_UTC line, the PID_P32 line, and, the line `==== LOG_P32 ====` and the lines `head -n 20` printed under it in step 6. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for any launcher to finish.

## Write scope

The executor writes nothing itself. Each launched launch-remote writes only inside its own RECEIPTS_P32 (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/receipts-P32) and uploads its request and receipts to the S3 bucket named above, and the shell redirects write LOG_P32. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 exits with a status other than 0, or lacks any of its required lines (a missing S3_EMPTY line means the S3 prefix of that job name is not empty).
- Step 4 exits with a status other than 0, or lacks any of its required VALID_P lines.
- Step 5 prints anything other than the single line 0 or the single line 1, or prints anything on stderr.
- Step 6 does not print the line beginning with PID_P32.
- The file named LOG_P32 does not exist after step 6.

Each refusal is a specification_error back to the main session. A refusal in steps 1 to 5 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If the launcher reports an error in its log (for example a failed StartBuild), return the log as printed and run nothing further: an ambiguous submission is never retried automatically.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs a detached launch with the log outside the receipts directory and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 6 waits with python3 for that reason. Each launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. P24 and P28 are launched by a separate card.
