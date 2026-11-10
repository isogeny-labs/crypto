#!/bin/sh
# Third claim run: the m = 17 and m = 18 sessions on the checkpointing binary
# (see src/cryptanalysis/ckpt.rs). A restart kills the processes, but each
# long run's state is in ./ecbench-ckpt/ and rerunning this script continues it.
# Idempotent: a complete session is skipped, an incomplete one is kept as
# <name>.attemptN-killed and rerun (which resumes its checkpoint).
cd "$(dirname "$0")/../.."
R=research/gaudry_k4_char2_crossover_20261015
E=${ECBENCH:-/tmp/claude-0/-home-user-crypto/a65cd574-0a47-5f6a-aba3-d0d21b4d15ff/scratchpad/wt-dev/bin/ecbench-ckpt}
mkdir -p ecbench-ckpt
run() { # cpu lock spec out
  d=$R/sessions/$4
  if [ -f $d/session.json ] && grep -q '"status": *"complete"' $d/session.json; then return 0; fi
  if [ -d $d ]; then n=1; while [ -d $d.attempt$n-killed ]; do n=$((n+1)); done; mv $d $d.attempt$n-killed; mv $d.log $d.attempt$n-killed.log 2>/dev/null; fi
  $E run --spec $R/specs/$3.json --out $d --cpus $1 --lock /tmp/bench-$2.lock --wait --allow-busy > $d.log 2>&1
}
mkdir -p $R/sessions
( run 0 s0 SPEC-E-m18-ic e-m18-ic ) &
( run 1 s1 SPEC-E-m18-rho e-m18-rho ) &
( run 2 s2 SPEC-E-m17-ic-t0 e-m17-ic-t0; run 2 s2 SPEC-E-m17b-ic e-m17b-ic ) &
( run 3 s3 SPEC-E-m16 e-m16; run 3 s3 SPEC-D-m14 d-m14 ) &
wait
