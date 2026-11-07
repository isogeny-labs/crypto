#!/usr/bin/env bash
# E39 validator: clean export of a recorded commit (validator's own script; no
# producer script used).  Paths: Cargo.toml, Cargo.lock, src/, examples/,
# benches/, the path-dependency directories (whole), every `path = "..."` of
# the root Cargo.toml, and every literal include_str!/include_bytes! target
# (resolved relative to the including file) present at the commit.
#   usage: export_v39.sh <repo> <commit> <dest> <pathlist-out>
set -eu
repo=$1; c=$2; dest=$3; list=$4
cd "$repo"
test ! -e "$dest" || { echo "dest exists: $dest" >&2; exit 1; }
{
  echo Cargo.toml; echo Cargo.lock; echo src; echo examples; echo benches
  for d in crates/hashed-ecdl-tensor crates/ecdl-tensor-ic research/pencil_large_prime_20261029 \
           research/five_leads_e2e_20261019/native tools/factor-base tools/volcano-cli isogeny_algos; do
    echo "$d"
  done
  git show "$c:Cargo.toml" | sed -n 's/^path *= *"\([^"]*\)".*/\1/p'
} | sort -u > "$list.base"
mkdir -p "$dest"
git archive "$c" -- $(cat "$list.base") | tar -x -C "$dest"
# include targets, iterated to a fixed point over the exported .rs files
: > "$list.inc"
for round in 1 2 3; do
  ( cd "$dest" && grep -rlE --include='*.rs' 'include_(str|bytes)!\(' . ) | while read -r f; do
    dir=$(dirname "${f#./}")
    grep -oE 'include_(str|bytes)!\(\s*"[^"]+"' "$dest/$f" | sed -E 's/.*"([^"]+)"/\1/' | while read -r t; do
      p=$(python3 -I -c 'import os,sys; print(os.path.normpath(os.path.join(sys.argv[1], sys.argv[2])))' "$dir" "$t")
      echo "$p"
    done
  done | sort -u > "$list.inc.new"
  missing=$(comm -23 "$list.inc.new" <(cd "$dest" && find . -type f | sed 's#^\./##' | sort))
  present=$(for m in $missing; do if git cat-file -e "$c:$m" 2>/dev/null; then echo "$m"; fi; done)
  cat "$list.inc.new" >> "$list.inc"
  [ -z "$present" ] && break
  git archive "$c" -- $present | tar -x -C "$dest"
done
sort -u "$list.inc" -o "$list.inc"
cat "$list.base" "$list.inc" | sort -u > "$list"
# blob check: every exported file against git ls-tree of the commit
( cd "$dest" && find . -type f | sed 's#^\./##' | LC_ALL=C sort ) > "$list.files"
git ls-tree -r "$c" -- $(cat "$list.files") | awk '{print $4"\t"$3}' | LC_ALL=C sort > "$list.want"
( cd "$dest" && git --git-dir="$repo/.git" hash-object --no-filters --stdin-paths < "$list.files" ) > "$list.gothash"
paste "$list.files" "$list.gothash" | LC_ALL=C sort > "$list.got"
n=$(wc -l < "$list.files"); w=$(wc -l < "$list.want")
if cmp -s "$list.want" "$list.got"; then bad=0; else bad=$(diff "$list.want" "$list.got" | grep -c '^>' || true); fi
echo "exported=$n in_tree=$w mismatches=$bad"
