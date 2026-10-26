#!/usr/bin/env bash
# Cell G5 driver. DO NOT LAUNCH until `scripts/checkpoint.sh gate` passes (the repository forbids experiment
# runs without an S3 resume checkpoint; there is no bypass in this script).
# Usage: run_cells.sh [JOBS]   (default 3; the machine has 4 shared CPUs)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
( cd "$REPO" && scripts/checkpoint.sh gate )          # abort if the gate fails
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HERE/target}"
( cd "$HERE" && cargo test --release --quiet )         # identities must pass on the exact build that runs
( cd "$HERE" && cargo build --release --quiet )
BIN="$CARGO_TARGET_DIR/release/g5census"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN"
git -C "$REPO" rev-parse HEAD > "$RUN/commit.txt"
sha256sum "$BIN" > "$RUN/binary.sha256"
{ date -u; nproc; uname -a; cat /proc/loadavg; } > "$RUN/host.txt"
"$BIN" cells > "$RUN/plan.tsv"                         # a n m l reps  (the frozen cell list)
# job lines: kind a n m l reps
{
  # overlap cells, largest field and largest m first so the tail is short
  sort -k2,2nr -k3,3nr "$RUN/plan.tsv" | awk '{print "overlap", $1, $2, $3, $4, $5}'
  # rank cells: m in {2,3,5,8}, last listed (largest) l per (a,n,m), 6 bases per half
  awk '($3==2||$3==3||$3==5||$3==8){k=$1" "$2" "$3; l[k]=$4} END{for(k in l) print "rank", k, l[k], 6}' "$RUN/plan.tsv" | sort -k3,3nr -k4,4nr
  # Proposition T cells: n in {11,13,17,19} x m in 2..8
  for n in 11 13 17 19; do for m in 2 3 4 5 6 7 8; do echo "propt 0 $n $m 0 0"; done; done
} > "$RUN/jobs.txt"
export BIN RUN
xargs -P "$JOBS" -L1 sh -c '
  case "$0" in
    overlap) "$BIN" overlap --a "$1" --n "$2" --m "$3" --l "$4" --reps "$5" --out "$RUN" ;;
    rank)    "$BIN" rank    --a "$1" --n "$2" --m "$3" --l "$4" --reps "$5" --targets 16 --rowcap 40 --out "$RUN" ;;
    propt)   "$BIN" propt   --n "$2" --m "$3" --trials 20000 --assigns 4 --out "$RUN" ;;
  esac 2> "$RUN/log_$0_a$1_n$2_m$3_l$4.txt" || true' < "$RUN/jobs.txt"
"$BIN" analyze --dir "$RUN"
echo "done: $RUN"
