#!/usr/bin/env bash
# Put the workspace's Rust build inputs back at the last revision that the
# studies' frozen Cargo.lock files still describe.
#
# Replay workflows copy a study's frozen Cargo.lock over the root one and build
# with --locked.  That stopped working when 83ff1b0c0 (#2001) added rusqlite and
# volcano-cli to Cargo.toml: current source cannot build against those locks.
# Every frozen lock resolves against the manifest of 83ff1b0c0^, so this script
# restores, from that revision, exactly the files a build reads (the manifest,
# src/, examples/, tests/, benches/, the explicitly declared targets and every
# include_str!/include_bytes! input, listed in frozen-lock-source.files) and
# leaves Cargo.lock alone, so the frozen copy stays byte-identical.  Custody
# checks that compare the workspace lock with the frozen one, scripts that run
# cargo from the workspace root and target/ paths all keep working.
#
# Run it from the repository root, after the frozen lock is in place and before
# any file is copied into examples/.
#
# With --restore it puts every file it touched back as HEAD has it (including
# Cargo.lock), for jobs that check the tree is clean after the frozen build.
set -euo pipefail
REV=8d22188acf33940d69caf0fa29ed4c636eb4407b
LIST=.github/scripts/frozen-lock-source.files
if [ "${1-}" = "--restore" ]; then
  mapfile -t files < "$LIST"
  rm -rf src examples tests benches
  # Restore what this checkout really had on disk ("H": tracked and not
  # skip-worktree, so sparse checkouts stay sparse); remove anything else the
  # overlay wrote: untracked paths and paths outside the sparse set.
  mapfile -t present < <(git ls-files -v -- src examples tests benches Cargo.toml Cargo.lock "${files[@]}" \
    | awk '$1 == "H" { sub(/^H /, ""); print }')
  if [ "${#present[@]}" -gt 0 ]; then git checkout -- "${present[@]}"; fi
  for f in "${files[@]}"; do
    [ "$(git ls-files -v -- "$f" | cut -c1)" = "H" ] || rm -f -- "$f"
  done
  echo "frozen-lock-source: restored HEAD build inputs"
  exit 0
fi
if ! git cat-file -e "$REV^{commit}" 2>/dev/null; then
  git fetch --no-tags --quiet --depth=1 origin "$REV"
fi
# Nothing newer than the frozen revision may be auto-discovered as a target.
rm -rf src examples tests benches
mapfile -t files < "$LIST"
git archive "$REV" -- "${files[@]}" | tar -x
echo "frozen-lock-source: restored ${#files[@]} build inputs from $REV"
