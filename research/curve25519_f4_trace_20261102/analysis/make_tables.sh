#!/usr/bin/env bash
# Tables of README.md from f4trace.jsonl (jq + awk). Prints markdown to stdout.
set -eu
D=$(cd "$(dirname "$0")" && pwd)
jq -rs '
  def g(o; k): (o.native[k] // 0);
  [ .[] | select(.trace == false) ] as $p | [ .[] | select(.trace == true) ] as $q
  | $q[] as $c
  | ($p[] | select(.t == $c.t and .guess == $c.guess)) as $b
  | ($c.targets_done) as $n
  | (g($c; "pkm_trace_replay_muladds")) as $rep
  | (g($c; "pkm_trace_fallback_plain_muladds")) as $fb
  | ($c.setup_native.pkm_trace_learn_muladds) as $L
  | ($b.muladds_per_target) as $P
  | ($c.f4_muladds / $n) as $T
  | [ $c.t, $c.guess,
      "\($b.targets_done)/\($c.targets_done)" + (if $b.capped or $c.capped then " CAP" else "" end),
      $P, ($rep / $n), ($fb / $n), $T,
      (if $P > 0 then ($rep / $n) / $P else null end),
      (if $P > 0 then $T / $P else null end),
      $L, (($L + $c.f4_muladds) / $n),
      (if $T < $P then ($L / ($P - $T)) else null end),
      (g($c; "pkm_trace_fallbacks")), (g($c; "pkm_trace_replays")),
      "\($b.hits)/\($c.hits)", "\($b.hits_verified)/\($c.hits_verified)",
      $c.coverage_exact,
      (if $c.coverage_exact > 0 then $P / $c.coverage_exact else null end),
      (if $c.coverage_exact > 0 then (($L + $c.f4_muladds) / $n) / $c.coverage_exact else null end),
      $c.setup_native.pkm_trace_learn_runs, $c.setup_native.pkm_trace_branches ] | @tsv
' "$D/f4trace.jsonl" | awk -F'\t' '
function f(x) { if (x == "null" || x == "") return "-"; v = x + 0; if (v == 0) return "0"; if (v >= 1000 || v < 0.01) return sprintf("%.3g", v); return sprintf("%.3f", v) }
BEGIN {
  print "| t | G | targets plain/trace | plain /tgt | apply /tgt | fallback /tgt | trace /tgt (no learn) | apply/plain | trace/plain | learn L | amortised /tgt | break-even n* | fallbacks/replays | hits p/t | verified p/t | cov | PR(cov) plain | PR(cov) trace amort. | learn runs/branches |"
  print "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
}
{ printf "| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s/%s (%s) | %s | %s | %s | %s | %s | %s/%s |\n",
    $1,$2,$3,f($4),f($5),f($6),f($7),f($8),f($9),f($10),f($11),f($12),$13,$14,($14>0? sprintf("%.1f%%",100*$13/$14):"-"),$15,$16,f($17),f($18),f($19),$20,$21 }
'
