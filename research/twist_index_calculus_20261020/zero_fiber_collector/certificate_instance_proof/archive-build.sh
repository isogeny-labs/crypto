#!/usr/bin/env bash
# Deterministic packaging of closed runs only; execute from repository root.
set -Eeuo pipefail
instance_archive=${1:?fresh output archive path}
instance_parent=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$instance_archive"
for instance_version in v1 v2; do
  test "$(cat "$instance_parent/runs/certificate-instance-native-gate-$instance_version/metadata/exit")" = 0
  test "$(cat "$instance_parent/runs/certificate-instance-proof-$instance_version/metadata/terminal-exit")" = 1
done
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$instance_parent" -cf - \
  runs/certificate-instance-native-gate-v1 \
  runs/certificate-instance-native-gate-v2 \
  runs/certificate-instance-proof-v1 \
  runs/certificate-instance-proof-v2 \
  runs/certificate-instance-layout-v1 \
  runs/degree-closure-probe-progress-v1 \
  runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-raw16 \
  runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-closure8 \
  runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys1-raw16 \
  | xz -6 -T1 > "$instance_archive"
xz -t "$instance_archive"
