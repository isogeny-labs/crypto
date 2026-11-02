#!/usr/bin/env bash
# Thin capture; invoke through the common busy reservation.
set -Eeuo pipefail
cert_out=${1:?fresh workload output}
cert_gate=$(cd "${2:?successful native implementation gate}" && pwd)
cert_root=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$cert_out"
test "$(cat "$cert_gate/metadata/exit")" = 0
mkdir -p "$cert_out/metadata"
cp "$0" "$cert_out/metadata/executed-script.sh"
git rev-parse HEAD > "$cert_out/metadata/source-commit.txt"
date -u +%FT%TZ > "$cert_out/metadata/started-utc.txt"
sha256sum -c "$cert_gate/metadata/sources.sha256" > "$cert_out/metadata/gate-sources-unchanged.txt"
sha256sum -c "$cert_gate/metadata/certificate.sha256" > "$cert_out/metadata/binary-unchanged.txt"
printf '%q ' "$cert_gate/certificate" prepare "$cert_root/phase2/workload-v1.json" 47604fa481c5c26ef17e9e9a41494461556c1b9356e20e9652a499926a499b77 "$cert_out/prepared" > "$cert_out/metadata/prepare.command.txt"
cert_rc=0
/usr/bin/time -v -o "$cert_out/metadata/prepare.time.txt" prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
  "$cert_gate/certificate" prepare "$cert_root/phase2/workload-v1.json" 47604fa481c5c26ef17e9e9a41494461556c1b9356e20e9652a499926a499b77 "$cert_out/prepared" \
  > "$cert_out/metadata/prepare.stdout" 2> "$cert_out/metadata/prepare.stderr" || cert_rc=$?
printf '%s\n' "$cert_rc" > "$cert_out/metadata/prepare.exit"
date -u +%FT%TZ > "$cert_out/metadata/finished-utc.txt"
if test "$cert_rc" != 0; then exit "$cert_rc"; fi
sha256sum "$cert_out/prepared/workload.json" "$cert_out/prepared/workload.tsv" "$cert_out/prepared/inputs/"*.json > "$cert_out/metadata/workload-and-inputs.sha256"
printf '0\n' > "$cert_out/metadata/exit"
