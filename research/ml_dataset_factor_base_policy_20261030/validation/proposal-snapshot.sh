#!/bin/bash
# Thin source-copy/build preparation. No cryptographic computation or research run.
set -euo pipefail
[[ $# == 5 ]] || { echo 'usage: proposal-snapshot.sh BASE_ROOT BASE_COMMIT PATCH PATCH_SHA DEST' >&2; exit 2; }
base_root=$1
base_commit=$2
patch=$3
patch_sha=$4
destination=$5
[[ ! -e "$destination" && ! -L "$destination" ]]
[[ "$patch_sha" =~ ^[0-9a-f]{64}$ ]]
printf '%s  %s\n' "$patch_sha" "$patch" | sha256sum -c -
base_commit=$(git -C "$base_root" rev-parse "$base_commit^{commit}")
git -C "$base_root" diff --exit-code "$base_commit" -- . \
  ':(exclude)docs/session-logs/**' \
  ':(exclude)research/ml_dataset_factor_base_policy_20261030/**'
mkdir -m 700 "$destination"
mkdir "$destination/source"
git -C "$base_root" ls-tree -rz "$base_commit" > "$destination/base-tree.bin"
(cd "$base_root"; rg --files --hidden --no-ignore -0 \
  -g '!target/**' -g '!.git/**' -g '!.resume/**' -g '!store/**' \
  -g '!docs/session-logs/**' -g '!research/ml_dataset_factor_base_policy_20261030/**') \
  > "$destination/materialized.bin"
# Include committed local dataset golden inputs, but exclude mutable delivery records.
git -C "$base_root" ls-tree -rz "$base_commit" -- \
  research/ml_dataset_factor_base_policy_20261030/legacy-fixtures \
  > "$destination/goldens-tree.bin"
jq -Rs 'split("\u0000") | map(select(length>0)) | INDEX(.)' \
  "$destination/materialized.bin" > "$destination/materialized.json"
jq -Rs --slurpfile present "$destination/materialized.json" '
  [split("\u0000")[] | select(length>0) | split("\t") |
   {path:.[1], mode:(.[0]|split(" ")[0]), blob:(.[0]|split(" ")[2]), kind:(.[0]|split(" ")[1])} |
   select($present[0][.path] != null or (.path|startswith("research/ml_dataset_factor_base_policy_20261030/legacy-fixtures/")))] |
  sort_by(.path) |
  if all(.[]; .kind=="blob" and (.mode=="100644" or .mode=="100755") and
       (.path|test("^[A-Za-z0-9._/+@=-]+$")) and (.path|split("/")|all(. != ".." and . != "")))
  then . else error("unsupported snapshot input path or mode") end
' "$destination/base-tree.bin" > "$destination/base-inputs.json"
jq -j '.[]|.path+"\u0000"' "$destination/base-inputs.json" > "$destination/paths.bin"
tar -C "$base_root" --null -T "$destination/paths.bin" -cf - | tar -C "$destination/source" -xf -
jq -r '.[].path' "$destination/base-inputs.json" > "$destination/paths.txt"
(cd "$destination/source"; git hash-object --no-filters --stdin-paths < "$destination/paths.txt") \
  > "$destination/observed-blobs.txt"
jq -r '.[].blob' "$destination/base-inputs.json" > "$destination/expected-blobs.txt"
cmp "$destination/expected-blobs.txt" "$destination/observed-blobs.txt"
while IFS=$'\t' read -r path expected; do
  [[ -f "$destination/source/$path" && ! -L "$destination/source/$path" ]]
  mode=$(stat -c %a -- "$destination/source/$path")
  observed=100644
  if (( (8#$mode & 0100) != 0 )); then observed=100755; fi
  [[ "$observed" == "$expected" ]] || { echo "base mode mismatch: $path" >&2; exit 1; }
done < <(jq -r '.[]|[.path,.mode]|@tsv' "$destination/base-inputs.json")
(cd "$destination/source"; git apply --check --whitespace=error-all "$patch"; git apply --whitespace=error-all "$patch")
cp -- "$patch" "$destination/overlay.patch"
nonregular=$(find -P "$destination/source" ! -type d ! -type f -print -quit)
[[ -z "$nonregular" ]] || { echo 'nonregular proposal input' >&2; exit 1; }
(cd "$destination/source"; rg --files --hidden --no-ignore -0 | sort -z) > "$destination/final-paths.bin"
jq -Rse 'split("\u0000") | map(select(length>0)) | all(.[];
  test("^[A-Za-z0-9._/+@=-]+$") and (split("/")|all(. != ".." and . != "")))' \
  "$destination/final-paths.bin" >/dev/null
(cd "$destination/source"; xargs -0 -r sha256sum -- < "$destination/final-paths.bin") > "$destination/source.sha256"
: > "$destination/source-modes.tsv"
while IFS= read -r -d '' path; do
  [[ -f "$destination/source/$path" && ! -L "$destination/source/$path" ]]
  printf '%s\t%s\n' "$path" "$(stat -c %a -- "$destination/source/$path")" >> "$destination/source-modes.tsv"
done < "$destination/final-paths.bin"
input_sha=$(sha256sum "$destination/source.sha256" | cut -d' ' -f1)
mode_sha=$(sha256sum "$destination/source-modes.tsv" | cut -d' ' -f1)
path_sha=$(sha256sum "$destination/final-paths.bin" | cut -d' ' -f1)
jq -n --arg base "$base_commit" --arg patch "$patch_sha" --arg inputs "$input_sha" \
  --arg modes "$mode_sha" --arg paths "$path_sha" --arg source "$destination/source" \
  '{schema:"isolated-proposal-source/v1",base_commit:$base,overlay_sha256:$patch,
    source_directory:$source,input_hash_manifest_sha256:$inputs,mode_manifest_sha256:$modes,
    path_manifest_sha256:$paths,base_git_blobs_and_modes_verified:true,
    source_identity:"base plus uncommitted reviewed overlay; not a delivered clean commit",
    required_build_git_dirty:true,scientific_samples_added:0,verified_cloud_dataset_cells:0}' \
  > "$destination/manifest.json"
cat "$destination/manifest.json"
