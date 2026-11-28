#!/usr/bin/env bash
# Thin build/run orchestration. Crypto computation and timing are native.
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
study="$repo/research/trace_incidence_20261116"
here="$study/runpod"
out=${TRACE_RESULT_DIR:?TRACE_RESULT_DIR is required}
mkdir -p "$out"
stage() { jq -n --arg stage "$1" --arg utc "$(date -u +%FT%TZ)" '{stage:$stage,utc:$utc}' > "$out/status.new"; mv "$out/status.new" "$out/status.json"; }
stage source-validation
sha256sum -c "$here/SOURCES.sha256" > "$out/source-check.log"
git rev-parse HEAD > "$out/source-revision.txt"
cp "$here/PROTOCOL.json" "$out/protocol.json"
cp "$here/SOURCES.sha256" "$out/sources.sha256"
uname -a > "$out/uname.txt"
lscpu --json > "$out/lscpu.json"
cat /proc/self/status | rg 'Cpus_allowed_list|Mems_allowed_list' > "$out/cgroup-affinity.txt"
for path in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /sys/fs/cgroup/cpuset.cpus.effective; do
  if test -f "$path"; then printf '%s ' "$path" >> "$out/cgroup-limits.txt"; cat "$path" >> "$out/cgroup-limits.txt"; fi
done
dpkg-query -W build-essential libflint-dev libgmp-dev libmpfr-dev nlohmann-json3-dev pari-gp > "$out/native-packages.txt"
g++ --version > "$out/compiler.txt"
stage native-build-and-qualification
make -C "$here" > "$out/native-build.log" 2>&1
cp "$here/build/benchmark" "$out/benchmark.qualifier"
chmod 0555 "$out/benchmark.qualifier"
sha256sum "$out/benchmark.qualifier" > "$out/build-binary.sha256"
export FLINT_PREFIX=/usr JSON_INCLUDE=/usr/include
for check in translation paper integration cli edge; do
  bash "$study/native/check-$check.sh" > "$out/native-$check-check.log" 2>&1
done
bash "$here/check.sh" > "$out/runpod-driver-check.log" 2>&1
stage rho-build
git sparse-checkout add --no-cone --stdin < "$here/rho-sparse-paths.txt"
export RUSTUP_TOOLCHAIN=1.100.0 CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=/workspace/cargo-target
set +e
cargo build --locked --release --example trace_incidence_rho > "$out/rho-build.log" 2>&1
rho_build=$?
set -e
jq -n --argjson status "$rho_build" '{build_exit:$status}' > "$out/rho-build-status.json"
if ((rho_build==0)); then
  cp "$CARGO_TARGET_DIR/release/examples/trace_incidence_rho" "$out/rho.qualifier"
  chmod 0555 "$out/rho.qualifier"
  sha256sum "$out/rho.qualifier" > "$out/rho-build-binary.sha256"
  rustc -Vv > "$out/rustc.txt"
fi
stage timing
first_cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1)
printf '%s\n' "$first_cpu" > "$out/selected-cpu.txt"
bench="$out/benchmark.qualifier"
set -o noclobber
sha256sum "$bench" > "$out/pre-call-binary.sha256"
if ((rho_build==0)); then sha256sum "$out/rho.qualifier" > "$out/rho-pre-call-binary.sha256"; fi
set +o noclobber
row_index=0
one() {
  local phase=$1 round=$2 label=$3 fixture=$4 attempt=$5
  shift 5
  row_index=$((row_index+1))
  local stem="$out/row-$row_index" code=0
  /usr/bin/time -q -f '{"wall_seconds":%e,"user_seconds":%U,"system_seconds":%S,"peak_rss_kib":%M}' -o "$stem.time" \
    timeout -k 2 45 taskset -c "$first_cpu" "$@" > "$stem.json" 2> "$stem.stderr" || code=$?
  if ! jq -e 'type=="object"' "$stem.json" >/dev/null 2>&1; then
    jq -n --argjson exit "$code" '{schema:"trace-runpod-row/v1",status:"failed",wrapper_exit:$exit}' > "$stem.json"
  fi
  jq --arg phase "$phase" --arg label "$label" --arg fixture "$fixture" --argjson round "$round" --argjson attempt "$attempt" --argjson exit "$code" --argjson index "$row_index" \
    '.+{phase:$phase,round:$round,arm_label:$label,source_fixture:$fixture,attempt:$attempt,wrapper_exit:$exit,row_index:$index}' "$stem.json" >> "$out/rows.jsonl"
  if test -s "$stem.time"; then
    jq -c --argjson index "$row_index" '.+{row_index:$index}' "$stem.time" >> "$out/process-times.jsonl" 2>/dev/null || true
  fi
  cat "$stem.stderr" >> "$out/row-stderr.log"
  LAST_STATUS=$(jq -r '.status' "$stem.json")
  rm "$stem.json" "$stem.time" "$stem.stderr"
}
root_arms=(dense trace pullback dense-isogeny trace-isogeny pullback-isogeny flint pullback-aa)
incidence_arms=(known known-aa pairs trace dense pullback pullback-isogeny)
cases=("$study"/historical/prime_fixtures/b???_c?_natural_s?.json)
test "${#cases[@]}" -eq 60
for round in 0 1 2 3 4 5; do
  phase=measured; if ((round==0)); then phase=warmup; fi
  for ((position=0;position<${#root_arms[@]};position++)); do
    arm=${root_arms[$(((position+round)%${#root_arms[@]}))]}
    actual_arm=$arm; if [[ $arm == pullback-aa ]]; then actual_arm=pullback; fi
    one "$phase" "$round" "$arm" paper83 0 "$bench" root "$actual_arm" 32
  done
  while IFS= read -r fixture; do
    if ((round%2)); then arms=(trace dense); else arms=(dense trace); fi
    for arm in "${arms[@]}"; do one "$phase" "$round" "$arm" "$(basename "$fixture")" 0 "$bench" translation "$arm" "$fixture"; done
  done < <(rg --files "$study/historical/fast_trace/results/fixtures" "$study/historical/fast_trace/results_large/fixtures" | sort)
  for fixture in "${cases[@]}"; do
    base_seed=$(jq -r '.public_seed' "$fixture")
    for ((position=0;position<${#incidence_arms[@]};position++)); do
      arm=${incidence_arms[$(((position+round)%${#incidence_arms[@]}))]}
      for attempt in 0 1 2 3; do
        pool_seed=$((base_seed+attempt*2654435761))
        one "$phase" "$round" "$arm" "$(basename "$fixture")" "$attempt" "$bench" incidence "$arm" "$fixture" "$pool_seed" "$here/auxiliary.gp"
        if [[ $LAST_STATUS == relation || $LAST_STATUS == failed ]]; then break; fi
      done
    done
    if ((rho_build==0)); then
      one "$phase" "$round" rho "$(basename "$fixture")" 0 env TRACE_INCIDENCE_SOURCE_REVISION="$(git rev-parse HEAD)" \
        "$out/rho.qualifier" "$fixture" "$base_seed" 4 8 0 10000000
    fi
  done
done
sha256sum "$bench" > "$out/post-call-binary.sha256"
cmp "$out/pre-call-binary.sha256" "$out/post-call-binary.sha256"
if ((rho_build==0)); then
  sha256sum "$out/rho.qualifier" > "$out/rho-post-call-binary.sha256"
  cmp "$out/rho-pre-call-binary.sha256" "$out/rho-post-call-binary.sha256"
fi
stage completed
