#!/usr/bin/env bash
# Wait for one reconciled native-gate PID. Never restart a missing owner.
set -euo pipefail
zero_control=${1:?fresh controller directory}
zero_gate_pid=${2:?verified live busy-wrapper PID}
zero_gate=${3:?exact gate directory argument}
zero_panel=${4:?fresh panel output directory}
zero_public=${5:?public input directory}
zero_reference=${6:?immutable reference root}
zero_point_summary=${7:?immutable point replay summary}
zero_study=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$zero_control"
mkdir -p "$zero_control"
cp "$0" "$zero_control/executed-script.sh"
printf '%s\n' "$$" > "$zero_control/controller-pid.txt"
printf '%s\n' "$zero_gate_pid" > "$zero_control/dependency-pid.txt"
printf '%s\n' "$zero_gate" > "$zero_control/dependency-directory.txt"
git rev-parse HEAD > "$zero_control/source-commit.txt"
date -u +%FT%TZ > "$zero_control/started-utc.txt"
while true; do
  if test -f "$zero_gate/metadata/exit"; then
    zero_gate_rc=$(cat "$zero_gate/metadata/exit")
    printf '%s\n' "$zero_gate_rc" > "$zero_control/dependency-exit.txt"
    test "$zero_gate_rc" = 0
    break
  fi
  if ! kill -0 "$zero_gate_pid" 2>/dev/null; then
    printf 'Dependency owner missing without a successful terminal receipt; no restart.\n' > "$zero_control/refusal.txt"
    exit 75
  fi
  if ! tr '\0' '\n' < "/proc/$zero_gate_pid/cmdline" | grep -Fqx -- "$zero_gate"; then
    printf 'Dependency PID does not name the recorded gate; no restart.\n' > "$zero_control/refusal.txt"
    exit 75
  fi
  sleep 10
done
sha256sum -c "$zero_gate/metadata/sources.sha256" > "$zero_control/gate-sources-unchanged.txt"
date -u +%FT%TZ > "$zero_control/panel-launch-utc.txt"
exec bash "$zero_study/run-panel.sh" "$zero_panel" "$zero_public" "$zero_reference" "$zero_point_summary" "$zero_gate"
