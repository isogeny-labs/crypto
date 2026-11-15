#!/bin/bash
# Cells of PLAN.md. Run from anywhere; outputs go to runs/<name>.jsonl and
# runs/<name>.log, and progress.txt logs start, end and exit status per cell.
#
#   run_cells.sh R                 regression cells (same options as the frozen run.sh d-A)
#   run_cells.sh T <cap> <k>       one t = 14 bracket system: capped F4 at <cap>, draw <k>
#   run_cells.sh G                 decomposability counts for every runs/*.jsonl row
#
# The binary is the one built from the commit recorded in commit.txt.
set -u
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)
bin=${BIN:-$repo/target/release/examples/pkm_tower_pilot}
gz=${GZ:-$repo/target/release/examples/pfd_grid_zeros}
mkdir -p "$here/runs"
cd "$here/runs"
P1=2013265921
common=(--p $P1 --ladder-t none --planted 0 --random 2 --engine tower --kinds kummer --m 2 --controls tower)
# Regression cells: the options of research/prime_fall_degree_20261027/run.sh (d-A).
f4=(--budget 1800 --max-nnz 2000000000)
# t = 14 (Amendment 7 A): --max-dense replaces --max-nnz. DENSE is the largest multiple
# of 1e8 with 4*C <= request - 8 GiB for this host's request of 14 GiB; BUDGET and the
# virtual-memory ceiling are in PLAN.md.
BUDGET=${BUDGET:-43200}
DENSE=${DENSE:-1600000000}
VMEM_KB=${VMEM_KB:-14500000}
THREADS=${THREADS:-4}
run() {
  name=$1
  shift
  echo "$(date -u +%FT%TZ) start $name: $*" >> ../progress.txt
  ( ulimit -v "$VMEM_KB"; RAYON_NUM_THREADS=$THREADS "$bin" "$@" --out "$name.jsonl" 2> "$name.log" )
  rc=$?
  echo "$(date -u +%FT%TZ) end $name exit $rc" >> ../progress.txt
}
case "${1:-}" in
  R)
    run R-t9-t10 "${common[@]}" "${f4[@]}" --t-min 9 --max-t 10
    for k in 0 1; do
      run R-t11-k$k-c5 "${common[@]}" "${f4[@]}" --t-min 11 --max-t 11 --targets $k --cap 5
      run R-t11-k$k-c6 "${common[@]}" "${f4[@]}" --t-min 11 --max-t 11 --targets $k --cap 6
    done
    ;;
  T)
    cap=${2:?cap}
    k=${3:?draw}
    run T-t14-k$k-c$cap "${common[@]}" --budget "$BUDGET" --max-dense "$DENSE" --trace \
      --t-min 14 --max-t 14 --targets "$k" --cap "$cap"
    ;;
  G)
    shift
    "$gz" "$@"
    ;;
  *)
    echo "usage: $0 R | T <cap> <k> | G rows.jsonl…" >&2
    exit 2
    ;;
esac
