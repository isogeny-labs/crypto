# Mechanical task card CARD-XP-20261029-dg-blocked-launch

author_role: experiment-director
executor_role: executor-mechanical
experiment: dg-blocked (protocol backlog/XP-20261029-dg-blocked/PROTOCOL.md; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol with edits E1-E10 (APPROVAL.md in the protocol directory) and ruled on pre-launch check P2 (RULING-P2.md, which adds condition A1); the main (reasoning-tier) session decided the route and the requests. This card adds no authority.

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

- Freeze commit of the protocol: cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae; branch claude/ic-dg-blocked-20261029 (pushed). The freeze commit message records the six spec sha256 (pinned again below).
- Coordinator ruling on pre-launch check P2: research/ic_dg_blocked_20261029/cards/RULING-P2.md, sha256 aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d (committed at df274b2aa3; pinned below). It adds condition A1, which the analysis card checks; this card launches nothing that the ruling changes.
- Resume checkpoint at the freeze: 20261029T124848Z-cd04cfd2d1df (s3://setec-astronomy-artifacts/checkpoints/crypto/claude__ic-dg-blocked-20261029/20261029T124848Z-cd04cfd2d1df/). Step 1 may write a newer one because the branch has later commits; only its exit status is read.
- SRC = /dev/shm/src-be2969ce (clean sparse checkout of source revision be2969cebf30a0cfb79088fd10094324c3efb3bb; the launcher scripts there are that revision's own)
- REMOTE = /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote
- Never use the launcher scripts of /home/user/crypto: its launch-remote differs from the one in SRC.
- Image of every job: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c (each request embeds it)
- ecbench sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c; source revision be2969cebf30a0cfb79088fd10094324c3efb3bb (each request embeds both).
- Every job: instance c8a.2xlarge, execution deadline 3600 s, provenance.harness claude-code-local, provenance.run_url https://github.com/isogeny-labs/crypto/blob/cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae/backlog/XP-20261029-dg-blocked/PROTOCOL.md
- DGB = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb (requests, receipts, logs and pid files; nothing larger than a few kilobytes is written there; large outputs, if any, go to /dev/shm)
- Protocol directory PD = /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked
- Protocol worktree WT = /dev/shm/wt-blk
- Backend: isolab-eks AWS launcher, AWS account 104186811093. AWS credentials are inherited from the process environment; never write, print or copy them.
- env of every AWS command: AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2 (awscli v1 in the venv reads only AWS_DEFAULT_REGION), ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin
- Working directory of every launch and validation command: /dev/shm/src-be2969ce
- Checkpoint gate: run in WT with WT's own scripts/checkpoint.sh.

The two jobs of a bit size (tag = the text after `request-` in the request file name):

| job | tag | job_name | REQUEST | RECEIPTS (must not exist before launch) | LOG (must not exist before launch) | PIDFILE (must not exist before launch) | S3 prefix under s3://isolab-results-104186811093-us-east-2/runs/ (must be empty before launch) |
|:--|:--|:--|:--|:--|:--|:--|:--|
| wall | P__B__ | ic-dgb-p__B__-20261029 | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__ | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__ | ic-dgb-p__B__-20261029/ |
| counted | P__B__-rho | ic-dgb-rho-p__B__-20261029 | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__-rho | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho | ic-dgb-rho-p__B__-20261029/ |

Pinned sha256 (requests, spec embedded byte for byte as JSON-equal content):
- request-P24.json 319b84c23d919b7d934fcfd95dbc3e74ac413a1181bee41d19dbbfd095ae36a3
- request-P24-rho.json 48bfbda18ddf74a2182cc35e83518f6cea91124bf3ba41533e6e939df28ba6af
- request-P28.json 30354790bbc40fb3a5b2a2aeb65c4512d6025b7e2ff879cf29e0bd70b45e68be
- request-P28-rho.json 7ff744af42dc2de275d7bb0fec3be787e6e34747bfdd67eb8ba0f3d8890421ac
- request-P32.json e45305d9eaf263b2dc06694719cea534a3300aa8ecf2936131bddabb943aeec7
- request-P32-rho.json a6fb4b7f9ef422948f2a7f976d207232a445101367fb0a7db3dbcdea6135e07e

Pinned sha256 (protocol files in PD, as committed at the freeze commit):
- PROTOCOL.md 4b55710d92c10e1aa9b5a0f1665371b20d2dbd32f667d213383bfa960096898a
- APPROVAL.md f0af35a925faa73953d973e831fef0225e64a11a342c63569a3e6ab33c73074e
- spec-P24.json 62267dc13d78ff7fbbf285933dc25581fa1de207464a3377bac4200441e3b54c
- spec-P24-rho.json f1e1a8081f58a57e2c6bfe6041cab9597c5aa16a4f6b03a0da981e511c01e669
- spec-P28.json 8f1908c39e46344f3d1a4edc64d4a50d1e3ef7a6eba04c8fb368ef9e154a68f6
- spec-P28-rho.json 3e69c4f9501b4d43291492da12bae551811d1bf54659f414c2cd6a81315cbfba
- spec-P32.json 1e0f5fb9979e8916b625594a868f54fb201fa85a0161027ec0c83c12e39ba120
- spec-P32-rho.json 9a23e873c9fd2d852601d691a9a89c19ef9d9116ccddcf70eb4f12b7167572e5

Pinned sha256 (files of this study outside PD, as committed; WT = /dev/shm/wt-blk):
- research/ic_dg_blocked_20261029/cards/prelaunch-checks.md a66af69491204ebd98154ce022220179f1d3afd67d79a8b269df2fd7bb4b6ae1
- research/ic_dg_blocked_20261029/cards/RULING-P2.md aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d

Pinned sha256 (scripts):
- REMOTE/validate-request dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426
- REMOTE/launch-remote 076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45
- WT/scripts/checkpoint.sh 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f
- /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py 23d2324cd222727b66e6c20d01426ae00b3131baf8b707c3b790b257bca46ac0

## Procedure L (MODE LAUNCH). Use a single Bash call per step; change nothing. Steps 0 to 7 are checks; if any of them fails, launch nothing.

Step 0. Card digest. Run (the left side of the comparison is the CARD_SHA256 value stated in the dispatch):

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/card-launch.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the text __B__ unsubstituted; the substitution is made only in the commands you run.)

Step 1. Checkpoint gate. Run:

```
cd /dev/shm/wt-blk && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash scripts/checkpoint.sh gate
```

The exit status must be 0.

Step 2. Hashes (the two requests, the protocol files, the launcher scripts, the checkpoint script, the wait script). Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
24) REQ_SHA=319b84c23d919b7d934fcfd95dbc3e74ac413a1181bee41d19dbbfd095ae36a3; REQR_SHA=48bfbda18ddf74a2182cc35e83518f6cea91124bf3ba41533e6e939df28ba6af; SPEC_SHA=62267dc13d78ff7fbbf285933dc25581fa1de207464a3377bac4200441e3b54c; SPECR_SHA=f1e1a8081f58a57e2c6bfe6041cab9597c5aa16a4f6b03a0da981e511c01e669;;
28) REQ_SHA=30354790bbc40fb3a5b2a2aeb65c4512d6025b7e2ff879cf29e0bd70b45e68be; REQR_SHA=7ff744af42dc2de275d7bb0fec3be787e6e34747bfdd67eb8ba0f3d8890421ac; SPEC_SHA=8f1908c39e46344f3d1a4edc64d4a50d1e3ef7a6eba04c8fb368ef9e154a68f6; SPECR_SHA=3e69c4f9501b4d43291492da12bae551811d1bf54659f414c2cd6a81315cbfba;;
32) REQ_SHA=e45305d9eaf263b2dc06694719cea534a3300aa8ecf2936131bddabb943aeec7; REQR_SHA=a6fb4b7f9ef422948f2a7f976d207232a445101367fb0a7db3dbcdea6135e07e; SPEC_SHA=1e0f5fb9979e8916b625594a868f54fb201fa85a0161027ec0c83c12e39ba120; SPECR_SHA=9a23e873c9fd2d852601d691a9a89c19ef9d9116ccddcf70eb4f12b7167572e5;;
*) echo BAD_B; exit 1;;
esac
sha256sum -c - <<HASHES
$REQ_SHA  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json
$REQR_SHA  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json
$SPEC_SHA  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__.json
$SPECR_SHA  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__-rho.json
4b55710d92c10e1aa9b5a0f1665371b20d2dbd32f667d213383bfa960096898a  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/PROTOCOL.md
f0af35a925faa73953d973e831fef0225e64a11a342c63569a3e6ab33c73074e  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/APPROVAL.md
a66af69491204ebd98154ce022220179f1d3afd67d79a8b269df2fd7bb4b6ae1  /dev/shm/wt-blk/research/ic_dg_blocked_20261029/cards/prelaunch-checks.md
aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d  /dev/shm/wt-blk/research/ic_dg_blocked_20261029/cards/RULING-P2.md
dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/validate-request
076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45  /dev/shm/src-be2969ce/taskq/deploy/eks-karpenter/remote/launch-remote
56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f  /dev/shm/wt-blk/scripts/checkpoint.sh
23d2324cd222727b66e6c20d01426ae00b3131baf8b707c3b790b257bca46ac0  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all 12 output lines must end with ": OK".

Step 3. Each request equals its frozen spec and its frozen fields; the freeze commit is in the worktree and pushed; the protocol directory equals the freeze commit. Run, as one Bash call, exactly these lines in this order:

```
python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__.json && echo SPEC_EQUAL_P__B__
python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__-rho.json && echo SPEC_EQUAL_P__B__-rho
diff <(jq -S .job.spec /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json) <(jq -S . /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__.json) && echo JQS_EQUAL_P__B__
diff <(jq -S .job.spec /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json) <(jq -S . /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P__B__-rho.json) && echo JQS_EQUAL_P__B__-rho
jq -e '.job.job_name=="ic-dgb-p__B__-20261029" and .job.instance_type=="c8a.2xlarge" and .job.seconds==3600 and .job.build.image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c" and .job.build.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .job.build.ecbench_sha256=="88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c" and .provenance.harness=="claude-code-local" and .provenance.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.workflow_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.run_url=="https://github.com/isogeny-labs/crypto/blob/cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae/backlog/XP-20261029-dg-blocked/PROTOCOL.md"' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json > /dev/null && echo FIELDS_OK_P__B__
jq -e '.job.job_name=="ic-dgb-rho-p__B__-20261029" and .job.instance_type=="c8a.2xlarge" and .job.seconds==3600 and .job.build.image_uri=="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c" and .job.build.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .job.build.ecbench_sha256=="88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c" and .provenance.harness=="claude-code-local" and .provenance.source_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.workflow_revision=="be2969cebf30a0cfb79088fd10094324c3efb3bb" and .provenance.run_url=="https://github.com/isogeny-labs/crypto/blob/cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae/backlog/XP-20261029-dg-blocked/PROTOCOL.md"' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json > /dev/null && echo FIELDS_OK_P__B__-rho
cd /dev/shm/wt-blk
git merge-base --is-ancestor cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae HEAD && echo HEAD_CONTAINS_COMMIT
git merge-base --is-ancestor cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae origin/claude/ic-dg-blocked-20261029 && echo COMMIT_PUSHED
git diff --quiet cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae -- backlog/XP-20261029-dg-blocked && echo PD_EQUALS_COMMIT
```

The output must contain the nine lines SPEC_EQUAL_P__B__, SPEC_EQUAL_P__B__-rho, JQS_EQUAL_P__B__, JQS_EQUAL_P__B__-rho, FIELDS_OK_P__B__, FIELDS_OK_P__B__-rho, HEAD_CONTAINS_COMMIT, COMMIT_PUSHED and PD_EQUALS_COMMIT, and nothing on stderr.

Step 4. Order: the size before this one has finished. Run, as one Bash call, exactly these lines in this order:

```
case __B__ in
24) echo ORDER_NOT_REQUIRED;;
28) python3 -I /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py 0 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P24 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P24-rho && test -f /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P24/result.json && test -f /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P24-rho/result.json && echo PREV_RESULTS_PRESENT;;
32) python3 -I /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py 0 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P28 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P28-rho && test -f /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P28/result.json && test -f /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P28-rho/result.json && echo PREV_RESULTS_PRESENT;;
*) echo BAD_B; exit 1;;
esac
```

For __B__ = 24 the output must be the line ORDER_NOT_REQUIRED. For __B__ = 28 or 32 the output must contain two `PID_STATE` lines each ending in `exited`, the line PAIR_DONE and the line PREV_RESULTS_PRESENT (a PID_STATE line ending in `alive`, or the line PAIR_RUNNING, means a launcher of the previous size is still running).

Step 5. Receipts directories, logs, pid files and S3 prefixes unused. Run, as one Bash call, exactly these lines in this order:

```
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__ && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__ && echo ABSENT_P__B__
test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__-rho && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log && test ! -e /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho && echo ABSENT_P__B__-rho
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dgb-p__B__-20261029/)" && echo S3_EMPTY_P__B__
test -z "$(aws s3 ls s3://isolab-results-104186811093-us-east-2/runs/ic-dgb-rho-p__B__-20261029/)" && echo S3_EMPTY_P__B__-rho
```

The exit status must be 0 and the output must contain the four lines ABSENT_P__B__, ABSENT_P__B__-rho, S3_EMPTY_P__B__ and S3_EMPTY_P__B__-rho.

Step 6. Validate both requests (one Bash call, exactly these lines in this order):

```
cd /dev/shm/src-be2969ce
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json > /dev/null && echo VALID_P__B__
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher PATH=/tmp/claude-0/awsvenv/bin:$PATH taskq/deploy/eks-karpenter/remote/validate-request /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json > /dev/null && echo VALID_P__B__-rho
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
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__ PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup bash -c 'echo $$ > "$2"; exec taskq/deploy/eks-karpenter/remote/launch-remote "$1"' launcher /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__.json /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__ > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log 2>&1 < /dev/null &
echo "LAUNCH_UTC_P__B__-rho $(date -u +%Y-%m-%dT%H:%M:%SZ)"
AWS_REGION=us-east-2 AWS_DEFAULT_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__-rho PATH=/tmp/claude-0/awsvenv/bin:$PATH setsid nohup bash -c 'echo $$ > "$2"; exec taskq/deploy/eks-karpenter/remote/launch-remote "$1"' launcher /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/request-P__B__-rho.json /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho > /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log 2>&1 < /dev/null &
python3 -c "import time; time.sleep(90)"
echo "PID_P__B__ $(cat /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__)"
echo "PID_P__B__-rho $(cat /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho)"
echo "LAUNCHER_ID_P__B__ $(sed -n 's/^Launcher: isolab-experiment-launcher://p' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log)"
echo "LAUNCHER_ID_P__B__-rho $(sed -n 's/^Launcher: isolab-experiment-launcher://p' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log)"
echo "==== LOG_P__B__ ===="
head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log
echo "==== LOG_P__B__-rho ===="
head -n 20 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log
```

(Each launcher writes the pid of its own process into its pid file, so the pid file holds the pid that Procedure W watches.)

## Procedure W (MODE WAIT). Use a single Bash call per step; change nothing. The wait reads only /proc entries of the recorded pids, never `pgrep` or `ps` with a pattern.

Step W0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step W1. Wait by pid. Run, with the Bash tool `timeout` parameter set to 570000, exactly this line:

```
python3 -I /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py 540 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__ /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho
```

The script prints one `PID_STATE` line per pid file, then PAIR_DONE (both launchers exited) or PAIR_RUNNING. If the last line is PAIR_RUNNING, run the identical line again; at most 11 runs in total. Stop repeating at the first PAIR_DONE.

Step W2. Read the outcome. Run, as one Bash call, exactly these lines in this order:

```
python3 -I /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-card-tools/pidwait.py 0 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__ /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/pid-P__B__-rho
echo "==== RESULT_P__B__ ===="
jq -c '{state,job_name,exit_code,launcher,request_sha256,results_uri,receipt_uri}' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__/result.json 2>&1
echo "==== RESULT_P__B__-rho ===="
jq -c '{state,job_name,exit_code,launcher,request_sha256,results_uri,receipt_uri}' /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__-rho/result.json 2>&1
echo "==== TAIL_P__B__ ===="
tail -n 30 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log
echo "==== TAIL_P__B__-rho ===="
tail -n 30 /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
echo "SLOTS_IN_PROGRESS $(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name isolab-experiment-launcher --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w)"
```

## Return (verbatim tool output, nothing else, then the table)

Procedure L: the output of steps 0 to 7 as printed, then the output of step 8 as printed (the two LAUNCH_UTC lines, the two PID lines, the two LAUNCHER_ID lines, and the two logs). Then this table, each cell copied from a printed line or from this card, nothing computed:

| job | job_name | request sha256 (pinned) | LAUNCH_UTC | PID | LAUNCHER_ID | RECEIPTS | LOG |
|:--|:--|:--|:--|:--|:--|:--|:--|
| wall | ic-dgb-p__B__-20261029 | (the pinned sha256 of request-P__B__.json, listed above) | (LAUNCH_UTC_P__B__) | (PID_P__B__) | (LAUNCHER_ID_P__B__) | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__ | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__.log |
| counted | ic-dgb-rho-p__B__-20261029 | (the pinned sha256 of request-P__B__-rho.json, listed above) | (LAUNCH_UTC_P__B__-rho) | (PID_P__B__-rho) | (LAUNCHER_ID_P__B__-rho) | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/receipts-P__B__-rho | /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/launch-P__B__-rho.log |

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
- Never launch a job twice. A job name that already has receipts, a log, a pid file or an S3 prefix is never launched again by this card: step 5 refuses. Whoever writes a rerun card (suffix `-r2`) must first verify that no job of the original name exists in the S3 bucket or in the CodeBuild project.
- A reported error or a stopped job is not a result for this card. Only the main session decides whether a new card (for example a host-loss rerun under the suffix `-r2`, PROTOCOL.md section 4) is written.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs detached launches with the logs outside the receipts directories, a 90-second partial report and a wait by pid, so the tool cannot express it.

## Note

A foreground `sleep` can be blocked by the harness; step 8 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. Each bit size is launched by a separate dispatch of this card; launch and wait are separate dispatches.
