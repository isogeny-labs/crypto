#!/bin/bash
# Finite retained software checks only. No research generation or cloud jobs.
set -euo pipefail
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
cd "$root"
source_commit=$(git rev-parse "$1^{commit}")
attempt=${2:-01}
[[ "$attempt" =~ ^[0-9]{2}$ ]]
# Existing original pipeline must have terminated successfully before target reuse.
jq -e '.exit_code == 0 and .last_phase == "completed" and .source_commit == "d9a69910198d14a829b689253a62a7663b41d753"' "$scratch/prefix-cap-fix-pipeline.receipt.json" >/dev/null
[[ ! -e "$scratch/algebraic-pipeline-$attempt.receipt.json" ]]
started=$(date -u +%FT%TZ)
phase=initialization
finish() {
 code=$?
 trap - EXIT
 jq -n --arg source "$source_commit" --arg start "$started" --arg finish "$(date -u +%FT%TZ)" --arg phase "$phase" --argjson code "$code" '{schema:"software-validation-pipeline/v1",source_commit:$source,started_utc:$start,finished_utc:$finish,last_phase:$phase,exit_code:$code,scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$scratch/algebraic-pipeline-$attempt.receipt.json"
 exit "$code"
}
trap finish EXIT
validation=research/ml_dataset_algebraic_targets_20261030/validation
validate() {
 phase=$1
 shift
 local result=0 archive_result=0 cli_archive_result=0
 bash "$validation/run-phase.sh" "$phase" "$source_commit" "$@" || result=$?
 bash "$validation/archive.sh" "$scratch/$phase.log" "$scratch/$phase.receipt.json" || archive_result=$?
 if [[ "$phase" == "algebraic-cli-$attempt" ]]; then
   # Native CLI stderr/stdout live inside the fixture directory. Preserve that
   # partial directory even when a command, assertion or preflight fails.
   bash "$validation/archive-cli.sh" "$attempt" || cli_archive_result=$?
 fi
 if [[ "$result" != 0 ]]; then return "$result"; fi
 if [[ "$archive_result" != 0 ]]; then return "$archive_result"; fi
 return "$cli_archive_result"
}
validate "algebraic-dataset-$attempt" cargo test --release --lib ml_dataset:: -- --test-threads=1
validate "algebraic-clippy-$attempt" bash scripts/clippy-maintained.sh
validate "algebraic-build-$attempt" cargo build --release --bin crypto
binary=$scratch/algebraic-export-crypto-$attempt
[[ ! -e "$binary" ]]
cp /home/ec2-user/crypto-ml-dataset-20261028/target/release/crypto "$binary"
validate "algebraic-cli-$attempt" bash research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh "$binary" "$scratch/algebraic-cli-fixture-run-$attempt"
validate "algebraic-full-$attempt" cargo test --release --lib -- --test-threads=1
phase=completed
