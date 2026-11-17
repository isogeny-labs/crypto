#!/usr/bin/env bash
set -euo pipefail

study=$(cd "$(dirname "$0")" && pwd)
binary=$1
coordinate_bit=$2
out=$3
results_uri=$4
protocol=$5

[[ "$coordinate_bit" =~ ^(2[7-9]|3[0-9])$ ]] || {
  echo "coordinate bit must be in 27..39" >&2
  exit 2
}
[[ ! -e "$out" ]] || {
  echo "fresh output directory required; preserve old attempt: $out" >&2
  exit 2
}
attempt_id=$(jq -r --arg bit "$coordinate_bit" '.extensions.attempt_by_coordinate_bit[$bit]' "$protocol")
[[ "$attempt_id" =~ ^[0-9]{2}$ ]] || {
  echo "protocol has no valid attempt binding for coordinate bit $coordinate_bit" >&2
  exit 2
}
attempt_number=$((10#$attempt_id))
[[ "$results_uri" == "s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r${coordinate_bit}-20261031-r${attempt_number}/" ]] || {
  echo "results URI does not match the frozen r${coordinate_bit} attempt binding" >&2
  exit 2
}
jq -e --argjson bit "$coordinate_bit" '.extensions.candidate_coordinate_bits | index($bit) != null' "$protocol" >/dev/null || {
  echo "coordinate bit is absent from the frozen protocol: $coordinate_bit" >&2
  exit 2
}
expected_binary=$(jq -r '.extensions.binary_sha256' "$protocol")
expected_source=$(jq -r '.freeze.source_commit' "$protocol")
protocol_sha=$(sha256sum "$protocol" | cut -d' ' -f1)
actual_binary=$(sha256sum "$binary" | cut -d' ' -f1)
[[ "$actual_binary" == "$expected_binary" ]] || {
  echo "binary hash mismatch: $actual_binary != $expected_binary" >&2
  exit 3
}

candidate_id="spike${coordinate_bit}"
job_id=${results_uri%/}
job_id=${job_id##*/}
run_id="$job_id"
dest="${results_uri%/}/attempt-${attempt_id}/"
mkdir -p "$out"
state_root=$(mktemp -d /tmp/n83-factor-base-upload.XXXXXX)
touch "$out/upload-history.jsonl"
started=$(date -u +%FT%TZ)
uploader_pid=

# Each uploaded state is immutable and content-addressed. This avoids both
# ListBucket permission requirements and overwriting a previous partial file.
upload() {
  while IFS= read -r -d '' file; do
    relative=${file#"$out"/}
    path_sha=$(printf '%s' "$relative" | sha256sum | cut -d' ' -f1)
    snapshot="$state_root/snapshot"
    cp -- "$file" "$snapshot"
    content_sha=$(sha256sum "$snapshot" | cut -d' ' -f1)
    size=$(stat -c %s "$snapshot")
    state="$state_root/$path_sha"
    old_sha=$(cat "$state" 2>/dev/null || true)
    [[ "$old_sha" == "$content_sha" ]] && continue
    key="${dest}snapshots/${path_sha}/${content_sha}"
    aws s3 cp "$snapshot" "$key" --region us-east-2 --only-show-errors
    printf '%s %s %s\n' "$content_sha" "$size" "$key" > "$state"
    jq -nc --arg at "$(date -u +%FT%TZ)" --arg path "$relative" --arg sha "$content_sha" --arg key "$key" --argjson bytes "$size" \
      '{uploaded_at:$at,path:$path,bytes:$bytes,sha256:$sha,s3_key:$key}' >> "$out/upload-history.jsonl"
  done < <(find "$out" -type f ! -name s3-file-index.json ! -name upload-history.jsonl -print0 | sort -z)
}

write_index() {
  python3 - "$out" "$dest" "$run_id" "$candidate_id" "$coordinate_bit" "$protocol_sha" <<'PY'
import hashlib,json,os,sys
root,dest,run_id,candidate_id,bit,protocol_sha=sys.argv[1:]
files=[]
for base,dirs,names in os.walk(root):
    dirs.sort()
    for name in sorted(names):
        if name=='s3-file-index.json': continue
        path=os.path.join(base,name)
        rel=os.path.relpath(path,root).replace(os.sep,'/')
        raw=open(path,'rb').read()
        path_sha=hashlib.sha256(rel.encode()).hexdigest()
        content_sha=hashlib.sha256(raw).hexdigest()
        files.append({'path':rel,'bytes':len(raw),'sha256':content_sha,'snapshot_key':f'{dest}snapshots/{path_sha}/{content_sha}'})
index={'schema':'n83.factor-base-s3-file-index/v2','run_id':run_id,'candidate_id':candidate_id,'coordinate_bit':int(bit),'protocol_sha256':protocol_sha,'preservation':'append-only content-addressed snapshots; this index maps final local bytes to immutable S3 keys','history_path':'upload-history.jsonl','files':files}
open(os.path.join(root,'s3-file-index.json'),'w').write(json.dumps(index,indent=2)+'\n')
PY
  local index_sha
  index_sha=$(sha256sum "$out/s3-file-index.json" | cut -d' ' -f1)
  aws s3 cp "$out/s3-file-index.json" "${dest}index/s3-file-index-${index_sha}.json" --region us-east-2 --only-show-errors
}

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
    '{schema:"n83.factor-base-variant-run/v2",run_id:$run_id,candidate_id:$candidate_id,coordinate_bit:(($candidate_id|sub("spike";""))|tonumber),started_at:$start,finished_at:$end,exit_code:$code,protocol_sha256:$protocol_sha,source_commit:$source,binary_sha256:$binary,wall_valid:false,s3_upload_mode:"immutable-content-addressed-snapshots"}' \
    > "$out/run-manifest.json"
  upload || code=91
  (
    cd "$out"
    find . -type f ! -name manifest.sha256 ! -name s3-file-index.json ! -name upload-history.jsonl -print0 | sort -z | xargs -0 sha256sum > manifest.sha256
  )
  upload || code=91
  history_sha=$(sha256sum "$out/upload-history.jsonl" | cut -d' ' -f1)
  history_path_sha=$(printf '%s' upload-history.jsonl | sha256sum | cut -d' ' -f1)
  aws s3 cp "$out/upload-history.jsonl" "${dest}snapshots/${history_path_sha}/${history_sha}" --region us-east-2 --only-show-errors || code=91
  write_index || code=92
  rm -rf "$state_root"
  exit "$code"
}
trap finish EXIT

cp "$protocol" "$out/protocol.json"
cp "$study/FACTOR_BASE_VARIANT_SWEEP_DESIGN.md" "$out/"
cp "$study/FORMAT.json" "$out/"
cp "$study/RUN_OUTPUT_FORMAT.json" "$out/"
cp "$study/run-base-variant-v2.sh" "$out/runner.sh"
cp "$binary" "$out/instrument"
uname -a > "$out/host.txt"
if command -v lscpu >/dev/null 2>&1; then
  lscpu >> "$out/host.txt"
elif [[ -r /proc/cpuinfo ]]; then
  printf 'lscpu unavailable; /proc/cpuinfo follows\n' >> "$out/host.txt"
  cat /proc/cpuinfo >> "$out/host.txt"
else
  printf 'lscpu and /proc/cpuinfo unavailable\n' >> "$out/host.txt"
fi
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
