#!/usr/bin/env bash
# Markdown tables of the smoke run, from smoke/native_*/ (solve JSON) and smoke/summary.tsv (msolve columns).
# Run from the repository root: bash research/pkm_present_panel_20261115/tables.sh > tables.md
S=research/pkm_present_panel_20261115/smoke
for cfg in "t2 3" "t2 4" "t3 3" "t3 4" "t3b 3" "t3b 4"; do
  set -- $cfg; c=$1; m=$2
  echo; echo "### cell $c, m = $m"; echo
  echo "| target | arm | nvars | neq | max deg | constr target / cell (mults) | solve mults | extract mults | D | last-productive deg | cpu_s | wall_s | censored | verified / unlifted / fp sols | matches classifier | msolve max F4 deg | msolve cpu_s | msolve status |"
  echo "|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|--:|--:|---|"
  for tg in $(jq -r '.targets[] | select(.label=="sat" or .label=="unsat") | .name' $S/targets_${c}_m$m/targets.json); do
    label=$(jq -r --arg n $tg '.targets[]|select(.name==$n)|.label' $S/targets_${c}_m$m/targets.json)
    for P in chain-tower tree-tower mono-tower chain-direct mono-direct sym-direct sym-tower; do
      f=$S/native_${c}_m$m/$tg.$P.solve.json
      ms=$(awk -F'\t' -v c=$c -v m=$m -v t=$tg -v p=$P '$1==c && $2==m && $3==t && $5==p {print $28 "|" $29 "|" $30}' $S/summary.tsv)
      IFS='|' read -r mst mdeg mcpu <<< "$ms"
      if [ ! -s $f ]; then echo "| $label | $P | skipped (B_real < m+1: arm not applicable) | | | | | | | | | | | | | | | |"; continue; fi
      jq -r --arg l $label --arg P $P --arg md "$mdeg" --arg mc "$mcpu" --arg ms "$mst" '
        "| \($l) | \($P) | \(.nvars) | \(.nequations) | \(.max_total_degree) | \(.construction_field_multiplications) / \(.construction_cell_field_multiplications) | \(.solve_field_multiplications) | \(.extract_field_multiplications) | \(.D) | \(.solving_degree) | \(.cpu_s*100|round/100) | \(.total_wall_s*100|round/100) | \(.censored) | \(if .verified_real_decompositions==null then "-" else "\(.verified_real_decompositions_count) / \(.unlifted_solutions) / \(.fp_rational_solutions)" end) | \(if .matches_classifier==null then "-" else (.matches_classifier|tostring) end) | \($md) | \($mc) | \($ms) |"' $f
    done
  done
done
