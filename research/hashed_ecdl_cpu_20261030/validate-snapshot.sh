#!/usr/bin/env bash
# Validate the unapplied candidate in a fresh, disposable CPU snapshot.
# The shared source and historical validation artifacts are read-only inputs.
set -euo pipefail

if [[ $# != 4 ]]; then
  echo 'usage: validate-snapshot.sh CANDIDATE-DIRECTORY PATCH-FILE FRESH-SNAPSHOT TARGET-DIRECTORY' >&2
  exit 64
fi
here=$(cd -- "$1" && pwd -P)
patch_name=$2
[[ "$patch_name" == *.patch && "$patch_name" != */* ]] || exit 65
repo=$(cd -- "$here/../../.." && pwd -P)
source_path=$(jq -er '.path' "$here/source.json")
case "/$source_path/" in
  *'/../'*|*'/./'*|//*) echo 'invalid relative source path' >&2; exit 65 ;;
esac
sha() { sha256sum -- "$1" | cut -d ' ' -f 1; }
check_hash() {
  if [[ $(sha "$1") != "$2" ]]; then
    echo "source identity mismatch: $1" >&2
    exit 65
  fi
}
source="$repo/$source_path"
shared_before=$(sha "$source")
base_commit=$(jq -er '.base_commit' "$here/source.json")
[[ "$base_commit" =~ ^[0-9a-f]{40}$ ]] || exit 65
check_hash "$here/$patch_name" "$(jq -er '.patch_sha256' "$here/source.json")"
snapshot=$(realpath -m -- "$3")
target=$(realpath -m -- "$4")
mkdir -p -- "$(dirname -- "$snapshot")"
# mkdir is an atomic refusal of any existing destination, including symlinks.
mkdir -- "$snapshot"
paths=(crates/hashed-ecdl-cpu crates/hashed-ecdl-tensor "$source_path")
for i in 0 1 2; do
  paths+=("research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-$i-1.txt")
done
for requested in "${paths[@]}"; do
  git -C "$repo" cat-file -e "$base_commit:$requested"
done
# Read only selected blobs. git archive may hydrate unrelated promisor objects
# in a partial clone before applying its path filter.
git -C "$repo" ls-tree -r -z "$base_commit" -- "${paths[@]}" |
  while IFS= read -r -d '' entry; do
    header=${entry%%$'\t'*}
    file=${entry#*$'\t'}
    read -r mode object_type object_id <<< "$header"
    [[ "$object_type" == blob && ( "$mode" == 100644 || "$mode" == 100755 ) ]] || exit 65
    mkdir -p -- "$snapshot/$(dirname -- "$file")"
    git -C "$repo" cat-file blob "$object_id" > "$snapshot/$file"
    if [[ "$mode" == 100755 ]]; then chmod +x -- "$snapshot/$file"; fi
  done
check_hash "$snapshot/$source_path" "$(jq -er '.base_sha256' "$here/source.json")"
cd -- "$snapshot"
git apply --check "$here/$patch_name"
git apply "$here/$patch_name"
check_hash "$snapshot/$source_path" "$(jq -er '.candidate_sha256' "$here/source.json")"
manifest="$snapshot/crates/hashed-ecdl-cpu/Cargo.toml"
common=(--locked --all-targets --manifest-path "$manifest" --target-dir "$target")
: > commands.jsonl
run_check() {
  local index=$1
  shift
  local log="validation-$index.log" status=0
  "$@" > "$log" 2>&1 || status=$?
  jq -cn --argjson command "$(jq -cn --args '$ARGS.positional' -- "$@")" \
    --argjson exit_code "$status" --arg log "$log" --arg sha256 "$(sha "$log")" \
    '{command:$command,exit_code:$exit_code,log:$log,sha256:$sha256}' >> commands.jsonl
  if (( status )); then
    cat -- "$log" >&2
    exit "$status"
  fi
}
run_check 0 cargo test --release "${common[@]}"
run_check 1 cargo clippy "${common[@]}" -- -D warnings
run_check 2 cargo fmt --check --manifest-path "$manifest"
check_hash "$source" "$shared_before"
jq -n --slurpfile source "$here/source.json" --slurpfile commands commands.jsonl --arg shared_source_sha256 "$shared_before" \
  '{source:$source[0],commands:$commands,shared_source_unchanged:true,shared_source_sha256:$shared_source_sha256,
    scope:"development correctness checks only; no timing or EKS execution"}' > validation.json
echo 'PASS pinned historical snapshot: release tests, strict Clippy and formatting; shared source unchanged'
