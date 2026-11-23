#!/usr/bin/env bash
# Correctness-only job. No clock sample or speedup estimate is produced.
set -euo pipefail
if [ "$#" -ne 3 ]; then
  echo 'usage: pod-run.sh BINARY EXPECTED_SHA256 FRESH_OUTPUT_DIRECTORY' >&2
  exit 2
fi
binary="$(realpath "$1")"
expected=$2
out=$3
printf '%s  %s\n' "$expected" "$binary" | sha256sum --check --status
mkdir "$out"  # refuse reuse; preserve interrupted and unsuccessful runs
out="$(realpath "$out")"
exec > >(tee "$out/job.log") 2>&1
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/start.txt"
trap 'rc=$?; printf "%s\n" "$rc" > "$out/exit-code.txt"; date -u +"%Y-%m-%dT%H:%M:%SZ" > "$out/end.txt"' EXIT
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
cat /proc/self/status | sed -n '/Cpus_allowed_list/p' >> "$out/host.txt"
for path in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /sys/fs/cgroup/cpuset.cpus.effective; do
  if [ -r "$path" ]; then printf '%s: ' "$path" >> "$out/host.txt"; cat "$path" >> "$out/host.txt"; fi
done
sha256sum "$binary" > "$out/binary.sha256"
for mode in toy trace; do
  timeout 120 "$binary" "$mode" > "$out/$mode.json"
  timeout 120 "$binary" "$mode" > "$out/$mode-repeat.json"
  cmp "$out/$mode.json" "$out/$mode-repeat.json"
done
(cd "$out" && sha256sum toy.json toy-repeat.json trace.json trace-repeat.json host.txt binary.sha256 > SHA256SUMS)
printf '%s\n' 'producer deterministic replay completed; no timing or signed verification'
