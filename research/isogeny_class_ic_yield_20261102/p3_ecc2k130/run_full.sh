#!/bin/sh
# Full-run commands for P3 (NOT executed by the builder).  Route is frozen by the director (isolab-eks default).
# Revision 2 (2026-11-02): b-adapted arms badapt/bsparse/nbadapt (PROTOCOL sections 1, 4, 6), per-cell dmax and attempts from the
# measured costs (PROTOCOL sections 5, 7).  Estimated total ~30.3 CPU-h incl. P3a-Y (~10.9 h without it), ~7.6 h on 4 cores.
# Build: CARGO_TARGET_DIR=<repo>/target cargo build --release \
#          --manifest-path research/isogeny_class_ic_yield_20261102/p3_ecc2k130/harness/Cargo.toml
# Each line is independent; shard long lines over workers with --first-attempt/--attempts.
# AMENDMENT A1 (2026-11-02, PROTOCOL.md section 10, filed before any data): the run is the task cards of cards/CARD-isogeny-ic-yield-p3-*,
# whose argv are these lines (sharded where section 10 says), local route, isolation L0, at most 2 workers.  The P3a-Y line below is
# DEFERRED (not in any card, no data taken).  The masked analysis lines are run by the main session on the pooled shard files.
set -e
P=${P:-target/release/p3h}; D=research/isogeny_class_ic_yield_20261102/p3_ecc2k130; export RAYON_NUM_THREADS=1
OUT=${OUT:-$D/full}; mkdir -p $OUT; SEED=20261102
CTRL=0,1,2,3,44,87,130,131,215,257   # control subset of the n=43 table: E0 + spread over orbits (idx)
CTRL131=0,1,66,131,132,200,262       # control subset of the ECC2K-130 table (E0, 3 curves of orbit A, 3 of orbit B)
N43=$D/data/n43_l257_curves.tsv; N131=$D/data/ecc2k130_curves.tsv
$P class --n 43 --ell 257 --a2 0 --out $OUT/n43_l257_curves.tsv   # must reproduce data/n43_l257_curves.tsv (sha256 in data/SHA256SUMS)
# Instrument and isomorphism checks first (both must print ALL OK; any FAIL voids the cells that use them).  ~2 min.
$P selftest > $OUT/selftest.txt
$P isocheck > $OUT/isocheck.txt
# ---- P3a  n=43, ell=257, m=3.  dmax per cell: k=3 9 (0.05 s), k=4 8 (Dreg 8, ~17 s), k=5 7 (atleast, ~1 s; dmax 8 72 s, dmax 9 capshit 100 s)
$P measure --n 43 --curves $N43 --ks 3 --attempts 20 --seed $SEED --arms main --dmax 9 --out $OUT/p3a_main_k3.jsonl            # ~0.07 CPU-h
$P measure --n 43 --curves $N43 --ks 4 --attempts 2 --seed $SEED --arms main --dmax 8 --out $OUT/p3a_main_k4.jsonl             # ~2.5 CPU-h
$P measure --n 43 --curves $N43 --ks 5 --attempts 2 --seed $SEED --arms main --dmax 7 --out $OUT/p3a_main_k5.jsonl             # ~0.15 CPU-h, bound only
# P3a controls + C2 family on the subset: main (needed by the iso / A-A identity checks), Frobenius-isomorphic, A/A, planted (+solve),
# standard V in 4 presentations, b-adapted (badapt), b-sparse family (bsparse), 16 random-b nulls and their b-adapted version (nbadapt)
$P measure --n 43 --curves $N43 --ks 3 --attempts 20 --seed $SEED --select $CTRL --arms main,iso,aa,planted,stdv,badapt,bsparse,null,nbadapt --null 16 --presentations 3 --dmax 9 --solve --out $OUT/p3a_ctrl_k3.jsonl   # ~0.05 CPU-h
$P measure --n 43 --curves $N43 --ks 4 --attempts 2 --seed $SEED --select $CTRL --arms main,iso,aa,planted,stdv,badapt,bsparse,null,nbadapt --null 16 --presentations 3 --dmax 8 --out $OUT/p3a_ctrl_k4.jsonl                # ~1.4 CPU-h
# P3a m=2 control (cheap)
$P measure --n 43 --m 2 --curves $N43 --ks 3,4,5,8 --attempts 20 --seed $SEED --arms main,null --null 16 --dmax 9 --out $OUT/p3a_m2.jsonl
# P3a-Y  [DEFERRED under A1: not run, no data]  exact m=3 yield (ordered x-solution count per attempt), k=13, E0 + 6 orbit representatives, 100 attempts (--dmax 1 skips Dreg); ~19.4 CPU-h, droppable
$P measure --n 43 --curves $N43 --ks 13 --attempts 100 --seed $SEED --select 0,1,44,87,130,173,216 --arms main --dmax 1 --out $OUT/p3a_yield.jsonl
# ---- P3b  n=131: 263 curves.  Random V: k=3,4 dmax 9; k=5 dmax 7 (atleast 8, 10 s; dmax 9 does not finish in 15 min)
$P measure --n 131 --curves $N131 --ks 3,4 --attempts 2 --seed $SEED --arms main --dmax 9 --out $OUT/p3b_main_k34.jsonl         # ~0.4 CPU-h
$P measure --n 131 --curves $N131 --ks 5 --attempts 2 --seed $SEED --arms main --dmax 7 --out $OUT/p3b_main_k5.jsonl           # ~1.5 CPU-h, bound only
# standard V in 3 presentations, k=3..6 (k=6: floor Dreg 7 at ~19 s, E0 Dreg 6 at 0.2 s; k=7 capshit, not run)
$P measure --n 131 --curves $N131 --ks 3,4,5,6 --attempts 1 --seed $SEED --arms stdv --presentations 2 --dmax 9 --out $OUT/p3b_stdv.jsonl   # ~4.3 CPU-h
# b-adapted presentation (modulus = minimal polynomial of b, b -> z; E0 in the borrowed floor presentations), all 263 curves, k=3..6; ~0.13 CPU-h
$P measure --n 131 --curves $N131 --ks 3,4,5,6 --attempts 2 --seed $SEED --arms badapt --dmax 9 --out $OUT/p3b_badapt.jsonl
# b-sparse family (6 presentations per curve; two of them cost ~20 s at k=6) + nbadapt control (8 random dense b), CTRL131; ~0.16 CPU-h
$P measure --n 131 --curves $N131 --ks 5,6 --attempts 2 --seed $SEED --select $CTRL131 --arms bsparse,nbadapt --null 8 --dmax 9 --out $OUT/p3b_bsparse.jsonl
# controls (main is in the file so that iso / A-A identity can be checked); dmax 7 keeps k=5 at 10 s; ~0.25 CPU-h
$P measure --n 131 --curves $N131 --ks 3,4,5 --attempts 2 --seed $SEED --select $CTRL131 --arms main,iso,aa,planted,null --null 8 --dmax 7 --solve --out $OUT/p3b_ctrl.jsonl
for f in $OUT/*.jsonl; do $P analyze --rows $f > $f.masked_analysis.txt; done
# Unmask only after the analysis code and decision rule are frozen:  $P analyze --rows <file> --unmask
# C2 reads the "presentation-family minima" block of the unmasked analysis of p3b_stdv + p3b_badapt + p3b_bsparse (pool the three files:
# cat them into one jsonl before analyze) and of p3a_ctrl_k3 / p3a_ctrl_k4.
