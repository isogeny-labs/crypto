#!/bin/sh
# Local known-input preparation and ecbench planning only. Never calls run.
set -eu
exporter=$1
consumer=$2
checker=$3
out=${4:?fresh result directory required}
test ! -e "$out"
mkdir -p "$out"
study=research/prime_conductor_gap80_20261031/paired-driver-20261118
prepare() {
  name=$1; shift
  "$exporter" --export 8 4 "$out/$name" --projective-batch 7 "$@" < "$study/inputs/$name.txt" > "$out/$name.stdout" 2> "$out/$name.stderr"
  "$consumer" plan --json --spec "$out/$name/spec.json" > "$out/$name-plan.json" 2> "$out/$name-plan.stderr"
  jq -e '.workloads | all(.target_law=="uniform_scalar_sha256_shared_v1")' "$out/$name-plan.json" > "$out/$name-law.check"
  jq -r '.[0] as $p | ["source","destination","coordinate_control"][] as $end | $p.targets[] | [.scalar,.[$end][0],.[$end][1]] | @tsv' -s "$out/$name/prepared.json" > "$out/$name-producer.tsv"
  jq -r '.workloads[] | [.planted,.target[0],.target[1]] | @tsv' "$out/$name-plan.json" > "$out/$name-consumer.tsv"
  "$checker" "$out/$name-producer.tsv" "$out/$name-consumer.tsv" > "$out/$name.check.json" 2> "$out/$name.check.stderr"
}
prepare main
prepare field223 --development-targets 8
prepare field229 --development-targets 8
negative() {
  label=$1; input=$2; shift 2
  set +e
  "$exporter" --export 8 4 "$out/rejected-$label" "$@" < "$input" > "$out/rejected-$label.stdout" 2> "$out/rejected-$label.stderr"
  code=$?
  set -e
  test "$code" -eq 2
  test ! -s "$out/rejected-$label.stdout"
  test ! -e "$out/rejected-$label"
  printf '{"case":"%s","exit":%s,"result_path_created":false}\n' "$label" "$code" >> "$out/rejections.jsonl"
}
negative field80 "$study/inputs/target80.txt"
negative in-kernel "$study/inputs/in-kernel.txt" --development-targets 2
negative zero-targets "$study/inputs/main.txt" --development-targets 0
negative bad-seed "$study/inputs/main.txt" --seed -1
negative bad-generator "$study/inputs/bad-generator.txt"
sha256sum "$out/main/prepared.json" "$out/main/spec.json" > "$out/existing-before.sha256"
set +e
"$exporter" --export 8 4 "$out/main" < "$study/inputs/main.txt" > "$out/existing.stdout" 2> "$out/existing.stderr"
code=$?
set -e
test "$code" -eq 2
sha256sum -c "$out/existing-before.sha256" > "$out/existing.check"
awk 'NR==1 {$1=$1+1} {print}' "$out/main-producer.tsv" > "$out/tampered-producer.tsv"
set +e
"$checker" "$out/tampered-producer.tsv" "$out/main-consumer.tsv" > "$out/tampered.stdout" 2> "$out/tampered.stderr"
code=$?
set -e
test "$code" -eq 2
