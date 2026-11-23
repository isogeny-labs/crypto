# Mechanical task card CARD-XP-20261029-dg-blocked-analysis

author_role: experiment-director
executor_role: executor-mechanical
experiment: dg-blocked (protocol backlog/XP-20261029-dg-blocked/PROTOCOL.md; one analysis task per bit size b; not an EXP-* record)
kind: plain frozen analysis card (no launch step: the jobs were launched with CARD-XP-20261029-dg-blocked-launch; not `tools/task_card.py` frozen_command; see "Why a plain card")
approval: the Coordinator approved the protocol with edits E1-E10 (APPROVAL.md in the protocol directory) and ruled on pre-launch check P2 (RULING-P2.md, which adds condition A1); the main (reasoning-tier) session decided the route. This card adds no authority and decides nothing.

## Parameters (the dispatch message gives exactly two values: the bit size b and CARD_SHA256; no other value is accepted)

- b is exactly one of 24, 28 or 32. In every command below the word `__B__` stands for the dispatch value of b. Type the dispatch value in place of `__B__` and change nothing else. The sourced prelude refuses any other value of b.
- CARD_SHA256 is used only in step 0.

| b | wall job (spec-P__B__.json) | counted job (spec-P__B__-rho.json) |
|--:|:--|:--|
| 24 | ic-dgb-p24-20261029 | ic-dgb-rho-p24-20261029 |
| 28 | ic-dgb-p28-20261029 | ic-dgb-rho-p28-20261029 |
| 32 | ic-dgb-p32-20261029 | ic-dgb-rho-p32-20261029 |

The job names are fixed by the prelude from b. A job rerun under the suffix `-r2` has another name and needs another card.

## Frozen values

- Protocol freeze commit: cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae. Protocol directory PD = /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked (its files are bound by sha256 in step 1). The freeze commit message records the six spec sha256 (pinned again below).
- Coordinator ruling on pre-launch check P2: research/ic_dg_blocked_20261029/cards/RULING-P2.md, sha256 aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d (committed at df274b2aa3). It adds condition A1 (packed layout) to the wall session's section 6.1 as a tenth condition and to the counted session's dgb records. This card prints the facts behind A1 and nothing else about it.
- Source revision that built the ecbench: be2969cebf30a0cfb79088fd10094324c3efb3bb. Image: 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c.
- S3 results prefix of a job: `s3://isolab-results-104186811093-us-east-2/runs/<job name>/` followed by exactly one pod-uid prefix (a UUID). The prefix named `launcher/` is never a pod prefix and is not downloaded.
- Output directory D = /dev/shm/wt-blk/research/ic_dg_blocked_20261029/P__B__ (the committed place for this study's outputs). Subdirectories: `wall/eks-results` and `wall/analysis` (the wall session), `counted/eks-results` and `counted/analysis` (the counted session). D must not exist before step 3.
- Helper files, all bound by sha256 in step 1, in TOOLS = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools: prelude.sh, dgb_facts.py, project_compare.py, records_checks.py, wall_gate.py, counted_gate.py, phase_sums.py, receipt.py, project_facts.py. They take every path as an argument.
- Binary for compare and band: /dev/shm/isolab-build/ecbench, sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c. It is the image's ecbench. No other binary is used. Every compare runs at its default `--resamples` and `--seed` (nothing is passed for either).
- AWS: the executor's existing identity. Never print, write or copy credentials. The prelude sets PATH to start with /tmp/claude-0/awsvenv/bin, AWS_DEFAULT_REGION=us-east-2 and AWS_REGION=us-east-2.
- Arms. Wall session (4): dg (baseline), dg-aa (control), dgb (candidate), dgb-edge (reference). Counted session (4): rho-neg (reference), dg (baseline), dgb (candidate), rho-neg-aa (control).
- Pairs. A pair written A:B is run as `compare --a A --b B`, so the ratio is B over A.
  - Wall session, 5 pairs, the pairs of PROTOCOL.md section 5: dg:dgb (the decision pair), dg:dg-aa, dg-aa:dgb, dg:dgb-edge (the position control), dgb-edge:dgb.
  - Counted session, 3 pairs: rho-neg:rho-neg-aa, rho-neg:dg, rho-neg:dgb.
- Weights (GAE per unpriced native operation): 0, 0.1 and 1. Every pair is run at each weight with `--weight W --json`. The wall block of a compare reads charged GAE and does not depend on the weight.
- Pinned sha256:
  - PROTOCOL.md 4b55710d92c10e1aa9b5a0f1665371b20d2dbd32f667d213383bfa960096898a
  - APPROVAL.md f0af35a925faa73953d973e831fef0225e64a11a342c63569a3e6ab33c73074e
  - spec-P24.json 62267dc13d78ff7fbbf285933dc25581fa1de207464a3377bac4200441e3b54c
  - spec-P24-rho.json f1e1a8081f58a57e2c6bfe6041cab9597c5aa16a4f6b03a0da981e511c01e669
  - spec-P28.json 8f1908c39e46344f3d1a4edc64d4a50d1e3ef7a6eba04c8fb368ef9e154a68f6
  - spec-P28-rho.json 3e69c4f9501b4d43291492da12bae551811d1bf54659f414c2cd6a81315cbfba
  - spec-P32.json 1e0f5fb9979e8916b625594a868f54fb201fa85a0161027ec0c83c12e39ba120
  - spec-P32-rho.json 9a23e873c9fd2d852601d691a9a89c19ef9d9116ccddcf70eb4f12b7167572e5
  - cards/prelaunch-checks.md a66af69491204ebd98154ce022220179f1d3afd67d79a8b269df2fd7bb4b6ae1
  - cards/RULING-P2.md aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d
  - prelude.sh cb6e6d7603ec2f8ad068f0d9a992f01e1fe629a79747acaa68353f274f49ab97
  - dgb_facts.py e30ad4b55f91aee25306c4ee23793bb5394db9691f870a469b14828dac2c2fee
  - project_compare.py 95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab
  - records_checks.py 97509ca78dacd94d4f1c760527b3307cb779685a5580d34a54188478e9d5ad04
  - wall_gate.py f67bf6caeddc4d2a964fa65252de887f635bac39b4d7b2e0375ced5949805c9b
  - counted_gate.py 0dd45ebddbd77ccd6ad77162d04f3066093044a4e0242dafdf3eaa8f8d39f08e
  - phase_sums.py a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1
  - receipt.py d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef
  - project_facts.py edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091
  - ecbench 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c

## What the helper scripts do (a description; the executor does nothing with it)

- dgb_facts.py reads the copy of a session's result files and records facts (build-info, audit, l2-gate, job-exit, input-spec/spec, binary) in `analysis/facts.json`. It never judges them.
- project_compare.py prints a fixed key projection of every compare JSON (the full JSON stays in `analysis/`). For the counted session it prints no field of any `wall` block.
- records_checks.py reads `records.jsonl` of both sessions and prints the record-level facts: the record half of wall condition 8 (each dgb and dgb-edge record, warm-ups included, against the dg record of the same curve, target and round: recovered logarithm, total_gae, the list of unpriced counters, every unpriced-operation count as ecbench band's `unpriced_counts` finds it, and per phase the name, adds, doubles, scalar_mults and gae), the counted-session clause (the counted dg and dgb records against the wall session's), and the facts behind condition A1 (p from the ICV1 curve string, signed points, and which source gave the signed points). Native maps of the phases are compared and printed as descriptive lines that belong to no condition. It reads no wall field and computes no statistic. It prints R_COND8_RECORDS=, R_COUNTED_EQUALS_WALL=, R_A1_WALL= and R_A1_COUNTED= as true or false by the literal wording.
- wall_gate.py reads files and compares values. It prints the facts behind the conditions 1 to 9 of PROTOCOL.md section 6.1 (wall session, as edited by APPROVAL.md E4) and A1, and the lines W_COND1= to W_COND9=, W_A1=, W_ALL_1_TO_9=, W_ALL_1_TO_9_AND_A1=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= as true or false by the literal wording. These lines are mechanical facts. The executor does not interpret them: a value of false is not a stop condition and is returned as printed.
- counted_gate.py does the same for the counted session: the lines GATE_COND1= to GATE_COND6=, EXIT42_CLAUSE= and EXIT43_CLAUSE=, the exit-42 clause recomputed from `l2-gate.json` and independently from `session/records.jsonl`. It reads no wall field of a record.
- phase_sums.py prints integer sums of the wall session's recorded times (PROTOCOL.md section 7 asks for the online windows, descriptively). It divides nothing.

## Procedure

Use one Bash call per step. Paste each command block exactly as written, with only the `__B__` replacement. A line that begins with `STOP:` in any output is a refusal (see "Stop and return refused"). Change no file in the repository, none of the helper files, and no file outside D. Do not open any other file: in particular nothing under research/ic_table_floor_20261029 and no S3 prefix other than the two named by the prelude (PROTOCOL.md section 8 seals the PR #2844 micro-benchmark until the verdict commit).

Step 0. Card digest. Run:

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb/card-analysis.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the word `__B__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Hashes of the protocol files, specs, ruling, helper files and binary. Run:

```
sha256sum -c - <<'HASHES'
4b55710d92c10e1aa9b5a0f1665371b20d2dbd32f667d213383bfa960096898a  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/PROTOCOL.md
f0af35a925faa73953d973e831fef0225e64a11a342c63569a3e6ab33c73074e  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/APPROVAL.md
62267dc13d78ff7fbbf285933dc25581fa1de207464a3377bac4200441e3b54c  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P24.json
f1e1a8081f58a57e2c6bfe6041cab9597c5aa16a4f6b03a0da981e511c01e669  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P24-rho.json
8f1908c39e46344f3d1a4edc64d4a50d1e3ef7a6eba04c8fb368ef9e154a68f6  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P28.json
3e69c4f9501b4d43291492da12bae551811d1bf54659f414c2cd6a81315cbfba  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P28-rho.json
1e0f5fb9979e8916b625594a868f54fb201fa85a0161027ec0c83c12e39ba120  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P32.json
9a23e873c9fd2d852601d691a9a89c19ef9d9116ccddcf70eb4f12b7167572e5  /dev/shm/wt-blk/backlog/XP-20261029-dg-blocked/spec-P32-rho.json
a66af69491204ebd98154ce022220179f1d3afd67d79a8b269df2fd7bb4b6ae1  /dev/shm/wt-blk/research/ic_dg_blocked_20261029/cards/prelaunch-checks.md
aaf30c43bac49f639e07a4793187a6a4f7d4ca26ddd02ce8827433e98252885d  /dev/shm/wt-blk/research/ic_dg_blocked_20261029/cards/RULING-P2.md
cb6e6d7603ec2f8ad068f0d9a992f01e1fe629a79747acaa68353f274f49ab97  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
e30ad4b55f91aee25306c4ee23793bb5394db9691f870a469b14828dac2c2fee  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/dgb_facts.py
95dcdefffe989ac5052f6b5ae84b16e3df6c029e6e2faf9763869c561094b2ab  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/project_compare.py
97509ca78dacd94d4f1c760527b3307cb779685a5580d34a54188478e9d5ad04  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/records_checks.py
f67bf6caeddc4d2a964fa65252de887f635bac39b4d7b2e0375ced5949805c9b  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/wall_gate.py
0dd45ebddbd77ccd6ad77162d04f3066093044a4e0242dafdf3eaa8f8d39f08e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/counted_gate.py
a5dafedca9b53e0195cdc3eabc4f527b5450204e244e69d079785dbe289fa7a1  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/phase_sums.py
d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/receipt.py
edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/project_facts.py
88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c  /dev/shm/isolab-build/ecbench
HASHES
```

The exit status must be 0 and all 20 output lines must end with ": OK". (The here-document ends only at the line that is exactly HASHES.)

Step 2. Readiness check (reads only; writes nothing). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
echo "FACTS_WALL"
python3 -I "$TOOLS/dgb_facts.py" "$DW/eks-results" "$DW/analysis" "$REV" "$FB" "$FBSHA" | tee "$DW/analysis/facts-stdout.txt"
echo "FACTS_COUNTED"
python3 -I "$TOOLS/dgb_facts.py" "$DC/eks-results" "$DC/analysis" "$REV" "$FB" "$FBSHA" | tee "$DC/analysis/facts-stdout.txt"
echo STEP5_OK
```

The output must contain two lines beginning FACTS_OK and the line STEP5_OK.

Step 6. The wall-session comparisons (5 pairs at 3 weights = 15 files). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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

Step 7. The counted-session comparisons (3 pairs at 3 weights = 9 files). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
want=9
[ "$n" -eq "$want" ] || { echo "STOP: counted compare json count is $n, not $want"; exit 1; }
for F in "$DC"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP7_OK
```

The output must contain 9 lines beginning COMPARE_OK, the line `COMPARE_JSON_COUNT counted 9`, and the line STEP7_OK.

Step 8. The band tables of the two sessions. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
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
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
python3 -I "$TOOLS/project_compare.py" "$DC/analysis" counted > "$DC/analysis/compare-projection.txt" || { cat "$DC/analysis/compare-projection.txt"; echo "STOP: project_compare failed (counted session)"; exit 1; }
sha256sum "$DC/analysis/compare-projection.txt"
wc -c "$DC/analysis/compare-projection.txt"
grep -c '^COMPARE_FILE ' "$DC/analysis/compare-projection.txt"
tail -n 1 "$DC/analysis/compare-projection.txt"
echo STEP10_OK
```

The output must contain a sha256sum line, a byte count, the number 9, the line PROJECT_COMPARE_OK, and the line STEP10_OK. Do not print the file.

Step 11. The record-level facts of both sessions: wall condition 8 (records half), the counted-equals-wall clause and condition A1 (mechanical facts; read the section "What the helper scripts do"). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
python3 -I "$TOOLS/records_checks.py" "$DW/eks-results" "$DC/eks-results" "$DW/analysis" "$DC/analysis" "$SPECW" "$SPECC" "$FB" | tee "$DW/analysis/records_checks-stdout.txt"
sha256sum "$DW/analysis/records_checks.json"
echo STEP11_OK
```

The output must contain the line RECORDS_CHECKS_OK, the lines R_COND8_RECORDS=, R_COUNTED_EQUALS_WALL=, R_A1_WALL= and R_A1_COUNTED= (each followed by true or false, either value is acceptable), the sha256sum line of records_checks.json, and the line STEP11_OK. The lines beginning COND8_RECORDS, COUNTED_VS_WALL, A1_WALL and A1_COUNTED are returned as printed.

Step 12. The wall-session instrument facts and conditions 1 to 9 (mechanical facts; read the section "What the helper scripts do"). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
python3 -I "$TOOLS/wall_gate.py" "$DW/eks-results" "$DW/analysis" "$SPECW" "$REV" "$FBSHA" | tee "$DW/analysis/wall_gate-stdout.txt"
sha256sum "$DW/analysis/wall_gate.json"
echo STEP12_OK
```

The output must contain the line WALL_GATE_OK, the lines beginning W_COND1= to W_COND9= (W_COND3_ROUNDED3= and W_COND8_ROUNDED3= included), W_A1=, W_ALL_1_TO_9=, W_ALL_1_TO_9_AND_A1=, W_ALL_1_TO_9_AND_A1_ROUNDED3=, CORRECTED_CI95_UPPER_LT_1= and CORRECTED_CI95_LOWER_GT_1= (each followed by true or false, either value is acceptable), six `AA ` lines, the sha256sum line of wall_gate.json, and the line STEP12_OK.

Step 13. The counted-session conditions and the exit-42 and exit-43 clauses (mechanical facts). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
python3 -I "$TOOLS/counted_gate.py" "$DC/eks-results" "$DC/analysis" "$SPECC" "$REV" "$FBSHA" | tee "$DC/analysis/counted_gate-stdout.txt"
sha256sum "$DC/analysis/counted_gate.json"
echo STEP13_OK
```

The output must contain the line COUNTED_GATE_OK, one `AA ` line per weight (three), the lines beginning GATE_COND1= GATE_COND2= GATE_COND2_ROUNDED3= GATE_COND3= GATE_COND4= GATE_COND5= GATE_COND6= EXIT42_CLAUSE= and EXIT43_CLAUSE= (each followed by true or false, either value is acceptable), the sha256sum line of counted_gate.json, and the line STEP13_OK.

Step 14. Integer sums of the wall session's recorded times (descriptive). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
python3 -I "$TOOLS/phase_sums.py" "$DW/eks-results" "$DW/analysis" | tee "$DW/analysis/phase_sums-stdout.txt"
echo STEP14_OK
```

The output must contain 4 lines beginning `PHASE_SUMS_ARM ` followed by an arm name, 8 lines beginning `PHASE_SUMS_ARM_CURVE `, the line PHASE_SUMS_OK, and the line STEP14_OK.

Step 15. The receipts. Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
PODW=$(tr -d '\n' < "$DW/analysis/pod-uid.txt")
PODC=$(tr -d '\n' < "$DC/analysis/pod-uid.txt")
python3 -I "$TOOLS/receipt.py" "$DW" "$b" "$JW" dg "$S3W" "$PODW" "$SPECW"
sha256sum "$DW/analysis/receipt.json"
python3 -I "$TOOLS/receipt.py" "$DC" "$b" "$JC" dg "$S3C" "$PODC" "$SPECC"
sha256sum "$DC/analysis/receipt.json"
echo STEP15_OK
```

The output must contain two lines beginning RECEIPT_OK, two sha256sum lines, and the line STEP15_OK.

Step 16. The recorded facts, projected (never print facts.json itself). Run:

```
b=__B__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dgb-analysis-tools/prelude.sh
echo "OUTPUT_DIR $D"
echo "FACTS_PROJECTION_WALL"
python3 -I "$TOOLS/project_facts.py" "$DW/analysis"
echo "FACTS_PROJECTION_COUNTED"
python3 -I "$TOOLS/project_facts.py" "$DC/analysis"
echo STEP16_OK
```

The output must contain two lines beginning FACTS_JSON_SHA256, two lines FACTS_PROJECTION_OK, and the line STEP16_OK.

## Return (verbatim tool output, nothing else)

For steps 0 to 16: the output lines of each step exactly as printed, in step order, including all COMPARE_OK lines of steps 6 and 7, both band blocks of step 8, and all of steps 11 to 16. Steps 9 and 10 return only what they print (the files they write are read by the main session). No interpretation of any ratio, interval, verdict, audit field, gate field, or any R_*, W_*, GATE_*, EXIT4*_CLAUSE or CORRECTED_* value; no summary; no recommendation; no other commands.

## Write scope

The executor writes only inside D (/dev/shm/wt-blk/research/ic_dg_blocked_20261029/P__B__ for the dispatch value of b), through the commands above. The Bash tool call output is the only other record. Write nowhere else; do not edit the repository, the helper files, the specs, the protocol directory or the S3 bucket; do not run git. The S3 commands only list and download.

## Stop and return `refused`, naming the condition and the step, when

- The dispatch does not state exactly one bit size from 24, 28, 32 and CARD_SHA256.
- Step 0: the first field printed is not equal to CARD_SHA256 from the dispatch.
- Step 1 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Any output line begins with `STOP:`. This covers: the prelude refusing b; D already existing; less than 600 MB free; an S3 listing failing or finding nothing (the job has not finished or was not launched); a pod-uid prefix count other than 1; `job-exit.json` or `SHA256SUMS` absent from the S3 prefix; a failed download; `sha256sum -c` exiting with a status other than 0 or any of its lines not ending in ": OK"; session/session.json or session/records.jsonl absent; the binary being absent or having another sha256; a compare, band, projection or helper exiting with a nonzero status; a compare JSON count other than 15 (wall) or 9 (counted); a compare file not parsing as JSON.
- Any step exits with a status other than 0 for a reason not printed as `STOP:`.
- Any step's required output (listed under that step) is missing.

A refusal in steps 0 to 2 wrote nothing, and the card may be dispatched again later. A refusal in step 3 or later leaves D as it is. After a refusal run nothing further: no retry, no edit, no cleanup, no deletion of D, no re-download, no substitute binary, no substitute job name. Each refusal is a specification_error back to the main session.

Not a stop condition, returned as printed: a job exit code of 42 or 43 or any non-zero code; `l2_gate.accepted` being false; `audit.ok` being false; a non-empty `problems` list; a replay detail other than "identical"; `FILE_PRESENT ... no`; a `false` value of any R_*, W_*, GATE_*, EXIT42_CLAUSE, EXIT43_CLAUSE or CORRECTED_* line; an A/A ratio other than 1; a non-admitted wall block; `FACT_*` lines being false; a sha256 of `facts.json` or of any analysis file. The executor copies these fields and does nothing with them.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and waits for exit; this task discovers a pod prefix at run time, loops over pair and weight lists, and tees to files, so the tool cannot express it. The cards of the previous study (CARD-XP-20261029-dg-wall-launch and -analysis) are plain cards for the same reason. There is no runbook, so `tools/runbook.py lint` does not apply.

## Note

The S3 results of a launch-remote job hold `binary.sha256` and `build-info.json`, which record the ecbench hash, not the binary. The binary for compare and band is therefore the build in /dev/shm/isolab-build, which facts record as `binary_used_sha256_equals_build_info_ecbench_sha256`; a false value is returned as printed and is not a stop condition. /dev/shm is memory-backed: D is lost if the machine restarts. D lies inside the protocol worktree, so a later checkpoint of the worktree may bundle it; the executor never runs git.
