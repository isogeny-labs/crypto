#!/usr/bin/env bash
# Frozen bounded local cohort. No scientific clock and no postdata retry.
set -u
cd /workspace/crypto-audit/research/groebner_system_audit_20261113
python3 metadata.py admit || exit 2
mkdir -p runs
mkdir runs/main || exit 2
ulimit -c 0
export LD_LIBRARY_PATH=/workspace/msolve-deps/root/usr/lib/x86_64-linux-gnu
export OMP_NUM_THREADS=1
analyzer=/workspace/scratch/groebner-system-native/analyze
solver=/workspace/scratch/groebner-system-native/msolve
failed=0
call() {
  local label=$1
  local cap=$2
  shift 2
  printf '%s\0' "$@" > "runs/main/$label.argv.nul"
  date -u +%Y-%m-%dT%H:%M:%SZ > "runs/main/$label.start"
  timeout --signal=INT --kill-after=2s "${cap}s" "$@" > "runs/main/$label.stdout" 2> "runs/main/$label.stderr"
  local rc=$?
  printf '%s\n' "$rc" > "runs/main/$label.exit"
  date -u +%Y-%m-%dT%H:%M:%SZ > "runs/main/$label.end"
  if [ "$rc" -ne 0 ]; then failed=1; fi
}
call controls 5 "$analyzer" "$PWD" --controls
call negative 5 "$analyzer" "$PWD" --negative
for fixture in control edwards-affine edwards-top edwards-homogeneous jv-empty-affine jv-empty-homogeneous jv-split-affine jv-split-homogeneous; do
  for suffix in '' '-aa'; do
    id="$fixture$suffix"
    call "$id" 10 "$solver" -t 1 -l 1 -c 0 -v 2 -g 2 -f "inputs/$fixture.ms" -o "runs/main/$id.out"
  done
done
bash analyze.sh
arc=$?
printf '%s\n' "$arc" > runs/main/analysis.exit
python3 metadata.py export
erc=$?
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py \
  check-export export.json --protocol protocol.json --verify-files > evidence/export-check.log 2>&1
vrc=$?
printf '%s\n' "$vrc" > evidence/export-check.exit
test "$failed" -eq 0 && test "$arc" -eq 0 && test "$erc" -eq 0 && test "$vrc" -eq 0
