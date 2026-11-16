#!/bin/bash
# S0: compare the k=20 reproduction with lead D's JSONs, field by field.
cd "$(dirname "$0")"
LD=../../curve25519_cofactor_shared_20261102/analysis
F='f4_calls f4_muladds hits hits_verified hits_exact undecided columns covered_points covered_points_all_cosets covered_projected_points targets_done capped solving_degree_max solving_degree_mean'
bad=0
for L in 0 1 2; do
  a=$LD/isogeny_t4_g0_lifts$L.json; b=k20_isogeny_t4_g0_lifts$L.json
  [ -s $b ] || { echo "lifts=$L: missing"; continue; }
  for f in $F; do
    x=$(jq -c ".$f" $a); y=$(jq -c ".$f" $b)
    [ "$x" = "$y" ] || { echo "lifts=$L DIFF $f: leadD=$x scale=$y"; bad=1; }
  done
  for f in max_rows_max max_cols_max max_nnz_max refutations steps; do
    x=$(jq -c ".solver_extra.$f" $a); y=$(jq -c ".solver_extra.$f" $b)
    [ "$x" = "$y" ] || { echo "lifts=$L DIFF solver_extra.$f: leadD=$x scale=$y"; bad=1; }
  done
  echo "lifts=$L: compared $(echo $F | wc -w) fields + 5 solver_extra fields; f4_muladds=$(jq .f4_muladds $b) hits=$(jq .hits $b)"
done
[ $bad = 0 ] && echo "S0: ALL FIELDS EQUAL" || echo "S0: DIFFERENCES FOUND"
