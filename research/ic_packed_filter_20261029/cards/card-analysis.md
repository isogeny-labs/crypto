# Mechanical task card CARD-XP-20261029-packed-filter-analysis

author_role: experiment-director
executor_role: executor-mechanical
experiment: packed-filter (protocol backlog/XP-20261029-packed-filter/PROTOCOL.md; one task per wall session after the runs; not an EXP-* record)
kind: plain frozen command card for the work after the runs (no launch step: the jobs are launched with CARD-XP-20261029-packed-filter-launch; not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol with edits E1-E10 and the pre-launch checks P1-P6 (APPROVAL.md in the protocol directory; all checks passed, research/ic_packed_filter_20261029/cards/prelaunch-checks.md). The main (reasoning-tier) session decides when to dispatch this card and with which values. This card adds no authority and decides nothing.

## Parameters (the dispatch message gives exactly four values: b, WALL, COUNTED and CARD_SHA256; no other value is accepted)

- b is exactly one of 24, 28 or 32. In every command below the word `__B__` stands for the dispatch value of b.
- WALL names the wall session to process, exactly one of `first`, `r2`, `nr`, `nr-r2`. In every command below the word `__WALL__` stands for it.
- COUNTED is exactly one of `first`, `r2` (process that counted session, which creates the counted output directory) or `existing` (the counted output directory `counted` already exists from an earlier dispatch of this card; this dispatch processes only the wall session). In every command below the word `__COUNTED__` stands for it.
- CARD_SHA256 is used only in step 0.
- Type the dispatch values in place of `__B__`, `__WALL__` and `__COUNTED__` and change nothing else. The sourced prelude refuses any other value.

| WALL | wall job | output directory under D |
|:--|:--|:--|
| first | ic-dgbf-p__B__-20261029 | wall |
| r2 | ic-dgbf-p__B__-20261029-r2 | wall |
| nr | ic-dgbf-p__B__-20261029-nr | wall-nr |
| nr-r2 | ic-dgbf-p__B__-20261029-nr-r2 | wall-nr |

| COUNTED | counted job | output directory under D |
|:--|:--|:--|
| first | ic-dgbf-rho-p__B__-20261029 | counted |
| r2 | ic-dgbf-rho-p__B__-20261029-r2 | counted |
| existing | none (the earlier output in `counted` is used) | counted (read only) |

Which dispatch applies is the main session's decision (PROTOCOL.md section 4): the session the decision reads is the wall session of the first launch (WALL `first`), or its host-loss rerun (`r2`), or, after a noise rerun, the rerun alone (`nr`, or `nr-r2` after a host loss). A first session that was followed by a noise rerun is processed descriptively with WALL `first` and COUNTED `existing`, and only after the rerun's job-exit.json exists (step 2 refuses earlier).

## Frozen values

- Protocol freeze commit: f870408c1762a52dd133e8f1e0bca181cc2a76f5. Protocol directory PD = /dev/shm/src-pf/backlog/XP-20261029-packed-filter (its files are bound by sha256 in step 1).
- Source revision that built the ecbench: 3271a25d02dcef6da148293fb8f9e2149ce59858. Image: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b.
- S3 results prefix of a job: `s3://isolab-results-104186811093-us-east-2/runs/<job name>/` followed by exactly one pod-uid prefix (a UUID). The prefix named `launcher/` is never a pod prefix and is not downloaded.
- Output directory D = /dev/shm/src-pf/research/ic_packed_filter_20261029/P__B__ (the committed place for this study's outputs). Subdirectories: `wall/eks-results` and `wall/analysis` (WALL first or r2), `wall-nr/eks-results` and `wall-nr/analysis` (WALL nr or nr-r2), `counted/eks-results` and `counted/analysis`. D may already hold `P__B__-RERUN.md` and `rerun-trigger/`; the wall output directory and, for COUNTED first or r2, the counted directory must not exist before step 3.
- Helper files, all bound by sha256 in step 1, in TOOLS = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools: analysis-prelude.sh, session_facts.py, project_compare.py, records_checks.py, work_checks.py, sprime.py, positions.py, wall_gate.py, counted_gate.py, phase_sums.py, receipt.py, project_facts.py. They take every path as an argument.
- Binary for compare, band and work: /dev/shm/pf-bundle/ecbench, sha256 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503. It is the image's ecbench. No other binary is used. Every compare runs at its default `--resamples` and `--seed` (nothing is passed for either).
- AWS: the executor's existing identity. Never print, write or copy credentials. The prelude sets PATH to start with /tmp/claude-0/awsvenv/bin, AWS_DEFAULT_REGION=us-east-2 and AWS_REGION=us-east-2.
- Arms. Wall session (4): dgb (baseline), dgb-aa (control), dgbf (candidate), dgbf-edge (reference). Counted session (4): rho-neg (reference), dgb (baseline), dgbf (candidate), rho-neg-aa (control).
- Pairs. A pair written A:B is run as `compare --a A --b B`, so the ratio is B over A.
  - Wall session, 5 pairs, the pairs of PROTOCOL.md section 5: dgb:dgbf (the decision pair), dgb:dgb-aa, dgbf-edge:dgbf, dgb:dgbf-edge (the position control), dgb-aa:dgbf.
  - Counted session, 3 pairs: rho-neg:rho-neg-aa, rho-neg:dgb, rho-neg:dgbf.
- Weights (GAE per unpriced native operation): 0, 0.1 and 1. Every pair is run at each weight with `--weight W --json`. The wall block of a compare reads charged GAE and does not depend on the weight.
- Pinned sha256:
  - PROTOCOL.md 703bde26a197859c02e2e5f293ba6719cb5b71871f3044a7ac336d346d067e51
  - APPROVAL.md 0a35923cca184b68c212b66a70d2d42e2e2546e1cd50fd6ebe99d854889a6a40
  - spec-P24.json ce9510367b95c5038b4b200675943b2e81bc9f395921cfe10f87973ab7f151e7
  - spec-P24-rho.json c98bf0c914efceef796be5f00a86e18c75e426a1ca8cf9e63fab748c23db0321
  - spec-P28.json 9f54fdd0305c789f6b12202fbb81bee50a05135fca674fae65c7557f531053d9
  - spec-P28-rho.json 55a403714d9cb353eb1fe644eb9e04e628ef46b1a1ada5b8575709db40cc349a
  - spec-P32.json 9f950f3bcba5e4729798c82bcca1a1d665104329b5ed9368c9d98b6addbc30ab
  - spec-P32-rho.json 2095978592b5d21713d5de32dec6c1b862cc7e0d3810c3f82f8399c4a2c45389
  - cards/prelaunch-checks.md 6542ad8f2c8f33c0acca70dbabcf76492b872145cd1138de9cdf46a818d40b83
  - analysis-prelude.sh a38196bca5b56d8d3bb4972fb5bcbd5f40f253dc9c0e21c0ffad20f59ff94b7c
  - session_facts.py 1d36037c5efb342d936add3407f4be0c3c927534ef0284e170428f0589715f65
  - project_compare.py 95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab
  - records_checks.py 54f16312c02ce56a917b5cde57ebc7440e40e9c9b355dfe39321ed5491151406
  - work_checks.py 623a2b476d75eeea820d54271371fd0490be4dc350d52bcb767d010fca707294
  - sprime.py 1abc1511ba95caa569db84c12d6e8e268a11447cf36d32faef6feb91c9e2a862
  - positions.py ebd0327eb7046be82cf9ff64b3f94ceb8cdf55469056937ce4af36b8f232db44
  - wall_gate.py de6d8f77ea4c64b59d14fc94be422b17e6bf1a70dd420d0698db210b1e221256
  - counted_gate.py 93541ea09390b482c24a437644453b450ffb25b8eb44e92acf0f3dead3997881
  - phase_sums.py a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1
  - receipt.py d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef
  - project_facts.py edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091
  - ecbench 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503

## What the helper scripts do (a description; the executor does nothing with it)

- session_facts.py reads the copy of a session's result files and records facts (build-info, audit, l2-gate, job-exit, input-spec/spec, binary) in `analysis/facts.json`. It never judges them.
- project_compare.py prints a fixed key projection of every compare JSON (the full JSON stays in `analysis/`). For the counted session it prints no field of any `wall` block.
- records_checks.py reads `records.jsonl` of both sessions and prints the record-level facts: the record half of wall condition 8 (each dgbf and dgbf-edge record, warm-ups included, against the dgb record of the same curve, target and round: recovered logarithm, total_gae, the list of unpriced counters, every unpriced-operation count as ecbench band's `unpriced_counts` finds it, and per phase the name, adds, doubles, scalar_mults, gae, the native map and the work_native map without `table_filter_bit_sets`), the three requirements on `table_filter_bit_sets`, the counted-session clause (the counted dgb and dgbf records against the wall session's, `table_filter_bit_sets` included), and the facts behind condition 10 (p from the ICV1 curve string, signed points, and which source gave them). It reads no wall field and computes no statistic. It prints R_COND8_RECORDS=, R_TFBS_REQUIREMENTS=, R_COUNTED_EQUALS_WALL=, R_COND10_WALL= and R_COND10_COUNTED= as true or false by the literal wording.
- work_checks.py runs `ecbench work` on each session and checks the bitop clause of condition 8 (for each dgbf and dgbf-edge record, bitops minus the dgb record's equals 384 x table_filter_bit_sets, entirely in the oracle_setup phase); it prints the sums of `work.bitops` per arm and their ratios (PROTOCOL.md section 5, "bitop/v1"). It prints R_COND8_BITOPS= and R_COUNTED_BITOPS_IDENTITY=.
- sprime.py computes, from `records.jsonl`, S'_w = (priced_gae_w + w x table_filter_bit_sets) / sqrt(r) per record at w = 0, 0.1, 1 and the ratios of PROTOCOL.md section 5 ("The filter's bit sets in the section 5a band"), beside the `compare` band ratio of the same pair as a cross-check.
- positions.py tabulates each arm's block positions and predecessors from `records.jsonl` (seq, arm, round, workload id; no wall field).
- wall_gate.py reads files and compares values. It prints the facts behind the conditions 1 to 10 of PROTOCOL.md section 6.1 (wall session, as edited by APPROVAL.md E1 to E3), and the lines W_COND1= to W_COND10=, W_ALL_1_TO_10=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= as true or false by the literal wording. These lines are mechanical facts. The executor does not interpret them: a value of false is not a stop condition and is returned as printed.
- counted_gate.py does the same for the counted session: the lines GATE_COND1= to GATE_COND6=, EXIT42_CLAUSE= and EXIT43_CLAUSE=, the exit-42 clause recomputed from `l2-gate.json` and independently from `session/records.jsonl`. It reads no wall field of a record.
- phase_sums.py prints integer sums of the wall session's recorded times (PROTOCOL.md section 5, "Phase sums", descriptive). It divides nothing.

## Procedure

Use one Bash call per step. Paste each command block exactly as written, with only the `__B__`, `__WALL__` and `__COUNTED__` replacements. A line that begins with `STOP:` in any output is a refusal (see "Stop and return refused"). Change no file in the repository, none of the helper files, and no file outside D. Do not open any other file, and no S3 prefix other than the ones the prelude names.

Step 0. Card digest. Run:

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf/card-analysis.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the words `__B__`, `__WALL__` and `__COUNTED__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Hashes of the protocol files, specs, helper files and binary. Run:

```
sha256sum -c - <<'HASHES'
703bde26a197859c02e2e5f293ba6719cb5b71871f3044a7ac336d346d067e51  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/PROTOCOL.md
0a35923cca184b68c212b66a70d2d42e2e2546e1cd50fd6ebe99d854889a6a40  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/APPROVAL.md
ce9510367b95c5038b4b200675943b2e81bc9f395921cfe10f87973ab7f151e7  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P24.json
c98bf0c914efceef796be5f00a86e18c75e426a1ca8cf9e63fab748c23db0321  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P24-rho.json
9f54fdd0305c789f6b12202fbb81bee50a05135fca674fae65c7557f531053d9  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P28.json
55a403714d9cb353eb1fe644eb9e04e628ef46b1a1ada5b8575709db40cc349a  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P28-rho.json
9f950f3bcba5e4729798c82bcca1a1d665104329b5ed9368c9d98b6addbc30ab  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P32.json
2095978592b5d21713d5de32dec6c1b862cc7e0d3810c3f82f8399c4a2c45389  /dev/shm/src-pf/backlog/XP-20261029-packed-filter/spec-P32-rho.json
6542ad8f2c8f33c0acca70dbabcf76492b872145cd1138de9cdf46a818d40b83  /dev/shm/src-pf/research/ic_packed_filter_20261029/cards/prelaunch-checks.md
a38196bca5b56d8d3bb4972fb5bcbd5f40f253dc9c0e21c0ffad20f59ff94b7c  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
1d36037c5efb342d936add3407f4be0c3c927534ef0284e170428f0589715f65  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/session_facts.py
95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/project_compare.py
54f16312c02ce56a917b5cde57ebc7440e40e9c9b355dfe39321ed5491151406  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/records_checks.py
623a2b476d75eeea820d54271371fd0490be4dc350d52bcb767d010fca707294  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/work_checks.py
1abc1511ba95caa569db84c12d6e8e268a11447cf36d32faef6feb91c9e2a862  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/sprime.py
ebd0327eb7046be82cf9ff64b3f94ceb8cdf55469056937ce4af36b8f232db44  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/positions.py
de6d8f77ea4c64b59d14fc94be422b17e6bf1a70dd420d0698db210b1e221256  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/wall_gate.py
93541ea09390b482c24a437644453b450ffb25b8eb44e92acf0f3dead3997881  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/counted_gate.py
a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/phase_sums.py
d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/receipt.py
edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/project_facts.py
21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503  /dev/shm/pf-bundle/ecbench
HASHES
```

The exit status must be 0 and all 22 output lines must end with ": OK". (The here-document ends only at the line that is exactly HASHES.)

Step 2. Readiness check (reads only; writes nothing). It also enforces the blinding of PROTOCOL.md section 4: the records of a session that exited 42 stay sealed until the trigger has been evaluated and, if it holds, until the rerun has written its job-exit.json. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
test ! -e "$DW" || { echo "STOP: wall output directory exists: $DW"; exit 1; }
if [ "$DO_COUNTED" = yes ]; then
  test ! -e "$DC" || { echo "STOP: counted output directory exists: $DC"; exit 1; }
else
  test -f "$DC/analysis/receipt.json" && test -d "$DC/eks-results/session" || { echo "STOP: COUNTED existing, but $DC holds no earlier output"; exit 1; }
fi
avail=$(df -m --output=avail /dev/shm | tail -n 1 | tr -d ' ')
echo "DF_AVAIL_MB $avail"
[ "$avail" -ge 600 ] || { echo "STOP: less than 600 MB free on /dev/shm"; exit 1; }
if [ "$DO_COUNTED" = yes ]; then KINDS="wall counted"; else KINDS="wall"; fi
for K in $KINDS; do
  if [ "$K" = wall ]; then S3=$S3W; else S3=$S3C; fi
  LST=$(aws s3 ls "$S3/") || { echo "STOP: aws s3 ls failed or found nothing under $S3/ ($K job not finished or not launched)"; exit 1; }
  echo "S3_LISTING $K $S3/"
  echo "$LST"
  UIDS=$(echo "$LST" | awk '$1=="PRE"{print $2}' | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$' || true)
  n=$(echo "$UIDS" | grep -c . || true)
  echo "POD_UID_COUNT $K $n"
  [ "$n" -eq 1 ] || { echo "STOP: pod-uid prefix count of the $K job is $n, not 1"; exit 1; }
  aws s3 ls "$S3/${UIDS}job-exit.json" > /dev/null || { echo "STOP: job-exit.json absent under $S3/$UIDS"; exit 1; }
  aws s3 ls "$S3/${UIDS}SHA256SUMS" > /dev/null || { echo "STOP: SHA256SUMS absent under $S3/$UIDS"; exit 1; }
done
POD=$(aws s3 ls "$S3W/" | awk '$1=="PRE"{print $2}' | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$')
EC=$(aws s3 cp --only-show-errors "$S3W/${POD}job-exit.json" - | jq -r .exit_code)
echo "WALL_JOB_EXIT_CODE $JW $EC"
if [ "$EC" = 42 ] && { [ "$wall" = first ] || [ "$wall" = r2 ]; }; then
  R=$STUDY/P$b/P$b-RERUN.md
  test -f "$R" || { echo "STOP: sealed: $JW exited 42 and $R does not exist; the trigger has not been evaluated"; exit 1; }
  grep -q "job evaluated: \`$JW\`" "$R" || { echo "STOP: sealed: $R does not evaluate $JW"; exit 1; }
  if grep -qx 'NOISE_TRIGGER=true' "$R"; then
    nn=$(aws s3 ls --recursive "$S3RUNS/ic-dgbf-p$b-20261029-nr/" | awk '{print $4}' | grep -c '/job-exit.json$' || true)
    nn2=$(aws s3 ls --recursive "$S3RUNS/ic-dgbf-p$b-20261029-nr-r2/" | awk '{print $4}' | grep -c '/job-exit.json$' || true)
    echo "RERUN_JOB_EXIT_COUNT nr=$nn nr-r2=$nn2"
    [ $((nn + nn2)) -ge 1 ] || { echo "STOP: sealed: the trigger holds and the noise rerun has not written a job-exit.json"; exit 1; }
  fi
  echo "SEALING_CHECK_PASSED"
fi
echo STEP2_OK
```

The output must contain a DF_AVAIL_MB line with a number of at least 600, an S3_LISTING block and a `POD_UID_COUNT ... 1` line for each session to be processed, a `WALL_JOB_EXIT_CODE` line, and the line STEP2_OK. A refusal in steps 0 to 2 has written nothing, so this card may be dispatched again later with the same values.

Step 3. Output directories and the pod-uid prefixes. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
test ! -e "$DW" || { echo "STOP: wall output directory exists: $DW"; exit 1; }
mkdir -p "$DW/eks-results" "$DW/analysis"
if [ "$DO_COUNTED" = yes ]; then
  test ! -e "$DC" || { echo "STOP: counted output directory exists: $DC"; exit 1; }
  mkdir -p "$DC/eks-results" "$DC/analysis"
else
  mkdir -p "$ANC"
  cp "$DC"/analysis/compare-*.json "$ANC"/
  cp "$DC/analysis/pod-uid.txt" "$ANC/pod-uid.txt"
fi
if [ "$DO_COUNTED" = yes ]; then KINDS="wall counted"; else KINDS="wall"; fi
for K in $KINDS; do
  if [ "$K" = wall ]; then S3=$S3W; AN=$DW/analysis; else S3=$S3C; AN=$DC/analysis; fi
  aws s3 ls "$S3/" > "$AN/s3-prefixes.txt" || { echo "STOP: aws s3 ls failed under $S3/"; exit 1; }
  awk '$1=="PRE"{print $2}' "$AN/s3-prefixes.txt" | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$' > "$AN/pod-uid.txt" || true
  n=$(wc -l < "$AN/pod-uid.txt")
  echo "POD_UID_COUNT $K $n"
  [ "$n" -eq 1 ] || { echo "STOP: pod-uid prefix count of the $K job is $n, not 1"; exit 1; }
  echo "POD_UID $K $(cat "$AN/pod-uid.txt")"
done
echo STEP3_OK
```

The output must contain a `POD_UID_COUNT <kind> 1` line and a POD_UID line for each session processed in this dispatch, and the line STEP3_OK.

Step 4. Download and verify against SHA256SUMS. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
if [ "$DO_COUNTED" = yes ]; then KINDS="wall counted"; else KINDS="wall"; fi
for K in $KINDS; do
  if [ "$K" = wall ]; then S3=$S3W; DK=$DW; else S3=$S3C; DK=$DC; fi
  PODUID=$(tr -d '\n' < "$DK/analysis/pod-uid.txt")
  aws s3 cp --recursive --only-show-errors "$S3/$PODUID" "$DK/eks-results/"
  test -f "$DK/eks-results/SHA256SUMS" || { echo "STOP: SHA256SUMS absent from the $K copy"; exit 1; }
  ( cd "$DK/eks-results" && sha256sum -c SHA256SUMS > "$DK/analysis/sha256sum-check.txt" ) || { cat "$DK/analysis/sha256sum-check.txt"; echo "STOP: sha256sum -c failed for $K"; exit 1; }
  bad=$(grep -c -v ': OK$' "$DK/analysis/sha256sum-check.txt" || true)
  echo "SHA256_LINES $K $(wc -l < "$DK/analysis/sha256sum-check.txt") NOT_OK_LINES $bad"
  [ "$bad" -eq 0 ] || { echo "STOP: a SHA256SUMS line of the $K copy is not OK"; exit 1; }
  for F in job-exit.json audit.json l2-gate.json input-spec.json build-info.json control-comparison.json session/session.json session/records.jsonl; do
    if [ -f "$DK/eks-results/$F" ]; then echo "FILE_PRESENT $K $F yes"; else echo "FILE_PRESENT $K $F no"; fi
  done
  test -f "$DK/eks-results/session/session.json" || { echo "STOP: session/session.json absent from the $K copy"; exit 1; }
  test -f "$DK/eks-results/session/records.jsonl" || { echo "STOP: session/records.jsonl absent from the $K copy"; exit 1; }
done
df -m --output=avail /dev/shm | tail -n 1
echo STEP4_OK
```

The output must contain, for each session processed in this dispatch, a SHA256_LINES line ending in NOT_OK_LINES 0 and eight FILE_PRESENT lines, and the line STEP4_OK. (A `FILE_PRESENT ... no` for control-comparison.json is not a stop condition.)

Step 5. Facts of the copies (job exit, audit, L2 gate, build-info, binary). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
echo "FACTS_WALL"
python3 -I "$TOOLS/session_facts.py" "$DW/eks-results" "$DW/analysis" "$REV" "$FB" "$FBSHA" | tee "$DW/analysis/facts-stdout.txt"
if [ "$DO_COUNTED" = yes ]; then
  echo "FACTS_COUNTED"
  python3 -I "$TOOLS/session_facts.py" "$DC/eks-results" "$DC/analysis" "$REV" "$FB" "$FBSHA" | tee "$DC/analysis/facts-stdout.txt"
fi
echo STEP5_OK
```

The output must contain a line beginning FACTS_OK for each session processed in this dispatch, and the line STEP5_OK.

Step 6. The wall-session comparisons (5 pairs at 3 weights = 15 files). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
BIN=$(cat "$DW/analysis/binary-path.txt")
SESS="$DW/eks-results/session"
test -x "$BIN" || { echo "STOP: binary not executable: $BIN"; exit 1; }
echo "BINARY_USED $BIN"
for W in 0 0.1 1; do
  for P in $WALL_PAIRS; do
    A=${P%%:*}
    B2=${P##*:}
    "$BIN" compare --dir "$SESS" --a "$A" --b "$B2" --weight "$W" --json > "$DW/analysis/compare-${A}__${B2}-w${W}.json" 2> "$DW/analysis/compare-${A}__${B2}-w${W}.stderr" || { echo "STOP: compare failed for a=$A b=$B2 weight=$W (wall session)"; exit 1; }
    echo "COMPARE_OK wall a=$A b=$B2 weight=$W"
  done
done
n=$(ls "$DW"/analysis/compare-*.json | wc -l)
echo "COMPARE_JSON_COUNT wall $n"
[ "$n" -eq 15 ] || { echo "STOP: wall compare json count is $n, not 15"; exit 1; }
for F in "$DW"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP6_OK
```

The output must contain 15 lines beginning COMPARE_OK, the line `COMPARE_JSON_COUNT wall 15`, and the line STEP6_OK.

Step 7. The counted-session comparisons (3 pairs at 3 weights = 9 files). When COUNTED is `existing` this step runs the identical block and prints only the lines `COUNTED_STEP_SKIPPED` and STEP7_OK. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
if [ "$DO_COUNTED" = no ]; then echo COUNTED_STEP_SKIPPED; echo STEP7_OK; exit 0; fi
BIN=$(cat "$DC/analysis/binary-path.txt")
SESS="$DC/eks-results/session"
test -x "$BIN" || { echo "STOP: binary not executable: $BIN"; exit 1; }
echo "BINARY_USED $BIN"
for W in 0 0.1 1; do
  for P in $COUNTED_PAIRS; do
    A=${P%%:*}
    B2=${P##*:}
    "$BIN" compare --dir "$SESS" --a "$A" --b "$B2" --weight "$W" --json > "$DC/analysis/compare-${A}__${B2}-w${W}.json" 2> "$DC/analysis/compare-${A}__${B2}-w${W}.stderr" || { echo "STOP: compare failed for a=$A b=$B2 weight=$W (counted session)"; exit 1; }
    echo "COMPARE_OK counted a=$A b=$B2 weight=$W"
  done
done
n=$(ls "$DC"/analysis/compare-*.json | wc -l)
echo "COMPARE_JSON_COUNT counted $n"
[ "$n" -eq 9 ] || { echo "STOP: counted compare json count is $n, not 9"; exit 1; }
for F in "$DC"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP7_OK
```

The output must contain 9 lines beginning COMPARE_OK, the line `COMPARE_JSON_COUNT counted 9`, and the line STEP7_OK (or, for COUNTED `existing`, COUNTED_STEP_SKIPPED and STEP7_OK).

Step 8. The band tables of the sessions. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
BIN=$(cat "$DW/analysis/binary-path.txt")
"$BIN" band --dir "$DW/eks-results/session" --ranks > "$DW/analysis/band.md" 2> "$DW/analysis/band.stderr" || { echo "STOP: band failed (wall session)"; exit 1; }
echo "BAND_WALL_SESSION_BEGIN"
cat "$DW/analysis/band.md"
echo "BAND_WALL_SESSION_END"
if [ "$DO_COUNTED" = yes ]; then
  "$BIN" band --dir "$DC/eks-results/session" --ranks > "$DC/analysis/band.md" 2> "$DC/analysis/band.stderr" || { echo "STOP: band failed (counted session)"; exit 1; }
  echo "BAND_COUNTED_SESSION_BEGIN"
  cat "$DC/analysis/band.md"
  echo "BAND_COUNTED_SESSION_END"
fi
echo STEP8_OK
```

The output must contain the wall band block, the counted band block when COUNTED is `first` or `r2`, and the line STEP8_OK.

Step 9. The projection of the wall-session comparisons (written to a file; the lines below are what is printed). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/project_compare.py" "$DW/analysis" wall > "$DW/analysis/compare-projection.txt" || { cat "$DW/analysis/compare-projection.txt"; echo "STOP: project_compare failed (wall session)"; exit 1; }
sha256sum "$DW/analysis/compare-projection.txt"
wc -c "$DW/analysis/compare-projection.txt"
grep -c '^COMPARE_FILE ' "$DW/analysis/compare-projection.txt"
tail -n 1 "$DW/analysis/compare-projection.txt"
echo STEP9_OK
```

The output must contain a sha256sum line, a byte count, the number 15, the line PROJECT_COMPARE_OK, and the line STEP9_OK. The main session reads the file `compare-projection.txt`; do not print it.

Step 10. The projection of the counted-session comparisons (written to a file). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
if [ "$DO_COUNTED" = no ]; then echo COUNTED_STEP_SKIPPED; echo STEP10_OK; exit 0; fi
python3 -I "$TOOLS/project_compare.py" "$DC/analysis" counted > "$DC/analysis/compare-projection.txt" || { cat "$DC/analysis/compare-projection.txt"; echo "STOP: project_compare failed (counted session)"; exit 1; }
sha256sum "$DC/analysis/compare-projection.txt"
wc -c "$DC/analysis/compare-projection.txt"
grep -c '^COMPARE_FILE ' "$DC/analysis/compare-projection.txt"
tail -n 1 "$DC/analysis/compare-projection.txt"
echo STEP10_OK
```

The output must contain a sha256sum line, a byte count, the number 9, the line PROJECT_COMPARE_OK, and the line STEP10_OK (or COUNTED_STEP_SKIPPED and STEP10_OK). Do not print the file.

Step 11. The record-level facts of both sessions: wall condition 8 (records half), the `table_filter_bit_sets` requirements, the counted-equals-wall clause and condition 10 (mechanical facts; read the section "What the helper scripts do"). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/records_checks.py" "$DW/eks-results" "$DC/eks-results" "$DW/analysis" "$ANC" "$SPECW" "$SPECC" "$FB" | tee "$DW/analysis/records_checks-stdout.txt"
sha256sum "$DW/analysis/records_checks.json"
echo STEP11_OK
```

The output must contain the line RECORDS_CHECKS_OK, the lines R_COND8_RECORDS=, R_TFBS_REQUIREMENTS=, R_COUNTED_EQUALS_WALL=, R_COND10_WALL= and R_COND10_COUNTED= (each followed by true or false, either value is acceptable), the sha256sum line of records_checks.json, and the line STEP11_OK. The lines beginning COND8_RECORDS, TFBS_WALL, TFBS_COUNTED, COUNTED_VS_WALL, COND10_WALL and COND10_COUNTED are returned as printed.

Step 12. The bitop/v1 facts of both sessions (mechanical facts). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/work_checks.py" "$FB" "$DW/eks-results" "$DC/eks-results" "$DW/analysis" "$ANC" | tee "$DW/analysis/work_checks-stdout.txt"
sha256sum "$DW/analysis/work_checks.json"
echo STEP12_OK
```

The output must contain the line WORK_CHECKS_OK, a WORK_WALL line and a WORK_COUNTED line, the lines R_COND8_BITOPS= and R_COUNTED_BITOPS_IDENTITY= (each followed by true or false), the sha256sum line of work_checks.json, and the line STEP12_OK.

Step 13. The filter's bit sets in the section 5a band (mechanical facts). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/sprime.py" "$DW/eks-results" "$DC/eks-results" "$DW/analysis" "$ANC" | tee "$DW/analysis/sprime-stdout.txt"
sha256sum "$DW/analysis/sprime.json"
echo STEP13_OK
```

The output must contain the line SPRIME_OK, the SPRIME_* lines, the lines S_PRIME_EXACT_WALL= and S_PRIME_EXACT_COUNTED= (each followed by true or false), the sha256sum line of sprime.json, and the line STEP13_OK.

Step 14. The wall-session instrument facts and conditions 1 to 10 (mechanical facts; read the section "What the helper scripts do"). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/wall_gate.py" "$DW/eks-results" "$DW/analysis" "$SPECW" "$REV" "$FBSHA" | tee "$DW/analysis/wall_gate-stdout.txt"
sha256sum "$DW/analysis/wall_gate.json"
echo STEP14_OK
```

The output must contain the line WALL_GATE_OK, the lines beginning W_COND1= to W_COND10= (W_COND3_ROUNDED3= and W_COND8_ROUNDED3= included), W_ALL_1_TO_10=, W_ALL_1_TO_10_ROUNDED3=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= (each followed by true or false, either value is acceptable), six `AA ` lines, the sha256sum line of wall_gate.json, and the line STEP14_OK.

Step 15. The counted-session conditions and the exit-42 and exit-43 clauses (mechanical facts). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
if [ "$DO_COUNTED" = no ]; then echo COUNTED_STEP_SKIPPED; echo STEP15_OK; exit 0; fi
python3 -I "$TOOLS/counted_gate.py" "$DC/eks-results" "$DC/analysis" "$SPECC" "$REV" "$FBSHA" | tee "$DC/analysis/counted_gate-stdout.txt"
sha256sum "$DC/analysis/counted_gate.json"
echo STEP15_OK
```

The output must contain the line COUNTED_GATE_OK, one `AA ` line per weight (three), the lines beginning GATE_COND1= GATE_COND2= GATE_COND2_ROUNDED3= GATE_COND3= GATE_COND4= GATE_COND5= GATE_COND6= EXIT42_CLAUSE= and EXIT43_CLAUSE= (each followed by true or false, either value is acceptable), the sha256sum line of counted_gate.json, and the line STEP15_OK (or COUNTED_STEP_SKIPPED and STEP15_OK).

Step 16. Integer sums of the wall session's recorded times and the block positions (descriptive). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
python3 -I "$TOOLS/phase_sums.py" "$DW/eks-results" "$DW/analysis" | tee "$DW/analysis/phase_sums-stdout.txt"
python3 -I "$TOOLS/positions.py" "$DW/eks-results" "$DW/analysis" | tee "$DW/analysis/positions-stdout.txt"
echo STEP16_OK
```

The output must contain 4 lines beginning `PHASE_SUMS_ARM ` followed by an arm name, 8 lines beginning `PHASE_SUMS_ARM_CURVE `, the line PHASE_SUMS_OK, the lines beginning POSITIONS_ and PREDECESSORS_, the line POSITIONS_OK, and the line STEP16_OK.

Step 17. The receipts. Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
PODW=$(tr -d '\n' < "$DW/analysis/pod-uid.txt")
python3 -I "$TOOLS/receipt.py" "$DW" "$b" "$JW" dgb "$S3W" "$PODW" "$SPECW"
sha256sum "$DW/analysis/receipt.json"
if [ "$DO_COUNTED" = yes ]; then
  PODC=$(tr -d '\n' < "$DC/analysis/pod-uid.txt")
  python3 -I "$TOOLS/receipt.py" "$DC" "$b" "$JC" dgb "$S3C" "$PODC" "$SPECC"
  sha256sum "$DC/analysis/receipt.json"
fi
echo STEP17_OK
```

The output must contain a line beginning RECEIPT_OK and a sha256sum line for each session processed in this dispatch, and the line STEP17_OK.

Step 18. The recorded facts, projected (never print facts.json itself). Run:

```
b=__B__; wall=__WALL__; counted=__COUNTED__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/pf-analysis-tools/analysis-prelude.sh
echo "OUTPUT_DIR $D"
echo "FACTS_PROJECTION_WALL"
python3 -I "$TOOLS/project_facts.py" "$DW/analysis"
if [ "$DO_COUNTED" = yes ]; then
  echo "FACTS_PROJECTION_COUNTED"
  python3 -I "$TOOLS/project_facts.py" "$DC/analysis"
fi
echo STEP18_OK
```

The output must contain a line beginning FACTS_JSON_SHA256 and a line FACTS_PROJECTION_OK for each session processed in this dispatch, and the line STEP18_OK.

## Return (verbatim tool output, nothing else)

For steps 0 to 18: the output lines of each step exactly as printed, in step order, including all COMPARE_OK lines of steps 6 and 7, the band blocks of step 8, and all of steps 11 to 18. Steps 9 and 10 return only what they print (the files they write are read by the main session). No interpretation of any ratio, interval, verdict, audit field, gate field, or any R_*, W_*, GATE_*, EXIT4*_CLAUSE, CORRECTED_* or S_PRIME_* value; no summary; no recommendation; no other commands.

## Write scope

The executor writes only inside D (/dev/shm/src-pf/research/ic_packed_filter_20261029/P__B__ for the dispatch value of b), through the commands above. The Bash tool call output is the only other record. Write nowhere else; do not edit the repository, the helper files, the specs, the protocol directory or the S3 bucket; do not run git. The S3 commands only list and download.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly one bit size from 24, 28, 32, one WALL from first, r2, nr, nr-r2, one COUNTED from first, r2, existing, and CARD_SHA256.
- Step 0: the first field printed is not equal to CARD_SHA256 from the dispatch.
- Step 1 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Any output line begins with `STOP:`. This covers: the prelude refusing a value; an output directory already existing; less than 600 MB free; an S3 listing failing or finding nothing (the job has not finished or was not launched); a pod-uid prefix count other than 1; `job-exit.json` or `SHA256SUMS` absent from the S3 prefix; the sealing check of step 2 (a session that exited 42 whose trigger has not been evaluated, or whose noise rerun has not written a job-exit.json); a failed download; `sha256sum -c` exiting with a status other than 0 or any of its lines not ending in ": OK"; session/session.json or session/records.jsonl absent; the binary being absent or having another sha256; a compare, band, projection or helper exiting with a nonzero status; a compare JSON count other than 15 (wall) or 9 (counted); a compare file not parsing as JSON.
- Any step exits with a status other than 0 for a reason not printed as `STOP:`.
- Any step's required output (listed under that step) is missing.

A refusal in steps 0 to 2 wrote nothing, and the card may be dispatched again later. A refusal in step 3 or later leaves D as it is. After a refusal run nothing further: no retry, no edit, no cleanup, no deletion of D, no re-download, no substitute binary, no substitute job name. Each refusal is a specification_error back to the main session.

Not a stop condition, returned as printed: a job exit code of 42 or 43 or any non-zero code (for a session that passes step 2); `l2_gate.accepted` being false; `audit.ok` being false; a non-empty `problems` list; a replay detail other than "identical"; `FILE_PRESENT ... no`; a `false` value of any R_*, W_*, GATE_*, EXIT42_CLAUSE, EXIT43_CLAUSE, CORRECTED_* or S_PRIME_* line; an A/A ratio other than 1; a non-admitted wall block; `FACT_*` lines being false; a sha256 of `facts.json` or of any file the steps write. The executor copies these fields and does nothing with them.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task discovers a pod prefix at run time, loops over pair and weight lists, and tees to files, so the tool cannot express it. The cards of the previous studies (CARD-XP-20261029-dg-blocked-analysis and its predecessors) are plain cards for the same reason. There is no runbook, so `tools/runbook.py lint` does not apply.

## Note

The S3 results of a launch-remote job hold `binary.sha256` and `build-info.json`, which record the ecbench hash, not the binary. The binary for compare, band and work is therefore the bundle at /dev/shm/pf-bundle/ecbench, which facts record as `binary_used_sha256_equals_build_info_ecbench_sha256`; a false value is returned as printed and is not a stop condition. /dev/shm is memory-backed: D is lost if the machine restarts. D lies inside the protocol worktree, so a later checkpoint of the worktree may bundle it; the executor never runs git. The comparisons of the first session after a noise rerun (WALL `first`, COUNTED `existing`) are descriptive and are never pooled with the rerun (PROTOCOL.md section 4).
