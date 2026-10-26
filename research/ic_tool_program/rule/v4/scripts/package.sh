#!/bin/bash
# Pack the raw run tree as one deterministic archive (AGENTS.md section 13a) and
# copy the report, graphs and renders into benchmarks/ (the performance-results
# rule).  Usage: package.sh  (run from anywhere; paths derive from this file).
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
REPO=$(cd "$ROOT/../../../.." && pwd)
B=$REPO/benchmarks/20261024-table-a-rule-v4
cd "$ROOT"
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner \
    --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs replay-rerun | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz > runs.tar.xz.sha256
cd training
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner \
    --mode='u+rwX,go+rX,go-w' --format=gnu -cf - raw | xz -6 -T1 > raw.tar.xz
sha256sum raw.tar.xz > raw.tar.xz.sha256
cd ..
printf "/runs/\n/replay-rerun/\n/training/raw/\n" > .gitignore
mkdir -p "$B"
cp report/report.pdf "$B/rule-v4-report.pdf"
cp report/paired-cold.svg report/paired-cold.png report/gap-to-rho.svg report/gap-to-rho.png "$B/"
echo "packed $(du -h runs.tar.xz | cut -f1); copied to $B"
