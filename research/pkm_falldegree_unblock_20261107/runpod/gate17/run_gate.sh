#!/bin/bash
# Reproduction gate for Amendment 17: registered stage A flags (run.sh), t <= 8, rebuilt musl binary, RAYON 12.
export RAYON_NUM_THREADS=12
B=$E/pkm_tower_pilot
hky=(--engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
f4=(--engine tower --budget 1800 --max-nnz 2000000000)
c1=(--p 2013265921 --ladder-t none --planted 0 --random 2)
c0=(--p 786433 --ladder-t none --planted 0 --random 2)
$B "${c1[@]}" "${hky[@]}" --kinds kummer --m 2 --controls tower --t-min 1 --max-t 8 --out A-m2-tower.jsonl 2> A-m2-tower.log; echo "A-m2-tower rc=$?"
$B "${c0[@]}" "${hky[@]}" --kinds kummer,isogeny --m 2 --controls tower --t-min 1 --max-t 8 --out A-m2-tower-p0.jsonl 2> A-m2-tower-p0.log; echo "A-m2-tower-p0 rc=$?"
$B "${c1[@]}" "${f4[@]}" --kinds kummer --m 2 --controls tower --t-min 1 --max-t 8 --out DA-m2-tower.jsonl 2> DA-m2-tower.log; echo "DA-m2-tower rc=$?"
$B "${c0[@]}" "${f4[@]}" --kinds kummer,isogeny --m 2 --controls tower --t-min 1 --max-t 8 --out DA-m2-tower-p0.jsonl 2> DA-m2-tower-p0.log; echo "DA-m2-tower-p0 rc=$?"
