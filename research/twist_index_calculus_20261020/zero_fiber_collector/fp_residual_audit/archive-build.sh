#!/usr/bin/env bash
set -Eeuo pipefail
fp_archive=${1:?fresh output archive path}
fp_parent=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$fp_archive"
test "$(cat "$fp_parent/runs/fp-residual-audit-native-gate-v1/metadata/exit")" = 0
test "$(cat "$fp_parent/runs/fp-residual-audit-v1/metadata/terminal-exit")" = 0
test "$(cat "$fp_parent/runs/fp-residual-audit-v1/metadata/parity.exit")" = 0
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -C "$fp_parent" -cf - \
  runs/fp-residual-audit-native-gate-v1 \
  runs/fp-residual-audit-v1 \
  runs/fp-residual-layout-v1 \
  | xz -6 -T1 > "$fp_archive"
xz -t "$fp_archive"
