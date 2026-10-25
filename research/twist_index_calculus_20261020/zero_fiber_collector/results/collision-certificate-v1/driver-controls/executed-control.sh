#!/usr/bin/env bash
# Bounded orchestration controls; invoke directly.
set -Eeuo pipefail
cert_control_out=${1:?fresh control output}
cert_control_gate=${2:?successful implementation gate}
cert_control_prepared=${3:?successful workload preparation}
cert_control_workload_sha=${4:?registered workload SHA256}
cert_driver=research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/run-panel.sh
test ! -e "$cert_control_out"
mkdir -p "$cert_control_out"
cp "$cert_driver" "$cert_control_out/executed-driver.sh"
cp "$0" "$cert_control_out/executed-control.sh"
sha256sum "$cert_control_out/executed-driver.sh" "$cert_control_out/executed-control.sh" > "$cert_control_out/sources.sha256"
date -u +%FT%TZ > "$cert_control_out/started-utc.txt"
cert_control_rc=0
python3 tools/isolated_bench.py busy -- timeout --signal=TERM --kill-after=2s 10s \
  bash "$cert_control_out/executed-driver.sh" "$cert_control_out/forbidden-output" x x x \
  > "$cert_control_out/outer-busy.stdout" 2> "$cert_control_out/outer-busy.stderr" || cert_control_rc=$?
printf '%s\n' "$cert_control_rc" > "$cert_control_out/outer-busy.exit"
test "$cert_control_rc" = 64
test ! -e "$cert_control_out/forbidden-output"
cert_duplicate="$cert_control_out/duplicate-phase"
mkdir -p "$cert_duplicate/metadata"
cp "$cert_control_out/executed-driver.sh" "$cert_duplicate/metadata/executed-script.sh"
printf 'preserved existing phase\n' > "$cert_duplicate/metadata/input-seals.command.txt"
sha256sum "$cert_duplicate/metadata/input-seals.command.txt" > "$cert_control_out/marker.sha256"
cert_control_rc=0
timeout --signal=TERM --kill-after=2s 120s bash "$cert_control_out/executed-driver.sh" \
  --frozen "$cert_duplicate" "$cert_control_gate" "$cert_control_prepared" "$cert_control_workload_sha" \
  > "$cert_control_out/duplicate-phase.stdout" 2> "$cert_control_out/duplicate-phase.stderr" || cert_control_rc=$?
printf '%s\n' "$cert_control_rc" > "$cert_control_out/duplicate-phase.exit"
test "$cert_control_rc" = 65
test ! -e "$cert_duplicate/cases"
test ! -e "$cert_duplicate/metadata/input-seals.queued-utc.txt"
sha256sum -c "$cert_control_out/marker.sha256" > "$cert_control_out/marker-unchanged.txt"
sha256sum -c "$cert_control_out/sources.sha256" > "$cert_control_out/sources-unchanged.txt"
cmp "$cert_control_out/executed-driver.sh" "$cert_driver"
date -u +%FT%TZ > "$cert_control_out/finished-utc.txt"
printf 'PASS: outer reservation refused with 64; existing phase refused with 65, unchanged marker and no candidate directory.\n' > "$cert_control_out/result.txt"
