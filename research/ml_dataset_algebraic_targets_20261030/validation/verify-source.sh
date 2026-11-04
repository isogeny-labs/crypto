#!/bin/bash
# Source/fixture preflight only: no compilation or experiment execution.
set -euo pipefail
source_commit=$1
inventory=$2
inventory_sha=$3
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
cd "$root"
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT

# Include every tracked dependency and validation input. Only the shared journal
# and this pipeline's changing publication index are outside the source boundary.
git diff --exit-code "$source_commit" -- . \
  ':(exclude)docs/session-logs/**' \
  ':(exclude)research/ml_dataset_algebraic_targets_20261030/store-index.json'
git ls-files --others --exclude-standard -z -- \
  '*.rs' '*.toml' Cargo.lock '**/Cargo.lock' '.cargo/**' > "$temporary/untracked"
# Cargo discovers files independently of Git ignore rules. Check its actual
# source/target directories again without --exclude-standard, excluding build
# caches by selecting source directories rather than entire dependency trees.
git ls-files --others -z -- \
  build.rs Cargo.toml Cargo.lock rust-toolchain rust-toolchain.toml .cargo \
  ':(glob)src/**/*.rs' ':(glob)examples/**/*.rs' ':(glob)tests/**/*.rs' ':(glob)benches/**/*.rs' \
  ':(glob)crates/*/src/**/*.rs' ':(glob)crates/*/Cargo.toml' \
  ':(glob)isogeny_algos/src/**/*.rs' isogeny_algos/Cargo.toml \
  ':(glob)tools/factor-base/src/**/*.rs' tools/factor-base/Cargo.toml \
  ':(glob)tools/volcano-cli/src/**/*.rs' tools/volcano-cli/Cargo.toml \
  ':(glob)research/pencil_large_prime_20261029/src/**/*.rs' \
  research/pencil_large_prime_20261029/Cargo.toml >> "$temporary/untracked"
if [[ -s "$temporary/untracked" ]]; then
  printf 'Untracked Rust/Cargo/config inputs are outside the frozen source:\n' >&2
  tr '\0' '\n' < "$temporary/untracked" >&2
  exit 1
fi

printf '%s  %s\n' "$inventory_sha" "$inventory" | sha256sum -c -
jq -e '
  .schema=="exact-source-materialization/v1" and
  (.entries|length)==.files and
  ([.entries[].bytes]|add)==.bytes and
  ([.entries[].path]|unique|length)==.files and
  all(.entries[];
    (.path|test("^[A-Za-z0-9._/-]+$")) and
    (.path|startswith("/")|not) and
    (.path|split("/")|all(. != ".." and . != "")) and
    (.git_blob|test("^[0-9a-f]{40}$")) and
    (.sha256|test("^[0-9a-f]{64}$")) and
    (.bytes|type=="number" and .>=0 and floor==.))
' "$inventory" >/dev/null
mapfile -t paths < <(jq -r '.entries[].path' "$inventory")
# One Git tree read binds every inventory blob to the execution commit. This
# deliberately permits a newer docs-only execution commit than the restoration.
git ls-tree -rz "$source_commit" -- "${paths[@]}" > "$temporary/tree"
jq -Rs '[split("\u0000")[] | select(length>0) | split("\t") |
  {path:.[1],blob:(.[0]|split(" ")|.[2]),type:(.[0]|split(" ")|.[1])}]' \
  "$temporary/tree" > "$temporary/tree.json"
jq -e --slurpfile tree "$temporary/tree.json" '
  (.entries|length)==($tree[0]|length) and
  all(.entries[]; . as $entry |
    any($tree[0][]; .path==$entry.path and .type=="blob" and .blob==$entry.git_blob))
' "$inventory" >/dev/null
# Explicit byte hashes catch modifications hidden by sparse skip-worktree bits.
jq -r '.entries[]|.sha256+"  "+.path' "$inventory" > "$temporary/hashes"
sha256sum -c "$temporary/hashes"
while IFS=$'\t' read -r path bytes; do
  [[ ! -L "$path" && -f "$path" ]]
  [[ $(stat -c %s -- "$path") == "$bytes" ]]
done < <(jq -r '.entries[]|[.path,(.bytes|tostring)]|@tsv' "$inventory")
