#!/bin/bash
# Check copied proposal inputs without consulting mutable live source.
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: verify-proposal-source.sh SNAPSHOT MANIFEST_SHA' >&2; exit 2; }
snapshot=$1
manifest_sha=$2
printf '%s  %s\n' "$manifest_sha" "$snapshot/manifest.json" | sha256sum -c -
jq -er '.schema=="isolated-proposal-source/v1" and .required_build_git_dirty==true' "$snapshot/manifest.json" >/dev/null
[[ -d "$snapshot/source" && ! -L "$snapshot/source" ]]
nonregular=$(find -P "$snapshot/source" ! -type d ! -type f -print -quit)
[[ -z "$nonregular" ]] || { echo 'nonregular proposal input' >&2; exit 1; }
for mapping in 'input_hash_manifest_sha256 source.sha256' 'mode_manifest_sha256 source-modes.tsv' 'path_manifest_sha256 final-paths.bin' 'overlay_sha256 overlay.patch'; do
  read -r key name <<< "$mapping"
  printf '%s  %s\n' "$(jq -er --arg key "$key" '.[$key]' "$snapshot/manifest.json")" "$snapshot/$name" | sha256sum -c -
done
(cd "$snapshot/source"; rg --files --hidden --no-ignore -0 | sort -z) | cmp "$snapshot/final-paths.bin" -
(cd "$snapshot/source"; sha256sum -c "$snapshot/source.sha256")
while IFS=$'\t' read -r path expected; do
  [[ -f "$snapshot/source/$path" && ! -L "$snapshot/source/$path" ]]
  [[ $(stat -c %a -- "$snapshot/source/$path") == "$expected" ]]
done < "$snapshot/source-modes.tsv"
