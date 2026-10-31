#!/usr/bin/env bash
# PS-8: the meet-in-the-middle pair table against the pairs-and-solve kernels (protocols/PS-8.md).
#   PS8_STAGES = all | correctness | timing | summary
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
R=research/pdp_oracle_speedup_20261011
OUT=$R/ps8_mitm/raw
C=$R/baseline_v1/corpus
CORPUS13=$R/ps3_bitsliced/raw/corpus_l13.txt
SCR=${PS8_SCRATCH:-/tmp/ps8_bin}
CPU=${PS8_CPU:-3}
ROUNDS=${PS8_ROUNDS:-5}
STAGES=${PS8_STAGES:-all}
mkdir -p "$OUT" "$SCR"
declare -A TARGETS=( [8]=16 [9]=16 [10]=16 [11]=16 [12]=6 [13]=3 )
declare -A PASSES=(  [8]=40 [9]=10 [10]=3  [11]=1  [12]=1  [13]=1 )
ARMS="baseline baseline2 group-scalar bs512 mitm"
S="$SCR/pdp_speedup"
BUSY="python3 tools/isolated_bench.py busy --wait --"

if [ "$STAGES" = all ] || [ "$STAGES" = correctness ]; then
  if [ "${PS8_REUSE_BINARY:-0}" != 1 ]; then
    $BUSY cargo build --release --example pdp_speedup
    cp target/release/examples/pdp_speedup "$S"
  fi
  sha256sum "$S" > "$OUT/binary_hashes.txt"
  git rev-parse HEAD > "$OUT/revision.txt"; git status --short >> "$OUT/revision.txt"
  bash $R/baseline_v1/manifest.sh "$S" > "$OUT/manifest.json"
  : > "$OUT/verify_arms.jsonl"
  for l in 8 9 10 11 12; do
    for arm in mitm group-scalar; do
      $BUSY "$S" verify-arm --arm "$arm" --l "$l" --corpus "$C/corpus_l$l.txt" | tee -a "$OUT/verify_arms.jsonl"
    done
  done
  : > "$OUT/verify_l13.jsonl"
  for arm in baseline group-scalar mitm; do
    $BUSY "$S" verify-arm --arm "$arm" --l 13 --corpus "$CORPUS13" | tee -a "$OUT/verify_l13.jsonl"
  done
fi
[ "$STAGES" = correctness ] && exit 0

[ -f "$OUT/iso.jsonl" ] || : > "$OUT/iso.jsonl"; [ -f "$OUT/times.jsonl" ] || : > "$OUT/times.jsonl"
timed_run() {   # tag arm l round
  local tag=$1 arm=$2 l=$3 r=$4 label="$1-l$3-r$4" extra=() corp=$C/corpus_l$3.txt
  [ "$l" = 13 ] && corp=$CORPUS13
  grep -q "\"label\":\"$label\"" "$OUT/times.jsonl" && return 0
  if [ "$arm" = baseline ]; then extra=(--passes "${PASSES[$l]}"); else extra=(--passes 1 --min-secs 3); fi
  for try in $(seq 1 200); do
    if python3 tools/isolated_bench.py run --cpus "$CPU" --wait --out "$OUT/iso.jsonl" --label "$label" -- \
         "$S" time --arm "$arm" --l "$l" --corpus "$corp" --targets "${TARGETS[$l]}" \
         "${extra[@]}" --label "$label" --out "$OUT/times.jsonl"; then return 0; fi
    echo "$label attempt $try refused to start (machine busy)" >> "$OUT/refused_starts.log"; sleep 3
  done
  echo "$label never started" >&2; return 1
}
if [ "$STAGES" = all ] || [ "$STAGES" = timing ]; then
  for l in 8 9 10 11 12 13; do
    for r in $(seq 1 "$ROUNDS"); do
      for tag in $ARMS; do
        arm=$tag; [ "$tag" = baseline2 ] && arm=baseline
        timed_run "$tag" "$arm" "$l" "$r"
      done
    done
  done
fi
if [ "$STAGES" = all ] || [ "$STAGES" = timing ] || [ "$STAGES" = summary ]; then
  {
    echo "## A/A: baseline vs baseline (ns per target)"; "$S" summarize --times "$OUT/times.jsonl" --iso "$OUT/iso.jsonl" --a baseline --b baseline2 --metric ns_per_target
    for b in group-scalar bs512 mitm; do
      echo; echo "## baseline vs $b (ns per target)"
      "$S" summarize --times "$OUT/times.jsonl" --iso "$OUT/iso.jsonl" --a baseline --b "$b" --metric ns_per_target
    done
    for b in bs512 mitm; do
      echo; echo "## group-scalar vs $b (ns per target)"
      "$S" summarize --times "$OUT/times.jsonl" --iso "$OUT/iso.jsonl" --a group-scalar --b "$b" --metric ns_per_target
    done
    echo; echo "## bs512 vs mitm (ns per target)"
    "$S" summarize --times "$OUT/times.jsonl" --iso "$OUT/iso.jsonl" --a bs512 --b mitm --metric ns_per_target
  } > "$OUT/ab_summary.md"
  {
    for b in group-scalar bs512 mitm; do
      echo "## baseline vs $b, contended rounds excluded (ns per target)"
      "$S" summarize --times "$OUT/times.jsonl" --iso "$OUT/iso.jsonl" --a baseline --b "$b" --metric ns_per_target --exclude-contended
    done
  } > "$OUT/ab_summary_excl_contended.md"
fi
