#!/usr/bin/env bash
# Refuse new or changed files over a size limit (AGENTS.md §13): a file that
# big goes through `crypto store push`, and a raw run tree lands as one
# runs.tar.xz (§13a). Every blob committed stays in every future clone.
#
#   scripts/check-large-files.sh BASE [HEAD] [LIMIT_BYTES]   # HEAD default HEAD, limit 10 MiB
#
# Checks the blobs that BASE..HEAD adds or modifies; files already in BASE are
# not re-judged. Generated text that git delta-compresses well is exempt (ALLOW):
# 33 versions of the 31 MB docs/browser/data.json pack into 3 MB. Thin git orchestration only, per AGENTS.md ("Implementation
# language"). Exit 1 lists each offending path with its size.
set -euo pipefail

base=${1:?usage: check-large-files.sh BASE [HEAD] [LIMIT_BYTES]}
head=${2:-HEAD}
limit=${3:-10485760}

# Regenerated on most PRs (CLAUDE.md: leaderboard and lab browser); shell globs.
ALLOW=(
  docs/browser/data.json
  'docs/curves/*.json'
  docs/curves/cover-links.yaml  # JSON graph regenerated with registry.json
)

# The diff below runs in a process substitution, whose failure set -e misses.
git rev-parse --quiet --verify "$base^{commit}" >/dev/null || { echo "check-large-files: not a commit: $base" >&2; exit 2; }
git rev-parse --quiet --verify "$head^{commit}" >/dev/null || { echo "check-large-files: not a commit: $head" >&2; exit 2; }

declare -a blobs=() paths=()
# --raw -z: ":<old mode> <new mode> <old blob> <new blob> <status>\0<path>\0"
while IFS= read -r -d '' meta && IFS= read -r -d '' path; do
  read -r _ mode _ blob _ <<<"$meta"
  [ "$mode" = 160000 ] && continue  # submodule commit, not a blob
  for pat in "${ALLOW[@]}"; do
    # shellcheck disable=SC2053  # $pat is a glob on purpose
    [[ $path == $pat ]] && continue 2
  done
  blobs+=("$blob")
  paths+=("$path")
done < <(git diff --raw -z --no-abbrev --no-renames --diff-filter=AMT "$base" "$head")

# A research PR can change thousands of blobs. Asking Git once per blob made
# the required check exceed its ten-minute job limit, whereas batch-check
# reads the same object metadata through one Git process.
declare -a sizes=()
if [ "${#blobs[@]}" -gt 0 ]; then
  mapfile -t sizes < <(printf '%s\n' "${blobs[@]}" | git cat-file --batch-check='%(objectsize)')
fi
[ "${#sizes[@]}" -eq "${#blobs[@]}" ] || {
  echo "check-large-files: incomplete git cat-file batch response" >&2
  exit 2
}

over=0
for i in "${!blobs[@]}"; do
  size=${sizes[$i]}
  [[ $size =~ ^[0-9]+$ ]] || {
    echo "check-large-files: invalid size for ${paths[$i]}: $size" >&2
    exit 2
  }
  if [ "$size" -gt "$limit" ]; then
    printf '%12d bytes  %s\n' "$size" "${paths[$i]}"
    over=$((over + 1))
  fi
done

if [ "$over" -gt 0 ]; then
  echo "check-large-files: $over file(s) over $limit bytes. Push them with 'crypto store push' and commit the store/index.json entry (AGENTS.md §13), or pack a raw run tree as runs.tar.xz (§13a)." >&2
  exit 1
fi
echo "check-large-files: no added or modified file over $limit bytes"
