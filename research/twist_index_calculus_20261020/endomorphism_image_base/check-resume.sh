#!/usr/bin/env bash
# Exercise the resume boundary on actual saved records, with S3 replaced locally.
set -euo pipefail
image_fixture=${1:?complete original cold job out directory}
image_test_root=${2:?new temporary control directory}
mkdir "$image_test_root"
image_script_dir=$(cd "$(dirname "$0")" && pwd)
sed -n '/^image_record_valid()/,/^image_run_one()/p' "$image_script_dir/run-cold.sh" | sed '$d' > "$image_test_root/functions.sh"
source "$image_test_root/functions.sh"
image_mock="$image_test_root/mock"
mkdir "$image_mock"
for image_test_pass in warmup first aa; do
  mkdir -p "$image_mock/$image_test_pass"
  cp -a "$image_fixture/$image_test_pass/p37-seed0" "$image_mock/$image_test_pass/"
done
for image_test_round in 0 1; do
  image_fixture_round=$((image_test_round+1))
  for image_test_suffix in '' -aa; do
    mkdir -p "$image_mock/round$image_test_round$image_test_suffix"
    cp -a "$image_fixture/round$image_fixture_round$image_test_suffix/p37-seed0" "$image_mock/round$image_test_round$image_test_suffix/"
  done
done
# Keep all ten timed arms but remove the repeat: this round must be restarted.
rm "$image_mock/round1-aa/p37-seed0/original.exit"
aws() {
  test "$1" = s3 && test "$2" = sync && test "$3" = --only-show-errors
  local image_relative=${4#"${COLD_PREVIOUS_PREFIX%/}/"}
  if test -d "$image_mock/$image_relative"; then cp -a "$image_mock/$image_relative/." "$5"; fi
}
export EPHEMERAL_OUT="$image_test_root/restored"
export COLD_PREVIOUS_PREFIX=s3://ephemeral-runs-104186811093-us-east-1/jobs/checkpoint-control/out
image_sha=d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c
image_p=37 image_seed=0
image_arms=(original psi-control phi-l2 phi-l0 union-l2 union-all union-l0 phi-l0-active l0-peel double-l0-active)
image_restore_case
test -f "$EPHEMERAL_OUT/round0/p37-seed0/restored-complete-round.txt"
test ! -e "$EPHEMERAL_OUT/round1/p37-seed0"
test -e "$EPHEMERAL_OUT/previous-records/checkpoint-control/round1/p37-seed0/original/cold.json"
for image_test_arm in "${image_arms[@]}"; do
  cmp "$image_fixture/round1/p37-seed0/$image_test_arm/cold.json" "$EPHEMERAL_OUT/round0/p37-seed0/$image_test_arm/cold.json"
  image_record_valid "$EPHEMERAL_OUT/round0/p37-seed0" "$image_test_arm" yes
  for image_test_pass in warmup first aa; do image_record_valid "$EPHEMERAL_OUT/$image_test_pass/p37-seed0" "$image_test_arm" no; done
done
printf '{"status":"pass","complete_round_preserved":true,"incomplete_round_rejected":true,"partial_inputs_retained":true,"copied_records_unchanged":true}\n' > "$image_test_root/resume-control.json"
cat "$image_test_root/resume-control.json"
