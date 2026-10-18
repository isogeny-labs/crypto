#!/bin/bash
# XP-20261015-kummer-p224-depth, PROTOCOL.md (card directory). One process per
# (prime, m, N, target); each writes runs/<name>.jsonl, .log (step trace), .meta.
# progress.txt logs starts and ends; a process with an end line is not re-run.
#   run.sh stage0 | stage1 | stage2 <cell> <prime> <threads> <as_gb> | verify
#     cell: m2N20 m3N15 m4N12 m4N16 m3N18 ; prime: v40 | P-224
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:-$here/../../target/release/examples/pkm_tower_pilot}
chk=${CHK:-$here/../../target/release/examples/pkm_tower_check}
mkdir -p "$here/runs"

prime_arg() { case $1 in v40) echo 29686813949953;; P-224) echo P-224;; esac; }
cell_args() { # m N -> tower flags
  case $1 in
    m2N20) echo "--m 2 --t-min 10 --max-t 10";;
    m3N15) echo "--m 3 --t-min 5 --max-t-m3 5";;
    m3N18) echo "--m 3 --t-min 6 --max-t-m3 6";;
    m4N12) echo "--m 4 --t-min 3 --max-t-m4 3";;
    m4N16) echo "--m 4 --t-min 4 --max-t-m4 4";;
  esac
}
launch() { # name cell prime target threads as_gb budget_s
  local name=$1 cell=$2 pr=$3 t=$4 th=$5 asgb=$6 budget=$7
  if grep -q " end $name " "$here/progress.txt" 2>/dev/null; then echo "$name done, skipped"; return; fi
  local extra=""; [ "$pr" = "P-224" ] && extra="--verify"
  local args="--engine generic --p $(prime_arg $pr) --kinds kummer --controls tower --ladder-t none --planted 0 --random 2 --trace $(cell_args $cell) --targets $t --budget $budget $extra"
  echo "$(date -u +%FT%TZ) start $name threads=$th as=${asgb}GB: $args" >> "$here/progress.txt"
  (
    s=$(date +%s)
    ( RAYON_NUM_THREADS=$th prlimit --as=$((asgb*1024*1024*1024)) -- "$bin" $args --out "$here/runs/$name.jsonl" 2> "$here/runs/$name.log" ) &
    pid=$!; peak=0
    while kill -0 $pid 2>/dev/null; do
      r=$(ps -o rss= --ppid $pid 2>/dev/null | sort -n | tail -1); r=${r:-0}; [ "$r" -gt "$peak" ] && peak=$r; sleep 10
    done
    wait $pid; rc=$?; e=$(date +%s)
    echo "{\"name\":\"$name\",\"args\":\"$args\",\"threads\":$th,\"as_gb\":$asgb,\"exit\":$rc,\"wall_s\":$((e-s)),\"peak_rss_kB_sampled\":$peak}" > "$here/runs/$name.meta"
    echo "$(date -u +%FT%TZ) end $name exit $rc wall=$((e-s))s peak_rss_kB=$peak" >> "$here/progress.txt"
  ) &
}
case "${1:-}" in
  stage0)
    launch S0-m4N12-v40-t0 m4N12 v40 0 2 32 3600
    launch S0-m4N12-P-224-t0 m4N12 P-224 0 2 32 3600 ;;
  stage1)
    for pr in v40 P-224; do
      for cell in m2N20 m3N15; do for t in 0 1; do launch S1-$cell-$pr-t$t $cell $pr $t 2 32 172800; done; done
      launch S1-m4N12-$pr-t1 m4N12 $pr 1 2 32 172800
    done ;;
  stage2)
    for t in 0 1; do launch S2-$2-$3-t$t $2 $3 $t $4 $5 604800; done ;;
  verify)
    "$chk" verify "$here"/runs/*v40*.jsonl | tee "$here/verify.txt"
    grep -h '"verify' "$here"/runs/*P-224*.jsonl | head -50 >> "$here/verify.txt" ;;
  *) echo "usage: $0 stage0|stage1|stage2 <cell> <prime> <threads> <as_gb>|verify" >&2; exit 2 ;;
esac
wait
