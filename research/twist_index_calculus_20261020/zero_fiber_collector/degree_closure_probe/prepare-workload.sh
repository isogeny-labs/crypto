#!/usr/bin/env bash
set -Eeuo pipefail
probe_out=${1:?fresh preparation directory}
probe_gate=$(cd "${2:?successful native gate}" && pwd)
probe_base=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$probe_out"
test "$(cat "$probe_gate/metadata/exit")" = 0
mkdir -p "$probe_out/metadata"
probe_out=$(cd "$probe_out" && pwd)
cp "$0" "$probe_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$probe_out/metadata/started-utc.txt"
git rev-parse HEAD > "$probe_out/metadata/source-commit.txt"
sha256sum -c "$probe_gate/metadata/sources.sha256" > "$probe_out/metadata/sources-unchanged-before.txt"
sha256sum -c "$probe_gate/metadata/probe.sha256" > "$probe_out/metadata/binary-unchanged-before.txt"
probe_cmd=("$probe_gate/probe" prepare "$probe_base/collision_certificate/workload-v1.json" 17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10 "$probe_base/runs/collision-certificate-panel-v1" "$probe_out/prepared")
printf '%q ' "${probe_cmd[@]}" > "$probe_out/metadata/prepare.command.txt"
if /usr/bin/time -v -o "$probe_out/metadata/prepare.time.txt" prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s "${probe_cmd[@]}" > "$probe_out/metadata/prepare.stdout" 2> "$probe_out/metadata/prepare.stderr"; then probe_rc=0; else probe_rc=$?; fi
printf '%s\n' "$probe_rc" > "$probe_out/metadata/prepare.exit"
test "$probe_rc" = 0
sha256sum "$probe_out/prepared/workload.json" "$probe_out/prepared/workload.tsv" "$probe_out/prepared/inputs/"*.json > "$probe_out/metadata/workload-and-inputs.sha256"
sha256sum -c "$probe_gate/metadata/sources.sha256" > "$probe_out/metadata/sources-unchanged-after.txt"
date -u +%FT%TZ > "$probe_out/metadata/finished-utc.txt"
printf '0\n' > "$probe_out/metadata/exit"
