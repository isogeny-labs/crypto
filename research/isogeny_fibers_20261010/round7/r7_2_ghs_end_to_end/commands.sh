#!/bin/sh
# Exact commands behind raw/. Run from the repository root. Native Rust produced every number;
# python (plot.py) only draws the recorded values.
#
# Build (one job, release, private target dir; the first library build takes about 20 minutes here):
#   export CARGO_TARGET_DIR=<scratch>/r72target CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0
#   cargo build --release --example r7_2_ghs_end_to_end --example r7_ghs_smooth_cover
#   cargo test  --release --lib -- ghs_ic ghs_smooth hyperelliptic        # 96 passed, 0 failed, 2 ignored
B=${B:-target/release/examples/r7_2_ghs_end_to_end}
O=${O:-research/isogeny_fibers_20261010/round7/r7_2_ghs_end_to_end/raw}

$B selftest > $O/selftest.json

# N = 12 and 14, q = 4, the R7.1 instances (l,n,a,beta); 10 tuning logs + 20 holdout logs per instance
$B run --inst 2,6,0,0x28 --inst 2,6,0,0xe2 --inst 2,6,0,0x220 --inst 2,6,0,0x221 --inst 2,6,0,0x228 --inst 2,6,0,0x229 --tune 10 --logs 20 --out $O/run_n12_l2n6.json
$B run --inst 2,7,0,0x4c --inst 2,7,0,0x4d --inst 2,7,0,0x58 --tune 10 --logs 20 --out $O/run_n14_l2n7.json
# N = 12, q = 8: g = 4, 4, 2 (0x21 is not applicable), and the g = 8 instance that did not finish in R7.1
$B run --inst 3,4,0,0x8 --inst 3,4,0,0x16 --inst 3,4,0,0x21 --inst 3,4,0,0xa2 --inst 3,4,0,0x3 --tune 10 --logs 20 --out $O/run_n12_l3n4.json
# genus 4, n = 3: q = 16 ... 256 (and the (4,4) instances at q = 16)
$B run --inst 4,3,0,1867 --inst 4,3,0,879 --inst 4,4,1,15660 --inst 4,4,1,10564 --tune 10 --logs 20 --out $O/run_q16_g4.json
$B run --inst 5,3,1,7673 --inst 5,3,1,12710 --tune 10 --logs 20 --out $O/run_q32_g4.json
$B run --inst 6,3,1,124674 --inst 6,3,0,227089 --tune 10 --logs 20 --out $O/run_q64_g4.json
$B run --inst 7,3,0,1167019 --inst 7,3,1,1953888 --tune 10 --logs 20 --out $O/run_q128_g4.json
$B run --inst 8,3,0,2773531 --inst 8,3,1,12662514 --tune 10 --logs 20 --out $O/run_q256_g4.json
# genus 4, n = 4: q = 32, 64, 128
$B run --inst 5,4,0,593709 --inst 5,4,0,298675 --inst 6,4,0,5707476 --inst 6,4,1,11609626 --tune 10 --logs 20 --out $O/run_n4g4_q32_q64.json
$B run --inst 7,4,1,27160780 --tune 10 --logs 20 --out $O/run_n4g4_q128.json

# how the instances were found (beta with magic number 3, cofactor 8 or less)
#   $B pick 4 3 --magic 3 --count 4 --max-cofactor 8 --samples 3000 --seed 3      # and l = 5, 6, 7 likewise
#   $B pick 5 4 --magic 3 --count 3 --max-cofactor 8 --samples 4000 --seed 11
#   $B pick 6 4 --magic 3 --count 3 --max-cofactor 8 --samples 4000 --seed 11
#   $B pick 8 3 --magic 3 --count 3 --max-cofactor 4 --samples 400  --seed 5
#   $B pick 7 4 --magic 3 --count 2 --max-cofactor 8 --samples 20000 --seed 77    # raw/pick_l7_n4_run2.json

# the (l, n) = (3, 4), g = 8 'hang': phase timings, and R7.1's own example for one curve
$B profile 3 4 --magic 4 --maxk 8 --out $O/profile_l3_n4_m4.json
#   target/release/examples/r7_ghs_smooth_cover run 3 4 --curves 1 --samples 20000 --magic 4 --out $O/r71_rerun_l3n4_g8.json

# trend fits over the genus-4 families, then the figures
$B fit $O/run_q16_g4.json $O/run_q32_g4.json $O/run_q64_g4.json $O/run_q128_g4.json $O/run_q256_g4.json \
       $O/run_n4g4_q32_q64.json $O/run_n4g4_q128.json > $O/fit_genus4.json
#   python3 plot.py        (matplotlib; reads raw/, writes fig*.svg, fig*.png, figure_data.tsv)
#   python3 -c "import typst; typst.compile('report.typ', output='report.pdf')"
