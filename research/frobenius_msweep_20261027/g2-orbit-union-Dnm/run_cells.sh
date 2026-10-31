#!/usr/bin/env bash
# Cell G2 driver. DO NOT LAUNCH until `scripts/checkpoint.sh gate` passes (the
# repository forbids experiment runs without an S3 resume checkpoint).
# Usage: run_cells.sh [JOBS]   (default 3; the machine has 4 shared CPUs)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
( cd "$REPO" && scripts/checkpoint.sh gate )          # no bypass: abort if the gate fails
# Freeze check: the cell directory must be committed and clean (reports/ and target/ are ignored here).
DIRTY="$(cd "$REPO" && git status --porcelain -- "$HERE" | grep -v -e '/reports/' -e '/target/' | wc -l || true)"
if [ "$DIRTY" != "0" ]; then echo "refusing to run: $DIRTY uncommitted change(s) under $HERE" >&2; exit 3; fi
FROZEN_SHA="$(cd "$REPO" && git log -1 --format=%H -- "$HERE")"   # last commit that touched the cell = frozen sha
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HERE/target}"
( cd "$HERE" && cargo test --release --quiet )         # identities must pass on this build
( cd "$HERE" && cargo build --release --quiet )
BIN="$CARGO_TARGET_DIR/release/g2dnm"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN"
"$BIN" plan  > "$RUN/plan.tsv"
"$BIN" cells > "$RUN/cells.txt"
{
  echo "frozen_sha=$FROZEN_SHA"
  echo "head=$(git -C "$REPO" rev-parse HEAD)"
  echo "branch=$(git -C "$REPO" rev-parse --abbrev-ref HEAD)"
  echo "rustc=$(rustc -V)"
  echo "cargo=$(cargo -V)"
  echo "seed=20261027 (default of g2dnm run; every row stores its seed)"
  echo "NV,NT: see plan.rs constants (16, 48); effective nt per row is in the rows files"
  echo "jobs=$JOBS"
  echo "nproc=$(nproc)"
  echo "uname=$(uname -a)"
  echo "loadavg_at_start=$(cat /proc/loadavg)"
  echo "started_utc=$(date -u +%FT%TZ)"
} > "$RUN/run_meta.txt"
cp "$RUN/run_meta.txt" "$RUN/host.txt"
sha256sum "$BIN" > "$RUN/binary.sha256"
# largest cells first so the tail is short; each job writes its own rows file and log
export BIN RUN
FAIL=0
sort -k5,5 -nr "$RUN/cells.txt" | awk '{print $1, $2, $3, $4}' \
  | xargs -P "$JOBS" -L1 sh -c '"$BIN" run --a "$0" --n "$1" --m "$2" --l "$3" --out "$RUN" 2> "$RUN/log_a$0_n$1_m$2_l$3.txt"' || FAIL=1
( cd "$RUN" && sha256sum rows_*.tsv > rows.sha256 ) || true   # hash the raw rows before analysis
echo "finished_utc=$(date -u +%FT%TZ) failed_jobs=$FAIL" >> "$RUN/run_meta.txt"
# analyze whatever was written; a failed cell is a recorded NOT-RUN, never silently dropped
"$BIN" analyze --dir "$RUN"
echo "done: $RUN (failed=$FAIL)"
exit $FAIL
