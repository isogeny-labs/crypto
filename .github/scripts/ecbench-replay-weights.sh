#!/usr/bin/env bash
# Print "directory<TAB>weight_ms<TAB>longest_run_ms" for every committed ecbench
# session in a list file that is neither a hash-pinned incomplete scratch
# session nor a pinned defective one, and check the long-session pins.
# weight_ms is the recorded solver wall time (sum of time.solve_wall_ns over
# the verified records; a replay costs about as much), or, for a session with
# none, its record megabytes; longest_run_ms is its longest verified run.
#
# LONG_SESSIONS.tsv pins (path, sha256 of the committed records file, reason,
# longest run in seconds) every session with a verified run over 3.5 h.  The
# script fails if a pinned hash no longer matches, a listed path vanished or is
# not in the list, a listed session has no run over 3.5 h, or an unlisted session
# has one: the weekly rotation cannot fit such a run in a hosted job.
# Usage: ecbench-replay-weights.sh LIST [INCOMPLETE_TSV] [DEFECTIVE_TSV] [LONG_TSV] > weights.tsv
set -euo pipefail
list=${1:?usage: ecbench-replay-weights.sh LIST [INCOMPLETE_TSV] [DEFECTIVE_TSV] [LONG_TSV]}
incomplete=${2:-docs/ecbench/INCOMPLETE_SESSIONS.tsv}
defective=${3:-docs/ecbench/DEFECTIVE_SESSIONS.tsv}
long=${4:-docs/ecbench/LONG_SESSIONS.tsv}
threshold_ms=12600000
for f in "$list" "$incomplete" "$defective" "$long"; do
  test -s "$f" || { echo "::error::$f is missing or empty" >&2; exit 1; }
done
records_file() {
  for c in records.jsonl records.jsonl.gz ecbench_records.jsonl.gz; do
    if [[ -f "$1/$c" ]]; then echo "$1/$c"; return 0; fi
  done
  return 1
}
excluded=$(mktemp)
cut -f1 "$incomplete" "$defective" | { grep -v '^#' || true; } | { grep -v '^$' || true; } | sort -u > "$excluded"
pinned=$(mktemp)
fail=0
while IFS=$'\t' read -r dir sha reason seconds extra; do
  [[ -z "$dir" || "$dir" == \#* ]] && continue
  if [[ ! "$sha" =~ ^[0-9a-f]{64}$ || -z "${reason:-}" || ! "${seconds:-}" =~ ^[0-9]+$ || -n "${extra:-}" ]] \
     || grep -Fxq "$dir" "$pinned"; then
    echo "::error::malformed or duplicate long-session row: $dir" >&2; fail=1; continue
  fi
  printf '%s\n' "$dir" >> "$pinned"
  if ! grep -Fxq "$dir" "$list"; then
    echo "::error::long-session row names no committed ecbench session: $dir" >&2; fail=1; continue
  fi
  if ! rec=$(records_file "$dir"); then
    echo "::error::long-session records are missing: $dir" >&2; fail=1; continue
  fi
  actual=$(sha256sum "$rec"); actual=${actual%% *}
  if [[ "$actual" != "$sha" ]]; then
    echo "::error::long-session bytes changed: $rec" >&2; fail=1
  fi
  if [[ "$seconds" -le $((threshold_ms / 1000)) ]]; then
    echo "::error::$dir is pinned as long but its longest run is $seconds s (threshold 12600 s = 3.5 h)" >&2; fail=1
  fi
done < "$long"
while IFS= read -r dir; do
  grep -Fxq "$dir" "$excluded" && continue
  if rec=$(records_file "$dir"); then
    if [[ "$rec" == *.gz ]]; then reader=(gzip -dc "$rec"); else reader=(cat "$rec"); fi
    read -r weight longest < <("${reader[@]}" | { grep -F '"status":"verified"' || true; } | { grep -oP '"solve_wall_ns":\K[0-9]+' || true; } \
      | awk '{ s += $1; if ($1 > m) m = $1 } END { printf "%d %d\n", s / 1e6, m / 1e6 }')
  else
    weight=0; longest=0
  fi
  if [[ "${weight:-0}" -le 0 ]]; then
    weight=$(find "$dir" -maxdepth 1 -type f -printf '%s %f\n' | awk '{ b += ($2 ~ /\.gz$/) ? 8 * $1 : $1 } END { printf "%d", b / 1e6 + 1 }')
  fi
  if [[ "$longest" -gt "$threshold_ms" ]] && ! grep -Fxq "$dir" "$pinned"; then
    echo "::error::$dir has a verified run of $((longest / 1000)) s, over 3.5 h: pin it in $long" >&2; fail=1
  fi
  printf '%s\t%s\t%s\n' "$dir" "$weight" "$longest"
done < "$list"
rm -f "$excluded" "$pinned"
exit "$fail"
