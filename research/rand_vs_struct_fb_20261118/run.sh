#!/usr/bin/env bash
# Formal data run for rand_vs_struct_fb_20261118. Frozen prereg: PREREGISTRATION.md.
# Arms interleaved within each t. Deterministic metrics (d_F / D); wall + load
# recorded as context only. [pcc:unverified] pilot. No push, no pods.
set -u
cd "$(dirname "$0")/../.."                # repo/worktree root
BIN=target/release/examples/pkm_tower_pilot
D=research/rand_vs_struct_fb_20261118
RAW=$D/raw
: > $RAW/rows.jsonl                        # aggregated rows
LOG=$RAW/progress.txt; : > $LOG

run() { # arm t engine seed planted random budget tag
  local arm=$1 t=$2 eng=$3 seed=$4 pl=$5 rn=$6 bud=$7 tag=$8
  local out=$RAW/row_${tag}.jsonl err=$RAW/row_${tag}.err
  local la; la=$(cut -d' ' -f1 /proc/loadavg)
  local s; s=$(date +%s.%N)
  $BIN --kinds kummer --m 2 --controls $arm --engine $eng \
       --t-min $t --max-t $t --planted $pl --random $rn --ladder-t none \
       --seed $seed --hky-cmax 12 --budget $bud --out $out 2>$err
  local rc=$?; local e; e=$(date +%s.%N)
  # annotate each row with run context and append to aggregate
  if [ -s "$out" ]; then
    jq -c --arg tag "$tag" --arg eng "$eng" --argjson seed "$seed" \
          --arg la "$la" --argjson wall "$(echo "$e-$s"|bc)" \
      '. + {_tag:$tag,_engine:$eng,_seed:$seed,_loadavg_before:($la|tonumber),_wall_s:$wall}' \
      "$out" >> $RAW/rows.jsonl
  fi
  local dF; dF=$(grep -oE "d_F=Some\([0-9]+\)|Dmax= *[0-9]+" "$err" | head -1)
  printf "%-10s t=%-2s eng=%-4s seed=%-3s rc=%s load=%s wall=%5.1fs %s\n" \
    "$arm" "$t" "$eng" "$seed" "$rc" "$la" "$(echo "$e-$s"|bc)" "$dF" | tee -a $LOG
}

for t in 2 3 4 5 6 7 8 9 10; do
  # structured
  run tower    $t f4  1  1 1 60  tower_f4_t${t}
  # random base: A/A, 3 independent draws of F (seeds), f4 only
  for sd in 11 22 33; do run randbase $t f4 $sd 0 1 180 randbase_f4_t${t}_s${sd}; done
  # null context arm
  run null     $t f4  1  0 1 60  null_f4_t${t}
  # true d_F cross-check (tower/null), reference HKY engine, random targets, t<=8
  if [ "$t" -le 8 ]; then
    run tower $t hky 1 0 2 120 tower_hky_t${t}
    run null  $t hky 1 0 2 120 null_hky_t${t}
  fi
done
# ceiling: single randbase draw at t=11
run randbase 11 f4 11 0 1 300 randbase_f4_t11_s11
echo "DONE" | tee -a $LOG
