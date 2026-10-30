#!/usr/bin/env bash
# Validate the unapplied candidate in a fresh, disposable CPU snapshot.
# The shared source and historical validation artifacts are read-only inputs.
set -euo pipefail

if [[ $# != 2 ]]; then
  echo 'usage: validate.sh FRESH-SNAPSHOT TARGET-DIRECTORY' >&2
  exit 64
fi
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
repo=$(cd -- "$here/../../.." && pwd -P)
source_path=$(jq -er '.path' "$here/source.json")
case "/$source_path/" in
  *'/../'*|*'/./'*|//*|//) echo 'invalid relative source path' >&2; exit 65 ;;
esac
sha() { sha256sum -- "$1" | cut -d ' ' -f 1; }
check_hash() {
  if [[ $(sha "$1") != "$2" ]]; then
    echo "source identity mismatch: $1" >&2
    exit 65
  fi
}
source="$repo/$source_path"
check_hash "$source" "$(jq -er '.base_sha256' "$here/source.json")"
check_hash "$here/target-reuse.patch" "$(jq -er '.patch_sha256' "$here/source.json")"
snapshot=$(realpath -m -- "$1")
target=$(realpath -m -- "$2")
mkdir -p -- "$(dirname -- "$snapshot")"
# mkdir is an atomic refusal of any existing destination, including symlinks.
mkdir -- "$snapshot"
mkdir -p -- "$snapshot/crates"
for name in hashed-ecdl-cpu hashed-ecdl-tensor; do
  tar -C "$repo/crates" --exclude=target --exclude=.git -cf - "$name" |
    tar -C "$snapshot/crates" -xf -
done
mkdir -p -- "$snapshot/$(dirname -- "$source_path")"
cp -- "$source" "$snapshot/$source_path"
for i in 0 1 2; do
  certificate="research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-$i-1.txt"
  mkdir -p -- "$snapshot/$(dirname -- "$certificate")"
  cp -- "$repo/$certificate" "$snapshot/$certificate"
done
cd -- "$snapshot"
git apply --check "$here/target-reuse.patch"
git apply "$here/target-reuse.patch"
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
check_hash "$source" "$(jq -er '.base_sha256' "$here/source.json")"
jq -n --slurpfile source "$here/source.json" --slurpfile commands commands.jsonl \
  '{source:$source[0],commands:$commands,shared_source_unchanged:true,
    scope:"development correctness checks only; no timing or EKS execution"}' > validation.json
echo 'PASS unapplied snapshot: release tests, strict Clippy and formatting; shared source unchanged'
