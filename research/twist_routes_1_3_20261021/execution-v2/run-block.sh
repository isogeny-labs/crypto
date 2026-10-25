#!/usr/bin/env bash
# Thin native-run transport; Node records hashes/metadata only.
set -euo pipefail
task_command="${1:?block-build or block-panel}"
task_output="${2:?fresh output directory}"
task_input="${3:-}"
case "$task_command" in block-build|block-panel) ;; *) exit 2 ;; esac
task_binary="${TWIST_BLOCK_BIN:?pinned native binary required}"
task_study=research/twist_routes_1_3_20261021
task_receipt="$task_output.transport"
test ! -e "$task_output"
test ! -e "$task_receipt.json"
task_revision="$(git rev-parse HEAD)"
task_started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
task_cap=4000
if [ "$task_command" = block-build ]; then task_cap=3600; fi
set +e
(
  ulimit -c 0
  ulimit -v 4194304
  export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RAYON_NUM_THREADS=1
  task_args=(followup "$task_command" "$task_study/execution-v1/fixtures.json" "$task_output")
  if [ -n "$task_input" ]; then task_args+=("$task_input"); fi
  timeout --signal=TERM --kill-after=5s "${task_cap}s" "$task_binary" "${task_args[@]}"
) > "$task_receipt.stdout" 2> "$task_receipt.stderr"
task_exit=$?
set -e
node --input-type=module - "$task_receipt" "$task_revision" "$task_binary" "$task_command" "$task_output" "$task_input" "$task_started" "$task_exit" "$task_cap" <<'JS'
import fs from 'node:fs';
import crypto from 'node:crypto';
const [prefix,revision,binary,command,output,input,started,code,cap]=process.argv.slice(2);
const study='research/twist_routes_1_3_20261021';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const argv=[binary,'followup',command,study+'/execution-v1/fixtures.json',output];if(input)argv.push(input);
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.run-receipt/v3',source_commit:revision,binary_sha256:sha(binary),argv,started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(code),timeout_s:Number(cap),address_space_kib:4194304,threads:1,isolation:'L0',protocol_path:study+'/execution-v2/BLOCK_PROTOCOL_V1.md',protocol_sha256:sha(study+'/execution-v2/BLOCK_PROTOCOL_V1.md'),fixture_sha256:sha(study+'/execution-v1/fixtures.json'),stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),wall_time_admitted:false,performance_claim:false,novelty_established:false},null,2)+'\n');
JS
exit "$task_exit"
