#!/bin/bash
# Recompute Tables A, B and C of the IC leaderboard from their raw records and compare every board row with
# docs/ic/leaderboard.json: S (IC), S (reference), x reference and x floor, relative tolerance 1e-8.
# Exit 1 on any mismatch, any board row without a recomputation, or any recomputation without a board row.
# Needs only jq.  Run from the repository root.  See docs/VALIDATION_GATE.md ("Recompute check").
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
here=scripts/ic_audit
tmp=$(mktemp -d); trap 'rm -r "$tmp"' EXIT
sh $here/recompute_A.sh > "$tmp/A.json"
sh $here/recompute_B.sh > "$tmp/B.json"
sh $here/recompute_C.sh > "$tmp/C.json"
jq -n --slurpfile board "${IC_BOARD:-docs/ic/leaderboard.json}" --slurpfile A "$tmp/A.json" --slurpfile B "$tmp/B.json" --slurpfile C "$tmp/C.json" '
  def close($a; $b): (($a - $b) | fabs) <= 1e-8 * (($b | fabs) + 1e-300);
  def slugn: (capture("icv1-f2m(?<n>[0-9]+)-").n | tonumber);
  def cmp($row; $who; $s; $ref; $xr; $xf):
    [ (if close($row.best.s; $s) then empty else "\($who): S \($row.best.s) != recomputed \($s)" end),
      (if close($row.reference_s; $ref) then empty else "\($who): reference S \($row.reference_s) != recomputed \($ref)" end),
      (if close($row.best.ratio_rho; $xr) then empty else "\($who): x reference \($row.best.ratio_rho) != recomputed \($xr)" end),
      (if close($row.best.ratio_floor; $xf) then empty else "\($who): x floor \($row.best.ratio_floor) != recomputed \($xf)" end) ];
  ($board[0].board) as $rows |
  [ $rows[] | select(.family == "A") | . as $r |
      ($A[0] | map(select(.curve == $r.slug)) | .[0]) as $x |
      if $x == null then ["A \($r.slug): no recomputation"]
      else cmp($r; "A \($r.slug)"; $x.S_IC; $x.S_ref; $x.xref; $x.xfloor) end ] +
  [ $rows[] | select(.family == "B") | . as $r |
      ($B[0] | map(select(.instance == $r.legacy)) | .[0]) as $x |
      if $x == null then ["B \($r.slug): no recomputation"]
      else cmp($r; "B \($r.slug)"; $x.S_IC; $x.S_ref; $x.xref; $x.xfloor) end ] +
  [ $rows[] | select(.family == "C") | . as $r |
      ($C[0] | map(select(.n == ($r.slug | slugn))) | .[0]) as $x |
      if $x == null then ["C \($r.slug): no recomputation"]
      else cmp($r; "C \($r.slug)"; $x.s_after; $x.s_rho_s20; $x.ratio_after; $x.x_floor) end ]
  + [ ("A","B","C") as $f | ({"A":$A[0],"B":$B[0],"C":$C[0]}[$f] | length) as $n
      | ($rows | map(select(.family == $f)) | length) as $m
      | select($n != $m) | ["\($f): \($n) recomputed rows but \($m) board rows"] ]
  | add as $problems
  | { checked: ($rows | map(select(.family == "A" or .family == "B" or .family == "C")) | length),
      problems: $problems }' > "$tmp/result.json"
n=$(jq '.checked' "$tmp/result.json"); p=$(jq '.problems | length' "$tmp/result.json")
if [ "$p" -ne 0 ]; then jq -r '.problems[]' "$tmp/result.json"; echo "board recompute: $p problem(s) in $n rows"; exit 1; fi
echo "board recompute: $n rows, every S, reference S, x reference and x floor recomputes from raw records"
