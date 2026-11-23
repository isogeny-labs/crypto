#!/usr/bin/env bash
set -euo pipefail
root=research/vieta_coset_rust_20261115/speedup-20261117
binary=${1:?immutable symbolled profile binary}
out=${2:?new profile directory}
[[ ! -e "$out" ]] || exit 2
mkdir "$out"
config=$(cat "$root/profile-inputs/config-original.json")
target=$(jq -c '.target' "$root/profile-inputs/input-003-original.json")
export RAYON_NUM_THREADS=1
for arm in newton half; do
  code=0
  timeout --signal=TERM --kill-after=3s 45 gprofng collect app -p 1 -a off -F off -o "$out/$arm.er" "$binary" worker "$config" "$target" "$arm" 1397751811 > "$out/$arm.stdout" 2> "$out/$arm.stderr" || code=$?
  printf '{"arm":"%s","exit":%s,"profile_not_speed_measurement":true}\n' "$arm" "$code" >> "$out/driver.jsonl"
  if [[ "$code" -eq 0 ]]; then
    jq -Rs 'split("\n")|map(fromjson? | select(.schema=="vieta-speed/v1"))|if length==1 then .[0] else error("expected one worker row") end' "$out/$arm.stdout" > "$out/$arm.json"
    jq -e --slurpfile expected "$root/profile-inputs/input-003-original.json" '.status=="ok" and .relations==$expected[0].expected' "$out/$arm.json" > "$out/$arm-set-check.txt"
    gprofng display text -limit 40 -functions "$out/$arm.er" > "$out/$arm-functions.txt" 2> "$out/$arm-display.stderr"
    gprofng display text -statistics "$out/$arm.er" > "$out/$arm-statistics.txt" 2>> "$out/$arm-display.stderr"
  fi
  [[ "$code" -eq 0 ]] || exit "$code"
done
