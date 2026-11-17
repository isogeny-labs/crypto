#!/bin/sh
# Part D, E and F claim sessions (PROTOCOL.md), four streams on four CPUs,
# each with its own lock.  Counts are deterministic; wall time is
# descriptive (L1: the streams share the host).  Short sessions first, so
# that a container restart (which kills every run in flight; ecbench cannot
# resume a session) loses as little as possible.
cd "$(dirname "$0")/../.."
R=research/gaudry_k4_char2_crossover_20261015
E=target2/release/ecbench
run() { # cpu lock spec out
  [ -f $R/sessions/$4/session.json ] && grep -q '"status": *"complete"' $R/sessions/$4/session.json && return 0
  rm -rf $R/sessions/$4
  $E run --spec $R/specs/$3.json --out $R/sessions/$4 --cpus $1 --lock /tmp/bench-$2.lock --wait --allow-busy > $R/sessions/$4.log 2>&1
}
mkdir -p $R/sessions
( run 0 s0 SPEC-F-lp f-lp; run 0 s0 SPEC-E-m18-ic e-m18-ic ) &
( run 1 s1 SPEC-E-m15 e-m15; run 1 s1 SPEC-D-ladder d-ladder; run 1 s1 SPEC-D-m14 d-m14; run 1 s1 SPEC-E-m18-rho e-m18-rho ) &
( run 2 s2 SPEC-E-m17-ic e-m17-ic ) &
( run 3 s3 SPEC-E-m16 e-m16 ) &
wait
