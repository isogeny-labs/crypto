#!/bin/bash
# R7.1: every measurement run, as executed (3 jobs in parallel; wall times are contended).
# B is the built example; D the output directory.
B=${B:-target/release/examples/r7_1_complete_cost}
D=research/isogeny_fibers_20261010/round7/r7_1_complete_cost
mkdir -p $D/logs
cat > $D/jobs.txt <<JOBS
$B psi 109,131 100000 $D/psi_grid_c.json
$B ladder 500 900 0.25 100000 auto 1 $D/ladder_grid_b.json
$B project 13.25 16 0.25 auto,karatsuba $D/project_b.json
$B psi 79,97 100000 $D/psi_grid_b.json
$B modpoly 71 103 100000 103 $D/modpoly_grid_b.json
$B psi 83,89,101,103 100000 $D/psi_extra_b.json
$B ladder 37 470 0.25 100000 auto 3 $D/ladder_grid_a.json
$B psi 37,41,47,59,67 100000 $D/psi_grid_a.json
$B project 5 13 0.25 auto,karatsuba $D/project_a.json
$B project 5 12 0.25 ntt $D/project_ntt.json
$B modpoly 11 67 100000 103 $D/modpoly_grid_a.json
$B modpoly 107 131 100000 0 $D/modpoly_extra_a.json
$B psi 11,13,17,19,23,29,31,43,53,61,71,73 100000 $D/psi_extra_a.json
$B ladder_list 11,13,17,19,23,29,31,43,53,61,71,73,83,89,101,103 100000 auto 3 $D/ladder_extra_a.json
$B selftest 11,13,17,19,23,29,31 $D/selftest.json
JOBS
n=0
while read -r line; do
  n=$((n+1))
  echo "$line > $D/logs/job$n.log 2>&1"
done < $D/jobs.txt > $D/jobs.sh
xargs -P 3 -I{} bash -c "{}" < $D/jobs.sh
