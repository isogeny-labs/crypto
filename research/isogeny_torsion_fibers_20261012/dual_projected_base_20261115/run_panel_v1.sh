#!/usr/bin/env bash
set -u

if [[ $# -ne 3 ]]; then
  echo "usage: run_panel_v1.sh RUN_DIR RUNNER_BINARY CHECKER_BINARY" >&2
  exit 2
fi
run_dir=$1
runner=$2
checker=$3
if [[ -e "$run_dir" ]]; then
  echo "refusing to overwrite $run_dir" >&2
  exit 2
fi
mkdir -p "$run_dir/exhaustive" "$run_dir/children"
finalize() {
  final_rc=$?
  {
    date -u '+finished_utc=%Y-%m-%dT%H:%M:%SZ'
    echo "panel_exit=$final_rc"
  } > "$run_dir/FINISHED.txt"
  (
    cd "$run_dir" || exit 1
    find . -type f ! -name ARTIFACT_FILES.sha256 -print0 | sort -z | xargs -0 sha256sum
  ) > "$run_dir/ARTIFACT_FILES.sha256"
}
trap finalize EXIT

study=research/isogeny_torsion_fibers_20261012/dual_projected_base_20261115
full_base=research/isogeny_torsion_fibers_20261012/factor_base_grid_20261016/RUN_V2/BASE_PUBLIC_V1.json
parent=research/isogeny_torsion_fibers_20261012/different_j_order_batch_20261015/PUBLIC_V1.json
heldout=research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/FOUR_TARGET_PUBLIC_V1.json
answers=research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/FOUR_TARGET_CHECKER_V1.json
dual_base="$run_dir/DUAL_BASE_PUBLIC_V1.json"

cp examples/isogeny_fiber_dual_projected.rs "$run_dir/RUNNER_SOURCE_V1.rs.txt"
cp examples/isogeny_fiber_dual_projected_check.rs "$run_dir/CHECKER_SOURCE_V1.rs.txt"
cp "$runner" "$run_dir/RUNNER_BINARY_V1"
cp "$checker" "$run_dir/CHECKER_BINARY_V1"
sha256sum "$full_base" "$parent" "$heldout" "$answers" \
  "$study/PROTOCOL_V1.md" "$study/AMENDMENT_V2.md" "$study/run_panel_v1.sh" \
  "$run_dir/RUNNER_SOURCE_V1.rs.txt" "$run_dir/CHECKER_SOURCE_V1.rs.txt" \
  "$run_dir/RUNNER_BINARY_V1" "$run_dir/CHECKER_BINARY_V1" \
  > "$run_dir/INPUTS.sha256"
runner="$run_dir/RUNNER_BINARY_V1"
checker="$run_dir/CHECKER_BINARY_V1"
{
  date -u '+started_utc=%Y-%m-%dT%H:%M:%SZ'
  uname -a
  rustc -Vv
  echo 'isolation=Darwin shared L0; no physical-core reservation'
  echo 'wall_cap=300 s per child via timeout'
  echo 'address_space_cap=unsupported by this Darwin host; observed RSS recorded'
} > "$run_dir/HOST.txt"

timeout 300 "$runner" freeze-dual-input "$full_base" "$dual_base" \
  > "$run_dir/freeze.stdout" 2> "$run_dir/freeze.stderr"
freeze_rc=$?
echo "$freeze_rc" > "$run_dir/freeze.exit"
if [[ $freeze_rc -ne 0 ]]; then
  echo 'candidate input freeze failed; no identity or solve children launched' >&2
  exit 1
fi
sha256sum "$dual_base" > "$run_dir/DUAL_BASE_PUBLIC_V1.sha256"

timeout 300 "$runner" verify-dual > "$run_dir/DUAL_IDENTITY.json" \
  2> "$run_dir/DUAL_IDENTITY.stderr"
identity_rc=$?
echo "$identity_rc" > "$run_dir/DUAL_IDENTITY.exit"
if [[ $identity_rc -eq 0 ]]; then
  timeout 300 "$checker" verify-dual "$run_dir/DUAL_IDENTITY.json" \
    > "$run_dir/DUAL_IDENTITY.replay.json" 2> "$run_dir/DUAL_IDENTITY.replay.stderr"
  replay_rc=$?
  echo "$replay_rc" > "$run_dir/DUAL_IDENTITY.replay.exit"
else
  echo 97 > "$run_dir/DUAL_IDENTITY.replay.exit"
  replay_rc=97
fi
if [[ $identity_rc -ne 0 || $replay_rc -ne 0 ]]; then
  echo 'dual identity gate failed; no solve children launched' >&2
  exit 1
fi

exact_failed=0
for columns in 8 16 32 64; do
  prefix="$run_dir/exhaustive/b$columns"
  timeout 300 "$runner" exhaustive-grid "$full_base" "$columns" \
    > "$prefix.jsonl" 2> "$prefix.stderr"
  rc=$?
  echo "$rc" > "$prefix.exit"
  if [[ $rc -eq 0 ]]; then
    timeout 300 "$checker" "$full_base" "$parent" "$columns" "$prefix.jsonl" \
      > "$prefix.replay.json" 2> "$prefix.replay.stderr"
    replay_rc=$?
    echo "$replay_rc" > "$prefix.replay.exit"
    if [[ $replay_rc -ne 0 ]]; then exact_failed=1; fi
  else
    echo 97 > "$prefix.replay.exit"
    exact_failed=1
  fi
done
if [[ $exact_failed -ne 0 ]]; then
  echo 'exhaustive pair gate failed; no solve children launched' >&2
  exit 1
fi

: > "$run_dir/RUN_PLAN_V1.tsv"
arms=(chi-dual direct-dual chi-inverse chi-full direct-inverse direct-full destination-direct source-bsgs rho-negation rho-negation-aa)
for round in 0 1 2; do
  for base_index in 0 1 2 3; do
    columns=$((8 << base_index))
    for position in 0 1 2 3; do
      offset=$(((round + base_index + position) % 10))
      for arm_index in 0 1 2 3 4 5 6 7 8 9; do
        index=$(((arm_index + offset) % 10))
        printf '%s\t%s\t%s\t%s\n' "$round" "$columns" "$position" "${arms[$index]}" \
          >> "$run_dir/RUN_PLAN_V1.tsv"
      done
    done
  done
done
sha256sum "$run_dir/RUN_PLAN_V1.tsv" > "$run_dir/RUN_PLAN_V1.sha256"

failed=0
while IFS=$'\t' read -r round columns position arm; do
  prefix="$run_dir/children/r$round-b$columns-t$position-$arm"
  base="$full_base"
  if [[ "$arm" == chi-dual || "$arm" == direct-dual ]]; then base="$dual_base"; fi
  timeout 300 "$runner" run-grid "$base" "$heldout" "$round" "$arm" "$columns" "$position" \
    > "$prefix.json" 2> "$prefix.stderr"
  rc=$?
  echo "$rc" > "$prefix.exit"
  if [[ $rc -eq 0 ]]; then
    timeout 300 "$checker" run "$base" "$heldout" "$answers" "$prefix.json" "$full_base" \
      > "$prefix.replay.json" 2> "$prefix.replay.stderr"
    replay_rc=$?
    echo "$replay_rc" > "$prefix.replay.exit"
    if [[ $replay_rc -ne 0 ]]; then failed=1; fi
  else
    echo 97 > "$prefix.replay.exit"
    failed=1
  fi
done < "$run_dir/RUN_PLAN_V1.tsv"

timeout 300 "$runner" summarize-grid "$run_dir" "$run_dir/DUAL_GRAPH_V1.svg" \
  > "$run_dir/SUMMARY_V1.json" 2> "$run_dir/SUMMARY_V1.stderr"
summary_rc=$?
echo "$summary_rc" > "$run_dir/SUMMARY_V1.exit"
if [[ $summary_rc -ne 0 ]]; then failed=1; fi
timeout 300 "$runner" audit-dual "$run_dir" > "$run_dir/PAIR_AUDIT_V1.json" \
  2> "$run_dir/PAIR_AUDIT_V1.stderr"
audit_rc=$?
echo "$audit_rc" > "$run_dir/PAIR_AUDIT_V1.exit"
if [[ $audit_rc -ne 0 ]]; then failed=1; fi
exit "$failed"
