#!/usr/bin/env bash
# Thin transport replay of the unchanged E30 CAS input, followed by its Rust gate.
set -eu
task_study=research/twist_routes_1_3_20261021
task_output="$task_study/execution-v2/publication-20261025-setec-replay"
task_input="$task_study/execution-v2/publication-20261025-setec-v1/extracted/runs/incidence-20261023-v1/verify.gp"
task_gp=/workspace/shared/twist-tools-20261023/cas/root/usr/bin/gp
if [ -e "$task_output" ]; then
  printf '%s\n' 'fresh replay output required' >&2
  exit 1
fi
mkdir "$task_output"
cp "$task_study/replay-published-incidence.sh" "$task_output/REPLAYER.sh"
date -u +%Y-%m-%dT%H:%M:%SZ > "$task_output/started.utc"
set +e
(
  ulimit -v 4194304
  OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 timeout --signal=TERM --kill-after=5s 600s "$task_gp" -f -q "$task_input"
) > "$task_output/gp.stdout" 2> "$task_output/gp.stderr"
task_exit=$?
set -e
date -u +%Y-%m-%dT%H:%M:%SZ > "$task_output/finished.utc"
node --input-type=module - "$task_output" "$task_input" "$task_gp" "$task_exit" <<'JS'
import fs from 'node:fs';
import crypto from 'node:crypto';
const [out,input,gp,code]=process.argv.slice(2),study='research/twist_routes_1_3_20261021';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const historical=JSON.parse(fs.readFileSync(study+'/execution-v2/receipts/incidence-20261023-v1-gp.json'));
const receipt={schema:'twist.cas-transport/v1',tool_binary_sha256:sha(gp),argv:[gp,'-f','-q',input],
  script_sha256:sha(input),started_utc:fs.readFileSync(out+'/started.utc','utf8').trim(),
  finished_utc:fs.readFileSync(out+'/finished.utc','utf8').trim(),exit_code:Number(code),
  timeout_s:600,address_space_kib:4194304,threads:1,isolation:'L0',wall_time_admitted:false,
  stdout_sha256:sha(out+'/gp.stdout'),stderr_sha256:sha(out+'/gp.stderr'),
  source_revision:'644f1dbef70c5935db211a7d3a70ec7bb0601336',
  native_adjudicator_binary_sha256:sha(study+'/runner/target/release/twist-route-experiments'),
  historical_input_byte_identity:sha(input)===historical.script_sha256,
  historical_tool_byte_identity:sha(gp)===historical.tool_binary_sha256,
  frozen_protocol:study+'/execution-v2/INCIDENCE_PROTOCOL_V1.md',
  replayer_sha256:sha(out+'/REPLAYER.sh'),scientific_acceptance:'requires-native-adjudication',
  fresh_targets_generated:false,performance_claim:false,independent_host_replay:false};
fs.writeFileSync(out+'/gp.json',JSON.stringify(receipt,null,2)+'\n');
if(!receipt.historical_input_byte_identity||!receipt.historical_tool_byte_identity)throw Error('historical replay inputs changed');
JS
set +e
"$task_study/runner/target/release/twist-route-experiments" followup adjudicate-cas \
  "$task_study/execution-v1/fixtures.json" "$task_output/adjudication" "$task_output/gp" \
  > "$task_output/adjudication.stdout" 2> "$task_output/adjudication.stderr"
task_gate_exit=$?
set -e
printf '%s\n' "$task_gate_exit" > "$task_output/adjudication.exit"
exit "$task_gate_exit"
