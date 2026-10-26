#!/usr/bin/env bash
# Cell G2 driver. DO NOT LAUNCH until `scripts/checkpoint.sh gate` passes (the
# repository forbids experiment runs without an S3 resume checkpoint).
# Usage: run_cells.sh [JOBS]   (default 3; the machine has 4 shared CPUs)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
( cd "$REPO" && scripts/checkpoint.sh gate )          # no bypass: abort if the gate fails
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HERE/target}"
( cd "$HERE" && cargo test --release --quiet )         # identities must pass on this build
( cd "$HERE" && cargo build --release --quiet )
BIN="$CARGO_TARGET_DIR/release/g2dnm"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN"
"$BIN" plan > "$RUN/plan.tsv"
git -C "$REPO" rev-parse HEAD > "$RUN/commit.txt"
sha256sum "$BIN" > "$RUN/binary.sha256"
# largest cells first so the tail is short; each job writes its own rows file and log
jobs_list() { for n in 17 13 11 7; do for m in 3 5 4 2; do for a in 0 1; do echo "$a $n $m"; done; done; done; }
export BIN RUN
jobs_list | xargs -P "$JOBS" -L1 sh -c '"$BIN" run --a "$0" --n "$1" --m "$2" --out "$RUN" 2> "$RUN/log_a$0_n$1_m$2.txt"'
"$BIN" analyze --dir "$RUN"
echo "done: $RUN"
