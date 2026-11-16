# Mechanical task card CARD-XP-20261029-srenew-deferred-guard-v3-launch

author_role: experiment-director
executor_role: executor-mechanical
experiment: srenew-deferred-guard-v3 (one job per dispatch, for one bit size __B__ in {24, 28, 32}; protocol backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
background: yes (the launcher waits for its remote job, up to 5400 s; this task returns after about 90 seconds)
approval: the main (reasoning-tier) session decided this route and the requests, and the Coordinator approved the protocol (APPROVAL.md in the protocol directory). This card adds no authority.
dispatch value: the dispatch names exactly one value for __B__ (24, 28 or 32) and the sha256 of this card file (CARD_SHA256). Substitute the digits of __B__ everywhere the text __B__ appears in this card, and change nothing else.

## Frozen values

- SRC = /dev/shm/src-be2969ce (clean sparse checkout of source revision be2969cebf30a0cfb79088fd10094324c3efb3bb; the launcher scripts there are that revision's own)
- REMOTE = /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote
- Never use the launcher scripts of /home/user/crypto: its launch-remote differs from the one in SRC.
- Image of the job: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c (the request embeds it)
- One job, instance c8a.2xlarge and execution deadline 3600 s:
  - job_name ic-dg3-p__B__-20261029
  - REQUEST = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/request-P__B__.json
  - RECEIPTS = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/receipts-P__B__ (must not exist before launch)
  - LOG = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/launch-P__B__.log (outside RECEIPTS; must not exist before launch)
  - S3 prefix s3://isolab-results-104186811093-us-east-2/runs/ic-dg3-p__B__-20261029/ (must be empty before launch)
- Frozen protocol directory PD = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3
- Frozen protocol worktree WT = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg
- Backend: isolab-eks AWS launcher, AWS account 104186811093 (verified by the main session). AWS credentials are inherited from the process environment; never write, print or copy them.
- env: AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin, ISOLAB_CLIENT_OUTPUT equal to RECEIPTS
- Working directory of every launch and validation command: /dev/shm/src-be2969ce
- Checkpoint gate: run in WT with the script of the sibling worktree wt-pbo (wt-dg has no scripts directory).
- Pinned sha256 (requests, one per __B__):
  - request-P24.json 2d19365f8cd2d57eefcf8ea6e0b8105d45eee9005c192581ced5c5556528c688
  - request-P28.json 4ac95d173112283b93445fdb28d7b6a931577c9dedcf7fd5eee0c51b1a316290
  - request-P32.json 8df53d10c8905c34c6f0658df4c14981b8dd9d598f960cda143b6a627475ddcb
- Pinned sha256 (protocol files in PD):
  - PROTOCOL.md caf2519d87241ed0ab7f947ecfe7aa409e7eed3b6f5d648bab8df2569c6143be
  - spec-P24.json ce1b34733baabd92411fe069f05b837988191e4229a6f7f118e36db31f49617e
  - spec-P28.json d3e9a7247fc34effd9282a9a32590fa45b9d12700c7bb4fd282682c874503cb8
  - spec-P32.json 1d817e4a6261d7f31ab88bc5f4b72953e4dee3d06eaa2e97a0b956bdb8b9e8ba
- Pinned sha256 (scripts):
  - REMOTE/validate-request dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426
  - REMOTE/launch-remote 076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45
  - checkpoint.sh 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f
- Frozen source commit of the protocol: e277272384d683a4d4b9983a141b523bf28c9f4a; branch claude/ic-srenew-deferred-guard-run-20261029

## Procedure (use a single Bash call per step; change nothing). Steps 0 to 7 are checks; if any of them fails, launch nothing.

Step 0. Card digest. Run (the left side of the comparison is the CARD_SHA256 value stated in the dispatch):

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/card-launch-v3.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the text __B__ unsubstituted; the substitution is made only in the commands you run.)

Step 1. Checkpoint gate. Run:

```
cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash ../wt-pbo/scripts/checkpoint.sh gate
```

The exit status must be 0.

Step 2. Hashes (the request, the protocol files, the launcher scripts, the checkpoint script). Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
24) REQ_SHA=2d19365f8cd2d57eefcf8ea6e0b8105d45eee9005c192581ced5c5556528c688; SPEC_SHA=ce1b34733baabd92411fe069f05b837988191e4229a6f7f118e36db31f49617e;;
28) REQ_SHA=4ac95d173112283b93445fdb28d7b6a931577c9dedcf7fd5eee0c51b1a316290; SPEC_SHA=d3e9a7247fc34effd9282a9a32590fa45b9d12700c7bb4fd282682c874503cb8;;
32) REQ_SHA=8df53d10c8905c34c6f0658df4c14981b8dd9d598f960cda143b6a627475ddcb; SPEC_SHA=1d817e4a6261d7f31ab88bc5f4b72953e4dee3d06eaa2e97a0b956bdb8b9e8ba;;
*) echo BAD_B; exit 1;;
esac
sha256sum -c - <<HASHES
$REQ_SHA  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/request-P__B__.json
$SPEC_SHA  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P__B__.json
caf2519d87241ed0ab7f947ecfe7aa409e7eed3b6f5d648bab8df2569c6143be  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md
dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/validate-request
076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/launch-remote
56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-pbo/scripts/checkpoint.sh
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all 6 output lines must end with ": OK".

Step 3. Request spec equals the frozen spec, and the protocol commit is in the worktree and pushed. Run, as one Bash call, exactly these lines in this order:

```
python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/request-P__B__.json /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P__B__.json && echo SPEC_EQUAL_P__B__
cd /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg
git merge-base --is-ancestor e277272384d683a4d4b9983a141b523bf28c9f4a HEAD && echo HEAD_CONTAINS_COMMIT
git merge-base --is-ancestor e277272384d683a4d4b9983a141b523bf28c9f4a origin/claude/ic-srenew-deferred-guard-run-20261029 && echo COMMIT_PUSHED
```

The output must contain the three lines SPEC_EQUAL_P__B__, HEAD_CONTAINS_COMMIT and COMMIT_PUSHED, and nothing on stderr.

Step 4. Trigger file (required for __B__ = 32 only). Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
32) grep -qxF 'TRIGGER: first-study P32 failed §6.1' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/P32-TRIGGER.md && echo TRIGGER_OK_P32;;
*) echo TRIGGER_NOT_REQUIRED;;
esac
```

For __B__ = 32 the exit status must be 0 and the output must be the line TRIGGER_OK_P32 (a missing file or a missing line makes grep exit with a status other than 0 and prints no such line). For __B__ = 24 or 28 the output must be the line TRIGGER_NOT_REQUIRED.

Step 5. Receipts directory, log and S3 prefix unused. Run:

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/receipts-P__B__ && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/launch-P__B__.log && echo ABSENT_P__B__
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dg3-p__B__-20261029/)" && echo S3_EMPTY_P__B__
```

The exit status must be 0 and the output must contain the lines ABSENT_P__B__ and S3_EMPTY_P__B__.

Step 6. Validate the request (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/request-P__B__.json > /dev/null && echo VALID_P__B__
```

The exit status must be 0 and the output must contain the line VALID_P__B__.

Step 7. Build slots (one Bash call, exactly these two lines in this order):

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w
```

The only line printed must be 0 or 1, with nothing on stderr. (The project allows 2 concurrent builds and this launch needs one slot. An output of 2 means both slots are taken; refuse, do not wait.) (The export line is there because a `NAME=value` prefix on the aws line would not reach the aws command inside `$(...)`, which is expanded first. The list command takes no --max-items: that option appends a second line `None`, which batch-get-builds rejects as an invalid id.)

Step 8. Launch the job, wait, read the log (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
echo "LAUNCH_UTC $(date -u +%Y-%m-%dT%H:%M:%SZ)"
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/receipts-P__B__ PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup taskq/deploy/eks-karpenter/remote/launch-remote /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/request-P__B__.json > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/launch-P__B__.log 2>&1 < /dev/null &
echo "PID_P__B__ $!"
python3 -c "import time; time.sleep(90)"
echo "==== LOG_P__B__ ===="
head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/launch-P__B__.log
```

## Return (verbatim tool output, nothing else)

The output of steps 0 to 7 as printed, then the LAUNCH_UTC line, the PID_P__B__ line, the line `==== LOG_P__B__ ====` and the lines `head -n 20` printed under it in step 8. No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands. Do not wait for the launcher to finish.

## Write scope

The executor writes nothing itself. The launched launch-remote writes only inside RECEIPTS (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/receipts-P__B__) and uploads its request and receipts to the S3 bucket named above, and the shell redirect writes LOG. Write nowhere else.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly one value of __B__ from 24, 28, 32, or does not state CARD_SHA256.
- Step 0 prints a first field different from CARD_SHA256, or exits with a status other than 0.
- Step 1 exits with a status other than 0.
- Step 2 exits with a status other than 0, prints BAD_B, or any of its output lines does not end with ": OK".
- Step 3 exits with a status other than 0, prints anything on stderr, or lacks any of its three required lines.
- Step 4 exits with a status other than 0, or lacks its required line (TRIGGER_OK_P32 for __B__ = 32; TRIGGER_NOT_REQUIRED otherwise). For __B__ = 32 this refusal means the trigger of the protocol was not recorded, and nothing is launched.
- Step 5 exits with a status other than 0, or lacks any of its required lines (a missing S3_EMPTY line means the S3 prefix of that job name is not empty).
- Step 6 exits with a status other than 0, or lacks its required VALID_P__B__ line.
- Step 7 prints anything other than the single line 0 or the single line 1, or prints anything on stderr.
- Step 8 does not print the line beginning with PID_P__B__.
- The file named LOG does not exist after step 8.

Each refusal is a specification_error back to the main session. A refusal in steps 0 to 7 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch. If the launcher reports an error in its log (for example a failed StartBuild), return the log as printed and run nothing further: an ambiguous submission is never retried automatically.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs a detached launch with the log outside the receipts directory and a 90-second partial report, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 8 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. Each bit size is launched by a separate dispatch of this card.
