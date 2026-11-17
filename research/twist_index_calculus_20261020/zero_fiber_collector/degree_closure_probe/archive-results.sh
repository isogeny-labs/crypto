#!/usr/bin/env bash
set -Eeuo pipefail
probe_archive=${1:?fresh archive path}
probe_parent=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$probe_archive"
test "$(cat "$probe_parent/runs/degree-closure-probe-panel-v1/metadata/terminal-exit")" = 0
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$probe_parent" -cf - \
  runs/degree-closure-probe-driver-controls-v1 \
  runs/degree-closure-probe-driver-controls-v2 \
  runs/degree-closure-probe-inputs-v1 \
  runs/degree-closure-probe-layout-v1 \
  runs/degree-closure-probe-layout-v2 \
  runs/degree-closure-probe-native-gate-v1 \
  runs/degree-closure-probe-native-gate-v2 \
  runs/degree-closure-probe-panel-v1 \
  runs/degree-closure-probe-progress-v1 \
  runs/degree-closure-results-layout-v1 \
  | xz -6 -T1 > "$probe_archive"
xz -t "$probe_archive"
