#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do image_var="IMAGE_BUNDLE_$image_part"; printf '%s' "${!image_var-}"; done | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
for image_part in A B C D E F G H I J K L M N O P Q R S T; do unset "IMAGE_BUNDLE_$image_part"; done
image_source="$EPHEMERAL_OUT/source"
image_root="$image_source/research/twist_index_calculus_20261020"
image_manifest="$image_root/endomorphism_image_base/native/Cargo.toml"
export CARGO_TARGET_DIR=/work/twist-image-target
export RAYON_NUM_THREADS=1
export CARGO_BUILD_JOBS=4
python3 "$image_source/tools/isolated_bench.py" busy -- cargo build --locked --release --manifest-path "$image_manifest" > "$EPHEMERAL_OUT/build.log" 2>&1
python3 "$image_source/tools/isolated_bench.py" busy -- cargo test --locked --release --manifest-path "$image_manifest" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
image_binary="$CARGO_TARGET_DIR/release/twist-endomorphism-image-study"
image_tape="$image_root/quartic_twist_mixed_v1.ndjson"
image_sha=d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c
image_cpus=$(cat /sys/devices/system/cpu/cpu1/topology/thread_siblings_list)
printf '%s\n' "$image_cpus" > "$EPHEMERAL_OUT/reserved-cpus.txt"
uname -a > "$EPHEMERAL_OUT/uname.txt"
lscpu > "$EPHEMERAL_OUT/lscpu.txt"
image_arms=(original psi-control phi-l2 phi-l0 union-l2 union-all union-l0 phi-l0-active l0-peel double-l0-active)
sha256sum "$image_binary" "$image_tape" > "$EPHEMERAL_OUT/native-and-input.sha256"
image_record_valid() {
  local image_check_parent=$1 image_check_arm=$2 image_check_measured=$3
  test -f "$image_check_parent/$image_check_arm.exit" &&
    test "$(cat "$image_check_parent/$image_check_arm.exit")" = 0 &&
    jq -e --argjson p "$image_p" --argjson seed "$image_seed" --arg arm "$image_check_arm" --arg sha "$image_sha" \
      '.deterministic.p == $p and .deterministic.seed == $seed and .deterministic.arm == $arm and .deterministic.input_sha256 == $sha and .deterministic.undetermined == 0' \
      "$image_check_parent/$image_check_arm/cold.json" >/dev/null 2>&1 || return 1
  if test "$image_check_measured" = yes; then
    jq -e '.run.exit_status == 0 and .run.contended == false and (.left_on_reserved.user_threads | length == 0) and (.reserved_cpus | length > 0)' \
      "$image_check_parent/$image_check_arm.isolation.json" >/dev/null 2>&1 || return 1
  fi
}
image_copy_record() {
  local image_copy_from=$1 image_copy_to=$2 image_copy_arm=$3 image_copy_file
  mkdir -p "$image_copy_to"
  for image_copy_file in "$image_copy_from/$image_copy_arm" "$image_copy_from/$image_copy_arm.exit" "$image_copy_from/$image_copy_arm.stdout" "$image_copy_from/$image_copy_arm.stderr" "$image_copy_from/$image_copy_arm.isolation.json"; do
    if test -e "$image_copy_file"; then cp -a "$image_copy_file" "$image_copy_to/"; fi
  done
  printf '%s\n' "$COLD_PREVIOUS_PREFIX" > "$image_copy_to/origin-prefix.txt"
  printf '%s\n' "$image_previous_id" > "$image_copy_to/job-id.txt"
  printf '%s\n' "$image_previous_id" > "$image_copy_to/$image_copy_arm.origin-job.txt"
}
image_restore_case() {
  test -n "${COLD_PREVIOUS_PREFIX:-}" || return 0
  [[ "$COLD_PREVIOUS_PREFIX" =~ ^s3://ephemeral-runs-104186811093-us-east-1/jobs/[A-Za-z0-9._-]+/out/?$ ]] || { printf 'invalid checkpoint prefix\n' >&2; exit 2; }
  image_previous_id=${COLD_PREVIOUS_PREFIX%/}; image_previous_id=${image_previous_id%/out}; image_previous_id=${image_previous_id##*/}
  image_previous="$EPHEMERAL_OUT/previous-records/$image_previous_id"
  for image_pass in warmup first aa round0 round0-aa round1 round1-aa round2 round2-aa round3 round3-aa round4 round4-aa; do
    image_old_parent="$image_previous/$image_pass/p$image_p-seed$image_seed"
    mkdir -p "$image_old_parent"
    aws s3 sync --only-show-errors "${COLD_PREVIOUS_PREFIX%/}/$image_pass/p$image_p-seed$image_seed/" "$image_old_parent/"
  done
  for image_pass in warmup first aa; do
    image_old_parent="$image_previous/$image_pass/p$image_p-seed$image_seed"
    for image_arm in "${image_arms[@]}"; do
      if image_record_valid "$image_old_parent" "$image_arm" no; then
        image_copy_record "$image_old_parent" "$EPHEMERAL_OUT/$image_pass/p$image_p-seed$image_seed" "$image_arm"
      fi
    done
  done
  for image_round in 0 1 2 3 4; do
    image_old_parent="$image_previous/round$image_round/p$image_p-seed$image_seed"
    image_old_aa="$image_previous/round$image_round-aa/p$image_p-seed$image_seed"
    image_round_valid=yes
    for image_arm in "${image_arms[@]}"; do
      if ! image_record_valid "$image_old_parent" "$image_arm" yes; then image_round_valid=no; fi
    done
    if ! image_record_valid "$image_old_aa" original yes; then image_round_valid=no; fi
    if test "$image_round_valid" = yes; then
      for image_arm in "${image_arms[@]}"; do image_copy_record "$image_old_parent" "$EPHEMERAL_OUT/round$image_round/p$image_p-seed$image_seed" "$image_arm"; done
      image_copy_record "$image_old_aa" "$EPHEMERAL_OUT/round$image_round-aa/p$image_p-seed$image_seed" original
      printf '%s\n' "$COLD_PREVIOUS_PREFIX" > "$EPHEMERAL_OUT/round$image_round/p$image_p-seed$image_seed/restored-complete-round.txt"
    fi
  done
}
image_run_one() {
  image_pass=$1; image_p=$2; image_seed=$3; image_arm=$4; image_measured=$5
  image_parent="$EPHEMERAL_OUT/$image_pass/p$image_p-seed$image_seed"
  mkdir -p "$image_parent"
  if test "$image_measured" = no && image_record_valid "$image_parent" "$image_arm" no; then return 0; fi
  image_dest="$image_parent/$image_arm"
  printf '%s\n' "$EPHEMERAL_JOB_ID" > "$image_dest.origin-job.txt"
  if test "$image_measured" = yes; then printf '%s\n' "$EPHEMERAL_JOB_ID" > "$image_parent/job-id.txt"; fi
  for image_attempt in 0 1 2 3 4; do
    set +e
    if test "$image_measured" = yes; then
      python3 "$image_source/tools/isolated_bench.py" run --cpus "$image_cpus" --out "$image_dest.isolation.json" -- prlimit --as=8589934592 -- timeout 1200s "$image_binary" --image-cold "$image_tape" "$image_sha" "$image_p" "$image_seed" "$image_arm" "$image_dest" > "$image_dest.stdout" 2> "$image_dest.stderr"
    else
      python3 "$image_source/tools/isolated_bench.py" busy -- prlimit --as=8589934592 -- timeout 1200s "$image_binary" --image-cold "$image_tape" "$image_sha" "$image_p" "$image_seed" "$image_arm" "$image_dest" > "$image_dest.stdout" 2> "$image_dest.stderr"
    fi
    image_status=$?
    printf '%s\n' "$image_status" > "$image_dest.exit"
    set -e
    image_retry=no
    if test "$image_measured" = yes && test "$image_status" = 1 && ! test -e "$image_dest" && grep -q '^machine is busy:' "$image_dest.stderr"; then image_retry=yes; fi
    if test "$image_measured" = yes && test "$image_status" = 0 && ! jq -e '.run.contended == false and (.left_on_reserved.user_threads | length == 0)' "$image_dest.isolation.json" >/dev/null; then image_retry=yes; fi
    if test "$image_retry" = no || test "$image_attempt" = 4; then break; fi
    image_invalid="$EPHEMERAL_OUT/invalid-resource-attempts/$image_pass/p$image_p-seed$image_seed/$image_arm/attempt$image_attempt"
    mkdir -p "$image_invalid"
    for image_file in "$image_dest" "$image_dest.exit" "$image_dest.stdout" "$image_dest.stderr" "$image_dest.isolation.json"; do if test -e "$image_file"; then mv "$image_file" "$image_invalid/"; fi; done
  done
}
for image_p in $COLD_PRIMES; do
  for image_seed in ${COLD_SEEDS:-0 1 2}; do
    image_restore_case
    for image_pass in warmup first aa; do
      for image_arm in "${image_arms[@]}"; do image_run_one "$image_pass" "$image_p" "$image_seed" "$image_arm" no; done
    done
    for image_round in 0 1 2 3 4; do
      if test -f "$EPHEMERAL_OUT/round$image_round/p$image_p-seed$image_seed/restored-complete-round.txt"; then continue; fi
      if test $((image_round % 2)) = 0; then
        for image_arm in "${image_arms[@]}"; do image_run_one "round$image_round" "$image_p" "$image_seed" "$image_arm" yes; done
      else
        for ((image_i=9;image_i>=0;image_i--)); do image_run_one "round$image_round" "$image_p" "$image_seed" "${image_arms[image_i]}" yes; done
      fi
      image_run_one "round$image_round-aa" "$image_p" "$image_seed" original yes
    done
  done
done
printf 'all prescribed cold passes attempted; separate deterministic, isolation and relation audits required\n' > "$EPHEMERAL_OUT/complete.txt"
