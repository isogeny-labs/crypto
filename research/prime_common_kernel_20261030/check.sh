#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide a separate output directory}
mkdir -p "$out"
out=$(cd "$out" && pwd)
cd "$here"
sha256sum --quiet -c source-manifest.sha256
(cd lean-gauges && sha256sum --quiet -c PACKAGE.sha256)
(cd lean-exterior && sha256sum --quiet -c PACKAGE_SHA256.txt)
(cd lean-coded && sha256sum --quiet -c source-manifest.sha256)
(cd lean-packing && sha256sum --quiet -c source-manifest.sha256)
bash verify-gzip-manifest.sh "$here/coded-native" source-manifest.sha256
bash coded-native-independent/verify-original.sh
bash native/check.sh "$out/exterior-controls"
bash native/parity/check.sh "$out/parity"
bash coded-native/native/check.sh "$out/coded"
bash coded-native-independent/replay.sh
printf 'PASS all four native units and frozen package bindings\n'
