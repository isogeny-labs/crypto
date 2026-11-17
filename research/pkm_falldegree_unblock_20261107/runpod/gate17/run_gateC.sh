#!/bin/bash
# Amendment 17 gate part 2: registered stage C flags (run.sh hky-C), t = 2, m = 3..6, rebuilt binary.
export RAYON_NUM_THREADS=12
B=$E/pkm_tower_pilot
hky=(--engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
for m in 3 4 5 6; do
  $B --p 2013265921 --ladder-t none --planted 0 --random 2 "${hky[@]}" --kinds kummer --m $m --controls tower --t-min 2 --max-t 2 --max-t-m3 2 --max-t-m4 2 --out C-t2-m$m.jsonl 2> C-t2-m$m.log; echo "C-t2-m$m rc=$?"
  $B --p 786433 --ladder-t none --planted 0 --random 2 "${hky[@]}" --kinds kummer,isogeny --m $m --controls tower --t-min 2 --max-t 2 --max-t-m3 2 --max-t-m4 2 --out C-t2-m$m-p0.jsonl 2> C-t2-m$m-p0.log; echo "C-t2-m$m-p0 rc=$?"
done
