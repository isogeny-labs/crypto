# Mechanical task card CARD-XP-20261029-packed-filter-launch

author_role: experiment-director
executor_role: executor-mechanical
experiment: packed-filter (protocol backlog/XP-20261029-packed-filter/PROTOCOL.md; isolab-eks AWS launcher; not an EXP-* record)
kind: plain frozen launch card (not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol with edits E1-E10 and the pre-launch checks P1-P6 (APPROVAL.md in the protocol directory); the pre-launch checks all passed (research/ic_packed_filter_20261029/cards/prelaunch-checks.md, pinned below). The main (reasoning-tier) session decides the route and which dispatch to send next. This card adds no authority.

## Dispatch values

The dispatch names the values below, and the executor substitutes them and changes nothing else. A dispatch that names any other value, omits one, or names a value outside the listed set is refused.

- MODE: one of `LAUNCH`, `RELAUNCH`, `WAIT`, `VERIFY`, `TRIGGER`.
- the bit size: 24, 28 or 32. Substitute its digits everywhere the text __B__ appears in this card.
- UNIT: required for `RELAUNCH`, `WAIT` and `VERIFY`; absent for `LAUNCH` and `TRIGGER`. Substitute it everywhere the text __UNIT__ appears. The values:
  - `PAIR`: the two first-launch jobs of a size (only for `WAIT` and `VERIFY`; `LAUNCH` is `PAIR` by definition);
  - `r2`: the host-loss rerun of the wall job;
  - `rho-r2`: the host-loss rerun of the counted job;
  - `nr`: the noise rerun of the wall job (PROTOCOL.md section 4);
  - `nr-r2`: the host-loss rerun of the noise rerun.
- SESSION: required for `TRIGGER` only: `first` or `r2`. Substitute it everywhere the text __SESSION__ appears.
- CARD_SHA256: the sha256 of this card file.

| MODE | what it does | procedure |
|:--|:--|:--|
| LAUNCH | launches the two first-launch jobs of one size (the wall job and the counted job) | L (unit PAIR) |
| RELAUNCH | launches one contingent job (UNIT r2, rho-r2, nr or nr-r2) | L |
| WAIT | waits for the launcher processes of a unit and prints the outcome files | W |
| VERIFY | reads (never writes) whether the build, the S3 prefix and the local files of a unit exist | V |
| TRIGGER | evaluates the noise-rerun trigger of one wall session and writes `P<b>-RERUN.md` | T |

## What the launcher does, and the slot rule

- Each job is one `launch-remote` process on this machine. It starts one AWS CodeBuild build of the project isolab-experiment-launcher, and that build stays `IN_PROGRESS` for the whole remote job. The build is finished when its buildStatus is not `IN_PROGRESS`; the local `launch-remote` process has then exited too.
- The project allows at most 2 concurrent builds. A first launch of a size needs both slots (2 jobs). So the sizes run in this order, one size at a time: 24, then 28, then 32.
- `LAUNCH` of a size is accepted only when both launchers of the size before it have exited and their `result.json` files exist, and when the CodeBuild count `IN_PROGRESS` is 0. `RELAUNCH` is accepted only when that count is at most 1. The procedure checks these facts and refuses otherwise; it never waits. `WAIT` is how the main session learns that a unit has finished.
- The card does not read any outcome and does not interpret any file. The main session decides, from the reports, which dispatch to send next.

## Frozen values

- Freeze commit of the protocol: @@FREEZE@@; branch @@BRANCH@@ (pushed). The freeze commit message records that the six specs are byte-identical to the reviewed drafts; their sha256 are pinned below.
- Candidate code commit: @@REV@@ (the freeze commit adds only `backlog/XP-20261029-packed-filter/` to it).
- SRC = @@SRC@@ (sparse checkout of the freeze commit, clean). The launcher scripts there are used: REMOTE = @@REMOTE@@. Their sha256 equal those of the previous study's launcher (pinned below). Never use the launcher scripts of /home/user/crypto.
- Image of every job: @@IMAGE@@ (each request embeds it).
- ecbench sha256 @@FBSHA@@; source revision @@REV@@ (each request embeds both).
- Every job: instance c8a.2xlarge, execution deadline 3600 s, provenance.harness claude-code-local, provenance.run_url @@RUNURL@@
- PFD = @@PFD@@ (requests, receipts, logs and pid files; nothing larger than a few kilobytes is written there).
- TOOLS = @@TOOLS@@ (helper files, bound by sha256 below).
- Protocol directory PD = @@PD@@. Protocol worktree WT = @@WT@@. Study directory STUDY = @@STUDY@@.
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

@@REQTABLE@@

Pinned sha256 (protocol files in PD, as committed at the freeze commit):

@@PDTABLE@@

Pinned sha256 (other files):

@@OTHERTABLE@@

## Procedure L (MODE LAUNCH or RELAUNCH)

Use one Bash call per step; change nothing. Steps 0 to 7 are checks; if any of them fails, launch nothing. In every command the words `__B__` and `__UNIT__` stand for the dispatch values (for MODE `LAUNCH`, UNIT is `PAIR`). A line that begins with `STOP:` in any output is a refusal.

Step 0. Card digest. Run (the left side of the comparison is the CARD_SHA256 value stated in the dispatch):

```
sha256sum @@CARDPATH@@
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with `__B__`, `__UNIT__` and `__SESSION__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Checkpoint gate. Run:

```
cd @@WT@@ && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash scripts/checkpoint.sh gate; echo "gate_exit=$?"
```

The line `gate_exit=0` must be printed.

Step 2. Hashes (all requests, the protocol files, the launcher scripts, the checkpoint script, the helper files). Run, as one Bash call, exactly these lines in this order:

```
sha256sum -c - <<'HASHES'
@@HASHLINES@@
HASHES
```

(The here-document ends only at the line that is exactly HASHES.) The exit status must be 0 and all @@NHASH@@ output lines must end with ": OK".

Step 3. Each request of the unit equals its frozen spec and its frozen fields; the freeze commit is in the worktree and pushed; the protocol directory equals the freeze commit. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
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
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
if [ "$unit" = PAIR ]; then
  case "$b" in
    24) echo ORDER_NOT_REQUIRED ;;
    28|32)
      if [ "$b" = 28 ]; then PREV=24; else PREV=28; fi
      test -f $PFD/pid-P$PREV && test -f $PFD/pid-P$PREV-rho || { echo "STOP: a pid file of size $PREV is absent"; exit 1; }
      OUT=$(python3 -I $TOOLS/pidwait.py 0 $PFD/pid-P$PREV $PFD/pid-P$PREV-rho)
      echo "$OUT"
      [ "$(echo "$OUT" | tail -n 1)" = PAIR_DONE ] || { echo "STOP: a launcher of size $PREV is still running"; exit 1; }
      test -f $PFD/receipts-P$PREV/result.json && test -f $PFD/receipts-P$PREV-rho/result.json && echo PREV_RESULTS_PRESENT || { echo "STOP: result.json of size $PREV is absent"; exit 1; }
      ;;
  esac
else
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

For MODE LAUNCH and b = 24 the output must contain ORDER_NOT_REQUIRED. For MODE LAUNCH and b = 28 or 32 it must contain two PID_STATE lines each ending in `exited`, PAIR_DONE and PREV_RESULTS_PRESENT. For RELAUNCH r2, rho-r2 and nr-r2 it must contain one PID_STATE line ending in `exited`, PAIR_DONE, `PRED_JOB_EXIT_COUNT <job> 0` and PRED_HAS_NO_JOB_EXIT; for nr and nr-r2 it must also contain RERUN_MD_SHA256 local=X pushed=X (the same hash twice) and RERUN_MD_PUSHED. Every output ends with STEP4_OK.

Step 5. Nothing of the unit exists yet: local files, S3 prefixes, uploaded request, CodeBuild builds. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
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
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
cd $SRC
for T in $TAGS; do
  taskq/deploy/eks-karpenter/remote/validate-request $PFD/request-$T.json > /dev/null && echo VALID_$T || { echo "STOP: validate-request failed for $T"; exit 1; }
done
echo STEP6_OK
```

The output must contain VALID_TAG for each tag of the unit and STEP6_OK.

Step 7. Build slots. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
N=$(aws codebuild batch-get-builds --ids $(aws codebuild list-builds-for-project --project-name $ISOLAB_CODEBUILD_PROJECT --query 'ids[:5]' --output text) --query 'builds[?buildStatus==`IN_PROGRESS`].id' --output text | wc -w)
echo "SLOTS_IN_PROGRESS $N NEED $NEED"
[ $((N + NEED)) -le 2 ] || { echo "STOP: not enough free build slots"; exit 1; }
echo STEP7_OK
```

The output must contain a line `SLOTS_IN_PROGRESS N NEED K` with N + K at most 2 (for MODE LAUNCH, K is 2 and N must be 0; for RELAUNCH, K is 1 and N must be 0 or 1) and STEP7_OK.

Step 8. Launch the unit, wait 90 seconds, read the logs (one Bash call, exactly these lines in this order; run this call once and never again):

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
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

## Procedure W (MODE WAIT). Use one Bash call per step; change nothing. The wait reads only /proc entries of the recorded pids, never `pgrep` or `ps` with a pattern.

Step W0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step W1. Wait by pid. Run, with the Bash tool `timeout` parameter set to 570000, exactly these lines:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
python3 -I $TOOLS/pidwait.py 540 $(for T in $TAGS; do echo $PFD/pid-$T; done)
```

The script prints one `PID_STATE` line per pid file, then PAIR_DONE (every launcher of the unit exited) or PAIR_RUNNING. If the last line is PAIR_RUNNING, run the identical lines again; at most 11 runs in total. Stop repeating at the first PAIR_DONE.

Step W2. Read the outcome. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
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

## Procedure V (MODE VERIFY). Reads only; creates and changes nothing.

It answers, for a unit, whether anything of it exists, so that the main session can decide about an ambiguous submission. The card never relaunches by itself.

Step V0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step V1. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=__UNIT__; . @@TOOLS@@/launch-prelude.sh
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

## Procedure T (MODE TRIGGER). Evaluates the noise-rerun trigger of PROTOCOL.md section 4 for one wall session.

The trigger is read from `job-exit.json` and `l2-gate.json` of the session alone, after the `SHA256SUMS` check. This procedure opens no `records.jsonl`, runs no `compare`, `band` or `work`, and reads no wall field. It writes `STUDY/P__B__/P__B__-RERUN.md` and the three downloaded files under `STUDY/P__B__/rerun-trigger/`. It does not commit, push or launch anything: the main session commits and pushes `P__B__-RERUN.md`, and only then dispatches RELAUNCH with UNIT `nr`.

The session evaluated: SESSION `first` is the job ic-dgbf-p__B__-20261029; SESSION `r2` is the job ic-dgbf-p__B__-20261029-r2, which is valid only when the first job wrote no job-exit.json.

The jq program (the file `trigger.jq`, bound by sha256; it implements conditions 2 to 4 of the trigger):

```
@@TRIGGERJQ@@
```

Step T0. Run Step 0 (card digest) unchanged. The first field must equal CARD_SHA256 from the dispatch.

Step T1. Hashes. Run Step 2 unchanged. The exit status must be 0 and all @@NHASH@@ output lines must end with ": OK".

Step T2. Locate the session and download its three files. Run, as one Bash call, exactly these lines in this order:

```
b=__B__; unit=PAIR; session=__SESSION__; . @@TOOLS@@/launch-prelude.sh
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
b=__B__; unit=PAIR; session=__SESSION__; . @@TOOLS@@/launch-prelude.sh
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

Procedure L: the output of steps 0 to 7 as printed, then the output of step 8 as printed (the LAUNCH_UTC lines, the PID lines, the LAUNCHER_ID lines, and the logs). Then this table, one row per tag of the unit, each cell copied from a printed line or from this card, nothing computed:

| tag | job_name | request sha256 (pinned) | LAUNCH_UTC | PID | LAUNCHER_ID | RECEIPTS | LOG |
|:--|:--|:--|:--|:--|:--|:--|:--|

Procedure W: the output of step W0, the last output of step W1 (with the number of runs made), and the output of step W2 as printed. Then this table, one row per tag, each cell copied from a printed line:

| tag | PID_STATE | result.json state | result.json exit_code | launcher | SLOTS_IN_PROGRESS |
|:--|:--|:--|:--|:--|:--|

Procedure V: the output of steps V0 and V1 as printed.

Procedure T: the output of steps T0 to T3 as printed.

No retries, no resubmission, no job-name change, no edits, no interpretation, no other commands.

## Write scope

Procedures L, W and V write nothing themselves. The launched launch-remote writes only inside its RECEIPTS directory (PFD/receipts-TAG) and uploads its request and receipts to the S3 bucket named above. The shell redirects of step 8 write the LOG files and the PIDFILEs. Step 4 runs `git fetch origin <branch>`, which updates only the remote-tracking ref. Procedure T writes only `STUDY/P__B__/rerun-trigger/` and `STUDY/P__B__/P__B__-RERUN.md`. Write nowhere else; run no `git add`, `git commit` or `git push`.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly the values the MODE needs (see "Dispatch values"), or states a value outside the listed sets.
- Step 0 (or W0, V0, T0) prints a first field different from CARD_SHA256, or exits with a status other than 0.
- Any output line begins with `STOP:`.
- Step 1 does not print `gate_exit=0`.
- Step 2 (or T1) exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 3 exits with a status other than 0, prints anything on stderr, or lacks any of its required lines.
- Step 4 exits with a status other than 0, or lacks its required lines.
- Step 5 exits with a status other than 0, or lacks any of its required lines.
- Step 6 exits with a status other than 0, or lacks any VALID line.
- Step 7 prints a line `SLOTS_IN_PROGRESS N NEED K` with N + K above 2.
- Step 8 does not print a line `PID_<tag> ` followed by digits for every tag of the unit.
- A file named LOG does not exist after step 8.
- Procedure V lacks the line VERIFY_OK; procedure T lacks any required line.

Each refusal is a specification_error back to the main session. A refusal in steps 0 to 7 means no job was launched. After a refusal run nothing further: no retry, no edit, no cleanup, no relaunch.

## Never retry (applies to every procedure)

An ambiguous submission is NEVER retried. This means:
- Once step 8 has started, never run any `launch-remote` command a second time for any request of the unit, whatever any output shows: an empty log, a log without a `Launcher:` line, an AWS error, a failed StartBuild, a timeout of the Bash call, an exit of the process, a missing pid file.
- If the launcher reports an error in its log, or step 8 or W1 or W2 prints anything unexpected, return the output as printed and run nothing further. Do not inspect the job, do not list builds beyond the commands of this card, do not delete or move any file, do not resubmit under another name.
- A job name that already has receipts, a log, a pid file, an S3 prefix or a CodeBuild build carrying its request key is never launched again by this card: step 5 refuses. Only the main session can authorise a new attempt, and only after Procedure V has shown that no build carries the request key (`BUILDS_WITH_REQUEST_KEY <tag> []`) and that the S3 prefix of the job is empty (`S3_KEYCOUNT <job> 0`); the main session then moves the old receipts directory, log and pid file of the tag aside before it dispatches the launch again.
- A reported error or a stopped job is not a result for this card. The reruns of PROTOCOL.md section 4 are dispatches of their own: UNIT r2, rho-r2 and nr-r2 (a host loss: the predecessor wrote no job-exit.json), and UNIT nr (only after `P<b>-RERUN.md` with the line `NOISE_TRIGGER=true` is pushed). A job that wrote a job-exit.json with a non-zero code is never rerun by any other route.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task needs detached launches with the logs outside the receipts directories, a 90-second partial report, a wait by pid and a loop over the unit's tags, so the tool cannot express it. The cards of the previous studies (CARD-XP-20261029-dg-blocked-launch and its predecessors) are plain cards for the same reason. There is no runbook, so `tools/runbook.py lint` does not apply.

## Note

The receipts directories under PFD (`request.json`, `result.json`, `build.json` and the other files `launch-remote` writes) and the logs are copied into the study directory by the main session (PROTOCOL.md section 10); the executor never copies them. A foreground `sleep` can be blocked by the harness; step 8 waits with python3 for that reason. The launcher's own wait (up to 5400 s) is separate from the experiment's 3600 s execution deadline. Each dispatch of this card is one MODE and one unit. A first launch of a size runs both of its jobs (the builds of the previous study lasted 224 s, 337 s and 1060 s).
