#!/usr/bin/env bash
# Cell G5 driver. The checkpoint gate (`scripts/checkpoint.sh gate`) is a SESSION-SIDE launcher gate: the session runs it
# BEFORE submitting (RUN_CARD section 2) and passes the non-secret stamp line of .resume/.last-push in G5_CHECKPOINT_STAMP.
# This script never invokes the gate (it needs the gitignored .resume/RESUME.md and an upload credential, neither of which
# may be in the job); it refuses to start without the stamp and records it in commit.txt.
# The route of RUN_CARD section 1 is frozen (isolab-eks by default; the executor runs this script unchanged inside the job).
# Requirements: a git checkout (.git present), rustc/cargo >= 1.78, bash, awk, tar, xz, sha256sum; `aws` when
# ISOLAB_RESULTS_URI is set (the run directory is archived and uploaded there on every exit, success or abort).
# Usage: run_cells.sh [JOBS]   (default 3; the machine has 4 shared CPUs)
# Environment: G5_MAX_CPU_MIN (default 400) is the projected CPU-minute budget; the pre-flight aborts above it.
#              CARGO_TARGET_DIR defaults to a directory OUTSIDE the cell so the committed tree stays clean.
# Phases: 0 gate + clean-tree + tests + build + provenance + pre-flight timing, 1 Proposition T cells and their
# fail-fast gate (the instrument is validated before the expensive cells), 2 rank + overlap cells, 3 analyze.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
JOBS="${1:-3}"
MAXMIN="${G5_MAX_CPU_MIN:-400}"
[ -n "${G5_CHECKPOINT_STAMP:-}" ] || { echo "ABORT: G5_CHECKPOINT_STAMP is empty; the session must run scripts/checkpoint.sh gate before submitting and pass the .resume/.last-push line (RUN_CARD section 2)" >&2; exit 3; }
git -C "$REPO" rev-parse HEAD >/dev/null || { echo "ABORT: git metadata unavailable; the run must name its commit" >&2; exit 3; }
[ -f "$HERE/Cargo.lock" ] || { echo "ABORT: Cargo.lock missing (must be committed with the cell)" >&2; exit 3; }
if [ -n "$(git -C "$REPO" status --porcelain -- "$HERE")" ]; then
  echo "ABORT: the cell directory has uncommitted changes; commit (or stash) so commit.txt names the code that runs" >&2
  git -C "$REPO" status --short -- "$HERE" >&2
  exit 3
fi
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/g5census-target}"
( cd "$HERE" && cargo test --release --locked --quiet )   # identities must pass on the exact build that runs
( cd "$HERE" && cargo build --release --locked --quiet )
BIN="$CARGO_TARGET_DIR/release/g5census"
RUN="$HERE/reports/run-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$RUN/preflight"
finish() { # archive + sha256 + upload on EVERY exit; the evidence must outlive the job's temporary /work
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
  echo "repo_porcelain_lines $(git -C "$REPO" status --porcelain | wc -l)"
  echo "checkpoint_stamp $G5_CHECKPOINT_STAMP"
  echo "cell_tree $(git -C "$REPO" rev-parse "HEAD:${HERE#"$REPO"/}")"
} > "$RUN/commit.txt"
{ rustc -Vv; cargo -V; } > "$RUN/toolchain.txt"
( cd "$HERE" && sha256sum Cargo.toml Cargo.lock PROTOCOL.md RUN_CARD.md run_cells.sh $(find src tests -type f | sort) ) > "$RUN/inputs.sha256"
sha256sum "$BIN" > "$RUN/binary.sha256"
{ date -u; nproc; uname -a; cat /proc/loadavg; } > "$RUN/host.txt"
"$BIN" cells > "$RUN/plan.tsv" 2> "$RUN/plan_notes.txt"   # a n m l reps est_ops  (the frozen cell list)

# ---- phase 0: pre-flight timing on scratch output (never read by analyze; PROTOCOL 12) ----
t_run() { # prints elapsed seconds of "$@"
  local t0 t1; t0=$(date +%s.%N); "$@" 2>> "$RUN/preflight/log.txt" >/dev/null; t1=$(date +%s.%N); awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.3f", b-a}'
}
PF="$RUN/preflight"
NSOP=0
for key in "1 11" "0 13" "1 17" "0 19" "1 19"; do
  set -- $key
  line=$(awk -v a="$1" -v n="$2" '$1==a && $2==n && $6>mx {mx=$6; l=$0} END{print l}' "$RUN/plan.tsv")
  [ -n "$line" ] || continue
  set -- $line
  est=$("$BIN" estimate --a "$1" --n "$2" --m "$3" --l "$4" --reps 1)
  sec=$(t_run "$BIN" overlap --a "$1" --n "$2" --m "$3" --l "$4" --reps 1 --out "$PF")
  nsop=$(awk -v s="$sec" -v e="$est" 'BEGIN{printf "%.4f", s*1e9/e}')
  echo "preflight overlap a=$1 n=$2 m=$3 l=$4 reps=1 seconds=$sec est_ops=$est ns_per_op=$nsop" | tee -a "$PF/summary.txt"
  NSOP=$(awk -v a="$nsop" -v b="$NSOP" 'BEGIN{printf "%.4f", (a>b)?a:b}')
done
PT=$(t_run "$BIN" propt --n 19 --m 8 --trials 500 --assigns 4 --out "$PF")
RT=$(t_run "$BIN" rank --a 0 --n 19 --m 5 --l 8 --reps 1 --targets 16 --rowcap 40 --out "$PF")
echo "preflight propt n=19 m=8 trials=500 seconds=$PT; rank a=0 n=19 m=5 l=8 reps=1 seconds=$RT" | tee -a "$PF/summary.txt"
TOTOPS=$(awk '{s+=$6} END{printf "%.6e", s}' "$RUN/plan.tsv")
PROJ_MIN=$(awk -v n="$NSOP" -v t="$TOTOPS" -v p="$PT" -v r="$RT" 'BEGIN{printf "%.1f", (n*t/1e9 + p*40*28 + r*6*20)/60}')
printf "projected CPU minutes (upper-bound scaling of the pre-flight, max ns/op over curves): %s; budget %s\n" "$PROJ_MIN" "$MAXMIN" | tee -a "$PF/summary.txt"
if awk -v p="$PROJ_MIN" -v m="$MAXMIN" 'BEGIN{exit !(p>m)}'; then
  echo "ABORT: projected CPU minutes exceed G5_MAX_CPU_MIN; re-freeze the budget (RUN_CARD section 1) instead of launching" >&2
  exit 4
fi

# ---- jobs: kind a n m l reps (propt: reps column = trials) ----
{ for n in 11 13 17 19; do for m in 2 3 4 5 6 7 8; do echo "propt 0 $n $m 0 20000"; done; done; } > "$RUN/jobs_propt.txt"
{
  # rank cells first (cheap, informative controls), then overlap, largest field and m first so the tail is short
  awk '($3==2||$3==3||$3==5||$3==8){k=$1" "$2" "$3; l[k]=$4} END{for(k in l) print "rank", k, l[k], 6}' "$RUN/plan.tsv" | sort -k3,3nr -k4,4nr
  sort -k2,2nr -k3,3nr "$RUN/plan.tsv" | awk '{print "overlap", $1, $2, $3, $4, $5}'
} > "$RUN/jobs_main.txt"
cat "$RUN/jobs_propt.txt" "$RUN/jobs_main.txt" > "$RUN/jobs.txt"
export BIN RUN
runjobs() { # $1 = job file; every job leaves log_<stem>.txt and exit_<stem>.txt (exit code, never swallowed)
  xargs -P "$JOBS" -L1 sh -c '
    stem="$0_a$1_n$2_m$3_l$4"
    case "$0" in
      overlap) "$BIN" overlap --a "$1" --n "$2" --m "$3" --l "$4" --reps "$5" --out "$RUN" ;;
      rank)    "$BIN" rank    --a "$1" --n "$2" --m "$3" --l "$4" --reps "$5" --targets 16 --rowcap 40 --out "$RUN" ;;
      propt)   "$BIN" propt   --n "$2" --m "$3" --trials "$5" --assigns 4 --out "$RUN" ;;
    esac 2> "$RUN/log_$stem.txt"
    echo $? > "$RUN/exit_$stem.txt"' < "$1"
}

# ---- phase 1: Proposition T first, then fail fast ----
runjobs "$RUN/jobs_propt.txt"
if ! "$BIN" gate --dir "$RUN" 2> "$RUN/propt_gate.txt"; then
  cat "$RUN/propt_gate.txt" >&2
  "$BIN" analyze --dir "$RUN" || true
  echo "ABORT: Proposition T gate failed; the expensive cells were not launched. See $RUN/summary.md" >&2
  exit 5
fi
# ---- phase 2 and 3 ----
runjobs "$RUN/jobs_main.txt"
"$BIN" analyze --dir "$RUN"
echo "done: $RUN"
