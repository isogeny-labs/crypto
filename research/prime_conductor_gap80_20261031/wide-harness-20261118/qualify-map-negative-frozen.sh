#!/bin/sh
set -eu
two=$1
five=$2
out=${3:?fresh output directory required}
study=research/prime_conductor_gap80_20261031/wide-harness-20261118
test ! -e "$out"
mkdir -p "$out"
negative() {
  name=$1
  binary=$2
  set +e
  timeout 120 "$binary" "$out/$name.pari" < "$study/inputs/$name-bad-generator.txt" > "$out/$name.stdout" 2> "$out/$name.stderr"
  code=$?
  set -e
  test "$code" -eq 2
  test ! -s "$out/$name.stdout"
  test ! -e "$out/$name.pari"
  rg -q 'source.*control' "$out/$name.stderr"
  printf '{"case":"%s off-curve generator","exit":%s,"map_file_created":false}\n' "$name" "$code" >> "$out/rejections.jsonl"
}
negative two "$two"
negative five "$five"
