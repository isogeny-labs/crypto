# Mechanical task card CARD-XP-20261029-dg-wall-analysis

author_role: experiment-director
executor_role: executor-mechanical
experiment: dg-wall (protocol backlog/XP-20261029-dg-wall/PROTOCOL.md; one analysis task per bit size b; not an EXP-* record)
kind: plain frozen analysis card (no launch step: the jobs were launched with CARD-XP-20261029-dg-wall-launch; not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol with edits E1-E11 (APPROVAL.md in the protocol directory); the main (reasoning-tier) session decided the route. This card adds no authority and decides nothing.

## Parameters (the dispatch message gives exactly two values: the bit size b and CARD_SHA256; no other value is accepted)

- b is exactly one of 24, 28 or 32. In every command below the word `__B__` stands for the dispatch value of b. Type the dispatch value in place of `__B__` and change nothing else. The sourced prelude refuses any other value of b.
- CARD_SHA256 is used only in step 0.

| b | wall job (spec-P__B__.json) | counted job (spec-P__B__-rho.json) |
|--:|:--|:--|
| 24 | ic-dgw-p24-20261029 | ic-dgw-rho-p24-20261029 |
| 28 | ic-dgw-p28-20261029 | ic-dgw-rho-p28-20261029 |
| 32 | ic-dgw-p32-20261029 | ic-dgw-rho-p32-20261029 |

The job names are fixed by the prelude from b. A job rerun under the suffix `-r2` has another name and needs another card.

## Frozen values

- Protocol freeze commit: 6e4804417baa031f6a1aee9b9b54c67c90c4dafe. Protocol directory PD = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall (its files are bound by sha256 in step 1).
- Source revision that built the ecbench: be2969cebf30a0cfb79088fd10094324c3efb3bb. Image: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c.
- S3 results prefix of a job: `s3://isolab-results-104186811093-us-east-2/runs/<job name>/` followed by exactly one pod-uid prefix (a UUID). The prefix named `launcher/` is never a pod prefix and is not downloaded.
- Output directory D = /dev/shm/dgw-analysis/P__B__. Subdirectories: `wall/eks-results` and `wall/analysis` (the wall session), `counted/eks-results` and `counted/analysis` (the counted session). D must not exist before step 3.
- Helper files, all bound by sha256 in step 1, in TOOLS = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools: prelude.sh, dgw_facts.py, project_compare.py, wall_gate.py, counted_gate.py, e6.py, phase_sums.py, receipt.py, project_facts.py. (receipt.py and project_facts.py are byte-identical to the v3 analysis card's.) They take every path as an argument.
- Binary for compare and band: /dev/shm/isolab-build/ecbench, sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c. It is the image's ecbench. No other binary is used. Every compare runs at its default `--resamples` and `--seed` (nothing is passed for either).
- AWS: the executor's existing identity. Never print, write or copy credentials. The prelude sets PATH to start with /tmp/claude-0/awsvenv/bin, AWS_DEFAULT_REGION=us-east-2 and AWS_REGION=us-east-2.
- Arms. Wall session (5): walk (reference), srenew (baseline), dg (candidate), srenew-aa (control), dg-small (control). Counted session (4): rho-neg (reference), srenew (baseline), dg (candidate), rho-neg-aa (control).
- Pairs. A pair written A:B is run as `compare --a A --b B`, so the ratio is B over A.
  - Wall session, 6 pairs: srenew:dg (the decision pair of PROTOCOL.md section 5), srenew:srenew-aa, walk:srenew, walk:dg, walk:dg-small, srenew:dg-small. These are the 6 pairs of PROTOCOL.md section 5 "wall session" (the last 4 are the descriptive wall comparisons).
  - Counted session, 3 pairs: rho-neg:rho-neg-aa, rho-neg:srenew, rho-neg:dg. For b = 28 only, a 4th pair srenew:dg is added, for the per-curve conditions of step 13.
- Weights (GAE per unpriced native operation): 0, 0.1 and 1. Every pair is run at each weight with `--weight W --json`. The wall block of a compare reads charged GAE and does not depend on the weight.
- Pinned sha256:
  - PROTOCOL.md a20b5faf74e467e7c8f27cec796575af0c993d07f139d7cd16402efd787d2400
  - APPROVAL.md 50b82fa9d6f8c854bddeb6f135267bd9c3f6a579cd070f21f9a3196e9984b5ce
  - PRELAUNCH-CHECKS.md 8430fc483aa71a8d2ddbc828734a5abd383407f8297d293dfd22a95eabce8172
  - spec-P24.json 6b91d8241e1849c0fb45deca799b53573102f963f5a47627345d651371ede60b
  - spec-P24-rho.json 2c6474e7052acdcad67cc1c61700edeffdc3d4d8a059c5a9dbd614bca60f0859
  - spec-P28.json 83d5a397fa31cc5c28c2cab78949a4c1b7008a2546b333500ce4ac2b9c3feb21
  - spec-P28-rho.json b80f9bf83f7ab3341ac803d9f4d3d998b54f38cf0c608ace6a290367fb1475b3
  - spec-P32.json fd04a41d627ebf75d285d6bee23b4f8f87c014e46aef7372d7fcd8c467fb65d8
  - spec-P32-rho.json 619919448b528234836b4aa72952d65e8be1da5f0b7fe109778a4e1fb1382b37
  - prelude.sh 24fd05dd017ffe1b08dc20785a0b3af2314624e1e027da85bf887f3ef88d7136
  - dgw_facts.py baa3a23c4dd13e2c49ec0acb50cd6d9fd30f9c2ae703d6cedfff2972c2bbff0b
  - project_compare.py 95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab
  - wall_gate.py fb140465d012061b313057e7531fc3655a93f47ab97e125de5265f1dbc1a1d6e
  - counted_gate.py 017b3fad09d69b6d20bfb7bb38ded6d97f3dbc7ac44a802a1736cb21a7dcc3f8
  - e6.py ce41c08d279bd0a7082423a8806723fab2f76237a858fc6fed5c7b77f9035943
  - phase_sums.py a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1
  - receipt.py d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef
  - project_facts.py edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091
  - ecbench 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c

## What the helper scripts do (a description; the executor does nothing with it)

- dgw_facts.py reads the copy of a session's result files and records facts (build-info, audit, l2-gate, job-exit, input-spec/spec, binary) in `analysis/facts.json`. It never judges them.
- project_compare.py prints a fixed key projection of every compare JSON (the full JSON stays in `analysis/`). For the counted session it prints no field of any `wall` block.
- wall_gate.py reads files and compares values. It prints the facts behind each of the conditions 1 to 8 of PROTOCOL.md section 6.1 (wall session, as edited by APPROVAL.md E5), and the lines W_COND1= to W_COND8=, W_ALL_1_TO_8=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= as true or false by the literal wording. These lines are mechanical facts. The executor does not interpret them: a value of false is not a stop condition and is returned as printed.
- counted_gate.py does the same for the counted session: the lines GATE_COND1= to GATE_COND6=, EXIT42_CLAUSE= and EXIT43_CLAUSE=, the exit-42 clause recomputed from `l2-gate.json` and independently from `session/records.jsonl`. It reads no wall field of a record.
- e6.py prints the per-curve conditions 2, 3 and 4 of the 28-bit consequence (PROTOCOL.md section 6.2 as edited by E6) and, for the wall session, condition 1 from wall_gate.json. For b other than 28 it prints E6_NOT_APPLICABLE.
- phase_sums.py prints integer sums of the wall session's recorded times (PROTOCOL.md section 7 asks for the online windows, descriptively). It divides nothing.

## Procedure

Use one Bash call per step. Paste each command block exactly as written, with only the `__B__` replacement. A line that begins with `STOP:` in any output is a refusal (see "Stop and return refused"). Change no file in the repository, none of the helper files, and no file outside D. Do not open any other file: in particular nothing under research/ic_srenew_deferred_guard_20261029 and no S3 prefix other than the two named by the prelude (PROTOCOL.md section 8 seals the earlier sessions).

Step 0. Card digest. Run:

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw/card-analysis.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the word `__B__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Hashes of the protocol files, specs, helper files and binary. Run:

```
sha256sum -c - <<'HASHES'
a20b5faf74e467e7c8f27cec796575af0c993d07f139d7cd16402efd787d2400  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/PROTOCOL.md
50b82fa9d6f8c854bddeb6f135267bd9c3f6a579cd070f21f9a3196e9984b5ce  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/APPROVAL.md
8430fc483aa71a8d2ddbc828734a5abd383407f8297d293dfd22a95eabce8172  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/PRELAUNCH-CHECKS.md
6b91d8241e1849c0fb45deca799b53573102f963f5a47627345d651371ede60b  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P24.json
2c6474e7052acdcad67cc1c61700edeffdc3d4d8a059c5a9dbd614bca60f0859  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P24-rho.json
83d5a397fa31cc5c28c2cab78949a4c1b7008a2546b333500ce4ac2b9c3feb21  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P28.json
b80f9bf83f7ab3341ac803d9f4d3d998b54f38cf0c608ace6a290367fb1475b3  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P28-rho.json
fd04a41d627ebf75d285d6bee23b4f8f87c014e46aef7372d7fcd8c467fb65d8  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P32.json
619919448b528234836b4aa72952d65e8be1da5f0b7fe109778a4e1fb1382b37  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/backlog/XP-20261029-dg-wall/spec-P32-rho.json
24fd05dd017ffe1b08dc20785a0b3af2314624e1e027da85bf887f3ef88d7136  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
baa3a23c4dd13e2c49ec0acb50cd6d9fd30f9c2ae703d6cedfff2972c2bbff0b  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/dgw_facts.py
95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/project_compare.py
fb140465d012061b313057e7531fc3655a93f47ab97e125de5265f1dbc1a1d6e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/wall_gate.py
017b3fad09d69b6d20bfb7bb38ded6d97f3dbc7ac44a802a1736cb21a7dcc3f8  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/counted_gate.py
ce41c08d279bd0a7082423a8806723fab2f76237a858fc6fed5c7b77f9035943  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/e6.py
a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/phase_sums.py
d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/receipt.py
edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/project_facts.py
88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c  /dev/shm/isolab-build/ecbench
HASHES
```

The exit status must be 0 and all 19 output lines must end with ": OK". (The here-document ends only at the line that is exactly HASHES.)

Step 2. Readiness check (reads only; writes nothing). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
test ! -e "$D" || { echo "STOP: output directory exists: $D"; exit 1; }
avail=$(df -m --output=avail /dev/shm | tail -n 1 | tr -d ' ')
echo "DF_AVAIL_MB $avail"
[ "$avail" -ge 600 ] || { echo "STOP: less than 600 MB free on /dev/shm"; exit 1; }
for K in wall counted; do
  if [ "$K" = wall ]; then S3=$S3W; else S3=$S3C; fi
  LST=$(aws s3 ls "$S3/") || { echo "STOP: aws s3 ls failed or found nothing under $S3/ ($K job not finished or not launched)"; exit 1; }
  echo "S3_LISTING $K $S3/"
  echo "$LST"
  UIDS=$(echo "$LST" | awk '$1=="PRE"{print $2}' | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$' || true)
  n=$(echo "$UIDS" | grep -c . || true)
  echo "POD_UID_COUNT $K $n"
  [ "$n" -eq 1 ] || { echo "STOP: pod-uid prefix count of the $K job is $n, not 1"; exit 1; }
  aws s3 ls "$S3/$UIDS" > /dev/null || { echo "STOP: nothing under $S3/$UIDS"; exit 1; }
  aws s3 ls "$S3/${UIDS}job-exit.json" > /dev/null || { echo "STOP: job-exit.json absent under $S3/$UIDS"; exit 1; }
  aws s3 ls "$S3/${UIDS}SHA256SUMS" > /dev/null || { echo "STOP: SHA256SUMS absent under $S3/$UIDS"; exit 1; }
done
echo STEP2_OK
```

The output must contain a DF_AVAIL_MB line with a number of at least 600, two S3_LISTING blocks, `POD_UID_COUNT wall 1`, `POD_UID_COUNT counted 1`, and the line STEP2_OK. A refusal in steps 0 to 2 has written nothing, so this card may be dispatched again later with the same values.

Step 3. Output directory and the pod-uid prefixes. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
test ! -e "$D" || { echo "STOP: output directory exists: $D"; exit 1; }
mkdir -p "$DW/eks-results" "$DW/analysis" "$DC/eks-results" "$DC/analysis"
for K in wall counted; do
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

The output must contain `POD_UID_COUNT wall 1`, `POD_UID_COUNT counted 1`, one POD_UID line for each, and the line STEP3_OK.

Step 4. Download and verify against SHA256SUMS. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
for K in wall counted; do
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

The output must contain, for each of wall and counted, a SHA256_LINES line ending in NOT_OK_LINES 0 and eight FILE_PRESENT lines, and the line STEP4_OK. (A `FILE_PRESENT ... no` for control-comparison.json is not a stop condition.)

Step 5. Facts of the two copies (job exit, audit, L2 gate, build-info, binary). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
echo "FACTS_WALL"
python3 -I "$TOOLS/dgw_facts.py" "$DW/eks-results" "$DW/analysis" "$REV" "$FB" "$FBSHA" | tee "$DW/analysis/facts-stdout.txt"
echo "FACTS_COUNTED"
python3 -I "$TOOLS/dgw_facts.py" "$DC/eks-results" "$DC/analysis" "$REV" "$FB" "$FBSHA" | tee "$DC/analysis/facts-stdout.txt"
echo STEP5_OK
```

The output must contain two lines beginning FACTS_OK and the line STEP5_OK.

Step 6. The wall-session comparisons (6 pairs at 3 weights = 18 files). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
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
[ "$n" -eq 18 ] || { echo "STOP: wall compare json count is $n, not 18"; exit 1; }
for F in "$DW"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP6_OK
```

The output must contain 18 lines beginning COMPARE_OK, the line `COMPARE_JSON_COUNT wall 18`, and the line STEP6_OK.

Step 7. The counted-session comparisons (3 pairs at 3 weights = 9 files; for b = 28, 4 pairs = 12 files). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
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
if [ "$b" = 28 ]; then want=12; else want=9; fi
[ "$n" -eq "$want" ] || { echo "STOP: counted compare json count is $n, not $want"; exit 1; }
for F in "$DC"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP7_OK
```

The output must contain as many lines beginning COMPARE_OK as the count (9, or 12 for b = 28), the line `COMPARE_JSON_COUNT counted 9` (or `12` for b = 28), and the line STEP7_OK.

Step 8. The band tables of the two sessions. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
BIN=$(cat "$DW/analysis/binary-path.txt")
"$BIN" band --dir "$DW/eks-results/session" --ranks > "$DW/analysis/band.md" 2> "$DW/analysis/band.stderr" || { echo "STOP: band failed (wall session)"; exit 1; }
"$BIN" band --dir "$DC/eks-results/session" --ranks > "$DC/analysis/band.md" 2> "$DC/analysis/band.stderr" || { echo "STOP: band failed (counted session)"; exit 1; }
echo "BAND_WALL_SESSION_BEGIN"
cat "$DW/analysis/band.md"
echo "BAND_WALL_SESSION_END"
echo "BAND_COUNTED_SESSION_BEGIN"
cat "$DC/analysis/band.md"
echo "BAND_COUNTED_SESSION_END"
echo STEP8_OK
```

The output must contain both band blocks and the line STEP8_OK.

Step 9. The projection of the wall-session comparisons (written to a file; the lines below are what is printed). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
python3 -I "$TOOLS/project_compare.py" "$DW/analysis" wall > "$DW/analysis/compare-projection.txt" || { cat "$DW/analysis/compare-projection.txt"; echo "STOP: project_compare failed (wall session)"; exit 1; }
sha256sum "$DW/analysis/compare-projection.txt"
wc -c "$DW/analysis/compare-projection.txt"
grep -c '^COMPARE_FILE ' "$DW/analysis/compare-projection.txt"
tail -n 1 "$DW/analysis/compare-projection.txt"
echo STEP9_OK
```

The output must contain a sha256sum line, a byte count, the number 18, the line PROJECT_COMPARE_OK, and the line STEP9_OK. The main session reads the file `compare-projection.txt`; do not print it.

Step 10. The projection of the counted-session comparisons (written to a file). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
python3 -I "$TOOLS/project_compare.py" "$DC/analysis" counted > "$DC/analysis/compare-projection.txt" || { cat "$DC/analysis/compare-projection.txt"; echo "STOP: project_compare failed (counted session)"; exit 1; }
sha256sum "$DC/analysis/compare-projection.txt"
wc -c "$DC/analysis/compare-projection.txt"
grep -c '^COMPARE_FILE ' "$DC/analysis/compare-projection.txt"
tail -n 1 "$DC/analysis/compare-projection.txt"
echo STEP10_OK
```

The output must contain a sha256sum line, a byte count, the number 9 (12 for b = 28), the line PROJECT_COMPARE_OK, and the line STEP10_OK. Do not print the file.

Step 11. The wall-session instrument facts and conditions 1 to 8 (mechanical facts; read the section "What the helper scripts do"). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
python3 -I "$TOOLS/wall_gate.py" "$DW/eks-results" "$DW/analysis" "$SPECW" "$REV" "$FBSHA" | tee "$DW/analysis/wall_gate-stdout.txt"
sha256sum "$DW/analysis/wall_gate.json"
echo STEP11_OK
```

The output must contain the line WALL_GATE_OK, the lines beginning W_COND1= to W_COND8=, W_ALL_1_TO_8=, W_ALL_1_TO_8_ROUNDED3=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= (each followed by true or false, either value is acceptable), three `AA ` lines, the sha256sum line of wall_gate.json, and the line STEP11_OK.

Step 12. The counted-session conditions and the exit-42 and exit-43 clauses (mechanical facts). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
python3 -I "$TOOLS/counted_gate.py" "$DC/eks-results" "$DC/analysis" "$SPECC" "$REV" "$FBSHA" | tee "$DC/analysis/counted_gate-stdout.txt"
sha256sum "$DC/analysis/counted_gate.json"
echo STEP12_OK
```

The output must contain the line COUNTED_GATE_OK, one `AA ` line per weight (three), the lines beginning GATE_COND1= GATE_COND2= GATE_COND2_ROUNDED3= GATE_COND3= GATE_COND4= GATE_COND5= GATE_COND6= EXIT42_CLAUSE= and EXIT43_CLAUSE= (each followed by true or false, either value is acceptable), the sha256sum line of counted_gate.json, and the line STEP12_OK.

Step 13. The per-curve conditions 1 to 4 of the 28-bit consequence (printed as E6_NOT_APPLICABLE for b other than 28). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
echo "E6_WALL_SESSION"
python3 -I "$TOOLS/e6.py" wall "$DW/eks-results" "$DW/analysis" "$b" | tee "$DW/analysis/e6-stdout.txt"
echo "E6_COUNTED_SESSION"
python3 -I "$TOOLS/e6.py" counted "$DC/eks-results" "$DC/analysis" "$b" | tee "$DC/analysis/e6-stdout.txt"
echo STEP13_OK
```

The output must contain two lines `E6_OK` and the line STEP13_OK. For b = 28 it also contains E6_COND2_ALL_CURVES= E6_COND3_ALL_CURVES= E6_COND4_ALL_CURVES= in each block, and E6_COND1= and E6_ALL_FOUR= in the wall block (each followed by true or false, either value is acceptable). For b other than 28 it contains two E6_NOT_APPLICABLE lines.

Step 14. Integer sums of the wall session's recorded times (descriptive). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
python3 -I "$TOOLS/phase_sums.py" "$DW/eks-results" "$DW/analysis" | tee "$DW/analysis/phase_sums-stdout.txt"
echo STEP14_OK
```

The output must contain 5 lines beginning `PHASE_SUMS_ARM ` followed by an arm name, 10 lines beginning `PHASE_SUMS_ARM_CURVE `, the line PHASE_SUMS_OK, and the line STEP14_OK.

Step 15. The receipts. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
PODW=$(tr -d '\n' < "$DW/analysis/pod-uid.txt")
PODC=$(tr -d '\n' < "$DC/analysis/pod-uid.txt")
python3 -I "$TOOLS/receipt.py" "$DW" "$b" "$JW" srenew "$S3W" "$PODW" "$SPECW"
sha256sum "$DW/analysis/receipt.json"
python3 -I "$TOOLS/receipt.py" "$DC" "$b" "$JC" srenew "$S3C" "$PODC" "$SPECC"
sha256sum "$DC/analysis/receipt.json"
echo STEP15_OK
```

The output must contain two lines beginning RECEIPT_OK, two sha256sum lines, and the line STEP15_OK.

Step 16. The recorded facts, projected (never print facts.json itself). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgw-analysis-tools/prelude.sh
echo "OUTPUT_DIR $D"
echo "FACTS_PROJECTION_WALL"
python3 -I "$TOOLS/project_facts.py" "$DW/analysis"
echo "FACTS_PROJECTION_COUNTED"
python3 -I "$TOOLS/project_facts.py" "$DC/analysis"
echo STEP16_OK
```

The output must contain two lines beginning FACTS_JSON_SHA256, two lines FACTS_PROJECTION_OK, and the line STEP16_OK.

## Return (verbatim tool output, nothing else)

For steps 0 to 16: the output lines of each step exactly as printed, in step order, including all COMPARE_OK lines of steps 6 and 7, both band blocks of step 8, and all of steps 11 to 16. Steps 9 and 10 return only what they print (the files they write are read by the main session). No interpretation of any ratio, interval, verdict, audit field, gate field, or any W_*, GATE_*, EXIT4*_CLAUSE, CORRECTED_* or E6_* value; no summary; no recommendation; no other commands.

## Write scope

The executor writes only inside D (/dev/shm/dgw-analysis/P__B__ for the dispatch value of b), through the commands above. The Bash tool call output is the only other record. Write nowhere else; do not edit the repository, the helper files, the specs, the protocol directory or the S3 bucket. The S3 commands only list and download.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly one bit size from 24, 28, 32 and CARD_SHA256.
- Step 0: the first field printed is not equal to CARD_SHA256 from the dispatch.
- Step 1 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Any output line begins with `STOP:`. This covers: the prelude refusing b; D already existing; less than 600 MB free; an S3 listing failing or finding nothing (the job has not finished or was not launched); a pod-uid prefix count other than 1; `job-exit.json` or `SHA256SUMS` absent from the S3 prefix; a failed download; `sha256sum -c` exiting with a status other than 0 or any of its lines not ending in ": OK"; session/session.json or session/records.jsonl absent; the binary being absent or having another sha256; a compare, band, projection or helper exiting with a nonzero status; a compare JSON count other than 18 (wall) or 9 (counted; 12 for b = 28); a compare file not parsing as JSON.
- Any step exits with a status other than 0 for a reason not printed as `STOP:`.
- Any step's required output (listed under that step) is missing.

A refusal in steps 0 to 2 wrote nothing, and the card may be dispatched again later. A refusal in step 3 or later leaves D as it is. After a refusal run nothing further: no retry, no edit, no cleanup, no deletion of D, no re-download, no substitute binary, no substitute job name. Each refusal is a specification_error back to the main session.

Not a stop condition, returned as printed: a job exit code of 42 or 43 or any non-zero code; `l2_gate.accepted` being false; `audit.ok` being false; a non-empty `problems` list; a replay detail other than "identical"; `FILE_PRESENT ... no`; a `false` value of any W_*, GATE_*, EXIT42_CLAUSE, EXIT43_CLAUSE, CORRECTED_* or E6_* line; an A/A ratio other than 1; a non-admitted wall block; `FACT_*` lines being false; a sha256 of `facts.json` or of any analysis file. The executor copies these fields and does nothing with them.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task discovers a pod prefix at run time, loops over pair and weight lists, and tees to files, so the tool cannot express it. Cards in this study (CARD-XP-20261029-dg-wall-launch) and its predecessors (the v3 analysis card) are plain cards for the same reason. There is no runbook, so `tools/runbook.py lint` does not apply.

## Note

The S3 results of a launch-remote job hold `binary.sha256` and `build-info.json`, which record the ecbench hash, not the binary. The binary for compare and band is therefore the build in /dev/shm/isolab-build, which facts record as `binary_used_sha256_equals_build_info_ecbench_sha256`; a false value is returned as printed and is not a stop condition. /dev/shm is memory-backed: D is lost if the machine restarts.
