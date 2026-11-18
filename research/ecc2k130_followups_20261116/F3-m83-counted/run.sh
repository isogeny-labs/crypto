#!/bin/sh
# F3 local smoke and probe (PROTOCOL.md sections 3-4). Thin orchestration only.
# ECBENCH: ecbench built from 70b57c2b81 (Amendment 1). Cores 0-2, 2 GB, timeouts.
set -u
cd "$(dirname "$0")"
E=${ECBENCH:?set ECBENCH}
RUN="timeout 3000 prlimit --as=2000000000 taskset -c 0-2"
mkdir -p raw/sessions raw/audits raw/band
# Negative control: the m = 83 launch spec must not plan (no wide IC).
$RUN $E plan --spec specs/launch-k0n83.json > raw/plan-launch-k0n83.stdout 2> raw/plan-launch-k0n83.stderr
echo "plan launch-k0n83 rc=$?" >> raw/run.log
for s in probe-k0n83-rate smoke-k0n41 smoke-k0n53 smoke-k0n61; do
  start=$(date -u +%FT%TZ)
  $RUN $E run --spec specs/$s.json --out raw/sessions/$s --cpus none --allow-busy --quiet --lock raw/f3.lock
  rc=$?
  echo "$s started $start finished $(date -u +%FT%TZ) rc=$rc" >> raw/run.log
  $RUN $E verify --dir raw/sessions/$s --replay-all --out raw/audits/$s.json > /dev/null 2>&1
  echo "$s verify rc=$?" >> raw/run.log
done
for s in smoke-k0n41 smoke-k0n53 smoke-k0n61; do
  $RUN $E band --dir raw/sessions/$s --weights 0,0.1,1 --ranks --json raw/band/$s.json > raw/band/$s.txt 2>&1
  echo "$s band rc=$?" >> raw/run.log
  $RUN $E compare --dir raw/sessions/$s --a rho-strong --b ic-pipeline --json > raw/band/$s.compare.json 2>&1
  echo "$s compare rc=$?" >> raw/run.log
done
