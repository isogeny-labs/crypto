#!/bin/bash
# Gates and cells of PREREGISTRATION.md, one invocation per (stage, kind, m,
# control). Usage, from the repository root:
#
#     cargo build --release --example pkm_tower_pilot
#     research/prime_fall_degree_20261027/run.sh <stage>
#
# Stages: g4 (planted), hky-A, hky-B, hky-C, d-A, d-B (F4 solving degree
# on the same systems), cap (capped F4 for G3, needs the hky rows).
# Each call writes runs/<name>.jsonl and runs/<name>.log; progress.txt logs
# start, end and exit status. Rows are appended per system as they finish.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:-$here/../../target/release/examples/pkm_tower_pilot}
mkdir -p "$here/runs"
cd "$here/runs"
P0=786433
P1=2013265921
common=(--p $P1 --ladder-t none --planted 0 --random 2)
# Amendment 1: the isogeny arm (and its Kummer control) at P0, lambda < 0.1.
common0=(--p $P0 --ladder-t none --planted 0 --random 2)
# The largest t with lambda < 0.1 at P0, per m (Amendment 1).
tmax0() { case $1 in 2) echo 8;; 3) echo 5;; 4) echo 4;; 5) echo 3;; *) echo 2;; esac; }
hky=(--engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
f4=(--engine tower --budget 1800 --max-nnz 2000000000)
run() {
  name=$1
  shift
  echo "$(date -u +%FT%TZ) start $name: $*" >> ../progress.txt
  "$bin" "$@" --out "$name.jsonl" 2> "$name.log"
  rc=$?
  echo "$(date -u +%FT%TZ) end $name exit $rc" >> ../progress.txt
}
case "${1:-}" in
  g4)
    g=(--ladder-t none --planted 2 --random 0 "${hky[@]}" --hky-cmax 5)
    run g4-kummer-m2 --p $P1 "${g[@]}" --kinds kummer --m 2 --controls tower --t-min 1 --max-t 6
    run g4-kummer-m3 --p $P1 "${g[@]}" --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 3
    run g4-isogeny-m2 --p $P0 "${g[@]}" --kinds isogeny --m 2 --controls tower --t-min 1 --max-t 6
    run g4-isogeny-m3 --p $P0 "${g[@]}" --kinds isogeny --m 3 --controls tower --t-min 1 --max-t-m3 3
    ;;
  hky-A)
    run A-m2-tower "${common[@]}" "${hky[@]}" --kinds kummer --m 2 --controls tower --t-min 1 --max-t 10
    run A-m2-tower-big "${common[@]}" "${hky[@]}" --hky-cmax 5 --kinds kummer --m 2 --controls tower --t-min 11 --max-t 14
    run A-m2-tower-p0 "${common0[@]}" "${hky[@]}" --kinds kummer,isogeny --m 2 --controls tower --t-min 1 --max-t 8
    run A-m2-null "${common[@]}" "${hky[@]}" --kinds kummer --m 2 --controls null --t-min 1 --max-t 10
    run A-m2-null-big "${common[@]}" "${hky[@]}" --hky-cmax 5 --kinds kummer --m 2 --controls null --t-min 11 --max-t 14
    run A-m2-naive "${common[@]}" "${hky[@]}" --kinds kummer --m 2 --controls naive --t-min 1 --max-t 6
    ;;
  hky-B)
    for m in 3 4 5 6; do
      run B-m$m-tower "${common[@]}" "${hky[@]}" --kinds kummer --m $m --controls tower --t-min 1 --max-t-m3 8 --max-t-m4 6 --max-t 6
      t0=$(tmax0 $m)
      run B-m$m-tower-p0 "${common0[@]}" "${hky[@]}" --kinds kummer,isogeny --m $m --controls tower --t-min 1 --max-t-m3 $t0 --max-t-m4 $t0 --max-t $t0
    done
    run B-m3-null "${common[@]}" "${hky[@]}" --kinds kummer --m 3 --controls null --null-chain --t-min 1 --max-t-m3 8
    ;;
  hky-C)
    for t in 2 3; do
      for m in 2 3 4 5 6 7 8 9 10; do
        run C-t$t-m$m "${common[@]}" "${hky[@]}" --kinds kummer --m $m --controls tower --t-min $t --max-t $t --max-t-m3 $t --max-t-m4 $t
        if [ "$t" -le "$(tmax0 $m)" ]; then
          run C-t$t-m$m-p0 "${common0[@]}" "${hky[@]}" --kinds kummer,isogeny --m $m --controls tower --t-min $t --max-t $t --max-t-m3 $t --max-t-m4 $t
        fi
      done
    done
    ;;
  d-A)
    run DA-m2-tower "${common[@]}" "${f4[@]}" --kinds kummer --m 2 --controls tower --t-min 1 --max-t 14
    run DA-m2-tower-p0 "${common0[@]}" "${f4[@]}" --kinds kummer,isogeny --m 2 --controls tower --t-min 1 --max-t 8
    run DA-m2-null "${common[@]}" "${f4[@]}" --kinds kummer --m 2 --controls null --t-min 1 --max-t 14
    ;;
  d-B)
    for m in 3 4 5 6; do
      run DB-m$m-tower "${common[@]}" "${f4[@]}" --kinds kummer --m $m --controls tower --t-min 1 --max-t-m3 8 --max-t-m4 6 --max-t 6
      t0=$(tmax0 $m)
      run DB-m$m-tower-p0 "${common0[@]}" "${f4[@]}" --kinds kummer,isogeny --m $m --controls tower --t-min 1 --max-t-m3 $t0 --max-t-m4 $t0 --max-t $t0
    done
    ;;
  cap-A)
    # Amendment 3/5: d_F = D when capped F4 at D - 1 does not refute.
    for t in 11 12 13 14; do
      for k in 0 1; do
        D=$(grep -h "\"t\":$t," DA-m2-tower.jsonl | grep "\"target_index\":$k," | sed 's/.*"solving_degree_max":\([0-9]*\).*/\1/' | head -1)
        [ -n "$D" ] || continue
        run CAP-m2-t$t-k$k-c$((D-1)) "${common[@]}" "${f4[@]}" --kinds kummer --m 2 --controls tower --t-min $t --max-t $t --targets $k --cap $((D-1))
      done
    done
    ;;
  *)
    echo "usage: $0 g4|hky-A|hky-B|hky-C|d-A|d-B" >&2
    exit 2
    ;;
esac
