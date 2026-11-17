#!/usr/bin/env bash
set -euo pipefail

study=$(cd "$(dirname "$0")" && pwd)
binary=$1
coordinate_bit=$2
out=$3
dest=$4
protocol=$5

[[ "$coordinate_bit" =~ ^(2[7-9]|3[0-9])$ ]] || {
  echo "coordinate bit must be in 27..39" >&2
  exit 2
}
[[ ! -e "$out" ]] || {
  echo "fresh output directory required; preserve old attempt: $out" >&2
  exit 2
}
[[ "$dest" == s3://setec-astronomy-artifacts/runs/*/ ]] || {
  echo "destination must be a unique run prefix in setec-astronomy-artifacts" >&2
  exit 2
}

candidate_id="spike${coordinate_bit}"
run_id="n83-coordinate-bit-${candidate_id}-20261031-r1"
jq -e --argjson bit "$coordinate_bit" '.extensions.candidate_coordinate_bits | index($bit) != null' "$protocol" >/dev/null || {
  echo "coordinate bit is absent from the frozen protocol: $coordinate_bit" >&2
  exit 2
}
expected_binary=$(jq -r '.arms[0].instrument.binary_sha256' "$protocol")
expected_source=$(jq -r '.freeze.source_commit' "$protocol")
protocol_sha=$(sha256sum "$protocol" | cut -d' ' -f1)
actual_binary=$(sha256sum "$binary" | cut -d' ' -f1)
[[ "$actual_binary" == "$expected_binary" ]] || {
  echo "binary hash mismatch: $actual_binary != $expected_binary" >&2
  exit 3
}

mkdir -p "$out"
started=$(date -u +%FT%TZ)
uploader_pid=
upload() { aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
finish() {
  code=$?
  trap - EXIT
  if [[ -n "${uploader_pid:-}" ]]; then
    kill "$uploader_pid" 2>/dev/null || true
    wait "$uploader_pid" 2>/dev/null || true
  fi
  jq -n \
    --arg run_id "$run_id" \
    --arg candidate_id "$candidate_id" \
    --arg start "$started" \
    --arg end "$(date -u +%FT%TZ)" \
    --arg protocol_sha "$protocol_sha" \
    --arg source "$expected_source" \
    --arg binary "$actual_binary" \
    --argjson code "$code" \
    '{schema:"n83.factor-base-variant-run/v1",run_id:$run_id,candidate_id:$candidate_id,coordinate_bit:(($candidate_id|sub("spike";""))|tonumber),started_at:$start,finished_at:$end,exit_code:$code,protocol_sha256:$protocol_sha,source_commit:$source,binary_sha256:$binary,wall_valid:false}' \
    > "$out/run-manifest.json"
  (
    cd "$out"
    find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256
  )
  upload || exit 91
  exit "$code"
}
trap finish EXIT

cp "$protocol" "$out/protocol.json"
cp "$study/FACTOR_BASE_VARIANT_SWEEP_DESIGN.md" "$out/"
cp "$binary" "$out/instrument"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
"$binary" selfcheck > "$out/selfcheck.json" 2> "$out/selfcheck.expected-negative-controls.log"
upload
(while sleep 30; do upload; done) &
uploader_pid=$!

set +e
(ulimit -v 8388608; timeout --signal=TERM --kill-after=20 5400 \
  "$binary" run "$out/data" 27 20 6 "$coordinate_bit") > "$out/producer.log" 2>&1
code=$?
set -e

if [[ "$code" == 0 ]]; then
  jq -e --arg id "$candidate_id" --argjson bit "$coordinate_bit" \
    '.complete == true and .candidate_id == $id and .coordinate_bit_positions[-1] == $bit and (.families|length) == 3 and all(.families[]; .same_base_for_m == [2,3,4,5,6,7,8,9])' \
    "$out/data/index.json" > "$out/index-validation.json"
fi
exit "$code"
