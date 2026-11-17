#!/usr/bin/env bash
# Thin native measurement orchestration. Requires explicit user authorization.
set -euo pipefail
test "$#" -eq 3 || { echo 'usage: run-local.sh FROZEN_SOURCE_ROOT NEW_OUTPUT_DIR AUTHORIZATION_JSON' >&2;exit 2; }
snapshot=$(realpath "$1")
out=$2
authorization=$3
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study="$snapshot/research/trace_incidence_20261116"
source_revision=f1549fc821220641406b51a60324ef4ec81ab69b
rho_source=c0176f942c03a549a98cb680b816e864b8820333
jq -e '.authority=="explicit user instruction" and .local_execution==true and .temporary_local_checkpoint_and_raw_custody==true' "$authorization" >/dev/null
test ! -e "$out" || { echo 'Output path already exists; never overwrite a run' >&2;exit 2; }
mkdir "$out"
cp "$authorization" "$out/authorization.json"
cp "$here/LOCAL_PROTOCOL.json" "$out/protocol.json"
cp "$here/run-local-v2.sh" "$out/runner.sh"
cp "$here/LOCAL_AMENDMENT_2.json" "$out/amendment.json"
(cd "$snapshot" && sha256sum -c "$study/accelerated/SOURCES_v2.sha256") > "$out/source-check.txt"
cp "$study/accelerated/PROTOCOL.json" "$out/parent-protocol.json"
cp "$study/accelerated/SOURCES_v2.sha256" "$out/source-manifest.sha256"
cp /workspace/native-deps/time/usr/bin/time "$out/time.qualifier"
chmod 0555 "$out/time.qualifier"
cp /tmp/trace-accelerated-check.oVXRnv/benchmark.qualifier "$out/benchmark.qualifier"
cp /tmp/trace-accelerated-check.oVXRnv/qualifier "$out/audit.qualifier"
cp /workspace/crypto-pr/research/trace_incidence_20261116/accelerated/build/analyze "$out/analyze.qualifier"
cp /workspace/crypto_pr_context/runpod-c0176f942/extracted/rho.qualifier "$out/rho.qualifier"
chmod 0555 "$out/benchmark.qualifier" "$out/audit.qualifier" "$out/analyze.qualifier" "$out/rho.qualifier"
cat > "$out/expected-binary.sha256" <<EOF
efc0d1112e36ec76c14bf7508e287935cf1631d9d0d91fb532802d07aef51cea  $out/time.qualifier
3b19c063a36b6f50e5f7d9d83a476f05e961dfcd0ca54befe1d185be82de7317  $out/benchmark.qualifier
74c57ba0f5208bdf0c90f6a906f22678d4a9ff08ac06c8643294e37711f030f9  $out/audit.qualifier
b0e765985e1ce43582b12af94a1b6859297a65076142bfb7be589105208eed89  $out/analyze.qualifier
a22eea11abeb41aea88a60f99860a5f9d6ba2dd5f367e6b6e66001c0ef6d09e8  $out/rho.qualifier
EOF
sha256sum -c "$out/expected-binary.sha256" > "$out/binary-check.txt"
sha256sum "$out/benchmark.qualifier" "$out/audit.qualifier" "$out/analyze.qualifier" "$out/rho.qualifier" "$out/time.qualifier" > "$out/build-binary.sha256"
set -o noclobber
cp "$out/build-binary.sha256" "$out/pre-call-binary.sha256"
set +o noclobber
uname -a > "$out/uname.txt"
lscpu > "$out/lscpu.txt"
for path in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /proc/self/status; do
  if test -r "$path"; then printf '%s\n' "$path" >> "$out/cgroup-and-process.txt";cat "$path" >> "$out/cgroup-and-process.txt";fi
done
for name in benchmark rho; do
  ldd "$out/$name.qualifier" > "$out/$name-ldd.txt"
done
mapfile -t libraries < <(awk '/=> \//{print $3} /^\//{print $1}' "$out/benchmark-ldd.txt" "$out/rho-ldd.txt" | sort -u)
sha256sum "${libraries[@]}" > "$out/dependencies-pre-call.sha256"
stage() { jq -n --arg stage "$1" --arg utc "$(date -u +%FT%TZ)" '{stage:$stage,utc:$utc}' > "$out/status.json";jq -c . "$out/status.json"; }
trap 'code=$?; jq -n --argjson exit "$code" --arg utc "$(date -u +%FT%TZ)" "{exit_status:\$exit,utc:\$utc}" > "$out/runner-exit.json"' EXIT
stage qualification
"$out/audit.qualifier" "$study" > "$out/audit-checks.json"
jq -e '.status=="passed" and .timings_collected==false' "$out/audit-checks.json" >/dev/null
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1)
printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
mapfile -t arms < <(jq -r '.arms[]|[.label,.generator,.norm]|@tsv' "$study/accelerated/PROTOCOL.json")
fixtures=("$study"/historical/prime_fixtures/b???_c?_natural_s?.json)
test "${#fixtures[@]}" -eq 60
index=0
one() {
  local phase=$1 round=$2 arm=$3 fixture=$4 attempt=$5
  shift 5
  index=$((index+1))
  local stem="$out/row-$index" code=0
  "$out/time.qualifier" -q -f '{"wall_seconds":%e,"user_seconds":%U,"system_seconds":%S,"peak_rss_kib":%M}' -o "$stem.time" \
    timeout -k 2 45 taskset -c "$cpu" "$@" > "$stem.json" 2> "$stem.stderr" || code=$?
  if ! jq -e 'type=="object"' "$stem.json" >/dev/null 2>&1; then
    jq -n --argjson code "$code" '{schema:"trace-runpod-row/v1",status:"failed",wrapper_exit:$code}' > "$stem.json"
  elif ((code!=0)); then
    jq -c --argjson code "$code" '.+{status:"failed",wrapper_exit:$code}' "$stem.json" > "$stem.failed"
    mv "$stem.failed" "$stem.json"
  fi
  jq -c --arg phase "$phase" --arg arm "$arm" --arg fixture "$fixture" --argjson round "$round" --argjson attempt "$attempt" --argjson code "$code" --argjson index "$index" \
    '.+{phase:$phase,round:$round,arm_label:$arm,source_fixture:$fixture,attempt:$attempt,wrapper_exit:$code,row_index:$index,execution_backend:"local workspace CPU"}' "$stem.json" >> "$out/rows.jsonl"
  if test -s "$stem.time"; then jq -c --argjson index "$index" '.+{row_index:$index}' "$stem.time" >> "$out/process-times.jsonl" || true;fi
  cat "$stem.stderr" >> "$out/stderr.log"
  LAST_STATUS=$(jq -r '.status' "$stem.json")
  for temporary in "$stem.json" "$stem.time" "$stem.stderr"; do
    if test -f "$temporary"; then rm "$temporary"; fi
  done
}
for round in 0 1 2 3 4 5; do
  phase=measured;if ((round==0));then phase=warmup;fi
  stage "$phase-round-$round"
  for fixture in "${fixtures[@]}"; do
    base_seed=$(jq -r '.public_seed' "$fixture")
    for ((position=0;position<${#arms[@]};position++)); do
      IFS=$'\t' read -r label generator norm <<< "${arms[$(((position+round)%${#arms[@]}))]}"
      for attempt in 0 1 2 3; do
        seed=$((base_seed+attempt*2654435761))
        one "$phase" "$round" "$label" "$(basename "$fixture")" "$attempt" "$out/benchmark.qualifier" "$generator" "$norm" "$fixture" "$seed"
        if [[ $LAST_STATUS == relation || $LAST_STATUS == failed ]];then break;fi
      done
    done
    one "$phase" "$round" rho "$(basename "$fixture")" 0 env TRACE_INCIDENCE_SOURCE_REVISION="$rho_source" "$out/rho.qualifier" "$fixture" "$base_seed" 4 8 0 10000000
  done
done
sha256sum "$out/benchmark.qualifier" "$out/audit.qualifier" "$out/analyze.qualifier" "$out/rho.qualifier" "$out/time.qualifier" > "$out/post-call-binary.sha256"
cmp "$out/pre-call-binary.sha256" "$out/post-call-binary.sha256"
sha256sum "${libraries[@]}" > "$out/dependencies-post-call.sha256"
cmp "$out/dependencies-pre-call.sha256" "$out/dependencies-post-call.sha256"
"$out/analyze.qualifier" "$out/rows.jsonl" > "$out/summary.json"
jq -n --arg candidate "$source_revision" --arg rho "$rho_source" '{candidate_source_revision:$candidate,rho_binary_source_revision:$rho,backend:"local workspace CPU",fresh_source_build:false,s3_admission:false}' > "$out/provenance.json"
stage completed
