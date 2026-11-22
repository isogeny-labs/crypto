#!/usr/bin/env bash
set -Eeuo pipefail
binding_root=research/twist_index_calculus_20261020/zero_fiber_collector/curve_registry_binding
if test "${1:-}" != --frozen; then
  binding_out=${1:?fresh output directory}
  binding_gate=${2:?passing native gate}
  test ! -e "$binding_out"
  test "$(cat "$binding_gate/metadata/terminal-exit")" = 0
  sha256sum -c "$binding_gate/metadata/adapter-sources.sha256"
  sha256sum -c "$binding_gate/metadata/root-sources.sha256"
  sha256sum -c "$binding_gate/metadata/binary.sha256"
  mkdir -p "$binding_out/metadata"
  binding_out=$(realpath "$binding_out")
  binding_gate=$(realpath "$binding_gate")
  cp "$binding_root/run-binding.sh" "$binding_out/metadata/executed-script.sh"
  cp "$binding_gate/binder" "$binding_out/binder"
  cp research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/workload-v1.json "$binding_out/metadata/workload.json"
  git rev-parse HEAD > "$binding_out/metadata/source-commit.txt"
  sha256sum "$binding_out/metadata/executed-script.sh" "$binding_out/binder" "$binding_out/metadata/workload.json" > "$binding_out/metadata/execution.sha256"
  exec bash "$binding_out/metadata/executed-script.sh" --frozen "$binding_out" "$binding_gate"
fi
binding_out=${2:?frozen output}
binding_gate=${3:?native gate}
test ! -e "$binding_out/metadata/started-utc.txt"
trap 'binding_rc=$?; date -u +%FT%TZ > "$binding_out/metadata/finished-utc.txt"; printf "%s\n" "$binding_rc" > "$binding_out/metadata/terminal-exit"' EXIT
date -u +%FT%TZ > "$binding_out/metadata/started-utc.txt"
printf '%s\n' "$$" > "$binding_out/metadata/owner-pid.txt"
printf '%s\n' 'Both complete native catalog processes in one common reservation and one aggregate 1200-second / 8 GiB phase; no curve registration or source changes until terminal reconciliation' > "$binding_out/metadata/resource-envelope.txt"
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
binding_command=(python3 tools/isolated_bench.py busy -- /usr/bin/time -v -o "$binding_out/metadata/native-pair.time.txt" prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s /bin/bash -c 'set -eu; date -u +%FT%TZ > "$3/metadata/native-pair.actual-started-utc.txt"; "$1" bind "$2" "$3/first"; "$1" bind "$2" "$3/aa"; cmp "$3/first/catalog.json" "$3/aa/catalog.json"; date -u +%FT%TZ > "$3/metadata/native-pair.actual-finished-utc.txt"' binding-pair "$binding_out/binder" "$binding_out/metadata/workload.json" "$binding_out")
printf '%q ' "${binding_command[@]}" > "$binding_out/metadata/native-pair.command.txt"
date -u +%FT%TZ > "$binding_out/metadata/native-pair.queued-utc.txt"
if "${binding_command[@]}" > "$binding_out/metadata/native-pair.stdout" 2> "$binding_out/metadata/native-pair.stderr"; then binding_rc=0; else binding_rc=$?; fi
printf '%s\n' "$binding_rc" > "$binding_out/metadata/native-pair.exit"
test "$binding_rc" = 0
sha256sum "$binding_out/first/catalog.json" "$binding_out/aa/catalog.json" > "$binding_out/metadata/catalog-parity.sha256"
sha256sum -c "$binding_out/metadata/execution.sha256" > "$binding_out/metadata/execution-check-after.txt"
sha256sum -c "$binding_gate/metadata/root-sources.sha256" > "$binding_out/metadata/root-source-check-after.txt"
sha256sum -c "$binding_gate/metadata/adapter-sources.sha256" > "$binding_out/metadata/adapter-source-check-after.txt"
