#!/bin/sh
# Tiny public correctness checks; no clocks or benchmark session.
set -eu
checker=$1
out=${2:?fresh result directory required}
study=research/prime_conductor_gap80_20261031/paired-driver-20261118
test ! -e "$out"
mkdir -p "$out"
for name in main field223 field229; do
  jq -r '([.destination[0],.destination[1],.generators.destination[0],.generators.destination[1],.actual_target_count]|join(" ")), (.targets[]|[.scalar,.source[0],.source[1],.destination[0],.destination[1]]|join(" "))' "$study/results/qualification-a/$name/prepared.json" > "$out/$name.tsv"
  timeout 120 "$checker" "$study/inputs/$name.txt" "$out/$name.tsv" > "$out/$name.json" 2> "$out/$name.stderr"
  test ! -s "$out/$name.stderr"
done
awk 'NR==2 {$2=$2+1} {print}' "$out/main.tsv" > "$out/tampered.tsv"
set +e
"$checker" "$study/inputs/main.txt" "$out/tampered.tsv" > "$out/tampered.stdout" 2> "$out/tampered.stderr"
code=$?
set -e
test "$code" -eq 2
test ! -s "$out/tampered.stdout"
printf '{"case":"tampered source point","exit":%s}\n' "$code" > "$out/rejection.json"
