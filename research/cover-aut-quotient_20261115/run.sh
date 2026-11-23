#!/usr/bin/env bash
# Local development check; order revised after the p=503 naive cell proved too slow (see REPORT.md).
set -u
D=/home/user/crypto/research/cover-aut-quotient_20261115; R=$D/results
B=/home/user/crypto/target/release/cover_aut_quotient
export GIT_HEAD=$(git -C /home/user/crypto rev-parse HEAD | cut -c1-12)
T0=$(date +%s); LIMIT=$((50*60))
job() { l=$1; shift
  [ -s $R/$l.json ] && return
  [ $(( $(date +%s) - T0 )) -gt $LIMIT ] && { echo "$l skipped: launch deadline" >> $R/walls.txt; return; }
  a=$(date +%s); timeout 3600 $B paired "$@" --out $R/$l.json > $R/$l.log 2>&1
  echo "$l exit $? wall_s $(( $(date +%s)-a ))" >> $R/walls.txt; }
job paired_p1301_s1 --p 1301 --seed 1
job paired_p1601_s1 --p 1601 --seed 1
job paired_p1451_s1 --p 1451 --seed 1
job paired_p1301_s2 --p 1301 --seed 2
job paired_p1601_s2 --p 1601 --seed 2
job paired_p1451_s2 --p 1451 --seed 2
echo ALL-DONE >> $R/walls.txt
