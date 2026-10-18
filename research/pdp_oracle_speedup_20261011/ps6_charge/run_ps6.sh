#!/usr/bin/env bash
# PS-6: charge the pairs-and-solve kernels in the counted unit and compare (protocols/PS-6.md).
# Committed before any PS-6 session ran.  Every heavy step goes through the shared benchmark lock.
#
#   PS6_STAGES   space-separated subset of: build tests manifest counted calibrate opcount memory timed analyze
#                (default: all, in that order)
#   PS6_RUNGS    default "15 18 21 24 27"
#   PS6_CPU      the core the calibration pins to (tools/isolated_bench.py run), default 3
#   PS6_LOCKED   1 when the whole script runs inside `python3 tools/isolated_bench.py busy --wait -- ...`
#                (one acquisition of the shared benchmark lock for several stages): the script then does not
#                wrap its steps again, and the tools that take a lock themselves (ecbench run, isolated_bench
#                run) take the private file $PS6_INNER_LOCK instead, so that they do not wait on their parent.
#                Nothing else is benchmarked meanwhile: the shared lock is held for the whole script.
#
# Sessions are never overwritten: a stage skips a session directory that already exists, and a session
# interrupted or failed stays as evidence (start a new directory by renaming nothing: set PS6_TAG=b).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
R=research/pdp_oracle_speedup_20261011/ps6_charge
RAW=$R/raw
SESS=$RAW/sessions
LOGS=$RAW/logs
TAG=${PS6_TAG:-}
STAGES=${PS6_STAGES:-build tests manifest counted calibrate opcount memory timed analyze}
RUNGS=${PS6_RUNGS:-15 18 21 24 27}
CPU=${PS6_CPU:-3}
ECB=target/release/ecbench
TOOL=target/release/ps6_tool
BUSY="python3 tools/isolated_bench.py busy --wait --"
LOCKARG=()
if [ "${PS6_LOCKED:-0}" = 1 ]; then
  BUSY=""
  LOCKARG=(--lock "${PS6_INNER_LOCK:-/tmp/ps6-inner.lock}")
fi
mkdir -p "$SESS" "$LOGS"
has() { [[ " $STAGES " == *" $1 "* ]]; }

# The binaries embed the source commit and its dirty state at build time (src/build_provenance.rs).  The
# build stage fixes both once, from the tree it is run on, and every later stage reuses them, so that
# committing reports and data after the build neither rebuilds the library nor changes the recorded commit.
if has build; then
  commit=$(git rev-parse HEAD)
  dirty=0; [ -z "$(git status --porcelain -- src Cargo.toml Cargo.lock examples docs/ic/calibration.json)" ] || dirty=1
  printf 'CRYPTO_BUILD_GIT_COMMIT=%s\nCRYPTO_BUILD_GIT_DIRTY=%s\n' "$commit" "$dirty" > "$RAW/build_env.txt"
fi
if [ -e "$RAW/build_env.txt" ]; then
  export $(cat "$RAW/build_env.txt")
fi

if has build; then
  $BUSY cargo build --release --bin ecbench --bin ps6_tool 2>&1 | tee "$LOGS/build.log" | tail -3
fi

if has tests; then
  # The kernel's parity tests (exhaustive on small random binary curves, both values of a), then the
  # ecbench tests the ecbench-extend skill names.
  $BUSY cargo test --release --bin ecbench ps6 -- --nocapture 2>&1 | tee "$LOGS/tests_ps6.log" | tail -12
  $BUSY cargo test --release --lib ecbench 2>&1 | tee "$LOGS/tests_lib_ecbench.log" | tail -4
  $BUSY cargo test --release --test ecbench 2>&1 | tee "$LOGS/tests_integration_ecbench.log" | tail -4
fi

if has manifest; then
  bash research/pdp_oracle_speedup_20261011/baseline_v1/manifest.sh "$ECB" "$TOOL" > "$RAW/manifest.json"
  "$ECB" host > "$RAW/host.txt"
  git rev-parse HEAD > "$RAW/revision.txt"; git status --short >> "$RAW/revision.txt"
fi

session() {   # kind n cpus_flag  -> runs the spec into a new directory, audits it
  local kind=$1 n=$2 cpus=$3 dir="$SESS/${1}_n${2}${TAG}"
  if [ -e "$dir" ]; then echo "$dir exists; not overwritten" >&2; return 0; fi
  for try in $(seq 1 100); do
    if "$ECB" run --wait "${LOCKARG[@]}" --cpus "$cpus" --spec "$R/specs/${kind}_n${n}.json" --out "$dir" > "$LOGS/${kind}_n${n}${TAG}.run.log" 2>&1; then break; fi
    if [ -e "$dir" ]; then echo "$dir was started and failed; kept as evidence" >&2; break; fi
    echo "$kind n=$n attempt $try refused to start" >> "$LOGS/refused_starts.log"; sleep 5
  done
  local replays=12; [ "$n" -le 21 ] && replays=all
  if [ -e "$dir/session.json" ]; then
    if [ "$replays" = all ]; then
      $BUSY "$ECB" verify --dir "$dir" --replay-all --out "$dir.audit.json" > "$LOGS/${kind}_n${n}${TAG}.verify.log" 2>&1 || true
    else
      $BUSY "$ECB" verify --dir "$dir" --replay "$replays" --out "$dir.audit.json" > "$LOGS/${kind}_n${n}${TAG}.verify.log" 2>&1 || true
    fi
  fi
}

if has counted; then
  # Operation counts need no isolation level (README section 6): --cpus none.
  for n in $RUNGS; do session counted "$n" none; done
fi

if has calibrate; then
  # Wall time of one baseline pair, one kernel pair and one group addition, same process, interleaved;
  # a measured ratio (not a pinned one) for the kernel's pairs.  Reserved core via the repository tool.
  [ -e "$RAW/calibrate.jsonl" ] || python3 tools/isolated_bench.py run "${LOCKARG[@]}" --cpus "$CPU" --wait --out "$RAW/calibrate_iso.jsonl" --label ps6-calibrate -- \
      "$TOOL" calibrate --out "$RAW/calibrate.jsonl" --rungs "$(echo $RUNGS | tr ' ' ',')" --reps 11 2>&1 | tee "$LOGS/calibrate.log"
fi

if has opcount; then
  # Counted field operations (carry-less multiplies, products, squarings, inversions) of the baseline's
  # pair loop and the kernel's on the same targets: the feature `pdp-opcount` of semaev_decomp, a
  # different build of the library, in the same target directory (its artifacts sit beside the others;
  # the binary is copied out because it has the same output name), then the normal build is restored.
  if [ ! -e "$RAW/opcount.jsonl" ]; then
    $BUSY cargo build --release --bin ps6_tool --features pdp-opcount 2>&1 | tail -2
    cp "$TOOL" "$LOGS/ps6_tool_opcount"
    $BUSY cargo build --release --bin ps6_tool 2>&1 | tail -1
    $BUSY "$LOGS/ps6_tool_opcount" opcount --out "$RAW/opcount.jsonl" --rungs "$(echo $RUNGS | tr ' ' ',')" --targets 64
    rm -f "$LOGS/ps6_tool_opcount"
  fi
fi

if has memory; then
  [ -e "$RAW/memory.jsonl" ] || {
    for n in $RUNGS; do
      for fold in full negation; do
        f="$RAW/memory_n${n}_${fold}.jsonl"
        [ -e "$f" ] || $BUSY "$TOOL" memory --n "$n" --fold "$fold" --out "$f"
      done
    done
    cat "$RAW"/memory_n*_*.jsonl > "$RAW/memory.jsonl"
  }
fi

if has timed; then
  # Wall time needs the spec's isolation level (L2): ecbench reserves a whole core itself
  # (--cpus auto, away from CPU 0) under the same lock as tools/isolated_bench.py.
  for n in $RUNGS; do session timed "$n" auto; done
fi

if has analyze; then
  for n in $RUNGS; do
    d="$SESS/timed_n${n}${TAG}"
    if [ -e "$d/session.json" ]; then
      for pair in "s4-baseline s4-baseline-control" "s4-baseline s4-grouplaw" "s4-baseline mitm" "s4-baseline mitm-negfold" "s4-grouplaw mitm" "s4-grouplaw mitm-negfold" "rho-negation s4-baseline" "rho-negation s4-grouplaw" "rho-negation mitm"; do
        set -- $pair
        "$ECB" compare --dir "$d" --a "$1" --b "$2" --save > /dev/null 2>> "$LOGS/compare.err" || true
      done
    fi
    d="$SESS/counted_n${n}${TAG}"
    if [ -e "$d/session.json" ]; then
      for pair in "s4-baseline s4-grouplaw" "s4-baseline s4-grouplaw-audit" "s4-grouplaw s4-grouplaw-audit" "s4-baseline mitm" "s4-baseline mitm-negfold" "s4-grouplaw mitm" "rho-negation s4-baseline" "rho-negation s4-grouplaw" "rho-negation mitm"; do
        set -- $pair
        "$ECB" compare --dir "$d" --a "$1" --b "$2" --save > /dev/null 2>> "$LOGS/compare.err" || true
      done
    fi
  done
  "$ECB" table --dir $(ls -d "$SESS"/counted_n*"$TAG" | grep -v audit) > "$RAW/table_counted.txt" 2>&1 || true
  timed_dirs=$(ls -d "$SESS"/timed_n*"$TAG" 2>/dev/null | grep -v audit || true)
  [ -z "$timed_dirs" ] || "$ECB" table --dir $timed_dirs > "$RAW/table_timed.txt" 2>&1 || true
  counted_dirs=$(ls -d "$SESS"/counted_n*"$TAG" | grep -v audit | tr '\n' ' ')
  timed_args=""; [ -z "$timed_dirs" ] || timed_args="--timed $(echo $timed_dirs | tr '\n' ' ')"
  opc=""; [ ! -e "$RAW/opcount.jsonl" ] || opc="--opcount $RAW/opcount.jsonl"
  "$TOOL" analyze --counted $counted_dirs $timed_args --calibration "$RAW/calibrate.jsonl" $opc \
      --memory "$RAW/memory.jsonl" --out "$RAW/summary.json" --markdown "$RAW/summary.md"
fi
