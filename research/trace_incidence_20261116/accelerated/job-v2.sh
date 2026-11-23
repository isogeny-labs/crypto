#!/usr/bin/env bash
# Thin native workload orchestration; does not allocate a pod or bypass admission.
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
here="$repo/research/trace_incidence_20261116/accelerated"
study=$(dirname "$here")
out=${TRACE_RESULT_DIR:?external supervisor must set TRACE_RESULT_DIR}
receipt=${TRACE_ACCELERATED_CHECKPOINT_RECEIPT:?new source-bound S3 admission receipt required}
revision=$(git rev-parse HEAD)
jq -e --arg source "$revision" --arg closure "$(sha256sum "$here/SOURCES_v2.sha256" | cut -d' ' -f1)" --arg protocol "$(sha256sum "$here/PROTOCOL.json" | cut -d' ' -f1)" \
  '.gate_exit==0 and .source_revision==$source and .source_manifest_sha256==$closure and .protocol_sha256==$protocol and (.checkpoint_state|length)>0' "$receipt" >/dev/null
sha256sum -c "$here/SOURCES_v2.sha256" > /dev/null
mkdir -p "$out"
cp "$receipt" "$out/admission.json"
uname -a > "$out/uname.txt"
lscpu > "$out/lscpu.txt"
for path in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /proc/self/status; do
  if test -r "$path"; then cat "$path" >> "$out/cgroup-and-process.txt"; fi
done
export CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/workspace/cargo-target} RUSTUP_TOOLCHAIN=1.100.0
make -C "$here" > "$out/build.log" 2>&1
cargo test --manifest-path "$here/rust/Cargo.toml" --release --locked > "$out/rust-checks.log" 2>&1
bash "$here/check.sh" > "$out/native-checks.log" 2>&1
bash "$here/check-analyzer.sh" > "$out/analyzer-checks.log" 2>&1
cargo build --locked --release --example trace_incidence_rho > "$out/rho-build.log" 2>&1
rustc -Vv > "$out/rustc.txt"
g++ --version > "$out/compiler.txt"
dpkg-query -W libflint-dev libgmp-dev libmpfr-dev nlohmann-json3-dev > "$out/packages.txt"
cp "$here/build/benchmark" "$out/benchmark.qualifier"
cp "$CARGO_TARGET_DIR/release/examples/trace_incidence_rho" "$out/rho.qualifier"
cp "$here/build/analyze" "$out/analyze.qualifier"
chmod 0555 "$out/benchmark.qualifier" "$out/rho.qualifier" "$out/analyze.qualifier"
set -o noclobber
sha256sum "$out/benchmark.qualifier" "$out/rho.qualifier" "$out/analyze.qualifier" > "$out/build-binary.sha256"
cp "$out/build-binary.sha256" "$out/pre-call-binary.sha256"
set +o noclobber
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1)
printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
mapfile -t arms < <(jq -r '.arms[]|[.label,.generator,.norm]|@tsv' "$here/PROTOCOL.json")
fixtures=("$study"/historical/prime_fixtures/b???_c?_natural_s?.json)
test "${#fixtures[@]}" -eq 60
index=0
one() {
  local phase=$1 round=$2 arm=$3 fixture=$4 attempt=$5
  shift 5
  index=$((index+1))
  local stem="$out/row-$index" code=0
  /usr/bin/time -q -f '{"wall_seconds":%e,"user_seconds":%U,"system_seconds":%S,"peak_rss_kib":%M}' -o "$stem.time" \
    timeout -k 2 45 taskset -c "$cpu" "$@" > "$stem.json" 2> "$stem.stderr" || code=$?
  if ! jq -e 'type=="object"' "$stem.json" >/dev/null 2>&1; then
    jq -n --argjson code "$code" '{schema:"trace-runpod-row/v1",status:"failed",wrapper_exit:$code}' > "$stem.json"
  elif ((code!=0)); then
    jq -c --argjson code "$code" '.+{status:"failed",wrapper_exit:$code}' "$stem.json" > "$stem.failed"
    mv "$stem.failed" "$stem.json"
  fi
  jq -c --arg phase "$phase" --arg arm "$arm" --arg fixture "$fixture" --argjson round "$round" --argjson attempt "$attempt" --argjson code "$code" --argjson index "$index" \
    '.+{phase:$phase,round:$round,arm_label:$arm,source_fixture:$fixture,attempt:$attempt,wrapper_exit:$code,row_index:$index}' "$stem.json" >> "$out/rows.jsonl"
  if test -s "$stem.time"; then jq -c --argjson index "$index" '.+{row_index:$index}' "$stem.time" >> "$out/process-times.jsonl" || true; fi
  cat "$stem.stderr" >> "$out/stderr.log"
  LAST_STATUS=$(jq -r '.status' "$stem.json")
  rm "$stem.json" "$stem.time" "$stem.stderr"
}
for round in 0 1 2 3 4 5; do
  phase=measured;if ((round==0)); then phase=warmup; fi
  for fixture in "${fixtures[@]}"; do
    base_seed=$(jq -r '.public_seed' "$fixture")
    for ((position=0;position<${#arms[@]};position++)); do
      IFS=$'\t' read -r label generator norm <<< "${arms[$(((position+round)%${#arms[@]}))]}"
      for attempt in 0 1 2 3; do
        seed=$((base_seed+attempt*2654435761))
        one "$phase" "$round" "$label" "$(basename "$fixture")" "$attempt" "$out/benchmark.qualifier" "$generator" "$norm" "$fixture" "$seed"
        if [[ $LAST_STATUS == relation || $LAST_STATUS == failed ]]; then break; fi
      done
    done
    one "$phase" "$round" rho "$(basename "$fixture")" 0 env TRACE_INCIDENCE_SOURCE_REVISION="$revision" "$out/rho.qualifier" "$fixture" "$base_seed" 4 8 0 10000000
  done
done
sha256sum "$out/benchmark.qualifier" "$out/rho.qualifier" "$out/analyze.qualifier" > "$out/post-call-binary.sha256"
cmp "$out/pre-call-binary.sha256" "$out/post-call-binary.sha256"
"$out/analyze.qualifier" "$out/rows.jsonl" > "$out/summary.json"
