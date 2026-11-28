#!/bin/bash
# trace_summary.sh TRACE.jsonl C [BINS]: the batches of closure C grouped by rank (BINS equal slices of the
# final rank). Columns: slice, rank at the end of the slice, batches, products, share reduced to zero,
# pivots, mean terms of a source row, mean terms of a product, reducer rows used per product,
# mean terms of a reducer row used, multiply-adds per product, total multiply-adds (second pass included),
# mean age of the source row (pivots added since it entered) over the rank, share of products from rows of
# degree C-1, stored words per rank.
jq -r -s --argjson c "$2" --argjson bins "${3:-10}" '
  [.[] | select(.c == $c)] as $all
  | [$all[] | select(.ev == "batch")] as $b
  | ($b | last | .rank) as $R
  | ["slice","rank","batches","products","zero","pivots","src_nnz","prod_nnz","reducers/prod","reducer_nnz","muladds/prod","muladds","age/rank","deg=c-1","words/rank"],
    ( $b | group_by((((.rank - .pivots) * $bins) / ([$R,1]|max)) | floor) | .[]
      | (map(.formed)|add) as $f | (map(.items)|add) as $it
      | [ (((.[0].rank - .[0].pivots) * $bins / ([$R,1]|max)) | floor), (last|.rank), length, $f,
          ((map(.zero)|add) / ([$f,1]|max) * 1000 | round / 1000), (map(.pivots)|add),
          ((map(.src_nnz)|add) / ([$it,1]|max) | round), ((map(.prod_nnz)|add) / ([$f,1]|max) | round),
          ((map(.nred)|add) / ([$f,1]|max) * 10 | round / 10),
          ((map(.ops)|add) / ([(map(.nred)|add),1]|max) | round),
          ((map(.ops)|add) / ([$f,1]|max) | round), (map(.ops + .ops2)|add),
          ((map(.age)|add) / ([$it,1]|max) / ([(last|.rank),1]|max) * 1000 | round / 1000),
          ((map(.deg[$c - 1])|add) / ([$it,1]|max) * 100 | round / 100),
          ((last|.words) / ([(last|.rank),1]|max) | round) ] )
  | @tsv' "$1"
jq -r -s --argjson c "$2" '[.[] | select(.c == $c and .ev == "interreduce")] | if length > 0 then "interreductions: \(length), muladds \(map(.ops)|add), words before/after of the last: \(last|.words_before) / \(last|.words_after)" else empty end' "$1"
