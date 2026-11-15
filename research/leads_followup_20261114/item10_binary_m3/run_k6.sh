#!/bin/bash
# sequential: A then B; status files in out/
cd /home/user/crypto/research/leads_followup_20261114/item10_binary_m3
export OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2 MKL_NUM_THREADS=2
for c in k6_n9_koblitz1_rr k6_n18_koblitz1_rr; do
  echo "start $c $(date -u +%FT%TZ)" >> out/status.txt
  (ulimit -v 6000000; python3 /home/user/ml-cryptanalysis/scripts/ic_bounds_bin_big_run.py ladder --instance inst/$c.json --out out/$c --max-degree 7 --work-gb 1 --tile-mb 256 --batch 512 > out/$c.log 2>&1)
  echo "end $c rc=$? $(date -u +%FT%TZ)" >> out/status.txt
done
echo ALLDONE >> out/status.txt
