#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
checker="$repo/scripts/check-large-files.sh"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

git init -q --bare "$tmp/empty-remote.git"
git init -q "$tmp/repo"
cd "$tmp/repo"
git config user.name 'large-file check test'
git config user.email 'large-file-check@example.invalid'
printf 'base\n' > README
git add README
git commit -qm base
base=$(git rev-parse HEAD)

printf 'new blob whose size must be checked\n' > new.txt
blob=$(git hash-object -w new.txt)
git update-index --add --cacheinfo "100644,$blob,new.txt"
tree=$(git write-tree)
head=$(printf 'missing-blob test\n' | git commit-tree "$tree" -p "$base")
git update-ref refs/heads/test-head "$head"

# Make the changed blob genuinely unavailable while retaining the tree and
# commit objects, then force the script through its promisor-fetch path.
rm -f ".git/objects/${blob:0:2}/${blob:2}"
git remote add origin "$tmp/empty-remote.git"
git config remote.origin.promisor true

if output=$("$checker" "$base" "$head" 2>&1); then
  echo 'expected missing-blob sizing to fail closed' >&2
  exit 1
fi
case "$output" in
  *'batched fetch failed; checking and retrying missing blobs directly'*'cannot size new.txt'*) ;;
  *) printf 'unexpected failure output:\n%s\n' "$output" >&2; exit 1 ;;
esac

echo 'PASS: unavailable changed blobs fail closed after the batched fetch fails'
