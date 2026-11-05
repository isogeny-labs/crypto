#!/usr/bin/env bash
set -euo pipefail
native_unit_root=$(cd -- "$(dirname -- "$0")" && pwd)
cd "$native_unit_root"
sha256sum -c PROTOCOL.sha256 SOURCE.sha256
native_replay_dir=$(mktemp -d)
native_replay_success=0
trap 'if [ "$native_replay_success" = 1 ]; then rm -rf "$native_replay_dir"; else printf "Failed replay retained at %s\n" "$native_replay_dir" >&2; fi' EXIT
"${CXX:-g++}" -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror \
  "$native_unit_root/main.cpp" -lgmpxx -lgmp -o "$native_replay_dir/coded-kernel" \
  > "$native_replay_dir/build.log" 2>&1
set +e
(cd "$native_replay_dir" && ./coded-kernel > run.log 2>&1)
native_replay_status=$?
set -e
printf '%s\n' "$native_replay_status" > "$native_replay_dir/exit-code.txt"
if [ "$native_replay_status" != 0 ]; then
  cat "$native_replay_dir/run.log"
  exit "$native_replay_status"
fi
(cd "$native_replay_dir" && sha256sum -c "$native_unit_root/frozen-outputs.sha256")
cat "$native_replay_dir/run.log"
native_replay_success=1
