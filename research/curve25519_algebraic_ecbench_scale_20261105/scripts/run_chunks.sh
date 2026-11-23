#!/bin/bash
# run_chunks.sh k c [attempt] : run one chunk session; restart-safe (new out dir per attempt).
set -u
k=$1; c=$2; att=${3:-a1}
cd "$(dirname "$0")/.."
BIN=/home/user/frozen-bin/ecbench_scale
echo "928e9880c3e833e71c23e7fa37798d24bc60a7d529000b7fe1e106e735366067  $BIN" | sha256sum -c - || exit 3
OUT=/home/user/scratch_S2/k$k-c$c-$att
mkdir -p /home/user/scratch_S2
[ -e "$OUT" ] && { echo "refusing to reuse $OUT"; exit 4; }
export RAYON_NUM_THREADS=1
ulimit -v 3500000
echo -e "$(date -u +%FT%TZ)\tSTART\tk$k\tc$c\t$att\t$OUT\tload=$(cut -d' ' -f1-3 /proc/loadavg)" >> sessions/NOTES.tsv
$BIN run --wait --allow-busy --cpus none --quiet --spec specs/c25519-k$k-c$c.json --out "$OUT"
rc=$?
echo -e "$(date -u +%FT%TZ)\tEND\tk$k\tc$c\t$att\trc=$rc\tload=$(cut -d' ' -f1-3 /proc/loadavg)" >> sessions/NOTES.tsv
exit $rc
