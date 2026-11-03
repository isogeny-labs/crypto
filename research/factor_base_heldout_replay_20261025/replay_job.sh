#!/bin/sh
# Frozen N12 evaluation replay. The bundle hash and selector are fixed in PROTOCOL.md.
set -eu
input_uri='s3://isolab-results-104186811093-us-east-2/inputs/n12-heldout-replay-20261025/files-7f510189/'
out=${ISOLAB_OUTPUT_DIR:-/work/results}
mkdir -p "$out/raw" "$out/receipts" /work/repo
finish() {
    rc=$?
    trap - EXIT
    printf '%s\n' "$rc" > "$out/receipts/job-exit-code.txt"
    if command -v date >/dev/null 2>&1; then date -u '+%Y-%m-%dT%H:%M:%SZ'; else printf 'unknown\n'; fi > "$out/receipts/job-finished-utc.txt"
    if ! aws s3 cp "$out/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors; then
        echo 'Results upload failed' >&2
        exit 90
    fi
    exit "$rc"
}
trap finish EXIT
printf '%s\n' "${POD_UID:-unknown}" > "$out/receipts/pod-uid.txt"
printf '%s\n' "${NODE_NAME:-unknown}" > "$out/receipts/node-name.txt"
printf '%s\n' "${ISOLAB_JOB_ID:-unknown}" > "$out/receipts/job-id.txt"
if command -v uname >/dev/null 2>&1; then uname -a > "$out/receipts/uname.txt"; fi
if command -v sed >/dev/null 2>&1 && command -v head >/dev/null 2>&1 && [ -r /proc/cpuinfo ]; then sed -n '/^model name\|^Hardware/p' /proc/cpuinfo | head -n 1 > "$out/receipts/cpu-model.txt"; fi
if command -v date >/dev/null 2>&1; then date -u '+%Y-%m-%dT%H:%M:%SZ'; else printf 'unknown\n'; fi > "$out/receipts/job-started-utc.txt"
for tool in aws chmod; do command -v "$tool" > "$out/receipts/command-$tool.txt" 2>&1 || { echo "required command missing: $tool" >&2; exit 126; }; done
aws s3 cp "$input_uri" /work/repo/ --recursive --only-show-errors
chmod 755 /work/repo/bin/heldout /work/repo/bin/verify-inputs
/work/repo/bin/verify-inputs /work/repo > "$out/receipts/member-hash-checks.txt"
cd /work/repo
printf 'bits\texit_code\trows\tstarted_utc\tfinished_utc\n' > "$out/receipts/evaluation.tsv"
overall=0
for bits in 16 20 24 28 32 36 40; do
    start=$(if command -v date >/dev/null 2>&1; then date -u '+%Y-%m-%dT%H:%M:%SZ'; else printf 'unknown\n'; fi)
    raw="$out/raw/evaluation-bits${bits}.jsonl"
    if bin/heldout "$bits" 1 32 any evaluation > "$raw" 2> "$out/raw/evaluation-bits${bits}.stderr"; then
        code=0
    else
        code=$?
        overall=1
    fi
    rows=0
    while IFS= read -r line || [ -n "$line" ]; do rows=$((rows + 1)); done < "$raw"
    [ "$rows" -eq 60 ] || overall=1
    end=$(if command -v date >/dev/null 2>&1; then date -u '+%Y-%m-%dT%H:%M:%SZ'; else printf 'unknown\n'; fi)
    printf '%s\t%s\t%s\t%s\t%s\n' "$bits" "$code" "$rows" "$start" "$end" >> "$out/receipts/evaluation.tsv"
    echo "bits=$bits exit=$code rows=$rows"
done
if [ "$overall" -ne 0 ]; then exit 1; fi
