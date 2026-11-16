#!/bin/bash
set -euo pipefail
if [[ ${1:-} == phase ]]; then
 phase=$2;out=$3;dest=$4
 while IFS=$'\t' read -r key file digest; do
  printf '%s  %s\n' "$digest" "$out/prepared/$file" | sha256sum -c -
  if [[ $phase == producer ]]; then
   "$out/instrument" generate "$out/prepared/$file" "$out/results/$key"
   jq -e '.source_binding.status=="full_source_and_base_identity_verified"' "$out/results/$key/certificate.json" >/dev/null
   aws s3 cp "$out/results/$key" "$dest/results/$key" --recursive --region us-east-2 --only-show-errors
  else
   cmp "$out/prepared/$file" "$out/results/$key/input.json"
   "$out/instrument" verify "$out/results/$key"
  fi
  jq -c --arg key "$key" --arg phase "$phase" '{key:$key,phase:$phase,remainder_degree,projection_nonzero,centered_target,f_terms,m:.source_binding.m,arm:.source_binding.arm,target_origin:.source_binding.target_origin}' "$out/results/$key/certificate.json" >> "$out/$phase-rows.jsonl"
 done < <(jq -r '.cases[]|[.key,.file,.sha256]|@tsv' "$out/prepared/index.json")
 [[ $(wc -l < "$out/$phase-rows.jsonl") -eq 84 ]]
 exit 0
fi
study=$(cd "$(dirname "$0")" && pwd)
staged=$1;protocol=$2;out=$3;dest=$4
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out/results"
started=$(date -u +%FT%TZ)
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
source "$study/centered-panel-journal.sh"
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
lefschetz_phase producer bash "$study/run-centered-panel.sh" phase producer "$out" "$dest"
lefschetz_phase replay bash "$study/run-centered-panel.sh" phase replay "$out" "$dest"
