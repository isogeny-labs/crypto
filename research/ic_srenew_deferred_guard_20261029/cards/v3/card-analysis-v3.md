# Mechanical task card CARD-XP-20261029-srenew-deferred-guard-v3-analysis

author_role: experiment-director
executor_role: executor-mechanical
experiment: srenew-deferred-guard-v3 (protocol backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md; one analysis task per size b; not an EXP-* record)
kind: plain frozen analysis card (no launch step: the jobs were launched by the main session with the v3 launch card; not `tools/task_card.py` frozen_command; see "Why a plain card")
background: no
approval: the main (reasoning-tier) session decided this route and the Coordinator approved the protocol (APPROVAL.md in the protocol directory). This card adds no authority.

## Parameters (the dispatch message gives exactly three values: __B__, __J__ and CARD_SHA256; no other values are accepted)

| b | J (job name) | BASE arm |
|--:|:--|:--|
| 24 | ic-dg3-p24-20261029 | walk |
| 28 | ic-dg3-p28-20261029 | srenew |
| 32 | ic-dg3-p32-20261029 | walk |

In every command below the two words `__B__` and `__J__` stand for the dispatch values of b and J. Type the dispatch value in place of each of the two words and change nothing else. The sourced prelude refuses any pair that does not match the table (the pair must be exactly one table row). CARD_SHA256 is used only in step 0.

## Frozen values

- Source revision that built the ecbench: be2969cebf30a0cfb79088fd10094324c3efb3bb
- S3 results prefix: `s3://isolab-results-104186811093-us-east-2/runs/J/` followed by one pod-uid prefix.
- Output directory D = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/P{b}, with subdirectories `eks-results` (the S3 copy of the pod-uid prefix) and `analysis`. D must not exist before step 2.
- Frozen spec for size b: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P{b}.json
- Frozen protocol: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md
- Helper files (all bound by sha256 in step 1): check_spec.py, facts.py, receipt.py, gate_v3.py, project_facts.py and prelude.sh in /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools. (check_spec.py, facts.py and receipt.py are byte-identical copies of the first study's helpers; they take every path as an argument.)
- Binary for compare and band: /dev/shm/isolab-build/ecbench, sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c. It is this run's own build of the source revision above. No other binary is used. facts.py refuses when the file is absent or its hash differs, and writes the path into `analysis/binary-path.txt`.
- AWS: the executor's existing identity. Never print, write or copy credentials. The prelude sets PATH to start with /tmp/claude-0/awsvenv/bin and AWS_DEFAULT_REGION=us-east-2.
- Pairs: a pair written A__B is run as `compare --a A --b B`, so the ratio is B over A. The pairs are, in this order, with BASE = walk for b=24 and b=32 and BASE = srenew for b=28:
  1. BASE__dg-small (decision pair)
  2. srenew__dg, srenew__srenew-small, srenew-small__dg-small, walk__srenew (factorial pairs)
  3. BASE__BASE-aa, rho-neg__rho-neg-aa (A/A pairs)
  4. rho-neg__BASE, rho-neg__dg-small (distance to rho)
- Weights: 0, 0.1 and 1 (GAE per unpriced native operation).
- Pinned sha256:
  - spec-P24.json ce1b34733baabd92411fe069f05b837988191e4229a6f7f118e36db31f49617e
  - spec-P28.json d3e9a7247fc34effd9282a9a32590fa45b9d12700c7bb4fd282682c874503cb8
  - spec-P32.json 1d817e4a6261d7f31ab88bc5f4b72953e4dee3d06eaa2e97a0b956bdb8b9e8ba
  - PROTOCOL.md caf2519d87241ed0ab7f947ecfe7aa409e7eed3b6f5d648bab8df2569c6143be
  - check_spec.py ac4e827cef767fdbfcf41b8af62e65f9b3d9f83ee2284a53a064f29d22c7497e
  - facts.py 9fc06fd3c5fab21ef74bcaaf56e1ff3a1ebf5dc138fa55597228436d09765223
  - receipt.py d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef
  - gate_v3.py 63f4ea38ec88b2a9554d00790776910763abc1b146c33bf96fc445deb276e80d
  - project_facts.py edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091
  - prelude.sh 0a71de386517c3abc871bf4470e34095e914a34182bd085f669d1eb65a9a90df
  - /dev/shm/isolab-build/ecbench 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c

## What gate_v3.py does (a description; the executor does nothing with it)

gate_v3.py reads files and counts and filters. It computes, from `l2-gate.json` and independently from `session/records.jsonl`, the counts and lists named in step 8, and prints the lines `GATE_COND1=` to `GATE_COND6=`, `GATE_COND2_ROUNDED3=` and `EXIT42_CLAUSE=` as true or false by the literal wording of PROTOCOL.md section 6.1 (conditions 1 to 6 and the exit-42 clause). These lines are mechanical facts. The executor does not interpret them: a value of false is not a stop condition and is returned as printed. The same holds for `l2_gate.accepted=false`, a job exit code of 42, and any other field value.

## Procedure

Use one Bash call per step. Paste each command block exactly as written, with only the `__B__` and `__J__` replacement. A line that begins with `STOP:` in any output is a refusal (see "Stop and return refused"). Change no file in the repository and no file outside D.

Step 0. Card digest. Run:

```
sha256sum /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/card-analysis-v3.md
```

The first field printed must equal CARD_SHA256 from the dispatch. (This card file is the one with the words `__B__` and `__J__` unsubstituted; the substitution is made only in the commands you run.)

Step 1. Hashes of the specs, protocol, helper files and binary. Run:

```
sha256sum -c - <<'HASHES'
ce1b34733baabd92411fe069f05b837988191e4229a6f7f118e36db31f49617e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P24.json
d3e9a7247fc34effd9282a9a32590fa45b9d12700c7bb4fd282682c874503cb8  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P28.json
1d817e4a6261d7f31ab88bc5f4b72953e4dee3d06eaa2e97a0b956bdb8b9e8ba  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P32.json
caf2519d87241ed0ab7f947ecfe7aa409e7eed3b6f5d648bab8df2569c6143be  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md
ac4e827cef767fdbfcf41b8af62e65f9b3d9f83ee2284a53a064f29d22c7497e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/check_spec.py
9fc06fd3c5fab21ef74bcaaf56e1ff3a1ebf5dc138fa55597228436d09765223  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/facts.py
d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/receipt.py
63f4ea38ec88b2a9554d00790776910763abc1b146c33bf96fc445deb276e80d  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/gate_v3.py
edd47ab4fe17cee1a1450797957fb35e60c3be89acefedc5de8a86322080f091  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/project_facts.py
0a71de386517c3abc871bf4470e34095e914a34182bd085f669d1eb65a9a90df  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c  /dev/shm/isolab-build/ecbench
HASHES
```

The exit status must be 0 and all 11 output lines must end with ": OK". (The here-document ends only at the line that is exactly HASHES.)

Step 2. Parameters, output directory, free disk. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
test ! -e "$D" || { echo "STOP: output directory exists: $D"; exit 1; }
avail=$(df -m --output=avail "$SC/dg3" | tail -n 1 | tr -d ' ')
echo "DF_AVAIL_MB $avail"
[ "$avail" -ge 60 ] || { echo "STOP: less than 60 MB free"; exit 1; }
mkdir -p "$D/eks-results" "$D/analysis"
echo "$avail" > "$D/analysis/df-avail-mb-before.txt"
echo STEP2_OK
```

The output must contain the line PRELUDE_OK, a DF_AVAIL_MB line with a number of at least 60, and the line STEP2_OK.

Step 3. The one pod-uid prefix. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
aws s3 ls "$S3/" > "$D/analysis/s3-prefixes.txt" || { echo "STOP: aws s3 ls failed or found nothing under $S3/"; exit 1; }
cat "$D/analysis/s3-prefixes.txt"
awk '$1=="PRE"{print $2}' "$D/analysis/s3-prefixes.txt" | grep -E '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/$' > "$D/analysis/pod-uid.txt" || true
n=$(wc -l < "$D/analysis/pod-uid.txt")
echo "POD_UID_COUNT $n"
[ "$n" -eq 1 ] || { echo "STOP: pod-uid prefix count is $n, not 1"; exit 1; }
cat "$D/analysis/pod-uid.txt"
echo STEP3_OK
```

The output must contain POD_UID_COUNT 1 and the line STEP3_OK.

Step 4. Download and verify against SHA256SUMS. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
PODUID=$(tr -d '\n' < "$D/analysis/pod-uid.txt")
aws s3 cp --recursive --only-show-errors "$S3/$PODUID" "$D/eks-results/"
test -f "$D/eks-results/SHA256SUMS" || { echo "STOP: SHA256SUMS absent from the copy"; exit 1; }
cd "$D/eks-results"
sha256sum -c SHA256SUMS > "$D/analysis/sha256sum-check.txt" || { cat "$D/analysis/sha256sum-check.txt"; echo "STOP: sha256sum -c failed"; exit 1; }
bad=$(grep -c -v ': OK$' "$D/analysis/sha256sum-check.txt" || true)
echo "SHA256_LINES $(wc -l < "$D/analysis/sha256sum-check.txt") NOT_OK_LINES $bad"
[ "$bad" -eq 0 ] || { echo "STOP: a SHA256SUMS line is not OK"; exit 1; }
df -m --output=avail "$SC/dg3" | tail -n 1
echo STEP4_OK
```

The output must contain a SHA256_LINES line ending in NOT_OK_LINES 0, and the line STEP4_OK.

Step 5. Input spec, source revision, audit, L2 gate, binary. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
python3 -I "$TOOLS/check_spec.py" "$SPEC" "$D/eks-results" | tee "$D/analysis/spec-check.txt"
grep -r -l -F --include=build-info.json "$REV" "$D/eks-results" | tee "$D/analysis/revision-grep.txt"
python3 -I "$TOOLS/facts.py" "$D/eks-results" "$D/analysis" "$REV" "$FB" "$FBSHA" | tee "$D/analysis/facts-stdout.txt"
echo STEP5_OK
```

The output must contain a line beginning SPEC_EQUAL, one path line from the grep, a line beginning FACTS_OK, and the line STEP5_OK.

Step 6. The comparisons (27 files: 9 pairs at 3 weights). Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
BIN=$(cat "$D/analysis/binary-path.txt")
SESS="$D/eks-results/session"
test -x "$BIN" || { echo "STOP: binary not executable: $BIN"; exit 1; }
test -f "$SESS/session.json" || { echo "STOP: session.json absent: $SESS"; exit 1; }
echo "BINARY_USED $BIN"
PAIRS="$BASE:dg-small srenew:dg srenew:srenew-small srenew-small:dg-small walk:srenew $BASE:$BASE-aa rho-neg:rho-neg-aa rho-neg:$BASE rho-neg:dg-small"
for W in 0 0.1 1; do
  for P in $PAIRS; do
    A=${P%%:*}
    B2=${P##*:}
    "$BIN" compare --dir "$SESS" --a "$A" --b "$B2" --weight "$W" --json > "$D/analysis/compare-${A}__${B2}-w${W}.json" 2> "$D/analysis/compare-${A}__${B2}-w${W}.stderr" || { echo "STOP: compare failed for a=$A b=$B2 weight=$W"; exit 1; }
    echo "COMPARE_OK a=$A b=$B2 weight=$W"
  done
done
n=$(ls "$D"/analysis/compare-*.json | wc -l)
echo "COMPARE_JSON_COUNT $n"
[ "$n" -eq 27 ] || { echo "STOP: compare json count is $n, not 27"; exit 1; }
for F in "$D"/analysis/compare-*.json; do python3 -I -c "import json,sys; json.load(open(sys.argv[1]))" "$F" || { echo "STOP: not valid JSON: $F"; exit 1; }; done
echo STEP6_OK
```

The output must contain 27 lines beginning COMPARE_OK, the line COMPARE_JSON_COUNT 27, and the line STEP6_OK.

Step 7. The band table. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
BIN=$(cat "$D/analysis/binary-path.txt")
"$BIN" band --dir "$D/eks-results/session" --ranks > "$D/analysis/band.md" 2> "$D/analysis/band.stderr" || { echo "STOP: band failed"; exit 1; }
wc -c "$D/analysis/band.md"
echo STEP7_OK
```

The output must contain the line STEP7_OK.

Step 8. The gate facts and the A/A ratios (mechanical facts; read the section "What gate_v3.py does"). Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
python3 -I "$TOOLS/gate_v3.py" "$D/eks-results" "$D/analysis" "$SPEC" "$BASE" "$REV" "$FBSHA" | tee "$D/analysis/gate_v3-stdout.txt"
sha256sum "$D/analysis/gate_v3.json"
echo STEP8_OK
```

The output must contain the line GATE_V3_OK, six `AA ` lines, the lines beginning GATE_COND1= GATE_COND2= GATE_COND2_ROUNDED3= GATE_COND3= GATE_COND4= GATE_COND5= GATE_COND6= and EXIT42_CLAUSE= (each followed by true or false, either value is acceptable), the sha256sum line of gate_v3.json, and the line STEP8_OK.

Step 9. The receipt. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
PODUID=$(tr -d '\n' < "$D/analysis/pod-uid.txt")
python3 -I "$TOOLS/receipt.py" "$D" "$b" "$J" "$BASE" "$S3" "$PODUID" "$SPEC"
sha256sum "$D/analysis/receipt.json"
ls -1 "$D/analysis" | wc -l
echo STEP9_OK
```

The output must contain a line beginning RECEIPT_OK, the sha256sum line of receipt.json, and the line STEP9_OK.

Step 10. The recorded facts, projected (never print facts.json itself: it is about 1.4 MB). Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3-card-tools/prelude.sh
echo "OUTPUT_DIR $D"
python3 -I "$TOOLS/project_facts.py" "$D/analysis"
echo STEP10_OK
```

The output must contain a line beginning FACTS_JSON_SHA256, the line FACTS_PROJECTION_OK, and the line STEP10_OK.

## Return (verbatim tool output, nothing else)

For steps 0 to 10: the output lines of each step exactly as printed, in step order, including all 27 COMPARE_OK lines of step 6, all of step 8 and all of step 10. No interpretation of any ratio, interval, verdict, audit field, gate field or GATE_*/EXIT42_CLAUSE value, no summary, no recommendation, no other commands.

## Write scope

The executor writes only inside D (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg3/P{b} for the dispatch value of b), through the commands above. The Bash tool call output is the only other record. Write nowhere else; do not edit the repository, the helper files, the specs or the S3 bucket.

## Stop and return `refused`, naming the condition and the step, when

- Any output line begins with `STOP:`. This covers: the prelude refusing the pair of b and J; D already existing; less than 60 MB free (step 2); the S3 listing failing or the pod-uid prefix count not being 1 (step 3); SHA256SUMS absent, `sha256sum -c` exiting with a status other than 0, or any of its lines not ending in ": OK" (step 4); input-spec.json not found exactly once, or not JSON-equal to the frozen spec, or build-info.json absent or its source_revision not equal to the Frozen-values revision, or the binary /dev/shm/isolab-build/ecbench being absent or having a different sha256 (step 5); a compare exiting with a nonzero status, the compare JSON count not being 27, or a compare file not parsing as JSON (step 6); band exiting with a nonzero status (step 7); gate_v3.py or project_facts.py not finding or not parsing an input file (steps 8 and 10).
- Step 0: the first field printed is not equal to CARD_SHA256 from the dispatch.
- Step 1 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 5: the `grep` prints no path line (its exit status is then 1).
- Any step exits with a status other than 0 for a reason not printed as `STOP:`.
- Any step's required output (listed under that step) is missing.

The only stop conditions are these: a hash mismatch, a pairing mismatch, a failed download or listing, a spec or revision or binary mismatch, a failed command, or missing required output. Each refusal is a specification_error back to the main session. After a refusal run nothing further: no retry, no edit, no cleanup, no deletion of D, no re-download, no substitute binary, no substitute job name. Anything already written in D stays as it is.

Not a stop condition, returned as printed: `l2_gate.accepted` being false, a job exit code of 42 (expected), a non-empty `problems` list, a replay detail other than "identical", an `ok` of false, a `false` value of any GATE_COND line or EXIT42_CLAUSE, a non-zero count of "not_waived" executions, and an A/A ratio other than 1. The executor copies these fields and does nothing with them.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and has no step that downloads under a runtime-determined prefix and loops over a pair list. Cards in this study (the first study's analysis card, CARD-XP-20261029-srenew-deferred-guard-analysis, and the v3 launch card) are plain cards for the same reason.

## Note

The S3 results of a launch-remote job hold `binary.sha256` and `build-info.json`, which record the ecbench hash, not the binary. The binary for compare and band is therefore the build in /dev/shm/isolab-build. The receipt records its path, its sha256, and whether that sha256 equals the build-info ecbench_sha256 of the job's results; a false value is returned as printed and is not a stop condition.
