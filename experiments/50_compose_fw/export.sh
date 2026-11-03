#!/usr/bin/env bash
# E43 (copied from experiments/49_macaulay_tails/export.sh, unchanged): clean export of a committed SHA for building measured binaries.
# Archives Cargo.toml, Cargo.lock, src/, examples/, benches/, every
# `path = "..."` of Cargo.toml (targets and path dependencies, as
# directories when they are crates) and every literal include_str!/
# include_bytes! target present at the commit; then checks every exported
# file's blob against the commit.
#   usage: export.sh <sha> <dest dir>
set -euo pipefail
sha=$1; dest=$2
cd "$(git rev-parse --show-toplevel)"
paths=$(mktemp)
{
  echo Cargo.toml; echo Cargo.lock; echo src; echo examples; echo benches
  git show "$sha:Cargo.toml" | grep -oE 'path *= *"[^"]+"' | sed -E 's/path *= *"([^"]+)"/\1/' |
    while read -r p; do
      if git cat-file -e "$sha:$p/Cargo.toml" 2>/dev/null; then echo "$p"; else echo "$p"; fi
    done
  git grep -h -oE 'include_(str|bytes)!\("[^"]+"\)' "$sha" -- '*.rs' >/dev/null 2>&1 || true
  git grep -n -oE 'include_(str|bytes)!\("[^"]+"\)' "$sha" -- '*.rs' |
    sed -E "s/^$sha://" | while IFS=: read -r file _ inc; do
      rel=$(echo "$inc" | sed -E 's/include_(str|bytes)!\("([^"]+)"\)/\2/')
      t=$(realpath -m --relative-to=. "$(dirname "$file")/$rel")
      git cat-file -e "$sha:$t" 2>/dev/null && echo "$t"
    done
} | sort -u >"$paths"
rm -rf "$dest"; mkdir -p "$dest"
# shellcheck disable=SC2046
git archive "$sha" -- $(cat "$paths") | tar -x -C "$dest"
n=0; bad=0
while IFS= read -r f; do
  n=$((n + 1))
  [ "$(git hash-object "$dest/$f")" = "$(git rev-parse "$sha:$f")" ] || bad=$((bad + 1))
done < <(cd "$dest" && find . -type f | sed 's|^\./||')
echo "export $sha -> $dest: $(wc -l <"$paths") paths, $n files, $bad blob mismatches"
rm -f "$paths"
