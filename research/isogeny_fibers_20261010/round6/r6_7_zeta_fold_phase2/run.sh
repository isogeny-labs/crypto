#!/bin/sh
# Commands used for R6.7 records (run from this directory).
B=${B:-../../../../target/release/examples/r6_7_zeta_fold_phase2}
$B --exp iratio --bits 9,10,11,12,13,14,15,16 --seeds 3 --json iratio.json > iratio.txt 2>&1 &
$B --exp m2 --family torsion --bits 9,10,11,12,13,14 --seeds 3 --targets 1000 --boot 1000 --json m2_torsion.json > m2_torsion.txt 2>&1 &
$B --exp m2 --family prime --bits 10,12,14,16 --seeds 3 --targets 1000 --boot 1000 --json m2_prime.json > m2_prime.txt 2>&1 &
$B --exp stream --family torsion --bits 9,10,11 --seeds 3 --json stream_torsion.json > stream_torsion.txt 2>&1 &
$B --exp stream --family prime --bits 9,10,11 --seeds 3 --json stream_prime.json > stream_prime.txt 2>&1 &
$B --exp m3 --bits 9,10 --seeds 3 --targets 200 --boot 1000 --json m3_9_10.json > m3_9_10.txt 2>&1 &
$B --exp m3 --bits 11 --seeds 3 --targets 100 --boot 1000 --json m3_11.json > m3_11.txt 2>&1 &
wait
