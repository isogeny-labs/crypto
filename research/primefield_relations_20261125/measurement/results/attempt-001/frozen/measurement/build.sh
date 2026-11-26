#!/usr/bin/env bash
# Thin custody/build wrapper. All measured computations are in native Rust.
set -euo pipefail
study_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_dir=$(git -C "$study_dir" rev-parse --show-toplevel)
attempt=${1:?usage: build.sh ABSOLUTE_UNUSED_ATTEMPT_DIRECTORY}
[[ "$attempt" = /* ]] || { echo 'Attempt path must be absolute' >&2; exit 2; }
for tool in jq sha256sum timeout taskset rg; do command -v "$tool" >/dev/null; done
compiler=${RUSTC:-rustc}
compiler=$(command -v "$compiler")
mkdir "$attempt"
mkdir -p "$attempt/build" "$attempt/frozen/measurement/src" \
  "$attempt/frozen/shared-terminal-gate/results/attempt-001"
cp "$study_dir"/{Cargo.toml,Cargo.lock,QUALIFIER.md,PROTOCOL.md,seed-manifest.json,fixture-identity.json,build.sh,run.sh} \
  "$attempt/frozen/measurement/"
cp "$study_dir"/src/* "$attempt/frozen/measurement/src/"
cp "$study_dir/../shared-terminal-gate/results/attempt-001/result.json" \
  "$attempt/frozen/shared-terminal-gate/results/attempt-001/result.json"
(
  cd "$attempt"
  rg --files -0 frozen | sort -z | xargs -0 sha256sum > source-manifest.sha256
)
source_commit=$(git -C "$repo_dir" rev-parse HEAD)
source_branch=$(git -C "$repo_dir" branch --show-current)
compiler_hash=$(sha256sum "$compiler" | cut -d ' ' -f 1)
source_hash=$(sha256sum "$attempt/source-manifest.sha256" | cut -d ' ' -f 1)
jq -n --arg source_commit "$source_commit" --arg branch "$source_branch" \
  --arg compiler "$compiler" --arg compiler_sha256 "$compiler_hash" \
  --arg source_manifest_sha256 "$source_hash" --arg started "$(date -u +%FT%TZ)" \
  '{schema:"isogeny.m31-counts-build/v1",source_commit:$source_commit,branch:$branch,
    compiler:$compiler,compiler_sha256:$compiler_sha256,
    source_manifest_sha256:$source_manifest_sha256,started_utc:$started,
    flags:["--edition=2021","-O"],measured_workloads_executed:0}' > "$attempt/build-precall.json"
build_complete() {
  code=$?
  jq -n --arg completed "$(date -u +%FT%TZ)" --argjson code "$code" \
    '{schema:"isogeny.m31-counts-build-post/v1",completed_utc:$completed,exit_code:$code,
      measured_workloads_executed:0}' > "$attempt/build-postcall.json"
}
trap build_complete EXIT
"$compiler" -Vv > "$attempt/toolchain.txt"
timeout 60s "$compiler" --edition=2021 -O \
  "$attempt/frozen/measurement/src/main.rs" -o "$attempt/build/count-selector" \
  > "$attempt/compile.stdout" 2> "$attempt/compile.stderr"
timeout 60s "$compiler" --edition=2021 -O --test \
  "$attempt/frozen/measurement/src/main.rs" -o "$attempt/build/count-selector-tests" \
  > "$attempt/compile-tests.stdout" 2> "$attempt/compile-tests.stderr"
(
  cd "$attempt"
  sha256sum build/count-selector build/count-selector-tests > binaries.sha256
)
uname -a > "$attempt/host.txt"
sed -n '1,/^$/p' /proc/cpuinfo > "$attempt/cpuinfo-first-processor.txt"
cat /proc/self/status > "$attempt/process-status.txt"
printf '%s\n' "$attempt"
