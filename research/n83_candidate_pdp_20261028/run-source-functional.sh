#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
staged=$2
protocol=$3
out=$4
dest=$5
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out"
started=$(date -u +%FT%TZ)
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
source "$study/source-functional-journal.sh"
[[ $(jq -r '.freeze.status' "$protocol") == frozen ]]
printf '%s  %s\n' "$(jq -r '.arms[0].instrument.binary_sha256' "$protocol")" "$binary" | sha256sum -c -
(cd "$staged" && sha256sum -c manifest.sha256)
cp "$binary" "$out/instrument"
cp "$protocol" "$out/protocol.json"
cp -R "$staged/inputs" "$staged/terminal" "$staged/top" "$staged/source" "$out/"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
export N83_SOURCE_FUNCTIONAL_FROZEN=1
lefschetz_phase producer "$out/instrument" run "$out" "$out/protocol.json" "$out/results"
lefschetz_phase replay "$out/instrument" replay "$out" "$out/protocol.json" "$out/results"
