#!/bin/bash
set -euo pipefail
[[ $# == 1 ]] || { echo 'usage: qualify-clean-dataset-01.sh OUTPUT' >&2; exit 2; }
output=$1
[[ ! -e "$output" && ! -L "$output" ]]
mkdir "$output"
output=$(realpath "$output")
attempt=/home/ec2-user/ml-dataset-work-20261028/clean-dataset-01
package=/home/ec2-user/ml-dataset-work-20261028/clean-source-package-01
log_sha=08aa2d5a61103280807326084d235d99521f65fa6477e73defa5719f9c06195c
printf '%s  %s\n' "$log_sha" "$attempt/validation.log" | sha256sum -c -
jq -e --arg log "$log_sha" '
  .schema=="ml-dataset.clean-software-validation/v1" and .phase=="dataset" and
  .stage=="coverage_check" and .exit_code==1 and .command_exit_code==0 and
  .command_started and .source_preflight_matches and .source_readback_matches and
  .log_sha256==$log and .source_commit=="1d858fb9e4b8035b38f93db4423880627b09211f" and
  .binary_sha256=="4402eb3744e5961e7bce4ebb9e15e3d6567c10a7834f2677470cf6a9ca2cac80" and
  .runner_sha256=="2c51897a277eefec6b6809220f157e4917393e85d1db5e95438ab4a6d0754da7" and
  .command==["cargo","test","--locked","--release","--lib","ml_dataset::","--","--test-threads=1"]
' "$attempt/receipt.json" >/dev/null
mapfile -t results < <(rg '^test result:' "$attempt/validation.log")
[[ ${#results[@]} == 2 ]]
[[ ${results[0]} == 'test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5852 filtered out; finished in 0.00s' ]]
[[ ${results[1]} == 'test result: ok. 479 passed; 0 failed; 2 ignored; 0 measured; 5372 filtered out; finished in 9.83s' ]]
[[ $(rg -c '^running 481 tests$' "$attempt/validation.log") == 1 ]]
[[ $(rg -c '^running 1 test$' "$attempt/validation.log") == 1 ]]
rg -q '^test ml_dataset::telemetry::checked::tests::fifo_replacement_is_rejected_without_waiting_for_writer \.\.\. ok$' "$attempt/validation.log"
source_path=src/ml_dataset/telemetry/checked/tests.rs
expected=$(awk -v path="$source_path" '$2==path {print $1}' "$package/source.sha256")
[[ "$expected" =~ ^[a-f0-9]{64}$ ]]
printf '%s  %s\n' "$expected" "$package/source/$source_path" | sha256sum -c -
sed -n '300,329p' "$package/source/$source_path" > "$output/child-test-source.txt"
printf '%s\n' "${results[@]}" > "$output/test-summary.txt"
jq -n --arg date "$(date -u +%FT%TZ)" --arg log "$log_sha" \
  --arg receipt "$(sha256sum "$attempt/receipt.json" | cut -d' ' -f1)" \
  --arg source_path "$source_path" --arg source_sha "$expected" \
  --arg script_sha "$(sha256sum "$0" | cut -d' ' -f1)" \
  '{schema:"ml-dataset.terminal-test-qualification/v1",qualified_at:$date,status:"passed",
    source_commit:"1d858fb9e4b8035b38f93db4423880627b09211f",
    binary_sha256:"4402eb3744e5961e7bce4ebb9e15e3d6567c10a7834f2677470cf6a9ca2cac80",
    original_receipt_sha256:$receipt,raw_log_sha256:$log,qualifier_sha256:$script_sha,
    original_wrapper_exit_code:1,original_wrapper_stage:"coverage_check",cargo_exit_code:0,
    parent:{passed:479,failed:0,ignored:2,filtered_out:5372},
    child:{passed:1,failed:0,ignored:0,filtered_out:5852,counted_as_additional_parent_tests:false},
    child_spawn_source:{path:$source_path,sha256:$source_sha,lines:[300,329]},
    reason:"Known FIFO subprocess summary precedes successful parent result; wrapper single-summary assumption failed",
    reexecution_performed:false,raw_receipt_modified:false,independent_review:false,
    scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/receipt.json"
cat "$output/receipt.json"
