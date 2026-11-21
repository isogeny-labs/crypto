#!/bin/bash
# Reproduces every result (coverage p=11 is slow: tens of minutes on 2 threads).
set -e
cd /home/user/crypto
B=target/release/smith_walk_dlp; R=research/isogeny-walk-quartic_20261115/results; mkdir -p $R
: > $R/onestep.jsonl; for p in 101 211 521; do for s in 1 2 3; do $B onestep --p $p --seed $s --n 3000 >> $R/onestep.jsonl; done; done
: > $R/modeltest_d2to5_preregistered.jsonl
for p in 37 101; do for s in 1 2 3; do $B modeltest --p $p --seed $s --want 10 >> $R/modeltest_d2to5_preregistered.jsonl; done; done
: > $R/modeltest_extended_exploratory.jsonl; for s in 1 2 3; do $B modeltest --p 101 --seed $s --want 4 >> $R/modeltest_extended_exploratory.jsonl; done
: > $R/coverage.jsonl; RAYON_NUM_THREADS=2 $B coverage --p 7 >> $R/coverage.jsonl
for p in 37 101; do for s in 1 2 3; do $B canon --p $p --seed $s --want 10 >> $R/canon.jsonl.tmp; done; done; mv $R/canon.jsonl.tmp $R/canon.jsonl
for p in 37 101; do for s in 1 2 3; do $B pushfwd --p $p --seed $s --want 10 2>/dev/null >> $R/pushfwd.jsonl.tmp; done; done; mv $R/pushfwd.jsonl.tmp $R/pushfwd.jsonl
: > $R/dlp.jsonl; for p in 101 211 521; do for s in 11 12 13; do $B dlp --p $p --seed $s --want 4 2>/dev/null >> $R/dlp.jsonl; done; done
python3 research/isogeny-walk-quartic_20261115/make_results.py >/dev/null; python3 research/isogeny-walk-quartic_20261115/plot.py
