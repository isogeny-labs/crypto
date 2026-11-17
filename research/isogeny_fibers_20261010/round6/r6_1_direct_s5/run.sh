#!/usr/bin/env bash
# Round 6, lead R6.1 (round6/PROGRAM.md): direct S5 systems and the `ic`
# plug-in for the chained x-frame descent.  Native Rust only
# (examples/r6_1_direct_s5.rs); every command line is repeated in its log.
#
#   bash run.sh interpolate | discrepancy | panel | e2e | all
#
# Resumable: an output whose log ends in "# exit 0" is skipped.  Serial,
# single-threaded; the machine is shared, so every wall time is contended.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/claude-0/-home-user/717e4727-b5a7-5b25-924a-7b31edb43331/scratchpad/r6target}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
BIN="$CARGO_TARGET_DIR/release/examples/r6_1_direct_s5"
OUT="$HERE/results"
mkdir -p "$OUT"
BUDGET=4096

run() {
  local tag="$1"; shift
  local log="$OUT/$tag.log"
  if grep -q '^# exit 0' "$log" 2>/dev/null; then echo "skip $tag"; return; fi
  echo "run  $tag"
  {
    echo "# $(date -u +%FT%TZ) r6_1_direct_s5 $* --json results/$tag.json"
    timeout 900 "$BIN" "$@" --json "$OUT/$tag.json"
    echo "# exit $? at $(date -u +%FT%TZ)"
  } >"$log" 2>&1
}

(cd "$ROOT" && cargo build --release --example r6_1_direct_s5 >/dev/null 2>&1) || { echo "build failed"; exit 1; }
what="${1:-all}"

if [ "$what" = all ] || [ "$what" = interpolate ]; then
  run interpolate interpolate --n 29 --a 1
fi
if [ "$what" = all ] || [ "$what" = discrepancy ]; then
  # The round-4 gap: sym S4 at n = 23, invariant divisor 0;1, budget 4096.
  run discrepancy_n23 discrepancy --n 23 --a 0 --node-budget $BUDGET \
    --panel-targets 96 --max-trials 300
fi
if [ "$what" = all ] || [ "$what" = panel ]; then
  # One cell per (curve, arm group); a cell is capped at 15 min by timeout,
  # and its JSON is rewritten after every arm, so a killed cell keeps what
  # it measured.
  for na in 13:0 17:1 19:0 23:0; do
    n=${na%:*}; a=${na#*:}
    for arms in x-chained,sym-chained sym-s5 x-s5; do
      [ "$n" = 13 ] && [ "$arms" != x-chained,sym-chained ] && continue
      tag="panel_m4_n${n}_a${a}_${arms//,/+}"
      [ "$n" = 13 ] && { arms=x-chained,sym-chained,sym-s5,x-s5; tag="panel_m4_n13_a0"; }
      run "$tag" panel --n "$n" --a "$a" --targets 24 --arms "$arms" \
        --node-budget $BUDGET --cell-wall-s 600 --pre-cap-s 240
    done
  done
fi
if [ "$what" = all ] || [ "$what" = e2e ]; then
  for na in ${E2E:-13:0 17:1 19:0}; do
    n=${na%:*}; a=${na#*:}
    for arm in mitm-m2 chained-m3 chained-m4 sym-s5; do
      run "e2e_n${n}_a${a}_${arm}" e2e --n "$n" --a "$a" --node-budget $BUDGET \
        --arms "$arm" --arm-wall-s 600
    done
  done
fi

{
  echo "date: $(date -u +%FT%TZ)"
  echo "host: $(uname -srmo)"
  echo "cpu: $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //') x$(nproc) (shared with other agents: contended)"
  echo "rustc: $(rustc --version)"
  echo "commit base: $(cd "$ROOT" && git rev-parse HEAD)"
  [ -x "$BIN" ] && echo "example sha256: $(sha256sum "$BIN" | cut -d' ' -f1)"
} >"$OUT/host.txt"
