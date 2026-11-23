#!/bin/sh
# Second claim run (PROTOCOL.md amendment): the m = 17 and m = 18 sessions,
# four streams on four CPUs, single-target sessions so that a container
# restart loses as little as possible. Idempotent: a complete session is
# skipped, an incomplete one is kept as <name>.attemptN-killed and rerun.
cd "$(dirname "$0")/../.."
R=research/gaudry_k4_char2_crossover_20261015
E=target2/release/ecbench
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
( run 3 s3 SPEC-E-m17b-rho e-m17b-rho; run 3 s3 SPEC-E-m16 e-m16; run 3 s3 SPEC-D-m14 d-m14 ) &
wait
