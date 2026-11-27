#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
cp LOCAL_PROTOCOL.md LOCAL_ENVELOPE.json run-local.sh "$out/"
git rev-parse HEAD > "$out/source-commit.txt"
uname -a > "$out/environment.txt"
c++ --version >> "$out/environment.txt"
openssl version >> "$out/environment.txt"
run_cpu=$(rg '^Cpus_allowed_list:' /proc/self/status | awk '{print $2}' | cut -d, -f1 | cut -d- -f1)
[[ "$run_cpu" =~ ^[0-9]+$ ]]
printf '%s\n' "$run_cpu" > "$out/selected-cpu.txt"
sha256sum LOCAL_PROTOCOL.md LOCAL_ENVELOPE.json run-local.sh \
  ../collection_preflight_20261124/run-admitted.sh \
  ../collection_preflight_20261124/preflight.cpp \
  ../collection_preflight_20261124/pins.hpp \
  ../split_tangent_20261123/collector.cpp \
  ../split_tangent_20261123/checker.cpp \
  ../split_tangent_20261123/rank_roots.hpp > "$out/launch-source-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 7200 taskset -c "$run_cpu" \
  bash ../collection_preflight_20261124/run-admitted.sh --admitted \
  "$out/LOCAL_ENVELOPE.json" "$out/pipeline" \
  > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/local-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/LOCAL_EXIT_CODE.txt"
sha256sum -c "$out/launch-source-manifest.sha256" > "$out/source-bytes.stdout"
exit "$status"
