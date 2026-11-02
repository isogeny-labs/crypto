#!/bin/sh
# Full-run commands for P3 (NOT executed by the builder).  Route is frozen by the director (isolab-eks default).
# Build: CARGO_TARGET_DIR=<repo>/target cargo build --release \
#          --manifest-path research/isogeny_class_ic_yield_20261102/p3_ecc2k130/harness/Cargo.toml
# Each line is independent; shard long lines over workers with --first-attempt/--attempts.
set -e
P=${P:-target/release/p3h}; D=research/isogeny_class_ic_yield_20261102/p3_ecc2k130; export RAYON_NUM_THREADS=1
OUT=${OUT:-$D/full}; mkdir -p $OUT; SEED=20261102
CTRL=0,1,2,3,44,87,130,131,215,257   # control subset of the n=43 table: E0 + spread over orbits (idx)
CTRL131=0,1,66,131,132,200,262       # control subset of the ECC2K-130 table
$P class --n 43 --ell 257 --a2 0 --out $OUT/n43_l257_curves.tsv   # must reproduce data/n43_l257_curves.tsv (sha256 in data/SHA256SUMS)
# P3a-D  n=43, ell=257: m=3 resolving degree + time, k=3,4,5, all 259 curves, 20 attempts
$P measure --n 43 --curves $D/data/n43_l257_curves.tsv --ks 3,4,5 --attempts 20 --seed $SEED --arms main --dmax 9 --out $OUT/p3a_main.jsonl
# P3a controls on the subset: Frobenius-isomorphic, A/A, planted (+solve), standard-V in 4 presentations, 16 random-b nulls
$P measure --n 43 --curves $D/data/n43_l257_curves.tsv --ks 3,4,5 --attempts 20 --seed $SEED --select $CTRL --arms iso,aa,planted,stdv,null --null 16 --presentations 3 --dmax 9 --solve --out $OUT/p3a_ctrl.jsonl
# P3a m=2 control
$P measure --n 43 --m 2 --curves $D/data/n43_l257_curves.tsv --ks 3,4,5,8 --attempts 20 --seed $SEED --arms main,null --null 16 --dmax 9 --out $OUT/p3a_m2.jsonl
# P3a-Y  exact m=3 yield (ordered x-solution count per attempt), k=13, E0 + 6 orbit representatives, 100 attempts (--dmax 1 skips Dreg)
$P measure --n 43 --curves $D/data/n43_l257_curves.tsv --ks 13 --attempts 100 --seed $SEED --select 0,1,44,87,130,173,216 --arms main --dmax 1 --out $OUT/p3a_yield.jsonl
# P3b  n=131: 263 curves.  Random V: k=3,4,5 (k=6 does not finish, see PROTOCOL 9); standard V in 3 presentations: k=3..6
$P measure --n 131 --curves $D/data/ecc2k130_curves.tsv --ks 3,4,5 --attempts 2 --seed $SEED --arms main --dmax 9 --out $OUT/p3b_main.jsonl
$P measure --n 131 --curves $D/data/ecc2k130_curves.tsv --ks 3,4,5,6 --attempts 1 --seed $SEED --arms stdv --presentations 2 --dmax 9 --out $OUT/p3b_stdv.jsonl
$P measure --n 131 --curves $D/data/ecc2k130_curves.tsv --ks 3,4,5 --attempts 2 --seed $SEED --select $CTRL131 --arms iso,aa,planted,null --null 8 --dmax 9 --solve --out $OUT/p3b_ctrl.jsonl
for f in $OUT/*.jsonl; do $P analyze --rows $f > $f.masked_analysis.txt; done
# Unmask only after the analysis code and decision rule are frozen:  $P analyze --rows <file> --unmask
