#!/bin/sh
# Table C recomputation from raw isolated pair files. Read-only. Run from the repo root.
# Mirrors the published definition (analyse.py size_row) without importing it:
#  per set: figure pairs = first clean attempt of each slot r1..r5 (retries -retry1/-retry2),
#           or the "double-" slots when any double pair exists;
#  s_before = median over figure pairs of baseline median.s_per_target;
#  wall     = geomean over figure pairs of median_rep(total_ns baseline)/median_rep(total_ns candidate);
#  s_after  = s_before / wall;  per curve: mean over sets M1..M4.
#  reference S = research/ic_exponent_20260926/analysis.json sizes[].s_rho (S20), per curve.
R=research/ic_descent_20260930/runs-isolated/main
S20=research/ic_exponent_20260926/analysis.json
clean() { # isolation record of a price file is clean when exit 0 and not contended; missing record = unrecorded (accepted)
  iso="${1%.price.json}.isolation.jsonl"
  [ -f "$iso" ] || return 0
  tail -n 1 "$iso" | jq -e '.run.exit_status==0 and (.run.contended|not)' >/dev/null
}
pick() { # $1 dir, $2 tag -> prints "base cand" file pairs, one per slot that has a clean pair
  d="$1"; tag="$2"
  for i in 1 2 3 4 5; do
    for k in 0 1 2; do
      stem="${tag}r$i"; [ "$k" -gt 0 ] && stem="$stem-retry$k"
      b="$d/$stem-baseline.price.json"; c="$d/$stem-candidate.price.json"
      [ -f "$b" ] && [ -f "$c" ] || break
      if clean "$b" && clean "$c"; then echo "$b $c"; break; fi
    done
  done
}
for dir in $R/k*n*; do
  kn=$(basename "$dir"); a=${kn#k}; a=${a%%n*}; n=${kn#*n}
  for j in 1 2 3 4; do
    d="$dir/M$j"
    first=$(pick "$d" ""); double=$(pick "$d" "double-")
    fig="$first"; [ -n "$double" ] && fig="$double"
    # build the per-pair table with jq
    echo "$fig" | while read b c; do
      [ -n "$b" ] || continue
      jq -n --slurpfile b "$b" --slurpfile c "$c" '
        def med: sort|if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
        {sb: $b[0].median.s_per_target,
         wall: (($b[0].repetitions|map(.total_ns)|med) / ($c[0].repetitions|map(.total_ns)|med)),
         floor: $b[0].floor.per_target_k, log2r: $b[0].log2_r}'
    done | jq -s --arg a "$a" --arg n "$n" --arg j "$j" '
      def med: sort|if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
      {a:($a|tonumber), n:($n|tonumber), set:("M"+$j), pairs:length, floor:.[0].floor, log2r:.[0].log2r,
       s_before:(map(.sb)|med),
       geo:((map(.wall|log)|add/length)|exp)} | .s_after=(.s_before/.geo)'
  done
done | jq -s --slurpfile s20 $S20 '
  group_by([.a,.n]) | map(
    . as $sets | ($sets[0].a) as $a | ($sets[0].n) as $n |
    ($s20[0].sizes|map(select(.a==$a and .n==$n))|.[0]) as $p |
    {curve:$p.curve, a:$a, n:$n, log2r:$sets[0].log2r, sets:($sets|length), pairs_per_set:($sets|map(.pairs)),
     s_before:($sets|map(.s_before)|add/length), s_after:($sets|map(.s_after)|add/length),
     s_rho_s20:$p.s_rho, floor:$sets[0].floor} |
    .ratio_after=(.s_after/.s_rho_s20) | .x_floor=(.s_after/.floor))'
