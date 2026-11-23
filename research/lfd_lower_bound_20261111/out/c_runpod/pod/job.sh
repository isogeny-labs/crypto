#!/usr/bin/env bash
# job.sh N SEED EMAX MAXCOLS : solve, then verify every infeasible e>=3 (emin verify) and the feasible lambda (verify_lambda.py)
n=$1; s=$2; emax=$3; maxcols=$4
cd /work; d=cert/n${n}_s${s}; mkdir -p $d
inst=inst_n${n}_s${s}.txt
echo "count $(./emin count < $inst)" > $d/count.txt
st=$(date +%s.%N)
/usr/bin/time -v ./emin solve $emax $maxcols $s $d < $inst > $d/solve.json 2> $d/solve.log
en=$(date +%s.%N)
echo "{\"seed\": $s, \"wall_s\": $(awk -v a=$st -v b=$en 'BEGIN{printf "%.2f", b-a}'), \"result\": $(cat $d/solve.json)}" > $d/line.json
for f in $d/points_e*.txt; do
  e=$(basename $f .txt); e=${e#points_e}
  [ "$e" -ge 3 ] || continue
  /usr/bin/time -v ./emin verify $e $f < $inst > $d/verify_e$e.json 2> $d/verify_e$e.log
done
for f in $d/lambda_e*.bin; do
  [ -f "$f" ] || continue
  e=$(basename $f .bin); e=${e#lambda_e}
  python3 -I verify_lambda.py $n $s $e $f > $d/verify_lambda_e$e.txt 2>&1
done
echo done > $d/DONE
