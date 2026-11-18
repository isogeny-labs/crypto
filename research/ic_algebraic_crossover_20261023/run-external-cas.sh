#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
run=${1:?fresh output directory}
singular=${IC_SINGULAR:?local Singular executable}
export LD_LIBRARY_PATH=${IC_SINGULAR_LIBRARY_PATH:?local Singular shared libraries}
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
bin=target/release/examples/ic_cas_correctness
timer=${IC_TIMER:-/workspace/.tools/usr/bin/time}
test -x "$singular"
test -x "$bin"
test -x "$timer"
test ! -e "$run"
meta="${run}-launch"
test ! -e "$meta"
mkdir -p "$meta"
cp "$0" "$meta/runner.sh"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum "$bin" "$singular" "$timer" "$0" examples/ic_cas_correctness.rs \
  src/bin/ecbench/structured_s4_math.rs "$study/EXTERNAL_CAS_PROTOCOL.md" > "$meta/hashes.txt"
"$singular" --version > "$meta/singular-version.txt" 2>&1
cp /workspace/.cas-cache/singular-packages.json "$meta/package-manifest.json"
sha256sum /workspace/.cas-cache/Packages-bookworm-amd64.xz > "$meta/package-index-hash.txt"
uname -a > "$meta/host.txt"
lscpu >> "$meta/host.txt"
date -u +%FT%TZ > "$meta/started.txt"
set +e
"$timer" -v -o "$meta/export-rusage.txt" timeout --kill-after=15s 900s \
  prlimit --as=1073741824 -- "$bin" export "$study" "$run" \
  > "$meta/export-stdout.txt" 2> "$meta/export-stderr.txt"
rc=$?
set -e
jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$meta/export-status.json"
if test "$rc" != 0; then exit "$rc"; fi
for input in "$run"/*.sing; do
  prefix=${input%.sing}
  jq -n --args '{argv:$ARGS.positional,timeout_seconds:60,address_space_bytes:1073741824,isolation:"L0",threads:1}' \
    -- "$singular" -q "$input" > "${prefix}.command.json"
  sha256sum "$input" >> "$meta/input-hashes.txt"
  set +e
  "$timer" -v -o "${prefix}.rusage.txt" timeout --kill-after=5s 60s \
    prlimit --as=1073741824 -- "$singular" -q "$input" \
    > "${prefix}.stdout.txt" 2> "${prefix}.stderr.txt"
  rc=$?
  set -e
  jq -n --argjson rc "$rc" '{exit_code:$rc}' > "${prefix}.status.json"
  printf '%s %s\n' "$input" "$rc"
done
set +e
"$timer" -v -o "$meta/verify-rusage.txt" timeout --kill-after=15s 900s \
  prlimit --as=1073741824 -- "$bin" verify "$run" \
  > "$meta/replay.json" 2> "$meta/verify-stderr.txt"
rc=$?
set -e
jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$meta/verify-status.json"
date -u +%FT%TZ > "$meta/ended.txt"
exit "$rc"
