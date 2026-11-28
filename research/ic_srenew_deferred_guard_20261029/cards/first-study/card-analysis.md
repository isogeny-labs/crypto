# Mechanical task card CARD-XP-20261029-srenew-deferred-guard-analysis

author_role: experiment-director
executor_role: executor-mechanical
experiment: srenew-deferred-guard (protocol backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md; one analysis task per size b; not an EXP-* record)
kind: plain frozen analysis card (no launch step: the jobs were launched by the main session with GitHub workflow_dispatch; not `tools/task_card.py` frozen_command; see "Why a plain card")
background: no
approval: the main (reasoning-tier) session decided this route. This card adds no authority.

## Parameters (the dispatch message gives exactly two values; no other values are accepted)

| b | J (job name) | BASE arm |
|--:|:--|:--|
| 24 | ic-dg-p24-20261029 | walk |
| 28 | ic-dg-p28-20261029 | srenew |
| 32 | ic-dg-p32-20261029 | walk |

In every command below the two words `__B__` and `__J__` stand for the dispatch values of b and J. Type the dispatch value in place of each of the two words and change nothing else. The sourced prelude refuses any pair that does not match the table.

## Frozen values

- Source revision that built the ecbench: be2969cebf30a0cfb79088fd10094324c3efb3bb
- S3 results prefix: `s3://isolab-results-104186811093-us-east-2/runs/J/` followed by one pod-uid prefix.
- Output directory D = /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/P{b}, with subdirectories `eks-results` (the S3 copy of the pod-uid prefix) and `analysis`. D must not exist before step 2.
- Frozen spec for size b: /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard/spec-P{b}.json
- Helper files (all bound by sha256 in step 1): check_spec.py, facts.py, receipt.py and prelude.sh in /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools
- Binary for compare and band: /dev/shm/isolab-build/ecbench, sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c. It is this run's own build of the source revision above. No other binary is used. facts.py refuses when the file is absent or its hash differs, and writes the path into `analysis/binary-path.txt`.
- AWS: the executor's existing identity. Never print, write or copy credentials. The prelude sets PATH to start with /tmp/claude-0/awsvenv/bin and AWS_DEFAULT_REGION=us-east-2.
- Pairs: a pair written A__B is run as `compare --a A --b B`, so the ratio is B over A. The pairs are, in this order, with BASE = walk for b=24 and b=32 and BASE = srenew for b=28:
  1. BASE__dg-small (decision pair)
  2. srenew__dg, srenew__srenew-small, srenew-small__dg-small, walk__srenew (factorial pairs)
  3. BASE__BASE-aa, rho-neg__rho-neg-aa (A/A pairs)
  4. rho-neg__BASE, rho-neg__dg-small (distance to rho)
- Weights: 0, 0.1 and 1 (GAE per unpriced native operation).
- Pinned sha256:
  - spec-P24.json f0c1a723ae059559c3a1398ec61adac0c9ae3dadc0a8b7b81ad1f6bbad4f9e0c
  - spec-P28.json fd64f9038a464250249045bb60fba6c670fd169090d2bb9fda73fc538f067492
  - spec-P32.json 68d4dd5e66dd63a87f5680c2458b4987d917a9bf0d0b0b3af39ce3b7e5700d0c
  - check_spec.py ac4e827cef767fdbfcf41b8af62e65f9b3d9f83ee2284a53a064f29d22c7497e
  - facts.py 9fc06fd3c5fab21ef74bcaaf56e1ff3a1ebf5dc138fa55597228436d09765223
  - receipt.py d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef
  - prelude.sh b22b8ff985781e9dc65b4042a9e916db596ad8a14a53ce921f10052c83656964
  - /dev/shm/isolab-build/ecbench 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c

## Procedure

Use one Bash call per step. Paste each command block exactly as written, with only the `__B__` and `__J__` replacement. A line that begins with `STOP:` in any output is a refusal (see "Stop and return refused"). Change no file in the repository and no file outside D.

Step 1. Hashes of the specs, helper files and binary. Run:

```
sha256sum -c - <<'HASHES'
f0c1a723ae059559c3a1398ec61adac0c9ae3dadc0a8b7b81ad1f6bbad4f9e0c  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard/spec-P24.json
fd64f9038a464250249045bb60fba6c670fd169090d2bb9fda73fc538f067492  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard/spec-P28.json
68d4dd5e66dd63a87f5680c2458b4987d917a9bf0d0b0b3af39ce3b7e5700d0c  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-dg/backlog/XP-20261029-srenew-deferred-guard/spec-P32.json
ac4e827cef767fdbfcf41b8af62e65f9b3d9f83ee2284a53a064f29d22c7497e  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/check_spec.py
9fc06fd3c5fab21ef74bcaaf56e1ff3a1ebf5dc138fa55597228436d09765223  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/facts.py
d4758d18d2fc95b016a99d7347844335db01de0f4015f1407b4ef9337f6ddfef  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/receipt.py
b22b8ff985781e9dc65b4042a9e916db596ad8a14a53ce921f10052c83656964  /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c  /dev/shm/isolab-build/ecbench
HASHES
```

The exit status must be 0 and all 8 output lines must end with ": OK". (The here-document ends only at the line that is exactly HASHES.)

Step 2. Parameters, output directory, free disk. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
test ! -e "$D" || { echo "STOP: output directory exists: $D"; exit 1; }
avail=$(df -m --output=avail "$SC/dg" | tail -n 1 | tr -d ' ')
echo "DF_AVAIL_MB $avail"
[ "$avail" -ge 150 ] || { echo "STOP: less than 150 MB free"; exit 1; }
mkdir -p "$D/eks-results" "$D/analysis"
echo "$avail" > "$D/analysis/df-avail-mb-before.txt"
echo STEP2_OK
```

The output must contain the line PRELUDE_OK, a DF_AVAIL_MB line with a number of at least 150, and the line STEP2_OK.

Step 3. The one pod-uid prefix. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
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
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
PODUID=$(tr -d '\n' < "$D/analysis/pod-uid.txt")
aws s3 cp --recursive --only-show-errors "$S3/$PODUID" "$D/eks-results/"
test -f "$D/eks-results/SHA256SUMS" || { echo "STOP: SHA256SUMS absent from the copy"; exit 1; }
cd "$D/eks-results"
sha256sum -c SHA256SUMS > "$D/analysis/sha256sum-check.txt" || { cat "$D/analysis/sha256sum-check.txt"; echo "STOP: sha256sum -c failed"; exit 1; }
bad=$(grep -c -v ': OK$' "$D/analysis/sha256sum-check.txt" || true)
echo "SHA256_LINES $(wc -l < "$D/analysis/sha256sum-check.txt") NOT_OK_LINES $bad"
[ "$bad" -eq 0 ] || { echo "STOP: a SHA256SUMS line is not OK"; exit 1; }
df -m --output=avail "$SC/dg" | tail -n 1
echo STEP4_OK
```

The output must contain a SHA256_LINES line ending in NOT_OK_LINES 0, and the line STEP4_OK.

Step 5. Input spec, source revision, audit, L2 gate, binary. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
python3 -I "$TOOLS/check_spec.py" "$SPEC" "$D/eks-results" | tee "$D/analysis/spec-check.txt"
grep -r -l -F --include=build-info.json "$REV" "$D/eks-results" | tee "$D/analysis/revision-grep.txt"
python3 -I "$TOOLS/facts.py" "$D/eks-results" "$D/analysis" "$REV" "$FB" "$FBSHA" | tee "$D/analysis/facts-stdout.txt"
echo STEP5_OK
```

The output must contain a line beginning SPEC_EQUAL, one path line from the grep, a line beginning FACTS_OK, and the line STEP5_OK.

Step 6. The comparisons (27 files: 9 pairs at 3 weights). Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
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
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
BIN=$(cat "$D/analysis/binary-path.txt")
"$BIN" band --dir "$D/eks-results/session" --ranks > "$D/analysis/band.md" 2> "$D/analysis/band.stderr" || { echo "STOP: band failed"; exit 1; }
wc -c "$D/analysis/band.md"
echo STEP7_OK
```

The output must contain the line STEP7_OK.

Step 8. The receipt. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
PODUID=$(tr -d '\n' < "$D/analysis/pod-uid.txt")
python3 -I "$TOOLS/receipt.py" "$D" "$b" "$J" "$BASE" "$S3" "$PODUID" "$SPEC"
sha256sum "$D/analysis/receipt.json"
ls -1 "$D/analysis" | wc -l
echo STEP8_OK
```

The output must contain a line beginning RECEIPT_OK, the sha256sum line of receipt.json, and the line STEP8_OK.

Step 9. The recorded facts, printed. Run:

```
b=__B__; J=__J__; . /tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg-card-tools/prelude.sh
echo "OUTPUT_DIR $D"
cat "$D/analysis/facts.json"
echo
echo STEP9_OK
```

The output must contain the line STEP9_OK.

## Return (verbatim tool output, nothing else)

For steps 1 to 9: the output lines of each step exactly as printed, in step order, including all 27 COMPARE_OK lines of step 6 and all of step 9. No interpretation of any ratio, interval, verdict, audit field or gate field, no summary, no recommendation, no other commands.

## Write scope

The executor writes only inside D (/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/dg/P{b} for the dispatch value of b), through the commands above. The Bash tool call output is the only other record. Write nowhere else; do not edit the repository, the helper files, the specs or the S3 bucket.

## Stop and return `refused`, naming the condition and the step, when

- Any output line begins with `STOP:`. This covers: the prelude refusing the pair of b and J; D already existing; less than 150 MB free (step 2); the S3 listing failing or the pod-uid prefix count not being 1 (step 3); SHA256SUMS absent, `sha256sum -c` exiting with a status other than 0, or any of its lines not ending in ": OK" (step 4); input-spec.json not found exactly once, or not JSON-equal to the frozen spec, or build-info.json absent or its source_revision not equal to the Frozen-values revision, or the binary /dev/shm/isolab-build/ecbench being absent or having a different sha256 (step 5); a compare exiting with a nonzero status, the compare JSON count not being 27, or a compare file not parsing as JSON (step 6); band exiting with a nonzero status (step 7).
- Step 1 exits with a status other than 0, or any of its output lines does not end with ": OK".
- Step 5: the `grep` prints no path line (its exit status is then 1).
- Any step exits with a status other than 0 for a reason not printed as `STOP:`.
- Any step's required output (listed under that step) is missing.

Each refusal is a specification_error back to the main session. After a refusal run nothing further: no retry, no edit, no cleanup, no deletion of D, no re-download, no substitute binary, no substitute job name. Anything already written in D stays as it is.

The executor copies the status fields of the audit and L2 gate (printed in step 5 and stored in facts.json) and does nothing with them: a field value of false, a problems list that is not empty, a replay detail other than "identical", or an exit_code other than 0 is not a stop condition of this card and is returned as printed.

## Why a plain card

`tools/task_card.py` frozen_command runs each argv without a shell and has no step that downloads under a runtime-determined prefix and loops over a pair list. Cards in this study (the stride study, CARD-XP-20261029-stride-renew-03 and -04) are plain cards for the same reason.

## Note

The S3 results of a launch-remote job hold `binary.sha256` and `build-info.json`, which record the ecbench hash, not the binary. The binary for compare and band is therefore the build in /dev/shm/isolab-build. The receipt records its path, its sha256, and whether that sha256 equals the build-info ecbench_sha256 of the job's results; a false value is returned as printed and is not a stop condition.
