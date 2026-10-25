#!/usr/bin/env bash
# Bounded launcher controls. Invoke directly; the outer-reservation test owns it.
set -euo pipefail
control_out=${1:?fresh control directory}
control_workload=${2:?frozen workload JSON}
control_gate=${3:?successful native gate}
control_panel=${4:?successful panel-tool build}
control_study=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$control_out"
mkdir -p "$control_out"
cp "$control_study/phase2/run-panel.sh" "$control_out/executed-driver.sh"
cp "$0" "$control_out/executed-control.sh"
sha256sum "$control_out/executed-driver.sh" "$control_out/executed-control.sh" > "$control_out/executed-sources.sha256"
date -u +%FT%TZ > "$control_out/started-utc.txt"
control_rc=0
python3 tools/isolated_bench.py busy -- timeout --signal=TERM --kill-after=2s 10s \
  bash "$control_out/executed-driver.sh" "$control_out/forbidden-output" x x x \
  > "$control_out/outer-busy.stdout" 2> "$control_out/outer-busy.stderr" || control_rc=$?
printf '%s\n' "$control_rc" > "$control_out/outer-busy.exit"
test "$control_rc" = 64
test ! -e "$control_out/forbidden-output"

# Exercise the actual driver, with real sealed runners, but prevent its very
# first phase from starting. Preserve a pre-existing command byte-for-byte.
control_duplicate="$control_out/duplicate-phase"
mkdir -p "$control_duplicate/metadata"
cp "$control_out/executed-driver.sh" "$control_duplicate/metadata/executed-script.sh"
printf '%s\n' 'preserved pre-existing phase marker' > "$control_duplicate/metadata/input-seals.command.txt"
sha256sum "$control_duplicate/metadata/input-seals.command.txt" > "$control_out/original-marker.sha256"
control_rc=0
timeout --signal=TERM --kill-after=2s 120s bash "$control_out/executed-driver.sh" \
  --frozen "$control_duplicate" "$control_workload" "$control_gate" "$control_panel" \
  > "$control_out/duplicate-phase.stdout" 2> "$control_out/duplicate-phase.stderr" || control_rc=$?
printf '%s\n' "$control_rc" > "$control_out/duplicate-phase.exit"
test "$control_rc" = 65
sha256sum -c "$control_out/original-marker.sha256" > "$control_out/original-marker-unchanged.txt"
test ! -e "$control_duplicate/metadata/input-seals.started-utc.txt"
test ! -e "$control_duplicate/cases"
sha256sum -c "$control_out/executed-sources.sha256" > "$control_out/executed-sources-unchanged.txt"
cmp "$control_out/executed-driver.sh" "$control_study/phase2/run-panel.sh"
date -u +%FT%TZ > "$control_out/finished-utc.txt"
printf '%s\n' 'PASS: nested reservation exits 64 without output; existing phase exits 65 with unchanged marker and no candidate execution.' > "$control_out/result.txt"
