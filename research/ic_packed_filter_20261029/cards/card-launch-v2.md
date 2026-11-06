# Mechanical task card CARD-XP-20261029-packed-filter-launch-v2

author_role: experiment-director
executor_role: executor-mechanical
experiment: packed-filter (protocol backlog/XP-20261029-packed-filter/PROTOCOL.md; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card, revision 2 (not `tools/task_card.py` frozen_command; see "Why a plain card")
supersedes: research/ic_packed_filter_20261029/cards/card-launch.md (sha256 e0d24e3017b5082c7a3912a6dc7645a83d29a3edfada24cb8382016392da9159). That file stays unchanged as the record of the P24 and P28 launches; from the next dispatch on, only this card is dispatched.
ruling: research/ic_packed_filter_20261029/cards/RULING-P28.md (sha256 bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275), section (c) items C1 to C7 and section (b) item B5. This revision is a card revision written by the experiment director on that ruling. It is not an amendment of PROTOCOL.md, and it changes no specification, request, protocol file or analysis card.
approval: the Coordinator approved the protocol with edits E1-E10 and the pre-launch checks P1-P6 (APPROVAL.md in the protocol directory); the pre-launch checks all passed (research/ic_packed_filter_20261029/cards/prelaunch-checks.md, pinned below). The main (reasoning-tier) session decides the route and which dispatch to send next. This card adds no authority. A `STOP:` line printed by Procedure S or by Step 4 is a specification_error returned to the main session, and from there to the Coordinator (RULING-P28.md C7).

## Dispatch values

The dispatch names the values below, and the executor substitutes them and changes nothing else. A dispatch that names any other value, omits one, or names a value outside the listed set is refused.

- MODE: one of `LAUNCH`, `RELAUNCH`, `WAIT`, `VERIFY`, `S3VERIFY`, `TRIGGER`.
- the bit size: 24, 28 or 32. Substitute its digits everywhere the text __B__ appears in this card.
- UNIT: required for `RELAUNCH`, `WAIT`, `VERIFY` and `S3VERIFY`; absent for `LAUNCH` and `TRIGGER`. Substitute it everywhere the text __UNIT__ appears. The values:
  - `PAIR`: the two first-launch jobs of a size (only for `WAIT`, `VERIFY` and `S3VERIFY`; `LAUNCH` is `PAIR` by definition);
  - `r2`: the host-loss rerun of the wall job;
  - `rho-r2`: the host-loss rerun of the counted job;
  - `nr`: the noise rerun of the wall job (PROTOCOL.md section 4);
  - `nr-r2`: the host-loss rerun of the noise rerun.
- TAG: required for `S3VERIFY` only: one tag of the unit (the tags are in the table "The jobs"; unit PAIR has the tags P__B__ and P__B__-rho, every other unit has one tag). Substitute it everywhere the text __TAG__ appears.
- POD_STATUS_SHA256: required for `S3VERIFY` only: the text `NONE`, or 64 lowercase hex characters (the sha256 of the file PFD/pod-status-TAG.json, which the main session has put there from an authorized Kubernetes read). Substitute it everywhere the text __POD_STATUS_SHA256__ appears.
- PREV_WALL_TAG and PREV_COUNTED_TAG: required for `LAUNCH` and `RELAUNCH`; absent for every other MODE. They name the sessions of record of the size before the launched one, chosen by the main session under PROTOCOL.md section 4 and, for P28, under RULING-P28.md (b) B5. Substitute them everywhere the texts __PREV_WALL_TAG__ and __PREV_COUNTED_TAG__ appear. For `LAUNCH` with the bit size 28 or 32: PREV_WALL_TAG is one of `P<prev>`, `P<prev>-r2`, `P<prev>-nr`, `P<prev>-nr-r2` and PREV_COUNTED_TAG is one of `P<prev>-rho`, `P<prev>-rho-r2`, where <prev> is 24 when the bit size is 28 and 28 when the bit size is 32. For `LAUNCH` with the bit size 24 and for every `RELAUNCH`, both values are the text `NONE`.
- SESSION: required for `TRIGGER` only: `first` or `r2`. Substitute it everywhere the text __SESSION__ appears.
- CARD_SHA256: the sha256 of this card file.

| MODE | what it does | procedure |
|:--|:--|:--|
| LAUNCH | launches the two first-launch jobs of one size (the wall job and the counted job) | L (unit PAIR) |
| RELAUNCH | launches one contingent job (UNIT r2, rho-r2, nr or nr-r2) | L |
| WAIT | waits for the launcher processes of a unit and prints the outcome files | W |
| VERIFY | reads (never writes) whether the build, the S3 prefix and the local files of a unit exist | V |
| S3VERIFY | downloads the S3 outputs of one job of a unit, checks them against the frozen request without Kubernetes, and writes `PFD/s3verify-TAG/s3receipt.json` | S |
| TRIGGER | evaluates the noise-rerun trigger of one wall session and writes `P<b>-RERUN.md` | T |

## What the launcher does, and the slot rule

- Each job is one `launch-remote` process on this machine. It starts one AWS CodeBuild build of the project isolab-experiment-launcher, and that build stays `IN_PROGRESS` for the whole remote job. The build is finished when its buildStatus is not `IN_PROGRESS`; the local `launch-remote` process has then exited too.
- The project allows at most 2 concurrent builds. A first launch of a size needs both slots (2 jobs). So the sizes run in this order, one size at a time: 24, then 28, then 32.
- `LAUNCH` of a size is accepted only when the launchers of the two named sessions of record of the size before it (PREV_WALL_TAG and PREV_COUNTED_TAG) have exited and each of those two sessions holds a `result.json` or an S3VERIFY receipt with `S3_TERMINAL` true (Step 4), when the CodeBuild project reports `timeoutInMinutes` of at least 70 (Step 6A), and when the CodeBuild count `IN_PROGRESS` is 0. `RELAUNCH` is accepted only when that count is at most 1. The procedure checks these facts and refuses otherwise; it never waits. `WAIT` is how the main session learns that a unit has finished.
- The card does not read any outcome and does not interpret any file. The main session decides, from the reports, which dispatch to send next.

## Frozen values

- Freeze commit of the protocol: f870408c1762a52dd133e8f1e0bca181cc2a76f5; branch claude/ic-packed-filter-20261029 (pushed). The freeze commit message records that the six specs are byte-identical to the reviewed drafts; their sha256 are pinned below.
- Candidate code commit: 3271a25d02dcef6da148293fb8f9e2149ce59858 (the freeze commit adds only `backlog/XP-20261029-packed-filter/` to it).
- SRC = /dev/shm/src-pf (sparse checkout of the freeze commit, clean). The launcher scripts there are used: REMOTE = /dev/shm/src-pf/taskq/deploy/eks-karpenter/remote. Their sha256 equal those of the previous study's launcher (pinned below). Never use the launcher scripts of /home/user/crypto.
- Image of every job: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b (each request embeds it).
- ecbench sha256 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503; source revision 3271a25d02dcef6da148293fb8f9e2149ce59858 (each request embeds both).
- Every job: instance c8a.2xlarge, execution deadline 3600 s, provenance.harness claude-code-local, provenance.run_url https://github.com/isogeny-labs/crypto/blob/f870408c1762a52dd133e8f1e0bca181cc2a76f5/backlog/XP-20261029-packed-filter/PROTOCOL.md
- PFD = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf (requests, receipts, logs and pid files; nothing larger than a few kilobytes is written there).
- TOOLS = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools (helper files, bound by sha256 below; the new helper of this revision is s3verify.py).
- The card file for the dispatch is PFD/card-launch-v2.md, which the main session copies there from the committed and pushed file research/ic_packed_filter_20261029/cards/card-launch-v2.md of the branch (Step 0 checks both).
- Launcher budget: the CodeBuild project isolab-experiment-launcher must report `timeoutInMinutes` of at least 70, the value `remote/bridge.yaml` declares (`run-request` waits `job.seconds + 180` = 3780 s after submission, and setup, download and verification take more time). The card never changes the project, and runs no `codebuild update-project`, `deploy-remote` or CloudFormation command.
- Seal (RULING-P28.md (a)): the session ic-dgbf-p28-20261029 may be downloaded and hashed, and its file list, `job-exit.json` and `build-info.json` recorded. Procedure S downloads and hashes every file of a pod prefix and opens only the files it names; it never opens `records.jsonl`, `session.json` or `run.stderr`, and runs no `ecbench` command.
- Protocol directory PD = /dev/shm/src-pf/backlog/XP-20261029-packed-filter. Protocol worktree WT = /dev/shm/src-pf. Study directory STUDY = /dev/shm/src-pf/research/ic_packed_filter_20261029.
- Backend: isolab-eks AWS launcher, AWS account 104186811093. AWS credentials are inherited from the process environment; never write, print or copy them.
- env of every AWS command (the prelude sets it): AWS_REGION=us-east-2, AWS_DEFAULT_REGION=us-east-2, ISOLAB_BUCKET=isolab-results-104186811093-us-east-2, ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher, PATH prefixed with /tmp/claude-0/awsvenv/bin.
- Working directory of every launch and validation command: SRC.
- Checkpoint gate: run in WT with WT's own scripts/checkpoint.sh.
- The prelude `launch-prelude.sh` defines, for the dispatch values, the variables TAGS (the request tags of the unit), PREDTAG (the tag of the job a rerun replaces), NEED (the number of build slots) and the functions `job_of TAG` and `spec_of TAG`. Every step sources it.

### The jobs (tag = the text after `request-` in the request file name; b is the bit size)

| unit | tag | job_name | request file | spec file |
|:--|:--|:--|:--|:--|
| PAIR | P__B__ | ic-dgbf-p__B__-20261029 (wall) | PFD/request-P__B__.json | spec-P__B__.json |
| PAIR | P__B__-rho | ic-dgbf-rho-p__B__-20261029 (counted) | PFD/request-P__B__-rho.json | spec-P__B__-rho.json |
| r2 | P__B__-r2 | ic-dgbf-p__B__-20261029-r2 | PFD/request-P__B__-r2.json | spec-P__B__.json |
| rho-r2 | P__B__-rho-r2 | ic-dgbf-rho-p__B__-20261029-r2 | PFD/request-P__B__-rho-r2.json | spec-P__B__-rho.json |
| nr | P__B__-nr | ic-dgbf-p__B__-20261029-nr | PFD/request-P__B__-nr.json | spec-P__B__.json |
| nr-r2 | P__B__-nr-r2 | ic-dgbf-p__B__-20261029-nr-r2 | PFD/request-P__B__-nr-r2.json | spec-P__B__.json |

For each tag the files PFD/receipts-TAG (directory), PFD/launch-TAG.log and PFD/pid-TAG must not exist before the tag is launched, and the S3 prefix `runs/<job_name>/` of the bucket must be empty. The only difference between the requests of a size is the job name.

Pinned sha256 of the requests (each embeds its spec exactly as JSON-equal content):

- request-P24.json c33451a22c39dee84c15c81fa5bd6da13296b634763f25d6c05ea45638248358
- request-P24-rho.json 12dce514adc403d460f51168405840c282226738a324908cf9be3307a7ca43d6
- request-P24-r2.json 621c5c85a10e807e9a524d6fa559e63da10ab230718b44f4d5d8a77feb5573c5
- request-P24-rho-r2.json fad9680cfb31b60fb49ff538a127cfe5387bb4642258cbca1af9db26833378b0
- request-P24-nr.json 7e95197065e692c98df6e1906bcb0f7832cc5a70500d5300bb7bcd584fb4484e
- request-P24-nr-r2.json c8ac7eb02baa4f0ab3e219313a784dffa43878ee2ee648d9f90dde2d0007482f
- request-P28.json 53c64a25967a8b5aa6a766377c4430f8857cbcf007b9712a85333126f818df61
- request-P28-rho.json 19c881a3b6b62d5215d3b03f37f00505b6a2df35038815a430cdbce8372df2b6
- request-P28-r2.json 86416665153961bcd27ecf7af101eaeedad060e490e1922d9ebbdd02d4f04698
- request-P28-rho-r2.json 7919707346ae9a4a4b2e159d74451728181c69a6cf04e98c4822c7f736d69624
- request-P28-nr.json 510b3adc9fcc2e4fc80ec99fa31ba83dc60da5d5ef4d8800f531048d3d3a3bb7
- request-P28-nr-r2.json 53484252647bc574f3e68c40c41b051946e38294dc28bd8cfab254e1d57f72c9
- request-P32.json 2bc93a27d60034242f47221d78a8f539e81c95ddefb31c74a012c03a546415b2
- request-P32-rho.json 251648bdc7024368e9431c516c1917dd2dd1d7f16c476e73e39d2df58b8b8ec0
- request-P32-r2.json 890bb9342a88adf9c6fb13c91f17e8a94605cb6c53d2f04070f4f487049caeec
- request-P32-rho-r2.json 612c284329654fdcdc7d346a6a50c9c6dd5bf061e521726235af6b817282e571
- request-P32-nr.json f778a915be712633010eb4abfa74826a1007b830339b474dcfb84595f3c37e99
- request-P32-nr-r2.json 788c7706778a7525981bb96e78d0b7513c4d601ef778abd0f93ff14fa1ee30a5

Pinned sha256 (protocol files in PD, as committed at the freeze commit):

- PROTOCOL.md 703bde26a197859c02e2e5f293ba6719cb5b71871f3044a7ac336d346d067e51
- APPROVAL.md 0a35923cca184b68c212b66a70d2d42e2e2546e1cd50fd6ebe99d854889a6a40
- spec-P24.json ce9510367b95c5038b4b200675943b2e81bc9f395921cfe10f87973ab7f151e7
- spec-P24-rho.json c98bf0c914efceef796be5f00a86e18c75e426a1ca8cf9e63fab748c23db0321
- spec-P28.json 9f54fdd0305c789f6b12202fbb81bee50a05135fca674fae65c7557f531053d9
- spec-P28-rho.json 55a403714d9cb353eb1fe644eb9e04e628ef46b1a1ada5b8575709db40cc349a
- spec-P32.json 9f950f3bcba5e4729798c82bcca1a1d665104329b5ed9368c9d98b6addbc30ab
- spec-P32-rho.json 2095978592b5d21713d5de32dec6c1b862cc7e0d3810c3f82f8399c4a2c45389

Pinned sha256 (other files):

- research/ic_packed_filter_20261029/cards/prelaunch-checks.md 6542ad8f2c8f33c0acca70dbabcf76492b872145cd1138de9cdf46a818d40b83
- REMOTE/validate-request dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426
- REMOTE/launch-remote 076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45
- WT/scripts/checkpoint.sh 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f
- TOOLS/launch-prelude.sh 7d74516d9a41a6538ff123ad97c4096a7e606f4eda2e7f04cc96f7e0cc6c6928
- TOOLS/pidwait.py 23d2324cd222727b66e6c20d01426ae00b3131baf8b707c3b790b257bca46ac0
- TOOLS/rerun_trigger.py 681229d30b8322cbc715532c86fa15001395642fa82c65cf11d434780dcb3336
- TOOLS/trigger.jq 6027498ffa0f38e215e0619eb923ce712f7536fc654487d294833a00162680af
- research/ic_packed_filter_20261029/cards/RULING-P28.md bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275
- research/ic_packed_filter_20261029/cards/card-launch.md e0d24e3017b5082c7a3912a6dc7645a83d29a3edfada24cb8382016392da9159 (the superseded card)
- REMOTE/run-request e9706cc618afed952d5d073866bb30f8b31302b1410f1d77a78784aac1fa2192 (its checks of lines 68 to 79 are reproduced from S3 by s3verify.py)
- REMOTE/verify-build 6a020d533df4f18645726377b53d17fddb9efbe693c4b8a42a1d2e24169f7556 (its checks are reproduced from S3 by s3verify.py, except the pod image digest)
- TOOLS/s3verify.py 7c624b8c7761140ea321e55cb6c355360999f106ed69b7f7d8d68ff66c289bec

## Procedure L (MODE LAUNCH or RELAUNCH)

Use one Bash call per step; change nothing. Steps 0 to 7 are checks; if any of them fails, launch nothing. In every command the words `__B__` and `__UNIT__` stand for the dispatch values (for MODE `LAUNCH`, UNIT is `PAIR`). A line that begins with `STOP:` in any output is a refusal.

Step 0. Card digest. Run, as one Bash call, exactly these lines in this order (the value compared with is the CARD_SHA256 value stated in the dispatch):

```
set -o pipefail
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/card-launch-v2.md
cd /dev/shm/src-pf && git fetch --quiet origin claude/ic-packed-filter-20261029 && git show origin/claude/ic-packed-filter-20261029:research/ic_packed_filter_20261029/cards/card-launch-v2.md | sha256sum
```

The exit status must be 0, and the first field of each of the two printed lines must equal CARD_SHA256 from the dispatch. The second line proves that the card file is committed and pushed on the branch as it is on disk. (This card file is the one with `__B__`, `__UNIT__`, `__TAG__`, `__POD_STATUS_SHA256__`, `__PREV_WALL_TAG__`, `__PREV_COUNTED_TAG__` and `__SESSION__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Checkpoint gate. Run:

```
cd /dev/shm/src-pf && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash scripts/checkpoint.sh gate; echo "gate_exit=$?"
```

The line `gate_exit=0` must be printed.

Step 2. Hashes (all requests, the protocol files, the launcher scripts, the checkpoint script, the helper files). Run, as one Bash call, exactly these lines in this order:

```
sha256sum -c - <<'HASHES'
c33451a22c39dee84c15c81fa5bd6da13296b634763f25d6c05ea45638248358  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24.json
12dce514adc403d460f51168405840c282226738a324908cf9be3307a7ca43d6  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24-rho.json
621c5c85a10e807e9a524d6fa559e63da10ab230718b44f4d5d8a77feb5573c5  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24-r2.json
fad9680cfb31b60fb49ff538a127cfe5387bb4642258cbca1af9db26833378b0  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24-rho-r2.json
7e95197065e692c98df6e1906bcb0f7832cc5a70500d5300bb7bcd584fb4484e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24-nr.json
c8ac7eb02baa4f0ab3e219313a784dffa43878ee2ee648d9f90dde2d0007482f  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P24-nr-r2.json
53c64a25967a8b5aa6a766377c4430f8857cbcf007b9712a85333126f818df61  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28.json
19c881a3b6b62d5215d3b03f37f00505b6a2df35038815a430cdbce8372df2b6  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28-rho.json
86416665153961bcd27ecf7af101eaeedad060e490e1922d9ebbdd02d4f04698  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28-r2.json
7919707346ae9a4a4b2e159d74451728181c69a6cf04e98c4822c7f736d69624  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28-rho-r2.json
510b3adc9fcc2e4fc80ec99fa31ba83dc60da5d5ef4d8800f531048d3d3a3bb7  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28-nr.json
53484252647bc574f3e68c40c41b051946e38294dc28bd8cfab254e1d57f72c9  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P28-nr-r2.json
2bc93a27d60034242f47221d78a8f539e81c95ddefb31c74a012c03a546415b2  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32.json
251648bdc7024368e9431c516c1917dd2dd1d7f16c476e73e39d2df58b8b8ec0  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32-rho.json
890bb9342a88adf9c6fb13c91f17e8a94605cb6c53d2f04070f4f487049caeec  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32-r2.json
612c284329654fdcdc7d346a6a50c9c6dd5bf061e521726235af6b817282e571  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32-rho-r2.json
f778a915be712633010eb4abfa74826a1007b830339b474dcfb84595f3c37e99  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32-nr.json
788c7706778a7525981bb96e78d0b7513c4d601ef778abd0f93ff14fa1ee30a5  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/request-P32-nr-r2.json
703bde26a197859c02e2e5f293ba6719cb5b71871f3044a7ac336d346d067e51  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/PROTOCOL.md
0a35923cca184b68c212b66a70d2d42e2e2546e1cd50fd6ebe99d854889a6a40  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/APPROVAL.md
ce9510367b95c5038b4b200675943b2e81bc9f395921cfe10f87973ab7f151e7  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P24.json
c98bf0c914efceef796be5f00a86e18c75e426a1ca8cf9e63fab748c23db0321  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P24-rho.json
9f54fdd0305c789f6b12202fbb81bee50a05135fca674fae65c7557f531053d9  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P28.json
55a403714d9cb353eb1fe644eb9e04e628ef46b1a1ada5b8575709db40cc349a  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P28-rho.json
9f950f3bcba5e4729798c82bcca1a1d665104329b5ed9368c9d98b6addbc30ab  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P32.json
2095978592b5d21713d5de32dec6c1b862cc7e0d3810c3f82f8399c4a2c45389  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P32-rho.json
6542ad8f2c8f33c0acca70dbabcf76492b872145cd1138de9cdf46a818d40b83  /dev/shm/src-pf/research/ic_packed_filter_20261029/cards/prelaunch-checks.md
dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426  /dev/shm/src-pf/taskq/deploy/eks-karpenter/remote/validate-request
076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45  /dev/shm/src-pf/taskq/deploy/eks-karpenter/remote/launch-remote
56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f  /dev/shm/src-pf/scripts/checkpoint.sh
7d74516d9a41a6538ff123ad97c4096a7e606f4eda2e7f04cc96f7e0cc6c6928  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
23d2324cd222727b66e6c20d01426ae00b3131baf8b707c3b790b257bca46ac0  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/pidwait.py
681229d30b8322cbc715532c86fa15001395642fa82c65cf11d434780dcb3336  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/rerun_trigger.py
6027498ffa0f38e215e0619eb923ce712f7536fc654487d294833a00162680af  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/trigger.jq
bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275  /dev/shm/src-pf/research/ic_packed_filter_20261029/cards/RULING-P28.md
e0d24e3017b5082c7a3912a6dc7645a83d29a3edfada24cb8382016392da9159  /dev/shm/src-pf/research/ic_packed_filter_20261029/cards/card-launch.md
e9706cc618afed952d5d073866bb30f8b31302b1410f1d77a78784aac1fa2192  /dev/shm/src-pf/taskq/deploy/eks-karpenter/remote/run-request
6a020d533df4f18645726377b53d17fddb9efbe693c4b8a42a1d2e24169f7556  /dev/shm/src-pf/taskq/deploy/eks-karpenter/remote/verify-build
7c624b8c7761140ea321e55cb6c355360999f106ed69b7f7d8d68ff66c289bec  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/s3verify.py
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all 39 output lines must end with ": OK".

Step 3. Each request of the unit equals its frozen spec and its frozen fields; the freeze commit is in the worktree and pushed; the protocol directory equals the freeze commit. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
for T in $TAGS; do
  J=$(job_of $T); S=$(spec_of $T); R=$PFD/request-$T.json
  python3 -I -c "import json,sys; r=json.load(open(sys.argv[1])); s=json.load(open(sys.argv[2])); sys.exit(0 if r['job']['spec']==s else 1)" $R $S || { echo "STOP: request $T spec differs from $S"; exit 1; }
  echo SPEC_EQUAL_$T
  diff <(jq -S .job.spec $R) <(jq -S . $S) > /dev/null || { echo "STOP: jq -S spec differs for $T"; exit 1; }
  echo JQS_EQUAL_$T
  jq -e --arg job "$J" --arg image "$IMAGE" --arg rev "$REV" --arg fb "$FBSHA" --arg url "$RUNURL" '.job.job_name==$job and .job.instance_type=="c8a.2xlarge" and .job.seconds==3600 and .job.build.image_uri==$image and .job.build.source_revision==$rev and .job.build.ecbench_sha256==$fb and .provenance.harness=="claude-code-local" and .provenance.source_revision==$rev and .provenance.workflow_revision==$rev and .provenance.run_url==$url' $R > /dev/null || { echo "STOP: request fields differ for $T"; exit 1; }
  echo FIELDS_OK_$T
done
cd $WT
git merge-base --is-ancestor $FREEZE HEAD && echo HEAD_CONTAINS_COMMIT
git merge-base --is-ancestor $FREEZE origin/$BRANCH && echo COMMIT_PUSHED
git diff --quiet $FREEZE -- backlog/XP-20261029-packed-filter && echo PD_EQUALS_COMMIT
echo STEP3_OK
```

The output must contain, for each tag of the unit, the three lines SPEC_EQUAL_TAG, JQS_EQUAL_TAG and FIELDS_OK_TAG, and then the lines HEAD_CONTAINS_COMMIT, COMMIT_PUSHED, PD_EQUALS_COMMIT and STEP3_OK, and nothing on stderr.

Step 4. The preconditions of the unit. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; pw=__PREV_WALL_TAG__; pc=__PREV_COUNTED_TAG__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
accept_prev() {
  T=$1
  PJ=$( b=$PREV; unit=PAIR; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh > /dev/null; job_of $T )
  if test -f $PFD/receipts-$T/result.json; then
    echo "PREV_ACCEPTED $T result.json"
  elif test -f $PFD/s3verify-$T/s3receipt.json; then
    RSHA=$(sha256sum $PFD/request-$T.json | cut -d' ' -f1)
    jq -e --arg tag "$T" --arg job "$PJ" --arg rsha "$RSHA" '.schema=="pf.s3verify/v1" and .mode=="s3" and .tag==$tag and .job==$job and .request_sha256==$rsha and .s3_terminal==true and .facts.S3V_POD_UID_COUNT==1 and .facts.S3V_JOB_EXIT_PRESENT==true and .facts.S3V_JOB_EXIT_IDENTITY==true and .facts.S3V_SHA256SUMS_OK==true and .facts.S3V_MANIFEST_COMPLETE==true and .facts.S3V_BUILD_INFO==true and .facts.S3V_BINARY_SHA==true and .facts.S3V_INPUT_SPEC==true' $PFD/s3verify-$T/s3receipt.json > /dev/null || { echo "STOP: the S3VERIFY receipt of $T is not S3_TERMINAL for $PJ and its request"; exit 1; }
    echo "PREV_ACCEPTED $T s3receipt"
  else
    echo "STOP: neither receipts-$T/result.json nor s3verify-$T/s3receipt.json exists"; exit 1
  fi
}
if [ "$unit" = PAIR ]; then
  case "$b" in
    24)
      [ "$pw" = NONE ] && [ "$pc" = NONE ] || { echo "STOP: PREV_WALL_TAG and PREV_COUNTED_TAG are not NONE for size 24"; exit 1; }
      echo ORDER_NOT_REQUIRED ;;
    28|32)
      if [ "$b" = 28 ]; then PREV=24; else PREV=28; fi
      case "$pw" in
        "P$PREV"|"P$PREV-r2"|"P$PREV-nr"|"P$PREV-nr-r2") ;;
        *) echo "STOP: PREV_WALL_TAG $pw is not a wall tag of size $PREV"; exit 1 ;;
      esac
      case "$pc" in
        "P$PREV-rho"|"P$PREV-rho-r2") ;;
        *) echo "STOP: PREV_COUNTED_TAG $pc is not a counted tag of size $PREV"; exit 1 ;;
      esac
      test -f $PFD/pid-$pw && test -f $PFD/pid-$pc || { echo "STOP: the pid file of $pw or $pc is absent"; exit 1; }
      OUT=$(python3 -I $TOOLS/pidwait.py 0 $PFD/pid-$pw $PFD/pid-$pc)
      echo "$OUT"
      [ "$(echo "$OUT" | tail -n 1)" = PAIR_DONE ] || { echo "STOP: the launcher of $pw or $pc is still running"; exit 1; }
      accept_prev $pw
      accept_prev $pc
      echo PREV_RESULTS_PRESENT
      ;;
  esac
else
  [ "$pw" = NONE ] && [ "$pc" = NONE ] || { echo "STOP: PREV_WALL_TAG and PREV_COUNTED_TAG are not NONE for a RELAUNCH"; exit 1; }
  if [ -n "$PREDTAG" ]; then
    PJ=$(job_of $PREDTAG)
    test -f $PFD/pid-$PREDTAG || { echo "STOP: the pid file of $PREDTAG is absent"; exit 1; }
    OUT=$(python3 -I $TOOLS/pidwait.py 0 $PFD/pid-$PREDTAG)
    echo "$OUT"
    [ "$(echo "$OUT" | tail -n 1)" = PAIR_DONE ] || { echo "STOP: the launcher of $PREDTAG is still running or has no pid file"; exit 1; }
    n=$(aws s3 ls --recursive $S3RUNS/$PJ/ | awk '{print $4}' | grep -c '/job-exit.json$' || true)
    echo "PRED_JOB_EXIT_COUNT $PJ $n"
    [ "$n" -eq 0 ] || { echo "STOP: $PJ wrote a job-exit.json; this is not a host loss"; exit 1; }
    echo PRED_HAS_NO_JOB_EXIT
  fi
  if [ "$unit" = nr ] || [ "$unit" = nr-r2 ]; then
    F=research/ic_packed_filter_20261029/P$b/P$b-RERUN.md
    cd $WT
    git fetch --quiet origin $BRANCH || { echo "STOP: git fetch failed"; exit 1; }
    test -f $WT/$F || { echo "STOP: $F is absent"; exit 1; }
    [ "$(grep -c '^NOISE_TRIGGER=true$' $WT/$F)" -eq 1 ] || { echo "STOP: $F does not carry the line NOISE_TRIGGER=true"; exit 1; }
    L=$(sha256sum $WT/$F | cut -d' ' -f1)
    P=$(git show origin/$BRANCH:$F | sha256sum | cut -d' ' -f1)
    echo "RERUN_MD_SHA256 local=$L pushed=$P"
    [ "$L" = "$P" ] || { echo "STOP: $F is not pushed as it is on disk"; exit 1; }
    echo RERUN_MD_PUSHED
  fi
fi
echo STEP4_OK
```

For MODE LAUNCH and b = 24 the output must contain ORDER_NOT_REQUIRED. For MODE LAUNCH and b = 28 or 32 it must contain two PID_STATE lines each ending in `exited`, PAIR_DONE, one line `PREV_ACCEPTED <tag> result.json` or `PREV_ACCEPTED <tag> s3receipt` for PREV_WALL_TAG and one for PREV_COUNTED_TAG, and PREV_RESULTS_PRESENT. For RELAUNCH r2, rho-r2 and nr-r2 it must contain one PID_STATE line ending in `exited`, PAIR_DONE, `PRED_JOB_EXIT_COUNT <job> 0` and PRED_HAS_NO_JOB_EXIT; for nr and nr-r2 it must also contain RERUN_MD_SHA256 local=X pushed=X (the same hash twice) and RERUN_MD_PUSHED. Every output ends with STEP4_OK. (The S3VERIFY receipt route is RULING-P28.md C3; the RELAUNCH route is unchanged from card-launch.md.)

Step 5. Nothing of the unit exists yet: local files, S3 prefixes, uploaded request, CodeBuild builds. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
BUILDS=$(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name $ISOLAB_CODEBUILD_PROJECT --query 'ids[:50]' --output text) --output json)
for T in $TAGS; do
  J=$(job_of $T)
  test ! -e $PFD/receipts-$T && test ! -e $PFD/launch-$T.log && test ! -e $PFD/pid-$T || { echo "STOP: a local file of $T exists"; exit 1; }
  echo ABSENT_$T
  n=$(aws s3api list-objects-v2 --bucket $ISOLAB_BUCKET --prefix runs/$J/ --max-keys 1 --query KeyCount --output text)
  echo "S3_KEYCOUNT $J $n"
  [ "$n" = 0 ] || { echo "STOP: S3 prefix runs/$J/ is not empty"; exit 1; }
  DIG=$($REMOTE/validate-request $PFD/request-$T.json | sha256sum | cut -d' ' -f1)
  m=$(echo "$BUILDS" | jq --arg k "inputs/requests/$DIG.json" '[.builds[] | select(any(.environment.environmentVariables[]?; .name=="ISOLAB_REQUEST_KEY" and .value==$k))] | length')
  echo "BUILDS_WITH_REQUEST_KEY $T $DIG $m"
  [ "$m" = 0 ] || { echo "STOP: a CodeBuild build already carries the request key of $T"; exit 1; }
done
echo STEP5_OK
```

The output must contain, for each tag of the unit, ABSENT_TAG, a line `S3_KEYCOUNT <job> 0` and a line `BUILDS_WITH_REQUEST_KEY <tag> <64 hex> 0`, and then STEP5_OK.

Step 6. Validate the requests of the unit. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
cd $SRC
for T in $TAGS; do
  taskq/deploy/eks-karpenter/remote/validate-request $PFD/request-$T.json > /dev/null && echo VALID_$T || { echo "STOP: validate-request failed for $T"; exit 1; }
done
echo STEP6_OK
```

The output must contain VALID_TAG for each tag of the unit and STEP6_OK.

Step 6A. Launcher budget (fail-closed; RULING-P28.md C4). Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
P=$(aws codebuild batch-get-projects --names isolab-experiment-launcher --query 'projects[0].timeoutInMinutes' --output text)
echo "PROJECT_TIMEOUT_MINUTES $P"
[[ "$P" =~ ^[0-9]+$ ]] || { echo "STOP: launcher budget is not an integer: $P"; exit 1; }
[ "$P" -ge 70 ] || { echo "STOP: launcher budget below 70 minutes"; exit 1; }
echo STEP6A_OK
```

The output must contain a line `PROJECT_TIMEOUT_MINUTES N` with N an integer of at least 70, and STEP6A_OK. A smaller N, or a value that is not an integer, is the refusal `STOP: launcher budget ...`; the card never changes the project.

Step 7. Build slots. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
N=$(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name $ISOLAB_CODEBUILD_PROJECT --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w)
echo "SLOTS_IN_PROGRESS $N NEED $NEED"
[ $((N + NEED)) -le 2 ] || { echo "STOP: not enough free build slots"; exit 1; }
echo STEP7_OK
```

The output must contain a line `SLOTS_IN_PROGRESS N NEED K` with N + K at most 2 (for MODE LAUNCH, K is 2 and N must be 0; for RELAUNCH, K is 1 and N must be 0 or 1) and STEP7_OK.

Step 8. Launch the unit, wait 90 seconds, read the logs (one Bash call, exactly these lines in this order; run this call once and never again):

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
cd $SRC
for T in $TAGS; do
  echo "LAUNCH_UTC_$T $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  ISOLAB_CLIENT_OUTPUT=$PFD/receipts-$T setsid nohup bash -c 'echo $$ > "$2"; exec taskq/deploy/eks-karpenter/remote/launch-remote "$1"' launcher $PFD/request-$T.json $PFD/pid-$T > $PFD/launch-$T.log 2>&1 < /dev/null &
done
python3 -c "import time; time.sleep(90)"
for T in $TAGS; do
  echo "PID_$T $(cat $PFD/pid-$T)"
  echo "LAUNCHER_ID_$T $(sed -n 's/^Launcher: isolab-experiment-launcher://p' $PFD/launch-$T.log)"
done
for T in $TAGS; do
  echo "==== LOG_$T ===="
  head -n 20 $PFD/launch-$T.log
done
```

(Each launcher writes the pid of its own process into its pid file, so the pid file holds the pid that Procedure W watches. The AWS variables are exported by the prelude, so the launcher inherits them.)

Step 9. Report the launcher budget each build recorded (RULING-P28.md C4; a reported fact, no refusal). Run after step 8, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
for T in $TAGS; do
  echo "BUILD_TIMEOUT_MINUTES $T $(jq -r '.build.timeoutInMinutes // "ABSENT"' $PFD/receipts-$T/build-start.json 2> /dev/null || echo ABSENT)"
done
echo STEP9_OK
```

The output must contain, for each tag of the unit, a line `BUILD_TIMEOUT_MINUTES <tag> <value>` (the value is an integer, or ABSENT when the file or the key is absent), and STEP9_OK. The line is returned as printed.

## Procedure W (MODE WAIT). Use one Bash call per step; change nothing. The wait reads only /proc entries of the recorded pids, never `pgrep` or `ps` with a pattern.

Step W0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step W1. Wait by pid. Run, with the Bash tool `timeout` parameter set to 570000, exactly these lines:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
python3 -I $TOOLS/pidwait.py 540 $(for T in $TAGS; do echo $PFD/pid-$T; done)
```

The script prints one `PID_STATE` line per pid file, then PAIR_DONE (every launcher of the unit exited) or PAIR_RUNNING. If the last line is PAIR_RUNNING, run the identical lines again; at most 11 runs in total. Stop repeating at the first PAIR_DONE.

Step W2. Read the outcome. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
python3 -I $TOOLS/pidwait.py 0 $(for T in $TAGS; do echo $PFD/pid-$T; done)
for T in $TAGS; do
  J=$(job_of $T)
  echo "==== RESULT_$T ===="
  jq -c '{state,job_name,exit_code,launcher,request_sha256,results_uri,receipt_uri}' $PFD/receipts-$T/result.json 2>&1 || true
  echo "==== S3_PREFIXES_$T ===="
  aws s3 ls $S3RUNS/$J/ || true
  echo "==== JOB_EXIT_FILES_$T ===="
  for K in $(aws s3 ls --recursive $S3RUNS/$J/ | awk '{print $4}' | grep '/job-exit.json$' || true); do
    echo "$K"
    aws s3 cp s3://$ISOLAB_BUCKET/$K - 2>&1 || true
  done
  echo "==== TAIL_$T ===="
  tail -n 30 $PFD/launch-$T.log
done
N=$(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name $ISOLAB_CODEBUILD_PROJECT --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w)
echo "SLOTS_IN_PROGRESS $N"
```

Step W3. Routing facts (RULING-P28.md C5; W1 and W2 are unchanged). Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
for T in $TAGS; do
  J=$(job_of $T)
  if test -f $PFD/receipts-$T/result.json; then
    R=RESULT_JSON_PRESENT
  else
    n=$(aws s3 ls --recursive $S3RUNS/$J/ | awk '{print $4}' | grep -cE '^runs/[^/]+/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/job-exit\.json$' || true)
    k=$(aws s3api list-objects-v2 --bucket $ISOLAB_BUCKET --prefix runs/$J/ --max-keys 1 --query KeyCount --output text)
    echo "WAIT_JOB_EXIT_COUNT $T $n"
    echo "WAIT_S3_KEYCOUNT $T $k"
    if [ "$n" -ge 1 ]; then R=S3VERIFY_NEXT; else R=MAIN_SESSION; fi
  fi
  echo "WAIT_ROUTE $T $R"
done
echo STEPW3_OK
```

The output must contain, for each tag of the unit, a line `WAIT_ROUTE <tag> <value>` and STEPW3_OK. The value is `RESULT_JSON_PRESENT` (the file receipts-TAG/result.json exists), `S3VERIFY_NEXT` (it is absent and the S3 prefix of the job holds at least one object named `runs/<job>/<pod-uid>/job-exit.json`, a pod-uid being a UUID; a `job-exit.json` under `launcher/` does not count: the next dispatch is S3VERIFY for that tag) or `MAIN_SESSION` (it is absent and the prefix holds no such object, whether the prefix is empty or not: the case goes to the main session under RULING-P28.md (b)). The executor acts on none of the three values.

## Procedure V (MODE VERIFY). Reads only; creates and changes nothing.

It answers, for a unit, whether anything of it exists, so that the main session can decide about an ambiguous submission. The card never relaunches by itself.

Step V0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step V1. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
BUILDS=$(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name $ISOLAB_CODEBUILD_PROJECT --query 'ids[:50]' --output text) --output json)
echo "BUILDS_LISTED $(echo "$BUILDS" | jq '.builds | length')"
for T in $TAGS; do
  J=$(job_of $T)
  echo "==== VERIFY_$T $J ===="
  for X in receipts-$T launch-$T.log pid-$T; do
    if [ -e $PFD/$X ]; then echo "LOCAL_EXISTS $X"; else echo "LOCAL_ABSENT $X"; fi
  done
  if [ -f $PFD/pid-$T ]; then python3 -I $TOOLS/pidwait.py 0 $PFD/pid-$T; fi
  echo "S3_KEYCOUNT $J $(aws s3api list-objects-v2 --bucket $ISOLAB_BUCKET --prefix runs/$J/ --max-keys 1 --query KeyCount --output text)"
  DIG=$($REMOTE/validate-request $PFD/request-$T.json | sha256sum | cut -d' ' -f1)
  echo "REQUEST_DIGEST $T $DIG"
  echo "REQUEST_UPLOADED_KEYCOUNT $(aws s3api list-objects-v2 --bucket $ISOLAB_BUCKET --prefix inputs/requests/$DIG.json --max-keys 1 --query KeyCount --output text)"
  echo "$BUILDS" | jq -c --arg k "inputs/requests/$DIG.json" '[.builds[] | select(any(.environment.environmentVariables[]?; .name=="ISOLAB_REQUEST_KEY" and .value==$k)) | {id: (.id | split(":")[1]), buildStatus, startTime}]' | sed "s/^/BUILDS_WITH_REQUEST_KEY $T /"
done
echo VERIFY_OK
```

The output must contain the line VERIFY_OK. The lines are returned as printed.

## Procedure S (MODE S3VERIFY). Reads S3 and downloads; changes nothing in AWS, Kubernetes or S3.

It reproduces, from S3 alone, the checks that `remote/run-request` (lines 68 to 79) and `remote/verify-build` make after a job, for one tag, so that a job whose launcher timed out can still be shown terminal. The helper `s3verify.py` (bound by sha256 in Step 2) lists `runs/<job>/`, takes the pod-uid prefix from that listing (one UUID prefix, `launcher/` excluded), downloads that prefix, hashes every file (`sha256sum -c SHA256SUMS`), opens only `job-exit.json`, `SHA256SUMS`, `build-info.json`, `binary.sha256`, `input-spec.json` and, when they exist, `audit.json`, `l2-gate.json` and `control-comparison.json`, and compares them with the frozen request `PFD/request-TAG.json`. It never opens `records.jsonl`, `session.json` or `run.stderr`. Each fact is printed as `S3V_<NAME>=<value>`; the value is `true`, `false` or `absent` (the input of the fact does not exist, or the pod prefix is not unique), except `S3V_POD_UID_COUNT` (a number), `S3V_EXIT_CODE` (the JSON text of `exit_code` in `job-exit.json`, or `absent`) and `S3V_IMAGE_DIGEST`.

`S3_TERMINAL=true` is printed if and only if all of these hold: `S3V_POD_UID_COUNT` is 1; `S3V_JOB_EXIT_PRESENT` and `S3V_JOB_EXIT_IDENTITY` (`job_name` equals the job of the tag and `pod_uid` equals the prefix) are true; `S3V_SHA256SUMS_OK` and `S3V_MANIFEST_COMPLETE` are true; `S3V_BUILD_INFO`, `S3V_BINARY_SHA` and `S3V_INPUT_SPEC` are true. The facts `S3V_EXIT0`, `S3V_L2GATE`, `S3V_AUDIT` and `S3V_CONTROL` (the jq expressions of `run-request` lines 76 to 79) and `S3V_IMAGE_DIGEST` are printed and never enter the gate: a failed job can be terminal.

Two checks of `run-request` and `verify-build` cannot be made from S3. The executor prints them as they are and does not interpret them:
- the pod image digest of Kubernetes status (`verify-build` lines 12 to 16): `S3V_IMAGE_DIGEST=unverifiable_without_pod_status`, unless the dispatch names POD_STATUS_SHA256, when the value is `true` or `false` for the file PFD/pod-status-TAG.json;
- the Kubernetes Job state and job uid (`run-request` lines 56 to 58 and 68).

`S3V_MANIFEST_COMPLETE` is stricter than `run-request`: every downloaded file except SHA256SUMS is listed in SHA256SUMS, every listed file is downloaded, and the downloaded files are exactly the files that the S3 listing shows under the pod prefix.

Step S0. Run Step 0 (card digest) unchanged.

Step S1. Hashes. Run Step 2 unchanged. The exit status must be 0 and all 39 output lines must end with ": OK".

Step S2. The tag belongs to the unit, and no earlier S3VERIFY output of the tag exists. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; tag=__TAG__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
case " $TAGS " in *" $tag "*) ;; *) echo "STOP: tag $tag is not a tag of unit $unit"; exit 1 ;; esac
J=$(job_of $tag)
test ! -e $PFD/s3verify-$tag && test ! -e /dev/shm/pf-s3verify/$tag || { echo "STOP: the S3VERIFY output directory or the download directory of $tag exists"; exit 1; }
test -f $PFD/request-$tag.json || { echo "STOP: request-$tag.json is absent"; exit 1; }
echo "S3V_PRECHECK_OK $tag $J"
echo STEPS2_OK
```

The output must contain `S3V_PRECHECK_OK <tag> <job>` and STEPS2_OK.

Step S3. Run the helper. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; tag=__TAG__; ps=__POD_STATUS_SHA256__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
case " $TAGS " in *" $tag "*) ;; *) echo "STOP: tag $tag is not a tag of unit $unit"; exit 1 ;; esac
J=$(job_of $tag)
CS=$(sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/card-launch-v2.md | cut -d' ' -f1)
if [ "$ps" = NONE ]; then
  python3 -I $TOOLS/s3verify.py --b $b --tag $tag --job $J --request $PFD/request-$tag.json --s3-prefix $S3RUNS/$J/ --card-sha256 $CS --out $PFD/s3verify-$tag --download-dir /dev/shm/pf-s3verify/$tag
else
  python3 -I $TOOLS/s3verify.py --b $b --tag $tag --job $J --request $PFD/request-$tag.json --s3-prefix $S3RUNS/$J/ --card-sha256 $CS --out $PFD/s3verify-$tag --download-dir /dev/shm/pf-s3verify/$tag --pod-status $PFD/pod-status-$tag.json --pod-status-sha256 $ps
fi
echo STEPS3_OK
```

The output must contain the line `S3V_POD_UID_COUNT=<n>`, and, when n is 1, the lines for every fact named above, one line `S3_TERMINAL=true` or `S3_TERMINAL=false`, a line `S3V_RECEIPT_SHA256=<64 hex>`, S3V_OK and STEPS3_OK. Either value of S3_TERMINAL is a complete result: the executor does not act on it. A line `STOP:` printed by the helper is a refusal (its output directory is then absent or partial, and nothing is deleted).

Step S4. Read the written files. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; tag=__TAG__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
sha256sum $PFD/s3verify-$tag/s3receipt.json
ls -l $PFD/s3verify-$tag
ls /dev/shm/pf-s3verify/$tag | wc -l
echo STEPS4_OK
```

The first field of the first line must equal the S3V_RECEIPT_SHA256 value printed by Step S3, the listing must show the two files `check-output.txt` and `s3receipt.json` and no other, and STEPS4_OK must be printed.

## Procedure T (MODE TRIGGER). Evaluates the noise-rerun trigger of PROTOCOL.md section 4 for one wall session.

The trigger is read from `job-exit.json` and `l2-gate.json` of the session alone, after the `SHA256SUMS` check. This procedure opens no `records.jsonl`, runs no `compare`, `band` or `work`, and reads no wall field. It writes `STUDY/P__B__/P__B__-RERUN.md` and the three downloaded files under `STUDY/P__B__/rerun-trigger/`. It does not commit, push or launch anything: the main session commits and pushes `P__B__-RERUN.md`, and only then dispatches RELAUNCH with UNIT `nr`.

The session evaluated: SESSION `first` is the job ic-dgbf-p__B__-20261029; SESSION `r2` is the job ic-dgbf-p__B__-20261029-r2, which is valid only when the first job wrote no job-exit.json.

The jq program (the file `trigger.jq`, bound by sha256; it implements conditions 2 to 4 of the trigger):

```
# XP-20261029-packed-filter, PROTOCOL.md section 4 "Trigger", conditions 2 to 4, evaluated over l2-gate.json.
# Run as: jq -e -f trigger.jq l2-gate.json   (exit status 0 prints true; exit status 1 prints false)
def l2_ok:
  test("^[0-9]+ ticks of foreign work on the run CPU during the run$")
  or test("^[0-9]+ ticks of hypervisor steal on the run CPU$")
  or test("^[0-9]+ busy ticks on the run CPU.s idle siblings and the rest of the reservation$")
  or test("^run-queue delay [0-9]+ µs is [0-9.]+% of the solve$")
  or test("^[0-9]+ involuntary context switches (during a [0-9.]+ ms solve|over the process) \\(at most [0-9.]+\\)$")
  or test("^[0-9]+ interrupts on the run CPU during the solve \\(.*\\)$")
  or test("^[0-9]+ foreign task slices on the run CPU during the solve \\([0-9]+ µs\\)$");
(.invalid_outcomes == [])
and ((.below_l2 | type) == "array")
and ((.below_l2 | length) > 0)
and all(.below_l2[];
      (.isolation.level == "L1")
      and ((.isolation.blockers | type) == "array")
      and all(.isolation.blockers[];
            (type == "string")
            and (startswith("L3: ") or (startswith("L2: ") and (.[4:] | l2_ok)))))
```

Step T0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step T1. Hashes. Run Step 2 unchanged. The exit status must be 0 and all 39 output lines must end with ": OK".

Step T2. Locate the session and download its three files. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=PAIR; session=__SESSION__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
case "$session" in
  first) J=ic-dgbf-p$b-20261029 ;;
  r2) J=ic-dgbf-p$b-20261029-r2 ;;
  *) echo "STOP: session is not first or r2"; exit 1 ;;
esac
if [ "$session" = r2 ]; then
  n=$(aws s3 ls --recursive $S3RUNS/ic-dgbf-p$b-20261029/ | awk '{print $4}' | grep -c '/job-exit.json$' || true)
  echo "FIRST_JOB_EXIT_COUNT $n"
  [ "$n" -eq 0 ] || { echo "STOP: the first launch wrote a job-exit.json; session r2 is not the session"; exit 1; }
fi
LST=$(aws s3 ls $S3RUNS/$J/) || { echo "STOP: no S3 prefix for $J"; exit 1; }
echo "$LST"
UIDS=$(echo "$LST" | awk '$1=="PRE"{print $2}' | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$' || true)
n=$(echo "$UIDS" | grep -c . || true)
echo "POD_UID_COUNT $J $n"
[ "$n" -eq 1 ] || { echo "STOP: the pod-uid prefix count of $J is $n, not 1"; exit 1; }
POD=${UIDS%/}
echo "POD_UID $POD"
aws s3 ls $S3RUNS/$J/$POD/job-exit.json > /dev/null || { echo "STOP: job-exit.json is absent under $J (a host loss, not a noise case)"; exit 1; }
OUT=$STUDY/P$b/rerun-trigger
test ! -e $OUT && test ! -e $STUDY/P$b/P$b-RERUN.md || { echo "STOP: $OUT or P$b-RERUN.md exists"; exit 1; }
mkdir -p $OUT
for F in job-exit.json l2-gate.json SHA256SUMS; do
  aws s3 cp --only-show-errors $S3RUNS/$J/$POD/$F $OUT/$F || { echo "STOP: download of $F failed"; exit 1; }
done
ls -l $OUT
echo "$POD" > $OUT/pod-uid.txt
echo STEPT2_OK
```

The output must contain `POD_UID_COUNT <job> 1`, a POD_UID line, a listing of three files plus none other, and STEPT2_OK. (For SESSION r2 it must also contain `FIRST_JOB_EXIT_COUNT 0`.) A refusal here wrote nothing, except that a refusal after the mkdir leaves the directory as it is.

Step T3. Evaluate and write `P__B__-RERUN.md`. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=PAIR; session=__SESSION__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-card-tools/launch-prelude.sh
case "$session" in
  first) J=ic-dgbf-p$b-20261029 ;;
  r2) J=ic-dgbf-p$b-20261029-r2 ;;
  *) echo "STOP: session is not first or r2"; exit 1 ;;
esac
OUT=$STUDY/P$b/rerun-trigger
POD=$(tr -d '\n' < $OUT/pod-uid.txt)
python3 -I $TOOLS/rerun_trigger.py $b $J $session 1 $POD $OUT $TOOLS/trigger.jq $STUDY/P$b/P$b-RERUN.md
echo STEPT3_OK
```

The output must contain the text of the file written (it ends with a line `NOISE_TRIGGER=true` or `NOISE_TRIGGER=false` after the heading `## Verdict`), the lines RERUN_MD, RERUN_MD_SHA256 and TRIGGER_OK, and STEPT3_OK. Either verdict is a complete result: the executor does not act on it.

## Return (verbatim tool output, nothing else, then the table)

Procedure L: the output of steps 0 to 6A and 7 as printed, then the output of step 8 as printed (the LAUNCH_UTC lines, the PID lines, the LAUNCHER_ID lines, and the logs), then the output of step 9. Then this table, one row per tag of the unit, each cell copied from a printed line or from this card, nothing computed:

| tag | job_name | request sha256 (pinned) | LAUNCH_UTC | PID | LAUNCHER_ID | RECEIPTS | LOG | BUILD_TIMEOUT_MINUTES |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|

Procedure W: the output of step W0, the last output of step W1 (with the number of runs made), and the output of steps W2 and W3 as printed. Then this table, one row per tag, each cell copied from a printed line:

| tag | PID_STATE | result.json state | result.json exit_code | launcher | SLOTS_IN_PROGRESS | WAIT_ROUTE |
|:--|:--|:--|:--|:--|:--|:--|

Procedure V: the output of steps V0 and V1 as printed.

Procedure S: the output of steps S0 to S4 as printed. Then this table, one row, each cell copied from a printed line:

| tag | S3V_POD_UID_COUNT | S3V_JOB_EXIT_PRESENT | S3V_JOB_EXIT_IDENTITY | S3V_EXIT_CODE | S3V_SHA256SUMS_OK | S3V_MANIFEST_COMPLETE | S3V_BUILD_INFO | S3V_BINARY_SHA | S3V_INPUT_SPEC | S3V_IMAGE_DIGEST | S3V_EXIT0 | S3V_L2GATE | S3V_AUDIT | S3V_CONTROL | S3_TERMINAL | S3V_RECEIPT_SHA256 |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|

Procedure T: the output of steps T0 to T3 as printed.

No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands.

## Write scope

Procedures L, W and V write nothing themselves. Step 0 runs `git fetch origin <branch>`, which updates only the remote-tracking ref. Procedure S writes only `PFD/s3verify-TAG/` (the files `s3receipt.json` and `check-output.txt`) and the download directory `/dev/shm/pf-s3verify/TAG/`; the helper creates both and refuses if either exists. The launched launch-remote writes only inside its RECEIPTS directory (PFD/receipts-TAG) and uploads its request and receipts to the S3 bucket named above. The shell redirects of step 8 write the LOG files and the PIDFILEs. Step 4 runs `git fetch origin <branch>`, which updates only the remote-tracking ref. Procedure T writes only `STUDY/P__B__/rerun-trigger/` and `STUDY/P__B__/P__B__-RERUN.md`. Write nowhere else; run no `git add`, `git commit` or `git push`.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly the values the MODE needs (see "Dispatch values"), or states a value outside the listed sets.
- Step 0 (or W0, V0, S0, T0) prints a first field different from CARD_SHA256 on either of its two lines, or exits with a status other than 0.
- Any output line begins with `STOP:`.
- Step 1 does not print `gate_exit=0`.
- Step 2 (or S1, T1) exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 exits with a status other than 0, prints anything on stderr, or lacks any of its required lines.
- Step 4 exits with a status other than 0, or lacks its required lines.
- Step 5 exits with a status other than 0, or lacks any of its required lines.
- Step 6 exits with a status other than 0, or lacks any VALID line.
- Step 6A exits with a status other than 0, or lacks its line `PROJECT_TIMEOUT_MINUTES N` with N at least 70, or lacks STEP6A_OK.
- Step 7 prints a line `SLOTS_IN_PROGRESS N NEED K` with N + K above 2.
- Step 8 does not print a line `PID_<tag> ` followed by digits for every tag of the unit.
- A file named LOG does not exist after step 8.
- Step 9 exits with a status other than 0, or lacks any BUILD_TIMEOUT_MINUTES line or STEP9_OK (the launch has then already happened: return the output as printed and run nothing further).
- Step W3 exits with a status other than 0, or lacks a WAIT_ROUTE line for a tag of the unit, or lacks STEPW3_OK.
- Procedure V lacks the line VERIFY_OK; procedure T lacks any required line; procedure S exits with a status other than 0 in any step, or lacks any required line of its steps S0 to S4.

Each refusal is a specification_error back to the main session. A refusal in steps 0 to 7 (including 6A) means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch.

## Never retry (applies to every procedure)

An ambiguous submission is NEVER retried. This means:
- Once step 8 has started, never run any `launch-remote` command a second time for any request of the unit, whatever any output shows: an empty log, a log without a `Launcher:` line, an AWS error, a failed StartBuild, a timeout of the Bash call, an exit of the process, a missing pid file.
- If the launcher reports an error in its log, or step 8 or W1 or W2 prints anything unexpected, return the output as printed and run nothing further. Do not inspect the job, do not list builds beyond the commands of this card, do not delete or move any file, do not resubmit under another name.
- A job name that already has receipts, a log, a pid file, an S3 prefix or a CodeBuild build carrying its request key is never launched again by this card: step 5 refuses. Only the main session can authorise a new attempt, and only after Procedure V has shown that no build carries the request key (`BUILDS_WITH_REQUEST_KEY <tag> []`) and that the S3 prefix of the job is empty (`S3_KEYCOUNT <job> 0`); the main session then moves the old receipts directory, log and pid file of the tag aside before it dispatches the launch again.
- Procedure S is run once per tag: Step S2 refuses when `PFD/s3verify-TAG` or `/dev/shm/pf-s3verify/TAG` exists. After a refusal in S3 or S4, delete and move nothing; only the main session moves the two directories aside before it dispatches S3VERIFY for that tag again.
- A reported error or a stopped job is not a result for this card. The reruns of PROTOCOL.md section 4 are dispatches of their own: UNIT r2, rho-r2 and nr-r2 (a host loss: the predecessor wrote no job-exit.json), and UNIT nr (only after `P<b>-RERUN.md` with the line `NOISE_TRIGGER=true` is pushed). A job that wrote a job-exit.json with a non-zero code is never rerun by any other route.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs detached launches with the logs outside the receipts directories, a 90-second partial report, a wait by pid and a loop over the unit's tags, so the tool cannot express it. The cards of the previous studies (CARD-XP-20261029-dg-blocked-launch and its predecessors) are plain cards for the same reason. There is no runbook, so `tools/runbook.py lint` does not apply.

## Changes from card-launch.md (RULING-P28.md (c))

Everything not listed here is copied byte for byte from card-launch.md: L steps 0 (except its command), 1, 3, 5, 6, 7 and 8, W1, W2, V, T, the pinned requests, specs and tools, and the rules of "Never retry".

- C1: new file and new CARD_SHA256; the card cites RULING-P28.md and the superseded card by sha256; Step 0 also proves that the file is committed and pushed; new helper `s3verify.py` bound in Step 2 (39 lines instead of 34: the ruling, the superseded card, `run-request`, `verify-build` and `s3verify.py` are added after the original 34).
- C2: new MODE S3VERIFY (Procedure S).
- C3: Step 4 of LAUNCH names the sessions of record (PREV_WALL_TAG, PREV_COUNTED_TAG) and accepts `result.json` or an S3VERIFY receipt with `S3_TERMINAL` true; the RELAUNCH branch of Step 4 is unchanged except for the check that both values are `NONE`.
- C4: Step 6A (fail-closed budget check, LAUNCH and RELAUNCH) and Step 9 (reported build budget).
- C5: Step W3 (routing facts) after W2.
- C6 and C7: no change to the card text (card-analysis.md is unchanged; the card adds no authority).

## Note

The receipts directories under PFD (`request.json`, `result.json`, `build.json` and the other files `launch-remote` writes) and the logs are copied into the study directory by the main session (PROTOCOL.md section 10); the executor never copies them. A foreground `sleep` can be blocked by the harness; step 8 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. Each dispatch of this card is one MODE and one unit. A first launch of a size runs both of its jobs (the builds of the previous study lasted 224 s, 337 s and 1060 s).
