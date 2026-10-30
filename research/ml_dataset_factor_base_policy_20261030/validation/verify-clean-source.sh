#!/bin/bash
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: verify-clean-source.sh PACKAGE RECEIPT_SHA' >&2; exit 2; }
package=$(realpath "$1")
receipt_sha=$2
printf '%s  %s\n' "$receipt_sha" "$package/receipt.json" | sha256sum -c -
jq -e '.schema=="ml-dataset.clean-source-package/v1" and .exit_code==0 and .source_preflight_matches and .source_readback_matches' "$package/receipt.json" >/dev/null
commit=$(jq -er .source_commit "$package/receipt.json")
[[ "$commit" =~ ^[a-f0-9]{40}$ ]]
printf '%s  %s\n' "$(jq -er .input_manifest_sha256 "$package/receipt.json")" "$package/manifest.json" | sha256sum -c -
for mapping in 'input_hash_manifest_sha256 source.sha256' 'mode_manifest_sha256 source-modes.tsv' 'path_manifest_sha256 final-paths.bin'; do
  read -r key name <<< "$mapping"
  printf '%s  %s\n' "$(jq -er --arg key "$key" '.[$key]' "$package/manifest.json")" "$package/$name" | sha256sum -c -
done
(cd "$package"; sha256sum -c package.sha256)
[[ $(git -C "$package/source" rev-parse HEAD) == "$commit" ]]
[[ -z $(git -C "$package/source" status --porcelain --untracked-files=all) ]]
git -C "$package/source" ls-files -z | sort -z | cmp "$package/final-paths.bin" -
(cd "$package/source"; rg --files --hidden --no-ignore --glob '!.git/**' -0 | sort -z) | cmp "$package/final-paths.bin" -
(cd "$package/source"; sha256sum -c "$package/source.sha256")
while IFS=$'\t' read -r path mode; do
  [[ -f "$package/source/$path" && ! -L "$package/source/$path" ]]
  [[ $(stat -c %a -- "$package/source/$path") == "$mode" ]]
done < "$package/source-modes.tsv"
[[ -z $(find -P "$package/source" -path "$package/source/.git" -prune -o ! -type d ! -type f -print -quit) ]]
