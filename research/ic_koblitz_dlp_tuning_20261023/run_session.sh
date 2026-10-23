#!/usr/bin/env bash
# One session: run, replay-all audit, band. Thin orchestration (PROTOCOL.md section 9).
#   run_session.sh NAME SPEC     -> sessions/NAME, audits/NAME.audit.json, bands/NAME.band.json
set -euo pipefail
cd "$(dirname "$0")"
name=$1; spec=$2
E=$(pwd)/../../target/release/ecbench
export RAYON_NUM_THREADS=1
mkdir -p sessions audits bands
start=$(date +%s)
taskset -c 3 "$E" run --wait --spec "$spec" --out "$(pwd)/sessions/$name" --cpus none --allow-busy \
  --lock /tmp/claude-0/dlp.lock --quiet
mid=$(date +%s)
"$E" verify --dir "sessions/$name" --replay-all --out "audits/$name.audit.json" | tail -1
"$E" band --dir "sessions/$name" --weights 0,0.1,1 --json "bands/$name.band.json" > "bands/$name.band.md"
end=$(date +%s)
echo "$name run_s=$((mid-start)) audit_s=$((end-mid))" | tee -a timings.txt
