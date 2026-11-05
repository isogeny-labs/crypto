#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
staged=$1;protocol=$2;out=$3;dest=$4
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out"
started=$(date -u +%FT%TZ)
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
source "$study/radix-census-journal.sh"
[[ $(jq -r '.freeze.status' "$protocol") == frozen ]]
printf '%s  %s\n' "$(jq -r '.arms[0].instrument.binary_sha256' "$protocol")" "$staged/instrument" | sha256sum -c -
(cd "$staged" && sha256sum -c manifest.sha256)
printf '%s  %s\n' "$(jq -r '.extensions.prepared_index_sha256' "$protocol")" "$staged/prepared/index.json" | sha256sum -c -
cp "$staged/instrument" "$out/instrument"
cp "$protocol" "$out/protocol.json"
cp -R "$staged/prepared" "$staged/source" "$out/"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
lefschetz_phase producer "$out/instrument" generate "$out/prepared" "$out/census"
lefschetz_phase replay "$out/instrument" verify "$out/census"
