#!/usr/bin/env bash
# Bounded orchestration controls; invoke directly.
set -Eeuo pipefail
probe_control_out=${1:?fresh control output}
probe_control_gate=${2:?successful implementation gate}
probe_control_prepared=${3:?successful workload preparation}
probe_control_workload_sha=${4:?registered workload SHA256}
probe_driver=research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe/run-panel.sh
test ! -e "$probe_control_out"
mkdir -p "$probe_control_out"
cp "$probe_driver" "$probe_control_out/executed-driver.sh"
cp "$0" "$probe_control_out/executed-control.sh"
sha256sum "$probe_control_out/executed-driver.sh" "$probe_control_out/executed-control.sh" > "$probe_control_out/sources.sha256"
date -u +%FT%TZ > "$probe_control_out/started-utc.txt"
probe_control_rc=0
python3 tools/isolated_bench.py busy -- timeout --signal=TERM --kill-after=2s 10s \
  bash "$probe_control_out/executed-driver.sh" "$probe_control_out/forbidden-output" x x x \
  > "$probe_control_out/outer-busy.stdout" 2> "$probe_control_out/outer-busy.stderr" || probe_control_rc=$?
printf '%s\n' "$probe_control_rc" > "$probe_control_out/outer-busy.exit"
test "$probe_control_rc" = 64
test ! -e "$probe_control_out/forbidden-output"
probe_duplicate="$probe_control_out/duplicate-phase"
mkdir -p "$probe_duplicate/metadata"
cp "$probe_control_out/executed-driver.sh" "$probe_duplicate/metadata/executed-script.sh"
printf 'preserved existing phase\n' > "$probe_duplicate/metadata/input-seals.command.txt"
sha256sum "$probe_duplicate/metadata/input-seals.command.txt" > "$probe_control_out/marker.sha256"
probe_control_rc=0
timeout --signal=TERM --kill-after=2s 120s bash "$probe_control_out/executed-driver.sh" \
  --frozen "$probe_duplicate" "$probe_control_gate" "$probe_control_prepared" "$probe_control_workload_sha" \
  > "$probe_control_out/duplicate-phase.stdout" 2> "$probe_control_out/duplicate-phase.stderr" || probe_control_rc=$?
printf '%s\n' "$probe_control_rc" > "$probe_control_out/duplicate-phase.exit"
test "$probe_control_rc" = 65
test ! -e "$probe_duplicate/cases"
test ! -e "$probe_duplicate/metadata/input-seals.queued-utc.txt"
sha256sum -c "$probe_control_out/marker.sha256" > "$probe_control_out/marker-unchanged.txt"
sha256sum -c "$probe_control_out/sources.sha256" > "$probe_control_out/sources-unchanged.txt"
cmp "$probe_control_out/executed-driver.sh" "$probe_driver"
date -u +%FT%TZ > "$probe_control_out/finished-utc.txt"
printf 'PASS: outer reservation refused with 64; existing phase refused with 65, unchanged marker and no candidate directory.\n' > "$probe_control_out/result.txt"
