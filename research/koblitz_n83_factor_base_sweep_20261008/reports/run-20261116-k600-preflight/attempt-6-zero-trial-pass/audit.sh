#!/usr/bin/env bash
set -euo pipefail

# Receipt cross-check only. It does not replay a relation search.
here=$(cd "$(dirname "$0")" && pwd)
h2=${1:?pass the frozen H2 source worktree}
run=${2:?pass the retained attempt-2 output directory}
worker=${3:?pass the frozen Linux exporter ELF}
supervisor=${4:?pass the frozen macOS preflight supervisor}
expected_h2=a30e67492f821bc6f58ff97c508be9cfe144d7ab
expected_c=ba620214684e41def8bc6caddf2e782f43920218
expected_object=3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
hash() { shasum -a 256 "$1" | awk '{print $1}'; }
require() { test "$1" = "$2" || { echo "mismatch: $3" >&2; exit 1; }; }

require "$(git -C "$h2" rev-parse HEAD)" "$expected_h2" H2
require "$(git -C "$h2" rev-parse HEAD^)" "$expected_c" parent
test -z "$(git -C "$h2" status --porcelain)"
require "$(git -C "$h2" diff --name-only HEAD^ HEAD)" \
  'research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json' one_lock_diff
require "$(hash "$here/LOCK-H2.json")" \
  b97fd192c5ab2af87bd368062014b2c4bef643e791cf6a0ab335a1197c986cae lock_hash
require "$(hash "$here/native-outer.json")" "$(hash "$run/outer.json")" copied_outer
require "$(hash "$here/config.json")" "$(hash "$run/worker-output/run/config.json")" copied_config
require "$(hash "$here/summary.json")" "$(hash "$run/worker-output/run/summary.json")" copied_summary
require "$(hash "$here/events.jsonl")" "$(hash "$run/worker-output/run/events.jsonl")" copied_events
require "$(hash "$here/attestation.json")" "$(hash "$run/source/attestation.json")" copied_attestation
require "$(hash "$here/store-index.json")" "$(hash "$run/store-index.json")" copied_store_index
require "$(gzip -dc "$here/worker.stdout.log.gz" | shasum -a 256 | awk '{print $1}')" "$(hash "$run/stdout.log")" copied_stdout
require "$(gzip -dc "$here/worker.stderr.log.gz" | shasum -a 256 | awk '{print $1}')" "$(hash "$run/stderr.log")" copied_stderr
require "$(hash "$worker")" "$(jq -r .worker_sha256 "$here/LOCK-H2.json")" worker_elf
require "$(hash "$supervisor")" "$(jq -r .supervisor_sha256 "$here/LOCK-H2.json")" mac_supervisor
require "$(hash "$run/stdout.log")" "$(jq -r .stdout_sha256 "$here/native-outer.json")" worker_stdout
require "$(hash "$run/stderr.log")" "$(jq -r .stderr_sha256 "$here/native-outer.json")" worker_stderr
require "$(hash "$here/LOCK-H2.json")" \
  "$(jq -r .lock_sha256 "$here/native-outer.json")" native_lock_hash
require "$(hash "$here/config.json")" \
  "$(jq -r .worker_config_sha256 "$here/native-outer.json")" config_hash
require "$(hash "$here/summary.json")" \
  "$(jq -r .worker_summary_sha256 "$here/native-outer.json")" summary_hash

for file in $(jq -r '.source_sha256 | keys[]' "$here/LOCK-H2.json"); do
  expected=$(jq -r --arg file "$file" '.source_sha256[$file]' "$here/LOCK-H2.json")
  observed=$(git -C "$h2" show "$expected_c:$file" | shasum -a 256 | awk '{print $1}')
  require "$observed" "$expected" "source:$file"
done

object="$run/fresh-cache/sha256/3e/$expected_object"
require "$(hash "$object")" "$expected_object" fresh_s3_object
require "$(wc -c < "$object" | tr -d ' ')" 3025798 fresh_s3_bytes
require "$(jq -r .fresh_object_sha256 "$here/launch.json")" "$expected_object" launch_object
require "$(jq -r .fresh_object_bytes "$here/launch.json")" 3025798 launch_bytes
require "$(jq -r .object_sha256 "$here/LOCK-H2.json")" "$expected_object" lock_object
require "$(jq -r .source_commit "$here/summary.json")" "$expected_c" summary_source
jq -e '.schema == "n83.chain-s3-source-attestation/v1" and .status_clean == true
  and .source_commit == "ba620214684e41def8bc6caddf2e782f43920218"' \
  "$here/attestation.json" >/dev/null

jq -e '.schema == "n83.k600-m6-native-outer/v1" and .status == "PREFLIGHT_ONLY"
  and .worker_exit_code == 0 and (.timed_out | not) and .failure_reasons == []
  and .selected_best_total_runtime == null' "$here/native-outer.json" >/dev/null
jq -e '.schema == "n83.primary-store-chain/v1" and .status == "PREFLIGHT_ONLY"
  and .curve_a == 0 and .orbit_columns == 600 and .summands == 6
  and .max_trials == 0 and .report.trials == 0 and .report.relations == 0
  and .report.sat_calls == 0 and .report.matrix_rows == 0
  and .report.terminal_matrix_rank == 0 and .report.verification_failures == 0
  and (.solver_stage_executed | not) and .total_index_calculus_runtime_ms == null
  and .selected_best_total_runtime == null and .attempt_records == []
  and .relation_matrix == [] and .verified_log == null
  and .memory_cgroup_peak_bytes > 0 and .memory_cgroup_peak_bytes <= 4294967296
  and .chain_s3_limits == {conflict_budget:10000,max_domain_clauses:350000,max_models:1,max_variables:125000}' \
  "$here/summary.json" >/dev/null
jq -e '.schema == "n83.primary-store-chain-config/v1"
  and .source_commit == "ba620214684e41def8bc6caddf2e782f43920218"
  and .input_kind == "artifact_store_v3" and .max_trials == 0
  and .memory_cgroup_limit_bytes == 4294967296
  and .memory_cgroup_swap_limit_bytes == 0' "$here/config.json" >/dev/null
jq -e '.docker_argv | join(" ") |
  contains("--network none") and contains("--memory 4096m")
  and contains("--memory-swap 4096m") and contains("--cpus 1")
  and contains("--cpuset-cpus 0") and contains("primary-store-v3-orbit-chain-cold")' \
  "$here/launch.json" >/dev/null
test "$(wc -l < "$here/events.jsonl" | tr -d ' ')" = 6
test "$(cat "$here/exit.txt")" = 0
test ! -s "$here/residual-container.txt"
awk -F '\t' 'NR>0 {if ($2<20971520 || $3<6291456) exit 1}' "$here/disk-watch.tsv"
echo "PASS H2=$expected_h2 parent=$expected_c source_files=7 object_sha256=$expected_object bytes=3025798"
echo "PASS status=PREFLIGHT_ONLY worker_exit=0 trials=0 relations=0 solver_stage=false rank=0"
echo "PASS memory_cgroup_peak_bytes=$(jq -r .memory_cgroup_peak_bytes "$here/summary.json") memory_limit_bytes=4294967296"
echo "PASS elapsed_wall_ms=$(jq -r .elapsed_wall_ms "$here/native-outer.json") wrapper_start=$(cat "$here/start.utc") wrapper_end=$(cat "$here/end.utc")"
echo "PASS one_disk_sample_above_floors residual_container_empty=true"
