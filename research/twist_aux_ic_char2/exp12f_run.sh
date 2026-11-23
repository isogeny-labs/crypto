# EXP12f: clean (nothing else running) msolve per-trial cost: case P vs class-pinned line (pin_*),
# and a re-time of the case P / random systems at q ~ 2^20 (msolve_q1048573).
cd "$(dirname "$0")"
export DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/flint/3.6.0/lib OMP_NUM_THREADS=1
for q in 1009 1048573; do
  mkdir -p pin_q$q
  echo "read(\"exp12f_gen.gp\"); genQ($q, 2, 2, 7$q % 100000, \"pin_q$q\", 0)" | gp -q -s 2G
  python3 exp11d_run.py pin_q$q $q 1800 > results/exp12f_pinned_q$q.log 2>&1
done
python3 exp11d_run.py msolve_q1048573 1048573 1800 > results/exp12_trial_q1048573.log 2>&1
