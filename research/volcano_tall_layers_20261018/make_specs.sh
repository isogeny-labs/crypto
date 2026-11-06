#!/bin/sh
# Thin orchestration only: builds, verifies and writes the specs of every volcano with
# `crypto volcano sample`.  usage: ./make_specs.sh  (logs go to specs/<vid>.sample.log)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
CRYPTO=${CRYPTO:-$HERE/../../target/release/crypto}
A="--end-solves 300 --level-solves 80 --control-solves 60 --per-level 4 --rounds 2 --warmup 0"
B="--end-solves 120 --level-solves 20 --control-solves 40 --per-level 4 --rounds 2 --warmup 0"
run() { vid=$1; shift; "$CRYPTO" volcano sample "$@" --spec-dir "$HERE/specs/$vid" > "$HERE/specs/$vid.sample.log" 2>&1 || { echo "$vid FAILED"; tail -5 "$HERE/specs/$vid.sample.log"; exit 1; }; echo "$vid: $(grep -E '^p =' "$HERE/specs/$vid.sample.log")"; }
mkdir -p "$HERE/specs"
# tier a: r about 2^44, tall
run a1 --d -7  --ell 2 --depth 20 --bits 47 --slack 4  --bfs-levels 10 $A
run a2 --d -3  --ell 2 --depth 20 --bits 47 --slack 4  --bfs-levels 10 $A
run a3 --d -4  --ell 2 --depth 20 --bits 47 --slack 4  --bfs-levels 10 $A
run a4 --d -8  --ell 2 --depth 20 --bits 57 --m 6 --slack 14 --bfs-levels 10 $A
run a5 --d -11 --ell 3 --depth 12 --bits 47 --slack 4  --bfs-levels 7  $A
run a6 --d -3  --ell 3 --depth 12 --bits 47 --slack 4  --bfs-levels 7  $A
run a7 --d -19 --ell 5 --depth 8  --bits 47 --slack 4  --bfs-levels 5  $A
run a8 --d -43 --ell 7 --depth 6  --bits 47 --slack 4  --bfs-levels 4  $A
# tier b: r about 2^50, taller
run b1 --d -7  --ell 2 --depth 24 --bits 53 --slack 4  --bfs-levels 10 $B
run b2 --d -8  --ell 3 --depth 15 --bits 53 --slack 4  --bfs-levels 7  $B
