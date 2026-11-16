#!/bin/sh
# usage: sh run_all.sh <path-to-binary>
# The binary is `rustc -O --edition 2021 examples/r7_7_coset_label.rs -o <binary>`
# (std only; no crate dependency).  Run from this directory.
B=$1
$B selftest > selftest.txt
$B exhaustive --nmin 5 --nmax 13 --out exhaustive.json > exhaustive.txt 2> exhaustive.log
$B exhaustive --nmin 5 --nmax 9 --alla --out exhaustive_alla.json > exhaustive_alla.txt 2> exhaustive_alla.log
$B random --ns 17,19,21,23 --curves 1000 --pts 1500 --seed 77 --out random.json > random.txt 2> random.log
$B exhaustive --nmin 5 --nmax 7 --alla --control 1 --out control1_wrong_a.json > control1_wrong_a.txt 2>/dev/null
$B exhaustive --nmin 5 --nmax 7 --alla --control 2 --out control2_drop_term.json > control2_drop_term.txt 2>/dev/null
$B random --ns 17 --curves 200 --pts 600 --seed 79 --control 1 --out control1_random.json > control1_random.txt 2>/dev/null
$B cost --ns 17,19,21,23 --curves 60 --pts 400 --seed 78 --out cost.json > cost.txt 2> cost.log
$B beta --nmin 5 --nmax 11 --out beta.json > beta.txt
# added after the first runs (subcommands only; the label code is unchanged, checked by re-running n=5..9)
$B witness --n 5 --out witness.json > witness.txt
$B search --ns 7,9,11,13 --curves 60 --pts 400 --level 3 --seed 81 --out search_level3.json > search_level3.txt
$B search --ns 7,9,11,13 --curves 60 --pts 400 --level 2 --seed 81 --out search_level2_control.json > search_level2_control.txt
