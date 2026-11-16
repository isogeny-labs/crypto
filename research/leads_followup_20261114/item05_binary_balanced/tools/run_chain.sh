#!/bin/bash
# sequential degree<=4 ladders, k increasing.  usage: run_chain.sh name:instance_path ...
D=/home/user/crypto/research/leads_followup_20261114/item05_binary_balanced
export OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2 MKL_NUM_THREADS=2
cd /home/user/ml-cryptanalysis
for spec in "$@"; do
  name=${spec%%:*}; inst=${spec#*:}
  echo "$(date -u +%FT%TZ) start $name" >> $D/logs/chain.status
  s=$(date +%s)
  ( ulimit -v 6000000; python3 scripts/ic_bounds_bin_big_run.py ladder --instance $inst --max-degree 4 --device cpu \
      --batch 128 --work-gb 0.5 --tile-mb 32 --col-block 4096 --status $D/logs/$name.engine-status.json \
      --out $D/out/$name.json > $D/logs/$name.log 2>&1 )
  rc=$?
  echo "$(date -u +%FT%TZ) end $name rc=$rc wall=$(( $(date +%s)-s ))s" >> $D/logs/chain.status
done
echo "$(date -u +%FT%TZ) CHAIN DONE" >> $D/logs/chain.status
