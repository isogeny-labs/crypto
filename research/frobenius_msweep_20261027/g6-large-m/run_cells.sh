#!/usr/bin/env bash
# Cell G6 driver (prepared, not run). The checkpoint gate (`scripts/checkpoint.sh gate`) is a SESSION-SIDE launcher gate:
# the session runs it BEFORE submitting (RUN_CARD section 2) and passes the non-secret stamp line of .resume/.last-push in
# G6_CHECKPOINT_STAMP. This script never invokes the gate and holds no credential for it.
# Requirements: git checkout with .git, rustc/cargo >= 1.78, bash, awk, tar, xz, sha256sum, `aws` when ISOLAB_RESULTS_URI is set.
# Usage: run_cells.sh [JOBS]   (default 3). Memory: about 250 MB per n=23 job, so JOBS=4 needs 1 GiB.
# Environment: G6_MAX_CPU_MIN (default 600) is the budget on the TYPICAL projection (mu ~ 2 on every instance); the worst case
#              (every instance at the 2^28 tuple cap, CAP_LOG2_TUPLES) is printed, never gated, and the cap itself bounds every instance.
#              The pre-flight times an n = 19 cell (cache-resident) and projects n = 23 from it: the gate is NOT conservative; the
#              director freezes G6_MAX_CPU_MIN with a margin (RUN_CARD section 1, PROTOCOL section 11).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
MAXMIN="${G6_MAX_CPU_MIN:-600}"
[ -n "${G6_CHECKPOINT_STAMP:-}" ] || { echo "ABORT: G6_CHECKPOINT_STAMP is empty; the session must pass the gate before submitting (RUN_CARD section 2)" >&2; exit 3; }
git -C "$REPO" rev-parse HEAD >/dev/null || { echo "ABORT: git metadata unavailable; the run must name its commit" >&2; exit 3; }
[ -f "$HERE/Cargo.lock" ] || { echo "ABORT: Cargo.lock missing (must be committed with the cell)" >&2; exit 3; }
if [ -n "$(git -C "$REPO" status --porcelain -- "$HERE")" ]; then
  echo "ABORT: the cell directory has uncommitted changes; commit so commit.txt names the code that runs" >&2
  git -C "$REPO" status --short -- "$HERE" >&2
  exit 3
fi
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/g6census-target}"
( cd "$HERE" && cargo test --release --locked --quiet )   # identities must pass on the exact build that runs
( cd "$HERE" && cargo build --release --locked --quiet )
BIN="$CARGO_TARGET_DIR/release/g6census"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN/preflight"
finish() { # archive + sha256 + upload on EVERY exit
  rc=$?; trap - EXIT; set +e
  echo "$rc" > "$RUN/driver_exit.txt"
  OUT="${ISOLAB_OUTPUT_DIR:-$RUN.out}"; mkdir -p "$OUT"
  ( cd "$(dirname "$RUN")" && tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -cf - "$(basename "$RUN")" | xz -9 > "$OUT/$(basename "$RUN").tar.xz" )
  ( cd "$OUT" && sha256sum "$(basename "$RUN").tar.xz" > "$(basename "$RUN").tar.xz.sha256" )
  if [ -n "${ISOLAB_RESULTS_URI:-}" ]; then
    if ! { aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors \
        && aws s3 cp "$RUN/" "${ISOLAB_RESULTS_URI%/}/run/" --recursive --only-show-errors; }; then
      echo "ABORT: results upload failed (archive kept in $OUT)" >&2; exit 90
    fi
  else
    echo "note: ISOLAB_RESULTS_URI unset; archive left in $OUT (local run, not the frozen route)" >&2
  fi
  exit "$rc"
}
trap finish EXIT
{
  echo "head $(git -C "$REPO" rev-parse HEAD)"
  echo "branch $(git -C "$REPO" rev-parse --abbrev-ref HEAD)"
  echo "cell_dir_porcelain_lines $(git -C "$REPO" status --porcelain -- "$HERE" | wc -l)"
  echo "checkpoint_stamp $G6_CHECKPOINT_STAMP"
  echo "cell_tree $(git -C "$REPO" rev-parse "HEAD:${HERE#"$REPO"/}")"
} > "$RUN/commit.txt"
{ rustc -Vv; cargo -V; } > "$RUN/toolchain.txt"
( cd "$HERE" && sha256sum Cargo.toml Cargo.lock PROTOCOL.md RUN_CARD.md run_cells.sh plot_g6.py $(find src tests -type f | sort) ) > "$RUN/inputs.sha256"
sha256sum "$BIN" > "$RUN/binary.sha256"
{ date -u; nproc; uname -a; cat /proc/loadavg; } > "$RUN/host.txt"
"$BIN" cells > "$RUN/plan.tsv"            # n a m instances_per_cell tuple_cap_log2 (the frozen cell list)
"$BIN" sizes > "$RUN/sizes_table.tsv"     # exact formulas, no group (block A)

# ---- pre-flight: ns per enumerated tuple on a small scratch cell (never read by analyze) ----
PF="$RUN/preflight"
TUP=0; NS=0; PFM=none
for m in 9 10 11 12; do   # the first matchable small cell times the enumerator
  t0=$(date +%s.%N)
  "$BIN" cell --n 19 --a 0 --m "$m" --draws 300 --k 8 --out "$PF" 2>> "$PF/log.txt" || true
  t1=$(date +%s.%N)
  [ -f "$PF/rows_n19a0m$m.tsv" ] || { echo "pre-flight cell n=19 a=0 m=$m produced no rows file (see $PF/log.txt)" >&2; continue; }
  TUP=$(awk 'NR>1{s+=2^$4} END{printf "%.6e", s+0}' "$PF/rows_n19a0m$m.tsv")
  if awk -v t="$TUP" 'BEGIN{exit !(t>0)}'; then
    NS=$(awk -v a="$t0" -v b="$t1" -v t="$TUP" 'BEGIN{printf "%.2f", (b-a)*1e9/t}'); PFM=$m; break
  fi
done
[ "$PFM" != none ] || { echo "ABORT: no matchable pre-flight cell at n=19 (m = 9..12); cannot time the enumerator" >&2; exit 6; }
# typical projection: every instance at mu ~ 2 (2^(n+1) tuples), instances per cell from plan.tsv; worst: every instance at the cap
TYP=$(awk -v ns="$NS" '{s+=$4*2^($1+1)} END{printf "%.1f", s*ns/1e9/60}' "$RUN/plan.tsv")
WORST=$(awk -v ns="$NS" '{s+=$4*2^$5} END{printf "%.1f", s*ns/1e9/60}' "$RUN/plan.tsv")
echo "preflight n=19 a=0 m=$PFM tuples=$TUP ns_per_tuple(incl. size census and overheads)=$NS; typical projection ${TYP} CPU-min, unreachable ceiling (every instance at the cap) ${WORST} CPU-min, budget ${MAXMIN}" | tee "$PF/summary.txt"
if awk -v p="$TYP" -v m="$MAXMIN" 'BEGIN{exit !(p>m)}'; then
  echo "ABORT: typical projection exceeds G6_MAX_CPU_MIN; re-freeze the budget (RUN_CARD section 1) instead of launching" >&2
  exit 4
fi

# ---- block B: iota identity audits (exact; fail fast) ----
{ for m in 9 10 11 12; do echo "0 $m $m"; echo "0 $m 3"; done; echo "1 9 9"; } > "$RUN/jobs_iota.txt"
while read -r a m k; do
  "$BIN" iota-audit --n 13 --a "$a" --m "$m" --k "$k" >> "$RUN/iota_audit.tsv" 2>> "$RUN/iota_log.txt" || { echo "ABORT: iota identity audit failed (a=$a m=$m k=$k)" >&2; exit 5; }
done < "$RUN/jobs_iota.txt"

# ---- block C: the cells, largest field and m first ----
sort -k1,1nr -k3,3nr "$RUN/plan.tsv" | awk '{print $1, $2, $3}' > "$RUN/jobs_cells.txt"
export BIN RUN
xargs -P "$JOBS" -L1 sh -c '
  stem="n$0_a$1_m$2"
  "$BIN" cell --n "$0" --a "$1" --m "$2" --out "$RUN" 2> "$RUN/log_$stem.txt"
  echo $? > "$RUN/exit_$stem.txt"' < "$RUN/jobs_cells.txt"
"$BIN" analyze --dir "$RUN" > /dev/null
echo "done: $RUN"
