#!/usr/bin/env bash
# PS-7: CDCL with an algebraic leaf, and dynamic branching (protocols/PS-7.md).
#
#   PS7_STAGES=all|build|test|graph|fidelity|countcheck|counts|verify|reg|timing|summary   (default all)
#
# Every heavy command goes through `tools/isolated_bench.py busy --wait` (the benchmark lock); the timed stage uses
# `tools/isolated_bench.py run --cpus $PS7_CPU` (reserved core, contention recorded).  Counts do not depend on
# contention; wall time is a note.  A failing command is logged to raw/failures.log and the run continues; the
# script exits non-zero at the end if anything failed.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
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
busy() { python3 tools/isolated_bench.py busy --wait -- "$@"; }
try() {   # try LABEL CMD... : run, log a failure, keep going
  local label=$1; shift
  if ! "$@"; then echo "$(date -u +%FT%TZ) FAILED: $label :: $*" >> "$OUT/failures.log"; FAIL=1; fi
}
want() {   # want STAGE: is it selected?  Selected stages log the host they ran on (the container can move between hosts).
  if [ "$STAGES" = all ] || [ "$STAGES" = "$1" ]; then
    echo "$(date -u +%FT%TZ) stage=$1 host=$(hostname) cpu=\"$(lscpu | sed -n 's/^Model name: *//p')\" mhz=$(lscpu | sed -n 's/^CPU MHz: *//p') kernel=$(uname -r) up=$(cut -d' ' -f1 /proc/uptime)s" >> "$OUT/host_per_stage.log"
    return 0
  fi
  return 1
}

if want build; then
  busy cargo build --release --example pdp_ps7 2>&1 | tee "$OUT/build.log"
  sha256sum "$BIN" > "$OUT/binary_hash.txt"
  { echo "date_utc: $(date -u +%FT%TZ)"; echo "branch: $(git branch --show-current)"; echo "head: $(git rev-parse HEAD)";
    echo "base_ps2: $(git rev-parse exp/pdp-ps2-scalar)";
    echo "sat.rs blob: $(git hash-object src/cryptanalysis/sat.rs)"; echo "sat_hooked.rs sha256: $(sha256sum examples/pdp_ps7/sat_hooked.rs | cut -d' ' -f1)";
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
  for l in $RUNGS; do try graph-l$l busy "$BIN" graph-stats --l "$l" >> "$OUT/graph_stats.jsonl"; done
fi

if want fidelity; then
  : > "$OUT/fidelity.jsonl"
  for l in $RUNGS; do
    try fidelity-$l busy "$BIN" fidelity --l "$l" --corpus "$C/corpus_l$l.txt" --targets 16 --out "$OUT/fidelity.jsonl" > /dev/null
  done
fi

if want countcheck; then   # instrumented field-operation counters (a second build, feature pdp-opcount) against the harness' counts
  OPBIN=${PS7_OPTARGET:-target/ps7op}/release/examples/pdp_ps7
  CARGO_TARGET_DIR=${PS7_OPTARGET:-target/ps7op} busy cargo build --release --example pdp_ps7 --features pdp-opcount 2>&1 | tail -n 5
  sha256sum "$OPBIN" > "$OUT/binary_hash_opcount.txt"
  : > "$OUT/count_check.jsonl"
  for l in $RUNGS; do
    try countcheck-$l busy "$OPBIN" count-check --l "$l" --corpus "$C/corpus_l$l.txt" --targets 2 >> "$OUT/count_check.jsonl"
  done
fi

if want counts; then   # first model / refutation, accounting on; the counted unit
  for l in $RUNGS; do
    : > "$OUT/counts_l$l.jsonl"
    for arm in $ALL_ARMS; do
      try counts-$arm-$l busy "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16 \
        --label "counts-$arm-l$l" --out "$OUT/counts_l$l.jsonl" > /dev/null
    done
  done
fi

if want verify; then   # every model, blocking clauses: the full root set against the PS-0 corpus
  for l in $RUNGS; do
    : > "$OUT/verify_l$l.jsonl"
    for arm in $ALL_ARMS; do
      try verify-$arm-$l busy "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16 --all \
        --label "verify-$arm-l$l" --out "$OUT/verify_l$l.jsonl" > /dev/null
    done
  done
fi

if want reg; then
  : > "$OUT/reg_counts.jsonl"; : > "$OUT/reg_verify.jsonl"; : > "$OUT/reg_group.jsonl"
  for arm in $ALL_ARMS; do
    try reg-$arm busy "$BIN" reg --arm "$arm" --leaf quartic --label "reg-$arm" --out "$OUT/reg_counts.jsonl" > /dev/null
    try regv-$arm busy "$BIN" reg --arm "$arm" --leaf quartic --all --label "regv-$arm" --out "$OUT/reg_verify.jsonl" > /dev/null
  done
  for arm in a1 a1u a2 a2l a3; do
    try regg-$arm busy "$BIN" reg --arm "$arm" --leaf group --label "regg-$arm" --out "$OUT/reg_group.jsonl" > /dev/null
  done
fi

if want timing; then   # wall, a note: no accounting hooks, reserved core, ROUNDS interleaved rounds; a1b = the A/A of a1
  [ -f "$OUT/iso.jsonl" ] || : > "$OUT/iso.jsonl"; [ -f "$OUT/times.jsonl" ] || : > "$OUT/times.jsonl"
  for l in $RUNGS; do
    for r in $(seq 1 "$ROUNDS"); do
      for tag in a1 a1b a1u a2 a2l a3; do
        arm=$tag; [ "$tag" = a1b ] && arm=a1
        label="$tag-l$l-r$r"
        grep -q "\"label\":\"$label\"" "$OUT/times.jsonl" && continue
        for try_n in $(seq 1 100); do
          if python3 tools/isolated_bench.py run --cpus "$CPU" --wait --out "$OUT/iso.jsonl" --label "$label" -- \
               "$BIN" run --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" --gates "$GATES" --targets 16 --no-log --summary-only \
               --label "$label" --out "$OUT/times.jsonl" > /dev/null; then break; fi
          echo "$label attempt $try_n refused or failed" >> "$OUT/refused_starts.log"; sleep 3
        done
      done
    done
  done
fi

if want summary; then
  "$BIN" summarize "$OUT"/counts_l*.jsonl > "$OUT/../counts_summary.md"
  "$BIN" summarize "$OUT/reg_counts.jsonl" "$OUT/reg_group.jsonl" > "$OUT/../reg_summary.md"
  "$BIN" summarize --json "$OUT/../summary_rows.jsonl" "$OUT"/counts_l*.jsonl "$OUT/reg_counts.jsonl" "$OUT/reg_group.jsonl" > /dev/null
fi

[ "$FAIL" = 0 ] || { echo "some commands failed: see $OUT/failures.log" >&2; exit 1; }
