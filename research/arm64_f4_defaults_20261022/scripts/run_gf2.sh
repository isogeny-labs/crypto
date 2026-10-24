#!/bin/sh
# GF(2) elimination constants on an Arm64 host, one binary, arms by environment only:
#   t4      no switch set (4 Gray-code tables per pass, the x86-tuned default)
#   t2, t3  KIC_GF2_TABLES=2 / 3
#   reuse   KIC_GF2_REUSE_TABLE=1 (table buffers reused across pivot blocks; opt-in)
# f5: f4_f2_bench f5 cases (n <= 24), Reduced output, RAYON_NUM_THREADS 1 and the host's cores.
# f4: f4_f2_bench f4 cases (n <= 20), 1 thread.
#   BENCH  path to f4_f2_bench    OUT  output directory    ROUNDS (default 3)
BENCH=${BENCH:?}; OUT=${OUT:?}; ROUNDS=${ROUNDS:-3}
CORES=${CORES:-$(getconf _NPROCESSORS_ONLN)}
mkdir -p "$OUT/raw"
armenv() {
  case $1 in
    t4) echo "" ;;
    t2) echo "KIC_GF2_TABLES=2" ;;
    t3) echo "KIC_GF2_TABLES=3" ;;
    reuse) echo "KIC_GF2_REUSE_TABLE=1" ;;
  esac
}
stamp() { echo "{\"uptime\":\"$(uptime | sed 's/.*load average[s]*: //')\",\"utc\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"tool\":\"$1\",\"arm\":\"$2\",\"threads\":$3,\"round\":$4}"; }
rot() { echo $1 | awk -v r="$2" '{n=NF; s=""; for(i=0;i<n;i++){j=((i+r-1)%n)+1; s=s" "$j}; print s}'; }
ARMS="t4 t2 t3 reuse"
r=1
while [ "$r" -le "$ROUNDS" ]; do
  for th in 1 $CORES; do
    for arm in $(rot "$ARMS" $r); do
      f="$OUT/raw/r$r-f5-t$th-$arm.jsonl"
      stamp f5 $arm $th $r > "$f"
      env -u KIC_GF2_TABLES -u KIC_GF2_REUSE_TABLE $(armenv $arm) KIC_F5_ECHELON=0 RAYON_NUM_THREADS=$th "$BENCH" 3 24 f5 0 >> "$f" 2>/dev/null
      echo "exit=$?" > "${f%.jsonl}.status"
    done
  done
  for arm in $(rot "$ARMS" $r); do
    f="$OUT/raw/r$r-f4-t1-$arm.jsonl"
    stamp f4 $arm 1 $r > "$f"
    env -u KIC_GF2_TABLES -u KIC_GF2_REUSE_TABLE $(armenv $arm) RAYON_NUM_THREADS=1 "$BENCH" 3 20 f4 0 >> "$f" 2>/dev/null
    echo "exit=$?" > "${f%.jsonl}.status"
  done
  r=$((r + 1))
done
echo done
