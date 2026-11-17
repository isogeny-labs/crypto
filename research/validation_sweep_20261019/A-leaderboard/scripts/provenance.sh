#!/bin/sh
# Usage: provenance.sh <repo-relative source file>   (run from the repo root)
# Prints one JSON object: first commit (sha, ISO date, subject) of
#  - the source file, the protocol-like files in its study directory, and the
#    raw-run files under that directory,
# plus whether each protocol commit is a strict ancestor of the data commit.
# Read-only: only `git log` / `git merge-base` / `find`.
set -eu
src="$1"
dir=$(dirname "$src")
# study directory = the first two path components for research/<study>/..., else dirname
study=$(echo "$src" | awk -F/ '$1=="research"{print $1"/"$2; next}{print $1"/"$2"/"$3}')
first() { git log --diff-filter=A --follow --format='%H|%cI|%s' -- "$@" 2>/dev/null | tail -1; }
firstdir() { git log --diff-filter=A --format='%H|%cI|%s' -- "$@" 2>/dev/null | tail -1; }
srcc=$(first "$src")
# protocol-like files anywhere under the study directory (depth-limited)
protos=$(git ls-files "$study" | grep -iE '(^|/)(PROTOCOL|SPEC|PREREG[A-Z_]*|protocol|spec)[^/]*\.(md|json|yaml|toml)$' | head -20)
pbest=""
for p in $protos; do
  c=$(first "$p")
  d=$(echo "$c" | cut -d'|' -f2)
  pbest="$pbest
$d|$p|$c"
done
pfirst=$(echo "$pbest" | sed '/^$/d' | sort | head -1)
# raw runs: files under study dir whose path contains runs/ sessions/ measurements records/
raws=$(git ls-files "$study" | grep -E '/(runs|runs-isolated|sessions|measurements|records|profiles)/' | head -2000)
rfirst=""
if [ -n "$raws" ]; then
  rfirst=$(git log --diff-filter=A --format='%H|%cI|%s' -- $(echo "$raws" | head -400) 2>/dev/null | tail -1)
fi
sha() { echo "$1" | cut -d'|' -f1; }
anc="n/a"
if [ -n "$pfirst" ]; then
  psha=$(echo "$pfirst" | awk -F'|' '{print $3}')
  dsha=$(sha "${rfirst:-$srcc}")
  if [ -n "$psha" ] && [ -n "$dsha" ]; then
    if [ "$psha" = "$dsha" ]; then anc="same_commit"
    elif git merge-base --is-ancestor "$psha" "$dsha" 2>/dev/null; then anc="protocol_ancestor_of_data"
    else anc="protocol_NOT_ancestor_of_data"; fi
  fi
fi
jq -n --arg src "$src" --arg study "$study" --arg srcc "$srcc" --arg pfirst "$pfirst" --arg rfirst "$rfirst" --arg anc "$anc" \
  '{source:$src,study:$study,source_first_commit:$srcc,protocol_first:$pfirst,raw_run_first_commit:$rfirst,order:$anc}'
