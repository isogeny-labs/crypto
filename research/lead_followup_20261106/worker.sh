#!/usr/bin/env bash
# Payload only. Caller must have passed checkpoint, publication, budget,
# command-image and capacity admission. Does not submit or retry cloud jobs.
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: worker.sh charts|radix FRESH_OUTPUT_DIR' >&2; exit 2; }
mode=$1
out=$2
root=$(git rev-parse --show-toplevel)
cd "$root"
[[ ! -e "$out" ]] || { echo 'output already exists; preserve it' >&2; exit 2; }
[[ "$mode" == charts || "$mode" == radix ]] || exit 2
: "${LEAD_ADMISSION_RECEIPT:?point to the reviewed admission receipt}"
[[ -s "$LEAD_ADMISSION_RECEIPT" ]] || exit 2
mkdir -p "$out"
out=$(cd "$out" && pwd)
cp "$LEAD_ADMISSION_RECEIPT" "$out/admission.json"
git rev-parse HEAD > "$out/source.txt"
sha256sum target/release/examples/conjpair_decomp_cost target/release/examples/pkm_tower_pilot \
  research/lead_followup_20261106/native/target/release/radix_compare > "$out/binaries.sha256"
export RAYON_NUM_THREADS=4
printf '4\n' > "$out/rayon-threads.txt"
if [[ "$mode" == charts ]]; then
  for target in 'common0' 'planted[5, 6, 12, 19]'; do
    for seed in 0 1 2; do
      name=common0
      [[ "$target" == common0 ]] || name=planted
      timeout 1800 target/release/examples/conjpair_decomp_cost \
        --p 29 --curves 1 --inf --only-curve 1 --only-arm conj --only-target "$target" \
        --no-r3 --chart-seed "$seed" --out "$out/$name-s$seed.jsonl" \
        2> "$out/$name-s$seed.stderr"
    done
  done
else
  # Run/inspect the historical smoke and G0 qualification independently before
  # admission. The receipt must bind those results to this exact binary.
  for t in 10 11 12; do
    radices='2 4 8'
    (( t % 2 == 0 )) || radices='8 4 2'
    for radix in $radices; do
      (ulimit -v 60000000
       timeout 3700 target/release/examples/pkm_tower_pilot \
        --p 2013265921 --ladder-t none --random 2 --planted 0 --engine tower \
        --kinds kummer --m 2 --controls tower,null --budget 900 --max-nnz 2000000000 \
        --t-min "$t" --max-t "$t" --radix "$radix" --out "$out/t$t-r$radix.jsonl" \
        2> "$out/t$t-r$radix.stderr")
    done
    research/lead_followup_20261106/native/target/release/radix_compare "$t" \
      "$out/t$t-r2.jsonl" "$out/t$t-r4.jsonl" "$out/t$t-r8.jsonl" > "$out/t$t-comparison.json"
  done
  (ulimit -v 60000000
   timeout 3700 target/release/examples/pkm_tower_pilot \
    --p 2013265921 --ladder-t none --random 2 --planted 0 --engine tower \
    --kinds kummer --m 2 --controls tower,null --budget 900 --max-nnz 2000000000 \
    --t-min 10 --max-t 10 --radix 8 --out "$out/aa-t10-r8.jsonl" \
    2> "$out/aa-t10-r8.stderr")
fi
