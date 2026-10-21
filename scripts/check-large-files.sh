#!/usr/bin/env bash
# Size cap for new blobs (AGENTS.md §13): every file a change adds or
# rewrites must be at most LARGE_FILE_MAX bytes (5 MB) unless its path matches
# a pattern in .github/large-files-allowed.txt. Larger evidence goes through
# the store (`crypto store push`), with its store/index.json entry committed.
# Files already in the base are not checked until a change rewrites them.
#
#   scripts/check-large-files.sh [BASE [HEAD]]   # default: origin/main HEAD
#
# Compares HEAD with its merge base with BASE. Works in a blobless clone: it
# fetches the changed blobs in one request before sizing them.
# Thin git orchestration only, per AGENTS.md ("Implementation language").
set -euo pipefail

MAX=${LARGE_FILE_MAX:-5000000}
ALLOW=.github/large-files-allowed.txt
base_ref=${1:-origin/main}
head_ref=${2:-HEAD}

cd "$(git rev-parse --show-toplevel)"
base=$(git merge-base "$base_ref" "$head_ref") || { echo "check-large-files: no merge base of $base_ref and $head_ref" >&2; exit 2; }

changes=$(mktemp)
trap 'rm -f "$changes"' EXIT
# Added, copied, modified, renamed or retyped regular files: "<blob>\t<path>".
git diff --raw --no-renames --diff-filter=ACMRT -z "$base" "$head_ref" |
  tr '\0' '\n' | paste - - |
  awk -F'\t' '{ split($1, m, " "); if (m[2] ~ /^100/) print m[4] "\t" $2 }' > "$changes"
[ -s "$changes" ] || { echo "check-large-files: no changed files"; exit 0; }

if [ "$(git config --bool remote.origin.promisor 2>/dev/null)" = true ]; then
  cut -f1 "$changes" | sort -u |
    git cat-file --batch-check='%(objectname)' 2>/dev/null | awk '$2 == "missing" { print $1 }' |
    git -c fetch.negotiationAlgorithm=noop fetch -q --no-tags --no-write-fetch-head \
      --filter=blob:none --stdin origin 2>/dev/null || true
fi

patterns=()
if [ -f "$ALLOW" ]; then
  while IFS= read -r line; do
    line=${line%%#*}; line=${line%"${line##*[![:space:]]}"}
    [ -n "$line" ] && patterns+=("$line")
  done < "$ALLOW"
fi

allowed() {
  local p
  for p in ${patterns[@]+"${patterns[@]}"}; do
    # shellcheck disable=SC2053  # the pattern is a glob on purpose
    [[ $1 == $p ]] && return 0
  done
  return 1
}

over=0 checked=0
while IFS=$'\t' read -r size path; do
  case "$size" in ''|*[!0-9]*) echo "check-large-files: cannot size $path" >&2; exit 2 ;; esac
  checked=$((checked + 1))
  [ "$size" -le "$MAX" ] && continue
  if allowed "$path"; then
    echo "allowed  $size  $path"
  else
    echo "TOO BIG  $size  $path"
    over=$((over + 1))
  fi
done < <(paste <(cut -f1 "$changes" | git cat-file --batch-check='%(objectsize)') <(cut -f2- "$changes"))

if [ "$over" -gt 0 ]; then
  cat >&2 <<EOF
check-large-files: $over of $checked changed files exceed $MAX bytes.
Push each with \`crypto store push\` and commit its store/index.json entry
instead (AGENTS.md §13), or compress raw run trees into one archive under the
cap (§13a). A file the repository must carry in Git (a regenerated catalog, a
compile-time fixture) needs a line in $ALLOW, reviewed in the PR.
EOF
  exit 1
fi
echo "check-large-files: $checked changed files, none over $MAX bytes outside $ALLOW"
