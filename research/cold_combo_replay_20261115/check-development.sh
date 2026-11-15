#!/usr/bin/env bash
# Correctness fixtures only. Never invokes the IC solver or compares timings.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
bins=${1:?usage: check-development.sh BINARY_DIRECTORY}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
for engine in ref combo; do
  "$bins/$engine" fixture 8191 1 17 "$tmp/$engine.gp" > "$tmp/$engine.json"
  bash "$here/replay.sh" "$tmp/$engine.gp"
done
cmp "$tmp/ref.gp" "$tmp/combo.gp"
# Exercise integer transport above the exact range of IEEE-754 doubles.
"$bins/ref" fixture 1048507 1 9007199254740993 "$tmp/large-scalar.gp" > "$tmp/large-scalar.json"
bash "$here/replay.sh" "$tmp/large-scalar.gp"
sed 's/,17]$/,18]/' "$tmp/ref.gp" > "$tmp/scalar.gp"
sed 's/^\[8191,/[8193,/' "$tmp/ref.gp" > "$tmp/field.gp"
sed 's/,\[/,[8191,/' "$tmp/ref.gp" > "$tmp/shape.gp"
sed 's/\[3118, 4645, 26\]/[0,0,0]/' "$tmp/ref.gp" > "$tmp/target.gp"
cat "$tmp/ref.gp" > "$tmp/code.gp"
printf ';print("PARI_REPLAY_OK")\n' >> "$tmp/code.gp"
printf '[7,1,[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],3,1]\n' > "$tmp/reducible.gp"
for case in scalar field shape target code reducible; do
  if bash "$here/replay.sh" "$tmp/$case.gp" > "$tmp/$case.log" 2>&1; then
    echo "unexpected acceptance: $case" >&2
    exit 1
  fi
done
echo 'development: three PARI positive fixtures; six tamper rejections; no solver run'
