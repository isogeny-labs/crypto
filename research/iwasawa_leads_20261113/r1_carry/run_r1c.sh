#!/bin/sh
# R1c Iso Lab job (PROTOCOL_R1C.md). Exact computation, no timing claims.
set -eu
bin=/opt/r1c
out=${ISOLAB_OUTPUT_DIR:-/work/results}
mkdir -p "$out"
{ date -u +%FT%TZ; uname -a; nproc; grep -m1 'model name' /proc/cpuinfo || true; } > "$out/host.txt"
"$bin/r1_carry" 20261115 12 4 > "$out/armA_r1_seed20261115.jsonl"
"$bin/r1b" 20261115 > "$out/armA_r1b_seed20261115.jsonl"
"$bin/r1c" 20261115 6 2097152 > "$out/armB_r1c_seed20261115.jsonl"
date -u +%FT%TZ >> "$out/host.txt"
(cd "$bin" && sha256sum r1_carry r1b r1c run_r1c.sh) > "$out/BINARIES.sha256"
(cd "$out" && sha256sum host.txt armA_r1_seed20261115.jsonl armA_r1b_seed20261115.jsonl armB_r1c_seed20261115.jsonl BINARIES.sha256) > "$out/SHA256SUMS"
cat "$out/SHA256SUMS"
aws s3 cp --recursive --only-show-errors "$out" "${ISOLAB_RESULTS_URI:?}"
echo "uploaded to $ISOLAB_RESULTS_URI"
