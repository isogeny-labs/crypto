#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
out=${1:?usage: restore-runs.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p -- "$out"
tar -xJf "$here/runs.tar.xz" -C "$out"
cp -- "$here/registry-integration-review.json" "$out/registry-integration-review.json"
(cd "$out" && sha256sum -c "$here/RUNS.sha256")
