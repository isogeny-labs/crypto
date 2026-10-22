#!/bin/bash
# Equality check of PROTOCOL.md 12.3 (E1, E2 rows): every cell of NEW.jsonl that has a
# frozen counterpart in FROZEN.jsonl (same N, control, target_index, m, p) must equal it
# in every field except the wall-clock ones (`ms`, `wall_s`) and the fields the fix adds.
#   check_rows.sh NEW.jsonl FROZEN.jsonl
# Prints one line per compared row and exits 1 if any row differs or none was compared.
set -eu
new=${1:?NEW.jsonl}; old=${2:?FROZEN.jsonl}
out=$(jq -n -r --slurpfile new "$new" --slurpfile old "$old" '
  def key: ([.N, .control, .target_index, .m, .p] | tostring);
  def clean: del(.ms, .wall_s);
  ($old | map(select(.N != null)) | map({key: key, value: .}) | from_entries) as $o
  | [ $new[] | select(.N != null)
      | . as $n | ($o[$n | key]) as $f
      | if $f == null then empty
        else
          ($n | clean) as $a | ($f | clean) as $b
          | ([ ($a | keys[]) , ($b | keys[]) ] | unique
             | map(select(($a[.] // null) != ($b[.] // null)))) as $diff
          | { id: ($n | key), N: $n.N, target: $n.target_index, muladds: $n.muladds,
              equal: ($diff | length == 0), differing: $diff }
        end ]
  | (.[] | "N=\(.N) target=\(.target) muladds=\(.muladds) \(if .equal then "EQUAL" else "DIFFERS in " + (.differing | join(",")) end)"),
    (if length == 0 then "NO ROWS COMPARED" elif all(.[]; .equal) then "ALL \(length) ROWS EQUAL" else "MISMATCH" end)
')
echo "$out"
echo "$out" | tail -1 | grep -q '^ALL '
