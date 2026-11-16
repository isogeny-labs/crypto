#!/usr/bin/env bash
# S3 measurement (PLAN.md §9): exact inversion hygiene, old vs new builds.
#
#   research/hec_ic_speedup_20261113/run_s3.sh OUT_DIR
#
# OLD / NEW: target dirs of the two release builds (old = 10e4dde42f, the
# commit that froze §9; new = the S3 code commit).  OLDC / NEWC: their
# `--features count-inv` builds of t2_stream.  Stream: 7 repetitions of
# A-B-A' (old, new, old) of t2_stream on 13 rows; end to end: 3 repetitions
# of A-B-A' of hec_native_bench (HEC_SOLVER=dense, round eight's 55 runs).
# `uptime` is logged before every process in host.txt: the host is shared.
# Gates checked here: G8 on every stream process (hashes and oracle counts
# equal old's first process) and G9 on every end-to-end process (JSON equal
# apart from *_ms fields).  The summary is `summarise_s3.py OUT_DIR`.
set -euo pipefail
out=${1:?usage: run_s3.sh OUT_DIR}
OLD=${OLD:-/Users/adamburan/pq-work/hec-t2-old-target/release/examples}
NEW=${NEW:-/Users/adamburan/pq-work/hec-t2-new-target/release/examples}
OLDC=${OLDC:-/Users/adamburan/pq-work/hec-t2-old-count/release/examples}
NEWC=${NEWC:-/Users/adamburan/pq-work/hec-t2-new-count/release/examples}
STREAM_REPS=${STREAM_REPS:-7}
E2E_REPS=${E2E_REPS:-3}
mkdir -p "$out"
shasum -a 256 "$OLD/t2_stream" "$NEW/t2_stream" "$OLD/hec_native_bench" "$NEW/hec_native_bench" \
  "$OLDC/t2_stream" "$NEWC/t2_stream" > "$out/binaries.sha256"
uname -a > "$out/host.txt"
log() { echo "$1: $(uptime)" >> "$out/host.txt"; }

log count-old; "$OLDC/t2_stream" > "$out/count-old.txt"
log count-new; "$NEWC/t2_stream" > "$out/count-new.txt"
for i in $(seq 1 "$STREAM_REPS"); do
  log "stream $i A";  "$OLD/t2_stream" > "$out/stream$i-A.txt"
  log "stream $i B";  "$NEW/t2_stream" > "$out/stream$i-B.txt"
  log "stream $i A2"; "$OLD/t2_stream" > "$out/stream$i-A2.txt"
done
for i in $(seq 1 "$E2E_REPS"); do
  log "e2e $i A";  HEC_SOLVER=dense "$OLD/hec_native_bench" > "$out/e2e$i-A.txt"
  log "e2e $i B";  HEC_SOLVER=dense "$NEW/hec_native_bench" > "$out/e2e$i-B.txt"
  log "e2e $i A2"; HEC_SOLVER=dense "$OLD/hec_native_bench" > "$out/e2e$i-A2.txt"
done
log end
python3 "$(dirname "$0")/summarise_s3.py" "$out" --gates-only
