#!/usr/bin/env bash
# Resume after a container restart (~00:18 UTC 2026-11-17). Same frozen binary (binary.sha256).
# Incomplete curves are re-run whole into *_resume.jsonl; rows present in both files must be
# identical in every field but wall_ns (determinism check). Arms and flags as run.sh.
set -euo pipefail
d=research/genus4_lean_rho_20261116
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.lean
c="--genera 4 --seeds 2 --cc 2.0 --max-h 2 --calibrate --arms wBQ --targets 3"
$B run --primes 8209 --curves 2 $c --threads 1 --rho-cap 1.2e8 --curve-seeds 8209=393/474 --out $d/p8209_resume.jsonl > $d/p8209_resume.log 2>&1 &
$B run --primes 16411 --curves 3 $c --threads 2 --rho-cap 5e8 --curve-seeds 16411=180/172/2640 --out $d/p16411_resume.jsonl > $d/p16411_resume.log 2>&1 &
wait
echo resume-done
