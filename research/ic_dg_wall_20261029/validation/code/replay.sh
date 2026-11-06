#!/bin/bash
# Independent replay: ecbench verify --replay-all on a copy of each committed session.
set -u
EV=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/wt-wall/research/ic_dg_wall_20261029
OUT=/dev/shm/dgw-validation/replay
B=/dev/shm/isolab-build/ecbench
one() {
  local b=$1 s=$2 d=$OUT/P$1-$2
  mkdir -p $d/session
  for f in host.json plan.json records.jsonl session.json spec.json; do cp $EV/P$b/$s/eks-results/session/$f $d/session/$f; done
  (cd $d/session && sha256sum host.json plan.json records.jsonl session.json spec.json) > $d/session-files.before.sha256
  echo "start $(date -u +%FT%TZ) cpu=$(nproc) binary=$(sha256sum $B | cut -c1-64)" > $d/verify.log
  $B verify --dir $d/session --replay-all --exit-code --out $d/audit.json > $d/verify.stdout 2> $d/verify.stderr
  local rc=$?
  echo "end rc=$rc $(date -u +%FT%TZ)" >> $d/verify.log
  (cd $d/session && sha256sum host.json plan.json records.jsonl session.json spec.json) > $d/session-files.after.sha256
}
for b in 24 28 32; do
  one $b wall &
  one $b counted &
  wait
done
echo ALLDONE > $OUT/DONE
