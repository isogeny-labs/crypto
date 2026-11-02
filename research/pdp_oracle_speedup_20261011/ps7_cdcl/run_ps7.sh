#!/usr/bin/env bash
# PS-7: CDCL with an algebraic leaf, and dynamic branching (protocols/PS-7.md).
#
#   PS7_STAGES=all|core|build|test|graph|fidelity|countcheck|counts|verify|reg|timing|summary   (default all; core = all but timing and countcheck)
#
# Every heavy command goes through `tools/isolated_bench.py busy --wait` (the benchmark lock); the timed stage uses
# `tools/isolated_bench.py run --cpus $PS7_CPU` (reserved core, contention recorded).  Counts do not depend on
# contention; wall time is a note.  A failing command is logged to raw/failures.log and the run continues; the
# script exits non-zero at the end if anything failed.  The counts, verify and reg stages are resumable at the
# granularity of one (rung, arm): a finished arm (its summary record is in the file) is skipped, a partial one is rerun.
set -uo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/../../.."
D=research/pdp_oracle_speedup_20261011
OUT=$D/ps7_cdcl/raw
C=$D/baseline_v1/corpus
GATES=$D/ps1_algebra/raw/gates.jsonl
export CARGO_TARGET_DIR=${PS7_TARGET:-target/ps7}
BIN=$CARGO_TARGET_DIR/release/examples/pdp_ps7
CPU=${PS7_CPU:-3}
ROUNDS=${PS7_ROUNDS:-3}
STAGES=${PS7_STAGES:-all}
RUNGS=${PS7_RUNGS:-"8 9 10"}
SHELL_ARMS="a2 a2p a2l a3"
ALL_ARMS="a1 a1u $SHELL_ARMS"
mkdir -p "$OUT"
FAIL=0
# One lock for a whole stage: the driver runs `busy env PS7_LOCKED=1 PS7_STAGES=<stage> run_ps7.sh`; without it every
# command takes the lock on its own.  The timing stage never runs inside a lock (isolated_bench `run` takes it itself).
busy() { if [ -n "${PS7_LOCKED:-}" ]; then "$@"; else python3 tools/isolated_bench.py busy --wait -- "$@"; fi; }
try() {   # try LABEL CMD... : run, log a failure, keep going
  local label=$1; shift
  if ! "$@"; then echo "$(date -u +%FT%TZ) FAILED: $label :: $*" >> "$OUT/failures.log"; FAIL=1; fi
}
want() {   # want STAGE: is it selected?  Selected stages log the host they ran on (the container can move between hosts).
  # `core` = every stage that runs inside one lock acquisition (everything but the timed stage and the instrumented check)
  local core=no; case "$1" in build|test|graph|fidelity|counts|verify|reg|summary|clippy) core=yes;; esac
  if [ "$STAGES" = all ] || [ "$STAGES" = "$1" ] || { [ "$STAGES" = core ] && [ "$core" = yes ]; }; then
    echo "$(date -u +%FT%TZ) stage=$1 host=$(hostname) cpu=\"$(lscpu | sed -n 's/^Model name: *//p')\" kernel=$(uname -r) up=$(cut -d' ' -f1 /proc/uptime)s" >> "$OUT/host_per_stage.log"
    return 0
  fi
  return 1
}
run_arm() {   # run_arm FILE LABEL CMD... : skip a finished arm, drop the partial lines of an unfinished one, run, log failures
  local f=$1 label=$2; shift 2
  [ -f "$f" ] || : > "$f"
  if grep -q "\"kind\":\"summary\".*\"label\":\"$label\"" "$f"; then return 0; fi
  grep -v "\"label\":\"$label\"" "$f" > "$f.tmp" || true; mv "$f.tmp" "$f"
  try "$label" busy "$@" --label "$label" --out "$f" > /dev/null
}

if want build; then
  busy cargo build --release --example pdp_ps7 2>&1 | tee "$OUT/build.log"
  if [ ! -x "$BIN" ]; then echo "$(date -u +%FT%TZ) FAILED: build produced no binary (see build.log)" >> "$OUT/failures.log"; exit 1; fi
  sha256sum "$BIN" > "$OUT/binary_hash.txt"
  { echo "date_utc: $(date -u +%FT%TZ)";
    echo "tree: origin/main $PS7_MAIN_SHA plus the PS-7 files (examples/pdp_ps7, research/pdp_oracle_speedup_20261011/ps7_cdcl, tools/plot_ps7.py, PLAN.md row)";
    echo "sat.rs blob: $(git hash-object src/cryptanalysis/sat.rs)"; echo "sat_hooked.rs sha256: $(sha256sum examples/pdp_ps7/sat_hooked.rs | cut -d' ' -f1)";
    echo "semaev_decomp.rs blob: $(git hash-object src/cryptanalysis/semaev_decomp.rs)";
    echo "rustc: $(rustc --version)"; echo "cargo: $(cargo --version)"; echo "uname: $(uname -a)";
    lscpu | grep -E 'Model name|^CPU\(s\)|Thread|L1d|L2|L3|Flags' | sed 's/^/lscpu: /'; free -m | sed 's/^/free: /'; } > "$OUT/host_revision.txt"
  examples/pdp_ps7/gen_sat_hooked.sh --check | tee -a "$OUT/host_revision.txt"
fi

if want test; then
  busy cargo test --release --example pdp_ps7 2>&1 | tee "$OUT/unit_tests.log"
fi

if want graph; then
  : > "$OUT/graph_stats.jsonl"
  # the three families of the regression corpus (their own moduli) and the uniform rungs (the bench's moduli)
  try graph-15 busy "$BIN" graph-stats --n 15 --l 5 --low-terms 0,2,4,5 >> "$OUT/graph_stats.jsonl"
  try graph-17 busy "$BIN" graph-stats --n 17 --l 6 --low-terms 0,3 >> "$OUT/graph_stats.jsonl"
  try graph-19 busy "$BIN" graph-stats --n 19 --l 6 --low-terms 0,1,2,5 >> "$OUT/graph_stats.jsonl"
  for l in $RUNGS; do
    try graph-x-l$l busy "$BIN" graph-stats --l "$l" --x-only >> "$OUT/graph_stats.jsonl"                     # the part arm 3 reads
    try graph-full-l$l busy timeout 900 "$BIN" graph-stats --l "$l" >> "$OUT/graph_stats.jsonl"               # whole system, may time out
  done
fi

if want fidelity; then
  : > "$OUT/fidelity.jsonl"
  for l in $RUNGS; do
    try fidelity-$l busy "$BIN" fidelity --l "$l" --corpus "$C/corpus_l$l.txt" --targets 16 --out "$OUT/fidelity.jsonl" > /dev/null
  done
fi

if want countcheck; then   # instrumented field-operation counters (a second build, feature pdp-opcount) against the harness' counts
  OPBIN=${PS7_OPTARGET:-target/ps7op}/release/examples/pdp_ps7
  if CARGO_TARGET_DIR=${PS7_OPTARGET:-target/ps7op} busy cargo build --release --example pdp_ps7 --features pdp-opcount 2>&1 | tail -n 12 > "$OUT/build_opcount.log" && [ -x "$OPBIN" ]; then
    sha256sum "$OPBIN" > "$OUT/binary_hash_opcount.txt"
    : > "$OUT/count_check.jsonl"
    for l in $RUNGS; do
      try countcheck-$l busy "$OPBIN" count-check --l "$l" --corpus "$C/corpus_l$l.txt" --targets 2 >> "$OUT/count_check.jsonl"
    done
  else
    echo "$(date -u +%FT%TZ) FAILED: countcheck build (see build_opcount.log)" >> "$OUT/failures.log"; FAIL=1
  fi
fi

if want counts; then   # first model / refutation, accounting on; the counted unit
  for l in $RUNGS; do
    for arm in $ALL_ARMS; do
      run_arm "$OUT/counts_l$l.jsonl" "counts-$arm-l$l" "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16
    done
  done
fi

if want verify; then   # every model, blocking clauses: the full root set against the PS-0 corpus
  for l in $RUNGS; do
    for arm in $ALL_ARMS; do
      run_arm "$OUT/verify_l$l.jsonl" "verify-$arm-l$l" "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16 --all
    done
  done
fi

if want reg; then
  for arm in $ALL_ARMS; do
    run_arm "$OUT/reg_counts.jsonl" "reg-$arm" "$BIN" reg --arm "$arm" --leaf quartic
    run_arm "$OUT/reg_verify.jsonl" "regv-$arm" "$BIN" reg --arm "$arm" --leaf quartic --all
  done
  for arm in a1 a1u a2 a2l a3; do
    run_arm "$OUT/reg_group.jsonl" "regg-$arm" "$BIN" reg --arm "$arm" --leaf group
  done
fi

if want timing; then   # wall, a note: ONE isolated_bench acquisition (reserved core) around the whole block of runs
  started=no
  for try_n in $(seq 1 200); do   # the tool refuses to start on a busy machine: retry, and keep the refusals
    if python3 tools/isolated_bench.py run --cpus "$CPU" --wait --out "$OUT/iso.jsonl" --label timing-block -- env PS7_STAGES=timing_inner PS7_MAIN_SHA="${PS7_MAIN_SHA:-}" "$0"; then started=yes; break; fi
    echo "$(date -u +%FT%TZ) timing block attempt $try_n refused or failed" >> "$OUT/refused_starts.log"; sleep 20
  done
  [ "$started" = yes ] || { echo "$(date -u +%FT%TZ) FAILED: timing block never completed" >> "$OUT/failures.log"; FAIL=1; }
fi

if [ "$STAGES" = timing_inner ]; then   # the block itself: ROUNDS interleaved rounds, no accounting hooks; a1b = the A/A of a1
  for l in $RUNGS; do
    for r in $(seq 1 "$ROUNDS"); do
      for tag in a1 a1b a1u a2 a2l a3; do
        arm=$tag; [ "$tag" = a1b ] && arm=a1
        label="$tag-l$l-r$r"
        grep -q "\"label\":\"$label\"" "$OUT/times.jsonl" 2>/dev/null && continue
        "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16 --no-log --summary-only \
          --label "$label" --out "$OUT/times.jsonl" > /dev/null || echo "$label failed" >> "$OUT/refused_starts.log"
      done
    done
  done
fi

if want clippy; then   # lint the example; warnings of the library (a different file set) are in the log but are not this task's
  busy cargo clippy --release --example pdp_ps7 > "$OUT/clippy.log" 2>&1
  echo "clippy exit $?; warnings located in examples/pdp_ps7 or pdp_speedup: $(grep -c -E '^ *--> examples/pdp_(ps7|speedup)' "$OUT/clippy.log")" >> "$OUT/clippy.log"
fi

if want summary; then
  "$BIN" summarize "$OUT"/counts_l*.jsonl > "$OUT/../counts_summary.md"
  "$BIN" summarize "$OUT/reg_counts.jsonl" "$OUT/reg_group.jsonl" > "$OUT/../reg_summary.md"
  "$BIN" summarize --json "$OUT/../summary_rows.jsonl" "$OUT"/counts_l*.jsonl "$OUT/reg_counts.jsonl" "$OUT/reg_group.jsonl" > /dev/null
fi

[ "$FAIL" = 0 ] || { echo "some commands failed: see $OUT/failures.log" >&2; exit 1; }
