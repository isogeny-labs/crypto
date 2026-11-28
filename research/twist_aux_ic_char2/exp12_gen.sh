# EXP12: per-trial msolve cost at large q (case P vs random), no brute force
cd "$(dirname "$0")"
export DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/flint/3.6.0/lib OMP_NUM_THREADS=1
for q in 1048573 2147483647; do
  echo "read(\"exp11d_gen.gp\"); gen($q, 1, 2, 12$q % 100000, \"msolve_q$q\", 0)" | gp -q -s 2G > /dev/null 2>&1
  python3 exp11d_run.py msolve_q$q $q 1800 > results/exp12_trial_q$q.log 2>&1
done
