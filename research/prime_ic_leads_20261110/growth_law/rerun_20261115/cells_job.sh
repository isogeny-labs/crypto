#!/usr/bin/env bash
# Growth-law cells, rerun of 2026-11-15 on one RunPod pod (cwd /work). Two draws per cell run
# concurrently, as in ../falsify_runpod/. Every closure V_{F,c} is appended to out/prog_<cell>.jsonl
# as it finishes (--hky-progress), so a timeout keeps every decided degree.
set -u
cd /work
B=./in/pkm_tower_pilot
chmod +x $B
{ date -u +%FT%TZ; nproc; grep MemTotal /proc/meminfo; cat /sys/fs/cgroup/memory.max 2>/dev/null; head -2 /etc/os-release; ldd --version | head -1; sha256sum $B; } > out/host.txt
[ "$(sha256sum $B | cut -d' ' -f1)" = "$(cut -d' ' -f1 in/binary.sha256)" ] || { echo "BINARY HASH MISMATCH" >> out/host.txt; exit 3; }
cell() { # W T K CMIN CMAX TIMEOUT_S
  local W=$1 T=$2 K=$3 CMIN=$4 C=$5 TO=$6 N=nullw$1_m2_t$2_k$3
  local MIN=""; [ "$CMIN" != - ] && MIN="--hky-cmin $CMIN"
  echo "$(date -u +%FT%TZ) start $N cmin=$CMIN cmax=$C timeout=$TO" >> out/commands.log
  CHECKPOINT_SKIP=devcheck RAYON_NUM_THREADS=8 timeout $TO $B --engine hky --kinds kummer --controls null --planted 0 --random 2 --ladder-t none --p 2013265921 --seed 1347106132 --null-width $W --m 2 --t-min $T --max-t $T --max-t-m3 $T --targets $K $MIN --hky-cmax $C --hky-max-entries 2000000000 --budget $TO --hky-progress out/prog_$N.jsonl --out out/$N.jsonl > out/run_$N.log 2>&1
  local rc=$?
  echo "exit=$rc" >> out/run_$N.log
  echo "$(date -u +%FT%TZ) end $N exit=$rc peak_mb=$(( $(cat /sys/fs/cgroup/memory.peak 2>/dev/null || echo 0) / 1048576 ))" >> out/commands.log
}
pair() { cell $1 $2 0 $3 $4 $5 & cell $1 $2 1 $3 $4 $5 & wait; }
pair 1 12 6 7 86400   # (a) prediction d_F = 6; degrees <= 5 already decided (1 not in V_5), so resume at 6
pair 3 13 - 7 86400   # (b) prediction d_F = 7
pair 2 14 - 7 86400   # (c) prediction d_F = 7
echo "$(date -u +%FT%TZ) ALL_DONE" >> out/commands.log
