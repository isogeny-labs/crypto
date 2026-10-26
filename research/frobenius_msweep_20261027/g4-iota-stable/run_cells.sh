#!/usr/bin/env bash
# Cell G4 driver. DO NOT LAUNCH until `scripts/checkpoint.sh gate` passes (the repository forbids
# experiment runs without an S3 resume checkpoint; there is no bypass in this script).
# Usage: run_cells.sh [JOBS]   (default 3; the machine has 4 shared CPUs)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
( cd "$REPO" && scripts/checkpoint.sh gate )          # abort if the gate fails
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HERE/target}"
( cd "$HERE" && cargo test --release --quiet )         # identities must pass on this build
( cd "$HERE" && cargo build --release --quiet )
BIN="$CARGO_TARGET_DIR/release/g4iota"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN"
git -C "$REPO" rev-parse HEAD > "$RUN/commit.txt"
sha256sum "$BIN" > "$RUN/binary.sha256"
{ date -u; nproc; uname -a; cat /proc/loadavg; } > "$RUN/host.txt"
"$BIN" cells > "$RUN/plan.tsv"
export BIN RUN
# largest field first so the tail is short; one job per cell, each writes its own rows file and log
sort -k2,2nr -k3,3nr "$RUN/plan.tsv" | xargs -P "$JOBS" -L1 sh -c '"$BIN" run --a "$0" --n "$1" --m "$2" --l "$3" --out "$RUN" 2> "$RUN/log_a$0_n$1_m$2_l$3.txt" || true'
"$BIN" analyze --dir "$RUN"
echo "done: $RUN"
