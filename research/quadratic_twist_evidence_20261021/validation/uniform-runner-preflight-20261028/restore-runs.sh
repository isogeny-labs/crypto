#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
dest=${1:?usage: restore-runs.sh FRESH_DIRECTORY}
[[ ! -e $dest ]] || { echo 'fresh destination required' >&2; exit 2; }
(cd "$here" && sha256sum -c runs.tar.xz.sha256)
mkdir -p "$dest"
tar -xJf "$here/runs.tar.xz" -C "$dest"
(cd "$dest" && sha256sum -c "$here/RUNS.sha256")
