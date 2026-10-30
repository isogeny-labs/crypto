#!/bin/bash
# Thin source-custody helper. No research execution or source edits.
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: prepare-clean-source-package.sh SNAPSHOT MANIFEST_SHA DEST' >&2; exit 2; }
snapshot=$(realpath "$1")
manifest_sha=$2
destination=$3
scripts=$(cd -- "$(dirname -- "$0")" && pwd)
[[ ! -e "$destination" && ! -L "$destination" ]]
mkdir "$destination"
destination=$(realpath "$destination")
start=$(date -u +%FT%TZ)
stage=input_validation
commit=unknown
preflight=false
readback=false
runner_sha=$(sha256sum "$0" | cut -d' ' -f1)
guard_sha=$(sha256sum "$scripts/verify-proposal-source.sh" | cut -d' ' -f1)
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" \
    --arg stage "$stage" --arg snapshot "$snapshot" --arg manifest "$manifest_sha" \
    --arg commit "$commit" --arg runner "$runner_sha" --arg guard "$guard_sha" \
    --argjson preflight "$preflight" --argjson readback "$readback" --argjson exit "$result" \
    '{schema:"ml-dataset.clean-source-package/v1",started_utc:$start,finished_utc:$finish,
      stage:$stage,snapshot:$snapshot,input_manifest_sha256:$manifest,source_commit:$commit,
      runner_sha256:$runner,guard_sha256:$guard,source_preflight_matches:$preflight,
      source_readback_matches:$readback,exit_code:$exit,
      source_identity:"standalone validation source closure; not upstream delivery",
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$destination/receipt.json"
  cat "$destination/receipt.json"
  exit "$result"
}
trap finish EXIT
bash "$scripts/verify-proposal-source.sh" "$snapshot" "$manifest_sha"
preflight=true
while IFS= read -r -d '' path; do
  case "$path" in
    ''|/*|..|../*|*/../*|*/..|.git|.git/*|*/.git|*/.git/*|*$'\n'*|*$'\t'*) echo 'unsafe source path' >&2; exit 1 ;;
  esac
done < "$snapshot/final-paths.bin"
stage=copy
mkdir "$destination/source"
tar -C "$snapshot/source" --null --verbatim-files-from -T "$snapshot/final-paths.bin" -cf - |
  tar -C "$destination/source" -xf -
for file in manifest.json source.sha256 source-modes.tsv final-paths.bin; do
  cp "$snapshot/$file" "$destination/$file"
done
verify_copy() {
  (cd "$destination/source"; sha256sum -c "$destination/source.sha256")
  while IFS=$'\t' read -r path mode; do
    [[ -f "$destination/source/$path" && ! -L "$destination/source/$path" ]]
    [[ $(stat -c %a -- "$destination/source/$path") == "$mode" ]]
  done < "$destination/source-modes.tsv"
}
stage=copy_validation
(cd "$destination/source"; rg --files --hidden --no-ignore -0 | sort -z) | cmp "$destination/final-paths.bin" -
verify_copy
stage=git_commit
git -C "$destination/source" init --initial-branch=validation-source-09
git -C "$destination/source" -c core.hooksPath=/dev/null add --force \
  --pathspec-from-file="$destination/final-paths.bin" --pathspec-file-nul
git -C "$destination/source" -c core.hooksPath=/dev/null -c commit.gpgsign=false \
  commit -m 'Preserve tested dataset snapshot 09 as standalone validation source'
commit=$(git -C "$destination/source" rev-parse HEAD)
git -C "$destination/source" ls-files -z | sort -z | cmp "$destination/final-paths.bin" -
[[ -z $(git -C "$destination/source" status --porcelain --untracked-files=all) ]]
stage=bundle
git -C "$destination/source" bundle create "$destination/source.bundle" HEAD
git -C "$destination/source" bundle verify "$destination/source.bundle"
[[ $(git bundle list-heads "$destination/source.bundle" HEAD | cut -d' ' -f1) == "$commit" ]]
stage=source_readback
verify_copy
bash "$scripts/verify-proposal-source.sh" "$snapshot" "$manifest_sha"
[[ $(sha256sum "$0" | cut -d' ' -f1) == "$runner_sha" ]]
[[ $(sha256sum "$scripts/verify-proposal-source.sh" | cut -d' ' -f1) == "$guard_sha" ]]
[[ -z $(git -C "$destination/source" status --porcelain --untracked-files=all) ]]
readback=true
(cd "$destination"; sha256sum manifest.json source.sha256 source-modes.tsv final-paths.bin source.bundle > package.sha256)
stage=completed
