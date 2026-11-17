#!/usr/bin/env bash
set -euo pipefail
packet=/var/tmp/pencil-native-matrix-review-20261101
subject=/var/tmp/pencil-kummer-matrix-binding-20261101
checker="$packet/build/release/pencil-readonly-accounting"
bindings="$packet/published-input-binding.json"
"$checker" audit "$subject" "$bindings" > "$packet/result.json"
"$checker" controls "$subject" "$bindings" > "$packet/controls.json"
mkdir -p "$packet/cli-negative"
cp -a "$subject/." "$packet/cli-negative/input"
printf 'unrelated source\n' > "$packet/cli-negative/input/walk-source.rs"
if "$checker" audit "$packet/cli-negative/input" "$bindings" > "$packet/cli-negative/changed-source.json"; then
  printf 'unexpected source acceptance\n' >&2
  exit 1
else
  source_exit=$?
fi
test "$source_exit" = 1
printf 'altered binding\n' > "$packet/cli-negative/altered-binding.json"
if "$checker" audit "$subject" "$packet/cli-negative/altered-binding.json" > "$packet/cli-negative/changed-binding.json"; then
  printf 'unexpected binding acceptance\n' >&2
  exit 1
else
  binding_exit=$?
fi
test "$binding_exit" = 1
"$checker" audit "$subject" "$bindings" > "$packet/result-readback.json"
cmp "$packet/result.json" "$packet/result-readback.json"
printf '{"accepted":true,"native_audit":0,"native_fixed_controls":0,"changed_source_exit":%s,"changed_binding_exit":%s,"replay_equal":true}\n' "$source_exit" "$binding_exit" > "$packet/acceptance.json"
