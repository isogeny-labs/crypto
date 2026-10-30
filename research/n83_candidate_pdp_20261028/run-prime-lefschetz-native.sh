#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
certificate=$2
out=$3
dest=$4
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out/inputs" "$out/source"
started=$(date -u +%FT%TZ)
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
source "$study/lefschetz-journal.sh"
protocol="$study/protocol-prime-lefschetz-native.json"
[[ $(jq -r '.freeze.status' "$protocol") == frozen ]]
printf '%s  %s\n' "$(jq -r '.arms[0].instrument.binary_sha256' "$protocol")" "$binary" | sha256sum -c -
printf '%s  %s\n' "$(jq -r '.extensions.certificate_binary_sha256' "$protocol")" "$certificate" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$certificate" "$out/certificate-instrument"
cp "$protocol" "$out/protocol-prime-lefschetz-native.json"
while IFS=$'\t' read -r name hash;do
 printf '%s  %s\n' "$hash" "$study/$name" | sha256sum -c -
 mkdir -p "$out/source/$(dirname "$name")"
 cp "$study/$name" "$out/source/$name"
done < <(jq -r '.extensions.source_files[]|[.path,.sha256]|@tsv' "$protocol")
cp "$study/evidence/prime-terminal/summary.json" "$out/upper-summary.json"
while IFS=$'\t' read -r key directory inputhash systemhash;do
 mkdir "$out/inputs/$key"
 printf '%s  %s\n' "$inputhash" "$directory/input.json" "$systemhash" "$directory/system.sing" | sha256sum -c -
 cp "$directory/input.json" "$directory/system.sing" "$out/inputs/$key/"
done < <(jq -r '.extensions.cases[]|[.key,.directory,.input_sha256,.source_sha256]|@tsv' "$protocol")
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
export N83_LEFSCHETZ_FROZEN_PRODUCTION=1
lefschetz_phase source-producer "$out/instrument" run "$out" "$out/protocol-prime-lefschetz-native.json" "$out/source-audit"
lefschetz_phase certificate-producer "$out/certificate-instrument" run "$out" "$out/protocol-prime-lefschetz-native.json" "$out/results"
lefschetz_phase source-replay "$out/instrument" replay "$out" "$out/protocol-prime-lefschetz-native.json" "$out/source-audit"
lefschetz_phase certificate-replay "$out/certificate-instrument" replay "$out" "$out/protocol-prime-lefschetz-native.json" "$out/results"
