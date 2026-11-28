#!/bin/bash
# Frozen driver (PROTOCOL.md): before series (unmodified build) first, then after series; 15 measured trials after 2 warm-ups.
# No compile or other heavy job may run. Binaries are copies kept in the scratchpad (hashes in frozen.sha256 / data.sha256).
cd /home/user/crypto
S=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad
D=research/genus_ladder_wied_speed_20261122/raw
for SERIES in before after; do
  for P in 65537 131101 262147; do
    echo "start $SERIES $P load $(cut -d' ' -f1 /proc/loadavg) $(date -u +%H:%M:%S)" >> $D/driver.log
    $S/kappa_g3_$SERIES research/genus_ladder_g3_highp_20261121/run_$P.jsonl $D/${SERIES}_$P.jsonl 15 > $D/${SERIES}_$P.log 2>&1
  done
done
echo done > $D/all_done
