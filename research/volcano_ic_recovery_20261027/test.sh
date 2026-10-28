#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
relative=${here#"$root/"}
[[ $# == 1 && ! -e "$1" ]] || { echo 'usage: test.sh NEW_DEVELOPMENT_RECEIPT_DIRECTORY' >&2; exit 1; }
mkdir -p -- "$1/build"
out=$(cd -- "$1" && pwd)
g++ --version > "$out/compiler.txt"
(cd "$root" && sha256sum -c "$here/SOURCES.sha256") > "$out/source_check.stdout" 2> "$out/source_check.stderr"
(cd "$root" && sha256sum "$relative/SOURCES.sha256") > "$out/source_manifest.sha256"
for name in tests runner verify; do
  g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror "$here/$name.cpp" -o "$out/build/$name" > "$out/build_$name.stdout" 2> "$out/build_$name.stderr"
done
"$out/build/tests" "$out/control" "$here/public_source_case.json" > "$out/tests.json" 2> "$out/tests.stderr"
"$out/build/verify" "$out/control/input.json" "$out/control/config.json" "$out/control/records.jsonl" > "$out/independent_replay.json" 2> "$out/replay.stderr"
# This is a known-answer unit integration test on r=139, not the frozen
# full-width experiment. It exercises OS randomness and the real journal path.
"$out/build/runner" run "$out/control/input.json" "$out/control/config.json" "$out/native_control" --private-output --explicit > "$out/native_control.stdout" 2> "$out/native_control.stderr"
"$out/build/verify" "$out/control/input.json" "$out/control/config.json" "$out/native_control/records.private.jsonl" > "$out/native_control_replay.json" 2> "$out/native_control_replay.stderr"
for bad in bad_target bad_rank bad_bounds bad_identity missing_target; do
  if "$out/build/verify" "$out/control/input.json" "$out/control/config.json" "$out/control/$bad.jsonl" > "$out/$bad.stdout" 2> "$out/$bad.stderr"; then
    echo "negative control unexpectedly accepted: $bad" >&2; exit 1
  fi
done
if "$out/build/runner" plan "$out/control/forbidden_input.json" "$out/control/config.json" --explicit > "$out/forbidden_input.stdout" 2> "$out/forbidden_input.stderr"; then
  echo 'witness-bearing input unexpectedly accepted' >&2; exit 1
fi
if "$out/build/runner" plan "$out/control/input.json" "$out/control/low_memory_config.json" --explicit > "$out/low_memory.stdout" 2> "$out/low_memory.stderr"; then
  echo 'insufficient explicit memory budget unexpectedly accepted' >&2; exit 1
fi
"$out/build/runner" plan "$here/public_source_case.json" "$here/pilot_config.json" --explicit > "$out/full_width_static_plan.json" 2> "$out/plan.stderr"
(cd "$out" && sha256sum build/runner build/verify) > "$out/binaries.sha256"
echo 'Development controls passed; this is not EKS image or performance admission.' > "$out/READY_FOR_DIAGNOSTIC"
echo 'Native development controls and independent replay passed; full-width experiment was not run.'
