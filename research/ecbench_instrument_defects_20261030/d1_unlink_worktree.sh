#!/bin/bash
# D1 remedy for one git worktree whose files share inodes with other worktrees
# (jdupes -r -L, 2026-10-24).  For the owner to run; nothing here runs by itself.
#
#   d1_unlink_worktree.sh <worktree>            list only (default)
#   d1_unlink_worktree.sh <worktree> --apply    give every multi-link file its own inode
#   ... --apply --force                         also when a linked file is dirty
#
# The new inode is an APFS clone (cp -c, clonefile(2)): it shares data blocks
# copy-on-write, so it costs almost no space.  Plain cp would cost about 93 GB
# over the 19 linked worktrees (d1_census.json).  Measured 2026-10-30 on
# SSD990-2: a 200 MB linked file broken this way left free space unchanged, and
# an append to it no longer reached the other link.
#
# A dirty linked file may hold another worktree's uncommitted edit (pilot
# 2026-10-30 found one in inherited_f4.rs).  Breaking the link freezes that
# content into this worktree, so --apply stops on dirty linked files unless
# --force is given; decide per file first (git diff, then keep or
# `git checkout -- <file>`).
set -euo pipefail
wt=${1:?usage: d1_unlink_worktree.sh <worktree> [--apply [--force]]}
apply=0; force=0
for a in "${@:2}"; do
  case $a in --apply) apply=1 ;; --force) force=1 ;; *) echo "unknown $a" >&2; exit 2 ;; esac
done
cd "$wt"
linked=$(mktemp "${TMPDIR:-/tmp}/d1-linked.XXXXXX"); trap 'rm -f "$linked"' EXIT
find . -path ./.git -prune -o -path ./target -prune -o -type f -links +1 -print0 \
  | xargs -0 stat -f '%l %N' | sed 's# \./# #' > "$linked" || true
n=$(wc -l < "$linked" | tr -d ' ')
echo "$wt: $n files with link count > 1"
dirty=$(git status --porcelain=v1 | awk '{print $NF}' | grep -Fxf <(cut -d' ' -f2- "$linked") || true)
if [ -n "$dirty" ]; then
  echo "linked files with uncommitted changes (check whose edit each is):"
  printf '  %s\n' $dirty
fi
[ "$apply" = 1 ] || { echo "list only; rerun with --apply"; exit 0; }
if [ -n "$dirty" ] && [ "$force" = 0 ]; then
  echo "refusing: dirty linked files above; resolve them or pass --force" >&2; exit 1
fi
cut -d' ' -f2- "$linked" | while IFS= read -r f; do
  cp -c -p "$f" "$f.d1-unlink.$$" && mv "$f.d1-unlink.$$" "$f"
done
left=$(find . -path ./.git -prune -o -path ./target -prune -o -type f -links +1 -print | wc -l | tr -d ' ')
echo "done; files still multi-linked: $left"
