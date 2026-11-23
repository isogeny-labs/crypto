#!/usr/bin/env bash
# T11 Addendum C step: ecbench verify --replay 1 on every complete chunk, one at a time,
# each behind an exclusive flock on /tmp/crypto-bench.lock (queued behind other sessions).
ECB=/home/user/ctarget-t11/release/ecbench
D=/home/user/crypto/research/curve25519_leads_wave3_20261103/t11_ecbench_interval_base
LOG=$D/data/verify/verify_driver.log
for c in c00 c01 c02 c03 c04 c05; do
  out=$D/data/verify/$c.json
  if [ -s "$out" ] && grep -q '"status"' "$out"; then echo "$(date -u +%FT%TZ) $c receipt exists, skip" >> $LOG; continue; fi
  echo "$(date -u +%FT%TZ) $c waiting for /tmp/crypto-bench.lock" >> $LOG
  flock -x /tmp/crypto-bench.lock -c "echo \"\$(date -u +%FT%TZ) $c lock acquired\" >> $LOG; $ECB verify --dir $D/data/chunks/$c --replay 1 --out $out > $D/data/verify/$c.txt 2>&1; echo \"\$(date -u +%FT%TZ) $c verify rc=\$?\" >> $LOG"
done
echo "$(date -u +%FT%TZ) ALL VERIFIED" >> $LOG
