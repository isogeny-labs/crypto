#!/bin/sh
# validate_results.sh -- traceability check: every headline number of RESULTS.md recomputed from the raw/derived files. Prints PASS/FAIL lines; exit 1 on any FAIL.
D=${1:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census}
cd $D || exit 2
F=0
chk(){ if [ "$2" = "$3" ]; then echo "PASS $1 = $2"; else echo "FAIL $1: file says '$2', RESULTS.md says '$3'"; F=1; fi; }
S=summary.tsv
chk configs "$(awk 'NR>1' $S | wc -l)" 592
chk configs_m2 "$(awk -F'\t' 'NR>1&&$5==2' $S | wc -l)" 320
chk configs_m3 "$(awk -F'\t' 'NR>1&&$5==3' $S | wc -l)" 272
chk pairs_total "$(awk -F'\t' 'NR>1{s+=$18}END{print s}' $S)" 91200
chk pairs_m2 "$(awk -F'\t' 'NR>1&&$5==2{s+=$18}END{print s}' $S)" 64000
chk pairs_m3 "$(awk -F'\t' 'NR>1&&$5==3{s+=$18}END{print s}' $S)" 27200
chk flag_ok "$(awk -F'\t' 'NR>1&&$13=="ok"' $S | wc -l)" 524
chk flag_fallback "$(awk -F'\t' 'NR>1&&$13!="ok"' $S | wc -l)" 68
chk target_mismatches "$(awk -F'\t' 'NR>1{s+=$27}END{print s}' $S)" 0
chk m3_nT0_Eonly_nd "$(awk -F'\t' 'NR>1&&$5==3&&$12==0{s+=$21}END{print s}' $S)" 41815598
chk m3_nT0_mixed_nd "$(awk -F'\t' 'NR>1&&$5==3&&$12==0{s+=$23}END{print s}' $S)" 0
chk m2_mixed_total "$(awk -F'\t' 'NR>1&&$5==2{s+=$23+$24}END{print s}' $S)" 0
chk M_nd_equals_S_nd_all_configs "$(awk -F'\t' 'NR>1&&($23!=$25||$24!=$26)' $S | wc -l)" 0
chk targets_xR_in_V "$(awk -F'\t' 'NR>1{s+=$28}END{print s}' $S)" 4112
chk F4_configs "$(awk -F'\t' 'NR>1&&$45==1' $S | wc -l)" 6
chk H2_resolvable "$(awk -F'\t' 'NR>1&&$43==1' $S | wc -l)" 143
chk H2_point_pass "$(awk -F'\t' 'NR>1&&$43==1&&$44=="PASS"' $S | wc -l)" 130
chk H3_ref_pass "$(awk -F'\t' 'NR>1&&$6=="random"&&$47=="PASS"' $S | wc -l)" 148
chk H3_naive_pass "$(awk -F'\t' 'NR>1&&$6=="random"&&$46=="PASS"' $S | wc -l)" 142
chk ratio_ref_min "$(awk -F'\t' 'NR>1&&$6=="random"{if(m==""||$35<m)m=$35}END{printf "%.3f",m}' $S)" 0.828
chk ratio_ref_max "$(awk -F'\t' 'NR>1&&$6=="random"{if($35>M)M=$35}END{printf "%.3f",M}' $S)" 1.139
# pooled.tsv recomputed from summary.tsv (m,nT,fb)
awk -F'\t' 'NR>1{k=$5" "$12" "$6;E[k]+=$21;Ed[k]+=$22;M[k]+=$23;Md[k]+=$24;Sn[k]+=$25;Sd[k]+=$26;c[k]++}END{for(k in E)print k,c[k],E[k],Ed[k],M[k],Md[k],Sn[k],Sd[k]}' $S | sort > /tmp/e1_pool_a.$$
awk -F'\t' 'NR>1{print $1" "$2" "$3, $4,$5,$6,$7,$8,$9,$10}' pooled.tsv | sort > /tmp/e1_pool_b.$$
chk pooled_tsv_recomputed "$(cmp -s /tmp/e1_pool_a.$$ /tmp/e1_pool_b.$$ && echo identical || echo DIFF)" identical
rm -f /tmp/e1_pool_a.$$ /tmp/e1_pool_b.$$
# controls
C=controls_summary.tsv
chk V0_rows "$(awk -F'\t' '$1=="#stage0"{print $3}' $C)" 640
chk V0_solutions_A "$(awk -F'\t' '$1=="#stage0"{print $7}' $C)" 18008
chk V0_mixed_nd "$(awk -F'\t' '$1=="#stage0"{print $9}' $C)" 628
chk V0_mixed_deg "$(awk -F'\t' '$1=="#stage0"{print $11}' $C)" 12
chk TOY_rows "$(awk -F'\t' '$1=="#stage0"{print $13}' $C)" 320
chk TOY_mixed_nd "$(awk -F'\t' '$1=="#stage0"{print $17}' $C)" 3900
chk TOY_mixed_deg "$(awk -F'\t' '$1=="#stage0"{print $19}' $C)" 7940
chk stage0_disagreements "$(awk -F'\t' '$1=="#stage0"{print $5+$15+$21}' $C)" 0
for g in PC1 PC2 PC3; do
  chk ${g}_found "$(awk -F'\t' -v g=$g '$1 ~ "^"g"\\|"{a+=$2;b+=$3}END{print b"/"a}' $C)" "$(case $g in PC1) echo 400/400;; PC2) echo 400/400;; PC3) echo 800/800;; esac)"
done
chk NC1_groups "$(grep -c '^NC1|' $C)" 16
chk NC1big_groups "$(grep -c '^NC1big|' $C)" 16
chk NC1_mixed_zero "$(awk -F'\t' '$1 ~ /^NC1/{s+=$4}END{print s}' $C)" 0
chk NC3_groups "$(grep -c '^NC3|' $C)" 32
chk NC3_m2_mixed "$(awk -F'\t' '$1 ~ /^NC3.*m=2/{s+=$5+$6}END{print s+0}' $C)" 0
chk NC3_m3_mixed_nd_nonzero_list "$(awk -F'\t' '$1 ~ /^NC3.*m=3/&&$5>0{printf "%s ",$5}' $C | sed 's/ $//')" "35 58 16 43"
chk NC2_min_frac "$(grep -h '^NC2	' raw/stage1_controls.out | awk -F'\t' '{if(m==""||$10<m)m=$10}END{printf "%.4f",m}')" 0.7266
chk NC2_max_frac "$(grep -h '^NC2	' raw/stage1_controls.out | awk -F'\t' '{if($10>M)M=$10}END{printf "%.4f",M}')" 0.9101
chk NC2_max_dev "$(grep NC2_max results.json >/dev/null; awk '/NC2_max_abs_dev/{printf "%.4f",$2}' verdict.txt)" 0.0219
chk NC2_literal_pass "$(awk -F'\t' '/^NC2_literal|^controls/{print $0}' verdict.txt | grep -o 'NC2_literal_pass	[0-9/]*' | cut -f2)" 3/16
chk NC2_cluster_pass "$(grep -o 'NC2_cluster_pass	[0-9/]*' verdict.txt | cut -f2)" 14/16
chk stage1_result "$(grep -h CONTROLS_RESULT raw/stage1_controls.out | cut -f2)" PASS
chk stage0_result "$(grep -h V0_RESULT raw/stage0_oracle.out | cut -f2)" PASS
chk script_hashes "$(sha256sum e1_census.gp e1_aggregate.gp plot_e1.py | cmp -s - raw/script_hashes.txt && echo match || echo MISMATCH)" match
chk grid_files "$(ls raw/grid_*.out | wc -l)" 224
chk grid_JOBEND "$(cat raw/grid_*.out | grep -c '^JOBEND')" 224
chk opcounts_pkeys_total "$(sh ./opcounts_summary.sh opcounts.tsv summary.tsv | grep '"all"' | grep -o '"pkeys_total":[0-9]*' | cut -d: -f2)" 414817366
chk opcounts_solutions_total "$(awk -F'\t' 'NR>1&&$1!~/^TOTAL/{s+=$8}END{print s}' opcounts.tsv)" 95800902
chk opcounts_verify_m3_n52 "$(grep -c 'pkeys_calls_table_build=1378' opcounts_verify.txt)" 1
[ $F = 0 ] && echo "ALL_TRACEABILITY_CHECKS_PASS" || echo "SOME_TRACEABILITY_CHECKS_FAILED"
exit $F
