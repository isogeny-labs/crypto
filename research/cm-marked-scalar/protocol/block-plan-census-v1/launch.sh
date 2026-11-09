#!/usr/bin/env bash
set -euo pipefail
cm_dir=research/cm-marked-scalar
cm_p=$cm_dir/protocol/block-plan-census-v1
cm_out=$cm_dir/block-tuning-v1
cm_build=/tmp/cm-marked-scalar-build
test -s "$cm_p/freeze-publication.json"
test ! -e "$cm_out"
sha256sum -c "$cm_dir/qualification/block-tune-build-v2/binaries.sha256"
sha256sum -c "$cm_dir/qualification/block-tune-mutations-v1/binary.sha256"
mkdir -p "$cm_out/runs"
cp "$cm_p/launch.sh" "$cm_out/runs/launch.sh"
sha256sum "$cm_build/block-tune" "$cm_build/block-tune-replay" "$cm_build/block-tune-mutate" > "$cm_out/runs/binaries.sha256"
# Three build invocations and two software qualifications have already run.
cm_invocations=5
run_native() {
  local cm_label=$1 cm_expected=$2
  shift 2
  cm_invocations=$((cm_invocations+1))
  test "$cm_invocations" -le 20
  printf '%s\n' "$cm_invocations" > "$cm_out/runs/native-invocations.txt"
  set +e
  (ulimit -v 2097152
   env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 flock -w 5 /tmp/crypto-bench.lock timeout 120 "$@"
  ) > "$cm_out/runs/$cm_label.stdout" 2> "$cm_out/runs/$cm_label.stderr"
  local cm_status=$?
  printf '%s\n' "$cm_status" > "$cm_out/runs/$cm_label.exit"
  set -e
  if test "$cm_status" -ne "$cm_expected"; then
    printf 'Stopped at %s: actual exit %s, expected %s. No unchanged retry.\n' "$cm_label" "$cm_status" "$cm_expected" > "$cm_out/runs/stopping.txt"
    exit 1
  fi
}
run_native census 0 "$cm_build/block-tune" --scan "$cm_dir/fixtures-v2" "$cm_p/protocol.json" "$cm_out/runs/census"
run_native replay 0 "$cm_build/block-tune-replay" --replay "$cm_dir/fixtures-v2" "$cm_p/protocol.json" "$cm_out/runs/census"
cp "$cm_out/runs/replay.stdout" "$cm_out/plans.json"
run_native selected-points 0 "$cm_build/block-tune" --qualify-selected "$cm_dir/fixtures-v2" "$cm_out/plans.json"
for cm_mode in wrong-counter missing-row duplicate-row wrong-arm; do
  run_native "make-$cm_mode" 0 "$cm_build/block-tune-mutate" "$cm_mode" "$cm_out/runs/census" "$cm_out/runs/$cm_mode"
  run_native "reject-$cm_mode" 1 "$cm_build/block-tune-replay" --replay "$cm_dir/fixtures-v2" "$cm_p/protocol.json" "$cm_out/runs/$cm_mode"
  case "$cm_mode" in
    wrong-counter) cm_reason='independent GMP full row mismatch' ;;
    missing-row) cm_reason='replay complete census' ;;
    duplicate-row|wrong-arm) cm_reason='CSV finite unique design' ;;
  esac
  rg -Fq "$cm_reason" "$cm_out/runs/reject-$cm_mode.stderr"
done
printf '{"native_adversarial_controls_rejected":4,"timing_performed":false}\n' > "$cm_out/controls.json"
