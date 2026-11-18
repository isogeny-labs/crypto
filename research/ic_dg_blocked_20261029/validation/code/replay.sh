#!/bin/bash
# Independent replay for XP-20261029-dg-blocked (validator code).
# Copies each session from the S3 pod original (already checked byte-equal to the
# evidence directory) into /dev/shm/dgb-val/replay/P<b>-<s>/session and runs
# `ecbench verify --replay-all --exit-code` there. Never writes into the evidence dir.
set -u
SRC=/dev/shm/dgb-val/s3
OUT=/dev/shm/dgb-val/replay
B=/dev/shm/isolab-build/ecbench
FILES="host.json plan.json records.jsonl session.json spec.json"
one() {
  local b=$1 s=$2 d=$OUT/P$1-$2
  mkdir -p "$d/session"
  for f in $FILES; do cp "$SRC/P$b-$s/session/$f" "$d/session/$f"; done
  (cd "$d/session" && sha256sum $FILES) > "$d/session-files.before.sha256"
  {
    echo "start $(date -u +%FT%TZ) host=$(hostname) nproc=$(nproc) binary_sha256=$(sha256sum $B | cut -c1-64)"
    echo "cpu_model=$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')"
    echo "cmd=$B verify --dir $d/session --replay-all --exit-code --out $d/audit.json"
  } > "$d/verify.log"
  "$B" verify --dir "$d/session" --replay-all --exit-code --out "$d/audit.json" > "$d/verify.stdout" 2> "$d/verify.stderr"
  local rc=$?
  echo "end rc=$rc $(date -u +%FT%TZ)" >> "$d/verify.log"
  (cd "$d/session" && sha256sum $FILES) > "$d/session-files.after.sha256"
}
# Two at a time (4 vCPU, ~4 GB free memory on this host).
for b in 24 28 32; do
  one $b wall &
  one $b counted &
  wait
done
echo "ALLDONE $(date -u +%FT%TZ)" > $OUT/DONE
