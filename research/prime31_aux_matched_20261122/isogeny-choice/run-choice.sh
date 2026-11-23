#!/usr/bin/env bash
# Thin orchestration only. Run on the admitted IsoLab command-job backend;
# this script neither grants cloud access nor bypasses its budget/checkpoint gates.
set -euo pipefail
export LC_ALL=C
if [[ $# != 3 || "$1" != run ]]; then
  echo 'usage: bash run-choice.sh run ABSOLUTE_CELL.json ABSOLUTE_NEW_OUTPUT_DIR' >&2
  exit 64
fi
mode=$1
cell=$2
output=$3
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
[[ "$cell" == /* && -f "$cell" && "$output" == /* && ! -e "$output" ]] || {
  echo 'Input must exist; output must be a new absolute directory.' >&2
  exit 64
}
for program in jq timeout taskset sha256sum awk git realpath; do
  command -v "$program" >/dev/null || { echo "Missing $program" >&2; exit 69; }
done
# Admission receipts are produced by the existing publication/checkpoint and
# budget workflows, not by changing an environment variable to claim access.
: "${ISOLAB_PUBLICATION_RECEIPT:?path to existing verified source publication receipt}"
: "${ISOLAB_BUDGET_RECEIPT:?path to reconciled current account and lane budget receipt}"
[[ -s "$ISOLAB_PUBLICATION_RECEIPT" && -s "$ISOLAB_BUDGET_RECEIPT" ]] || exit 65
export SOURCE_REVISION
SOURCE_REVISION=$(git -C "$root" rev-parse HEAD)
[[ "$SOURCE_REVISION" =~ ^[0-9a-f]{40}$ ]] || exit 65
cell=$(realpath "$cell")
[[ "$cell" == "$root/"* ]] || { echo 'The cell must be inside the published repository.' >&2; exit 65; }
relative=${cell#"$root/"}
git -C "$root" ls-files --error-unmatch -- "$relative" >/dev/null
head_input_sha=$(git -C "$root" cat-file blob "HEAD:$relative" | sha256sum | awk '{print $1}')
[[ "$head_input_sha" == "$(sha256sum "$cell" | awk '{print $1}')" ]] || exit 65
if [[ -n $(git -C "$root" status --porcelain --untracked-files=normal) ]]; then
  echo 'Publish the exact experiment inputs and use a clean checkout before launch.' >&2
  exit 65
fi
(cd "$root" && bash scripts/checkpoint.sh gate)
binary="$here/../native/target/release/prime31-isogeny-choices"
[[ -x "$binary" ]] || { echo 'Build the source-pinned release binary before submission.' >&2; exit 69; }
seconds=$(jq -er '.execution_seconds | select(.==120)' "$cell")
memory=$(jq -er '.memory_mib | select(.==4096)' "$cell")
cpu=$(awk '/^Cpus_allowed_list:/ {split($2, groups, ","); split(groups[1], r, "-"); print r[1]; exit}' /proc/self/status)
[[ "$cpu" =~ ^[0-9]+$ ]] || { echo 'One affinity-bound logical CPU is required.' >&2; exit 69; }
"$binary" validate-cell "$cell" >/dev/null
mkdir "$output"
cp "$cell" "$output/input.json"
cp "$ISOLAB_PUBLICATION_RECEIPT" "$output/publication-receipt.json"
cp "$ISOLAB_BUDGET_RECEIPT" "$output/budget-receipt.json"
"$binary" source-check "$root" > "$output/source-check.json"
"$binary" source-manifest > "$output/source-manifest.json"
sha256sum "$binary" > "$output/binary.sha256"
input_sha=$(sha256sum "$cell" | awk '{print $1}')
binary_sha=$(sha256sum "$binary" | awk '{print $1}')
start=$(date -u +%Y-%m-%dT%H:%M:%SZ)
jq -n --arg revision "$SOURCE_REVISION" --arg mode "$mode" --arg input_sha "$input_sha" \
  --arg binary_sha "$binary_sha" --arg start "$start" --arg cpus "$cpu" \
  --argjson seconds "$seconds" --argjson memory "$memory" \
  '{schema:"prime31-isogeny-choice.process/v1",state:"started",source_revision:$revision,
    mode:$mode,input_sha256:$input_sha,binary_sha256:$binary_sha,started_utc:$start,
    cpuset:$cpus,threads:1,hard_kill_deadline_seconds:$seconds,term_deadline_seconds:($seconds-5),
    kill_grace_seconds:5,address_space_limit_mib:$memory,
    timing_admission:"diagnostic_only",retries:0}' > "$output/process-start.json"
finalize() {
  local exit_code=$1 end real user sys
  trap - EXIT
  set +e
  end=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  real=$(awk '/^real [0-9.]+$/ {v=$2} END {print v}' "$output/process.stderr")
  user=$(awk '/^user [0-9.]+$/ {v=$2} END {print v}' "$output/process.stderr")
  sys=$(awk '/^sys [0-9.]+$/ {v=$2} END {print v}' "$output/process.stderr")
  jq --arg end "$end" --argjson exit_code "$exit_code" --arg real "$real" --arg user "$user" --arg sys "$sys" \
    '.state="finished" | .finished_utc=$end | .exit_code=$exit_code |
     .elapsed_seconds_diagnostic=($real|try tonumber catch null) |
     .user_cpu_seconds=($user|try tonumber catch null) |
     .system_cpu_seconds=($sys|try tonumber catch null) | .peak_rss_bytes=null |
     .completion_class=(if $exit_code==0 then "recovered" elif $exit_code==2 then "capped_no_recovery"
       elif $exit_code==3 then "setup_failed" elif $exit_code==124 then "timeout"
       elif $exit_code==137 then "killed_cause_requires_job_receipt" else "failed" end)' \
    "$output/process-start.json" > "$output/process-result.json"
  (cd "$output" && sha256sum input.json publication-receipt.json budget-receipt.json source-check.json source-manifest.json \
    binary.sha256 process-start.json process-result.json events.jsonl process.stderr > SHA256SUMS)
  return "$exit_code"
}
: > "$output/process.stderr"
: > "$output/events.jsonl"
trap 'finalize "$?"' EXIT
trap 'exit 143' TERM
trap 'exit 130' INT
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1
set +e
{ time -p timeout --signal=TERM --kill-after=5s "$((seconds - 5))s" \
    taskset -c "$cpu" bash -c 'ulimit -v "$1" || exit 70; exec "$2" "$3" "$4"' \
    prime31-isogeny-choice "$((memory * 1024))" "$binary" "$mode" "$output/input.json"; \
} > "$output/events.jsonl" 2> "$output/process.stderr"
exit_code=$?
set -e
exit "$exit_code"
