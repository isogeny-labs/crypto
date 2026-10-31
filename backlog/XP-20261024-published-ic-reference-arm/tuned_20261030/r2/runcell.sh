#!/bin/bash
# usage: runcell.sh <cell name>   (run from tuned_20261030/, unsandboxed: /usr/bin/time -l needs sysctl)
set -e
c=$1; EXE=/Volumes/SSD990-2/crypto-worktrees/.r2build-tmp/target/release/ecbench
rm -rf r2/runs/R2_$c r2/runs/R2_${c}_tm; mkdir -p r2/runs/R2_${c}_tm
ECB_TIME_DIR=$PWD/r2/runs/R2_${c}_tm $EXE run --spec r2/specs/SPEC-R2_$c.json --out r2/runs/R2_$c --cpus none --wait
