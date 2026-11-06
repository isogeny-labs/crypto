#!/bin/bash
# Every complete ecbench session in a study directory that a leaderboard source file lives in must be listed in
# docs/ic/validation/replay-plan.json, so a session the board cites cannot silently go unreplayed.
# Needs jq.  Run from the repository root.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
board=${IC_BOARD:-docs/ic/leaderboard.json}
bad=0
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
jq -er '.sources | to_entries[] | .value.path | select(type == "string" and length > 0)' "$board" > "$tmp/sources"
test -s "$tmp/sources"
# Inspect the index as well as the checkout. Missing or migrated session bytes
# must never remove a cited session from the coverage check.
while IFS= read -r source; do
  study=$(dirname "$source")
  # Figure/analysis files can live below the study root. Find the nearest
  # containing sessions directory, stopping before a shared research namespace.
  while [[ "$study" != . && "$study" != / && "$study" != research && "$study" != docs ]]; do
    git ls-files -- "$study/sessions/*/session.json" > "$tmp/study-descriptors"
    for descriptor in "$study"/sessions/*/session.json; do
      [ ! -f "$descriptor" ] || printf '%s\n' "$descriptor" >> "$tmp/study-descriptors"
    done
    git ls-files -- "$study/.artifacts.json" "$study/sessions/**/.artifacts.json" > "$tmp/manifests"
    while IFS= read -r manifest; do
      directory=$(dirname "$manifest")
      jq -r '.files | keys[] | select(endswith("/session.json") or . == "session.json")' "$manifest" > "$tmp/names"
      while IFS= read -r name; do
        printf '%s/%s\n' "$directory" "$name" >> "$tmp/study-descriptors"
      done < "$tmp/names"
    done < "$tmp/manifests"
    if [ -s "$tmp/study-descriptors" ]; then
      cat "$tmp/study-descriptors" >> "$tmp/descriptors"
      break
    fi
    study=$(dirname "$study")
  done
done < "$tmp/sources"
touch "$tmp/descriptors"
sort -u "$tmp/descriptors" > "$tmp/cited"
if [ "${1:-}" = --list-inputs ]; then
  { sed 's#/session.json$##' "$tmp/cited"; jq -er '.sessions[].dir' "$plan"; } | sort -u > "$tmp/directories"
  while IFS= read -r directory; do
    for name in session.json spec.json host.json plan.json records.jsonl; do
      printf '%s/%s\n' "$directory" "$name"
    done
    # A second hydration pass, after records arrive, includes only stderr
    # attachments that the native auditor checks against a recorded hash.
    if [ -f "$directory/records.jsonl" ]; then
      jq -r 'select(.outcome.stderr_sha256 != null) | .seq' "$directory/records.jsonl" > "$tmp/stderr-seqs"
      while IFS= read -r seq; do
        printf '%s/exec/%06d.stderr\n' "$directory" "$seq"
      done < "$tmp/stderr-seqs"
    fi
  done < "$tmp/directories"
  exit 0
fi
while IFS= read -r descriptor; do
  sd=${descriptor%/session.json}
  if [ ! -f "$descriptor" ]; then
    echo "cited session $sd has no session.json; hydrate its frozen inputs"; bad=1; continue
  fi
  status=$(jq -er '.status | select(type == "string")' "$descriptor")
  [ "$status" = complete ] || continue
  if [ ! -s "$sd/records.jsonl" ]; then
    echo "complete cited session $sd has no records.jsonl; hydrate its frozen inputs"; bad=1
  fi
  jq -e --arg d "$sd" '.sessions[] | select(.dir == $d)' "$plan" > /dev/null || { echo "session $sd is cited but is not in $plan"; bad=1; }
done < "$tmp/cited"
[ $bad = 0 ] && echo "replay plan covers every complete session the board cites"
exit $bad
