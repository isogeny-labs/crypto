#!/usr/bin/env bash
# Amendment 3 of PROTOCOL.md: the L2 order sweep and full-order coverage,
# and the L1/L3/L4 bench runs.  Two queues side by side, two threads each.
# Usage: run_a3.sh OUTDIR   (from the repository root)
set -u
OUT=${1:?outdir}; mkdir -p "$OUT"
BIN=target/release/examples/pe_closure
BENCH=target/release/examples/koblitz_ic_leads_bench
DR=research/dreg_closure_driver_20261016/draws
export RAYON_NUM_THREADS=2
log() { echo "$(date -u +%H:%M:%SZ) $*" >> "$OUT/queue-a3.log"; }
one() { # tag file draw mode [flags]
  local tag=$1 f=$2 k=$3 mode=$4; shift 4
  local out="$OUT/$tag-$f.d$k.jsonl"
  [ -s "$out" ] && return
  { time "$BIN" --draws "$DR/$f.jsonl" --draw "$k" --degree 4 --mode "$mode" "$@" > "$out"; } 2> "${out%.jsonl}.log"
  log "$tag $f d$k exit $?"
}
sweep() {
  for spec in m4-11-5:8 m3-22-6:0; do f=${spec%:*}; k=${spec#*:}
    for order in low high random:1 random:2; do
      tag_o=${order/:/}
      for r in off on; do
        for sd in d1 s3; do
          flags=(--batch 64 --order "$order")
          [ "$r" = on ] && flags+=(--restrict)
          [ "$sd" = s3 ] && flags+=(--seed-degree 3)
          one "a3o-$tag_o-r$r-$sd" "$f" "$k" partial "${flags[@]}"
        done
      done
    done
    for b in 16 256; do one "a3b-$b" "$f" "$k" partial --batch "$b"; done
  done
  log SWEEP_DONE
}
coverage() {
  for spec in m3-24-6:0 m3-24-6:1 m3-24-6:2 m3-24-6:3 m4-12-5:24 m4-12-5:26 m4-12-5:47 m4-12-5:52; do
    f=${spec%:*}; k=${spec#*:}; one p1full "$f" "$k" full
  done
  [ -s "$OUT/l1-lambda-general.jsonl" ] || "$BENCH" lambda-general > "$OUT/l1-lambda-general.jsonl"
  [ -s "$OUT/l3-gluing-a3.jsonl" ] || "$BENCH" gluing > "$OUT/l3-gluing-a3.jsonl"
  [ -s "$OUT/l3-census.jsonl" ] || "$BENCH" census > "$OUT/l3-census.jsonl"
  [ -s "$OUT/l4-bounds-a3.jsonl" ] || "$BENCH" bounds > "$OUT/l4-bounds-a3.jsonl"
  log COVERAGE_DONE
}
sweep & coverage & wait
log DONE
