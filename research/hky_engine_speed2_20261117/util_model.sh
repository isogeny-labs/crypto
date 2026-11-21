#!/bin/bash
# util_model.sh TRACE.jsonl C: a model of the thread utilisation of the parallel section from the trace:
# a batch on T threads takes max(largest item, sum / T) multiply-adds of wall; utilisation = sum / (T × that).
# The second pass (ops2) is counted as one thread when serial (S) and as free (P) otherwise.
jq -r -s --argjson c "$2" '[.[] | select(.c == $c and .ev == "batch")] as $b
 | ($b | map(.ops) | add) as $tot | ($b | map(.ops2) | add) as $tot2
 | "muladds \($tot) second pass \($tot2) batches \($b|length) mean items \(($b|map(.items)|add) / ($b|length) | round)",
   ([4, 8, 16, 32, 64][] as $T
    | ($b | map([.max_ops, .ops / $T] | max) | add) as $w
    | "T=\($T): utilisation, parallel section only \(($tot / ($T * $w)) * 100 | round)%; with a serial second pass \((($tot + $tot2) / ($T * ($w + $tot2))) * 100 | round)%")' "$1"
