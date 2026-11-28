#!/bin/bash
# usage: run_arm.sh <arm> <repeats>   (runs every job of the arm <repeats> times through ecbench exec)
arm=$1; rep=${2:-1}; dir=$(dirname "$0")/$arm
for r in $(seq $rep); do for f in "$dir"/*.json; do /home/user/crypto/target/release/ecbench exec < "$f" > /dev/null; done; done
