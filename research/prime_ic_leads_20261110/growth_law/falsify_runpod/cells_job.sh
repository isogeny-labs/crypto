#!/usr/bin/env bash
# Growth-law falsifying cells on one RunPod pod (cwd /work). Two draws run concurrently per cell.
set -u
cd /work
B=./in/pkm_tower_pilot
chmod +x $B
{ date -u +%FT%TZ; nproc; grep MemTotal /proc/meminfo; cat /sys/fs/cgroup/memory.max 2>/dev/null; head -2 /etc/os-release; ldd --version | head -1; sha256sum $B; } > out/host.txt
[ "$(sha256sum $B | cut -d' ' -f1)" = "$(cut -d' ' -f1 in/binary.sha256)" ] || { echo "BINARY HASH MISMATCH" >> out/host.txt; exit 3; }
cell() { # W T K C E BUDGET_S TIMEOUT_S
  local W=$1 T=$2 K=$3 C=$4 E=$5 BS=$6 TO=$7 N=nullw$1_m2_t$2_k$3
  echo "$(date -u +%FT%TZ) start $N cmax=$C entries=$E budget=$BS timeout=$TO" >> out/commands.log
  CHECKPOINT_SKIP=devcheck RAYON_NUM_THREADS=8 timeout $TO $B --engine hky --kinds kummer --controls null --planted 0 --random 2 --ladder-t none --p 2013265921 --seed 1347106132 --null-width $W --m 2 --t-min $T --max-t $T --max-t-m3 $T --targets $K --hky-cmax $C --hky-max-entries $E --budget $BS --out out/$N.jsonl > out/run_$N.log 2>&1
  local rc=$?
  echo "exit=$rc" >> out/run_$N.log
  echo "$(date -u +%FT%TZ) end $N exit=$rc peak_mb=$(( $(cat /sys/fs/cgroup/memory.peak 2>/dev/null || echo 0) / 1048576 ))" >> out/commands.log
}
pair() { cell $1 $2 0 $3 $4 $5 $6 & cell $1 $2 1 $3 $4 $5 $6 & wait; }
echo "full command: CHECKPOINT_SKIP=devcheck RAYON_NUM_THREADS=8 timeout TO ./in/pkm_tower_pilot --engine hky --kinds kummer --controls null --planted 0 --random 2 --ladder-t none --p 2013265921 --seed 1347106132 --null-width W --m 2 --t-min T --max-t T --max-t-m3 T --targets K --hky-cmax C --hky-max-entries E --budget BS --out out/nullwW_m2_tT_kK.jsonl" >> out/commands.log
pair 1 12 7 2000000000 10500 10800   # (a) prediction d_F = 6
pair 3 13 7 2000000000 7000 7200     # (b) prediction 7
pair 2 14 7 2000000000 7000 7200     # (c) prediction 7
echo "$(date -u +%FT%TZ) ALL_DONE" >> out/commands.log
