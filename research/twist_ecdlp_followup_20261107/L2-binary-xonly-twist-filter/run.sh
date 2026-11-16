#!/usr/bin/env bash
# Single driver for L2-binary-xonly-twist-filter. Usage: ./run.sh [full|quick|analyze]
#   full  : every registered cell (about 25-35 min wall on 4 shared cores)
#   quick : reduced replication subset (cells with k <= 19, controls, rho k<=23, full-solve k<=19) ~ 3-5 min;
#           each cell it runs is bit-identical (counters and solutions) to the same cell of the full run.
#   analyze: re-run only the GP verification, analysis, plots and RESULTS.md from existing results/*.tsv
# Everything stays under this directory (< 150 MB); target/ is deleted at the end.
set -euo pipefail
cd "$(dirname "$0")"
export L2DIR="$PWD"
MODE="${1:-full}"
export L2THREADS="${L2THREADS:-3}"
export RUSTFLAGS="-C target-feature=+pclmulqdq,+sse4.1"
export CARGO_TARGET_DIR="$PWD/target"
PY=<scratch>/venv/bin/python
BIN=./target/release/l2twist
mkdir -p results results/samples results/pca figs
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a results/run.log; }
GPOPT="-q"

log "cargo build --release (std only, no dependencies)"
cargo build --release 2>&1 | tail -1 | tee -a results/run.log
if [ "$MODE" != "analyze" ]; then
  rm -f results/*.tsv results/samples/*.txt results/pca/*.txt results/gp_verify.txt
  log "mode=$MODE threads=$L2THREADS"
  log "gp: curve generation (ellcard), seeds from FNV-1a of role strings"
  gp $GPOPT gp/gen_curves.gp </dev/null 2>&1 | grep -v "new maximum stack" || true
  if [ "$MODE" = "quick" ]; then
    BK="17 19"; RK="17 19 23"; FK="17 19"
  else
    BK="17 19 23 29 31"; RK="17 19 23 29 31"; FK="17 19 23"
  fi
  log "controls PC-A..PC-D"; $BIN controls 2>&1 | tee -a results/run.log
  for k in $RK; do log "rho k=$k"; $BIN rho $k; done
  # binary arms
  for k in $BK; do
    log "arms B m=2 k=$k"; $BIN arms B 2 $k 2>&1 | tee -a results/run.log
    if [ "$k" != "31" ]; then log "arms B m=3 k=$k"; $BIN arms B 3 $k 2>&1 | tee -a results/run.log; fi
  done
  # prime control
  if [ "$MODE" = "quick" ]; then PK="17 19"; else PK="17 19 23 29 31"; fi
  for k in $PK; do
    log "arms P m=2 k=$k"; $BIN arms P 2 $k 2>&1 | tee -a results/run.log
    if [ "$k" != "29" ] && [ "$k" != "31" ]; then log "arms P m=3 k=$k"; $BIN arms P 3 $k 2>&1 | tee -a results/run.log; fi
  done
  # full solves: 5 curves x 2 instances
  for k in $FK; do
    log "full solve m=2 k=$k"; $BIN full 2 $k 5 2
    log "full solve m=3 k=$k"; $BIN full 3 $k 5 2
  done
fi
log "gp: independent verification of stored samples and PC-A"
gp $GPOPT gp/verify.gp </dev/null 2>&1 | grep -v "new maximum stack" || true
log "analysis"
$BIN analyze
log "plots"
$PY plot.py
log "RESULTS.md"
./make_results.sh
rm -rf target
log "target/ deleted"
du -sh . | tee -a results/run.log
