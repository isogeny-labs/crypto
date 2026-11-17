#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
m=${1:?module}; out=${2:?fresh output directory}
mkdir "$out"
cp source/"$m.lean" "$out/"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$PWD/source:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
set +e
timeout -k 5s 120s "$lean" -j1 -M4096 -o "$PWD/source/$m.olean" "$PWD/source/$m.lean" > "$out/stdout" 2> "$out/stderr"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit"
cat "$out/stdout"
exit "$code"
