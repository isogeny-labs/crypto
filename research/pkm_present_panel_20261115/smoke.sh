#!/usr/bin/env bash
# Smoke driver (not a measurement). Run from the repository root:
#   bash research/pkm_present_panel_20261115/smoke.sh
# Needs target/release/examples/{pkm_present_panel,pkm_m8_panel} built and `msolve` on PATH.
# Every call is kept under 120 s: solve --deadline 100 (F4 plus extraction share it), timeout 120 around
# solve and around msolve. The max degree bound is the non-binding default 64.
set -u
R=research/pkm_present_panel_20261115
S=$R/smoke
B=target/release/examples/pkm_present_panel
M8=target/release/examples/pkm_m8_panel
PRES="chain-tower tree-tower mono-tower chain-direct mono-direct sym-direct sym-tower"
export RAYON_NUM_THREADS=1   # one thread, like msolve -t 1

# pilot cells of seed 777 (m8 cell command, as briefed) and one wrapper cell with B_real >= 5 for the
# symmetric arms at m = 4
[ -f $S/cell_t2/cell.json ] || $M8 cell --t 2 --ratio 9 --master-seed 777 --out $S/cell_t2
[ -f $S/cell_t3/cell.json ] || $M8 cell --t 3 --ratio 6 --master-seed 777 --out $S/cell_t3
[ -f $S/cell_t3b/cell.json ] || $B cell --t 3 --ratio 6 --seed 777 --require-breal 5 --slug smoke-t3b --out $S/cell_t3b
for cfg in "t2 3" "t2 4" "t3 3" "t3 4" "t3b 3" "t3b 4"; do
  set -- $cfg
  [ -f $S/targets_$1_m$2/targets.json ] || \
    $B targets --cell $S/cell_$1 --n-sat 1 --n-unsat 1 --planted 0 --planted-distinct 1 --seed 777 --m $2 --out $S/targets_$1_m$2
done

TSV=$S/summary.tsv
printf 'cell\tm\ttarget\tlabel\tpresentation\tstatus\tnvars\tnequations\tmax_total_degree\tnterms_total\tconstruction_target_mults\tconstruction_cell_mults\tsolve_mults\textract_mults\tD\tsolving_degree\tdegree_reached\ttotal_wall_s\tcpu_s\tcpu_over_wall\tcensored\tinconsistent\tprogress_lb_mults\tverified_real\tunlifted\tfp_solutions\tmatches_classifier\tms_status\tms_max_f4_degree\tms_cpu_s\tms_wall_s\tms_maxrss_kb\tms_quotient_dim\tms_verified_real\n' > $TSV

for cfg in "t2 3" "t2 4" "t3 3" "t3 4" "t3b 3" "t3b 4"; do
  set -- $cfg; c=$1; m=$2
  X=$S/export_${c}_m$m; N=$S/native_${c}_m$m; O=$S/msolve_${c}_m$m
  mkdir -p $N $O
  for tg in $(jq -r '.targets[] | select(.label=="sat" or .label=="unsat") | .name' $S/targets_${c}_m$m/targets.json); do
    label=$(jq -r --arg n $tg '.targets[]|select(.name==$n)|.label' $S/targets_${c}_m$m/targets.json)
    $B export --cell $S/cell_$c --targets $S/targets_${c}_m$m --target $tg --m $m --presentation all --out $X >/dev/null
    for P in $PRES; do
      sj=$X/$tg.$P.system.json
      row="$c\t$m\t$tg\t$label\t$P"
      if [ ! -f $sj ]; then
        printf "$row\tskipped:sym_applicable=false(B_real<m+1)" >> $TSV
        for i in $(seq 1 28); do printf '\tNA' >> $TSV; done; printf '\n' >> $TSV
        continue
      fi
      nj=$N/$tg.$P.solve.json
      rm -f $nj $nj.progress.json
      timeout 120 $B solve --system $sj --deadline 100 --out $nj --extract >/dev/null 2>$N/$tg.$P.stderr
      src=$?
      st=ok; [ $src -eq 124 ] && st=killed120
      ml=$O/$tg.$P.log; mo=$O/$tg.$P.out; vj=$O/$tg.$P.verify.json
      rm -f $mo
      timeout 120 /usr/bin/time -v msolve -t 1 -v 2 -P 1 -f $X/$tg.$P.ms -o $mo > $ml 2>&1
      rc=$?
      if [ $rc -eq 124 ]; then mst=timeout120; elif [ $rc -ne 0 ]; then mst=exit$rc; else mst=ok; fi
      maxdeg=$(awk '$1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/ && $3 ~ /^[0-9]+$/ && $4 ~ /^[0-9]+$/ && $5=="x" {if ($1>m) m=$1} END{print m+0}' $ml)
      ut=$(awk -F: '/User time \(seconds\)/{u=$2} /System time \(seconds\)/{s=$2} END{printf "%.2f", u+s}' $ml)
      wall=$(awk '/Elapsed \(wall clock\)/{n=split($NF,a,":"); w=0; for(i=1;i<=n;i++) w=w*60+a[i]; printf "%.2f", w}' $ml)
      rss=$(awk -F: '/Maximum resident set size/{gsub(/ /,"",$2); print $2}' $ml)
      qd=$(grep -o 'Dimension of quotient: [0-9]*' $ml | awk '{print $NF}' | tail -1)
      mv_=NA
      if [ "$mst" = ok ] && [ -s $mo ]; then
        $B verify-msolve --system-json $sj --msolve-out $mo --out $vj >/dev/null 2>&1 && mv_=$(jq -r .verified_real_decompositions_count $vj)
      fi
      lb=$(tail -1 $nj.progress.json 2>/dev/null | jq -r '.ops // "NA"' 2>/dev/null || echo NA)
      if [ -s $nj ]; then
        jq -r '[.nvars,.nequations,.max_total_degree,.nterms_total,.construction_field_multiplications,.construction_cell_field_multiplications,.solve_field_multiplications,(.extract_field_multiplications//"NA"),.D,.solving_degree,.degree_reached,(.total_wall_s*100|round/100),(.cpu_s*100|round/100),(.cpu_over_wall*100|round/100),.censored,.inconsistent]|@tsv' $nj | tr '\n' '\t' > $N/row.tmp
        printf "$row\t$st\t" >> $TSV; cat $N/row.tmp >> $TSV
        printf '%s\t%s\t%s\t%s\t%s\t' "$lb" "$(jq -r ".verified_real_decompositions_count//\"NA\"" $nj)" "$(jq -r '.unlifted_solutions//"NA"' $nj)" "$(jq -r '.extraction.fp_rational_solutions//"NA"' $nj)" "$(jq -r '.matches_classifier//"NA"' $nj)" >> $TSV
      else
        printf "$row\t$st" >> $TSV
        for i in $(seq 1 16); do printf '\tNA' >> $TSV; done; printf '\t%s\tNA\tNA\tNA\tNA' "$lb" >> $TSV
      fi
      printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$mst" "$maxdeg" "$ut" "$wall" "$rss" "${qd:-NA}" "$mv_" >> $TSV
      echo "done $c m$m $tg $P" >&2
    done
  done
done
echo finished >&2
