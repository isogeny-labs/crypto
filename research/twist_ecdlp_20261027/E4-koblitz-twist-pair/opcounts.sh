#!/bin/bash
# Operation-count summary (primary cost metric). Pure awk over out/supp_totals.txt (itself summed from the raw stage CSVs by scripts/supp_totals.gp). No new computation.
D=$(cd "$(dirname "$0")" && pwd)
cd $D || exit 1
awk '/^TOTAL stage[12] [A-Za-z0-9_]+=[0-9]+$/ {split($3,a,"="); v[$2"."a[1]]=a[2]}
END{
 s1=v["stage1.np_same"]+v["stage1.np_cross"];
 printf "{\n";
 printf "  \"unit_notes\": \"candidate tuples = unordered pairs (x1,x2) of V\\\\{0} examined by exhaustive Semaev S3 root finding (stage 1); counted solve = one (base abscissa, target) loop iteration = one table inversion + one trace test + one half-trace extraction (stage 2). Field multiplications, group operations and polynomial evaluations are NOT separately counted by the frozen scripts.\",\n";
 printf "  \"stage1_candidate_pairs_total\": %d,\n", s1;
 printf "  \"stage1_pairs_same_type\": %d,\n  \"stage1_pairs_cross_type\": %d,\n", v["stage1.np_same"], v["stage1.np_cross"];
 printf "  \"stage1_rational_triples\": %d,\n", v["stage1.triples"];
 printf "  \"stage2_counted_solves_K0_targets_Eonly\": %d,\n  \"stage2_counted_solves_K0_targets_shared\": %d,\n", v["stage2.E_solves0"], v["stage2.S_solves0"];
 printf "  \"stage2_counted_solves_K1_targets_Eonly\": %d,\n  \"stage2_counted_solves_K1_targets_shared\": %d,\n", v["stage2.E_solves1"], v["stage2.S_solves1"];
 printf "  \"stage2_K0_relations_Eonly\": %d,\n  \"stage2_K0_relations_shared\": %d,\n", v["stage2.E_rel0"], v["stage2.S_rel0"];
 printf "  \"solves_per_K0_relation_Eonly\": %.4f,\n  \"solves_per_K0_relation_shared\": %.4f,\n", v["stage2.E_solves0"]/v["stage2.E_rel0"], v["stage2.S_solves0"]/v["stage2.S_rel0"];
 printf "  \"solves_ratio_shared_over_Eonly_pooled\": %.4f\n}\n", v["stage2.S_solves0"]/v["stage2.E_solves0"];
}' out/supp_totals.txt > out/opcounts.json
cat out/opcounts.json
