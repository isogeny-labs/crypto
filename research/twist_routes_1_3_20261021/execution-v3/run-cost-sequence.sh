#!/usr/bin/env bash
# Thin transport: one native solver/profile at a time. No scientific calculations.
set -euo pipefail
study=research/twist_routes_1_3_20261021
base=$study/execution-v3
while [ ! -f "$base/runs/sweep-20261026-v1.exit" ]; do sleep 5; done
test "$(cat "$base/runs/sweep-20261026-v1.exit")" = 0

set +e
/workspace/shared/twist-native-continue-20261026/31fec37f7029a3674759c53cf4865dc3d24855c7/sweep-audit \
  "$base/sweep-20261026/PLAN.json" "$base/runs/sweep-20261026-v1" \
  "$base/sweep-audit-20261026-v1" > "$base/checks/sweep-audit.stdout" 2> "$base/checks/sweep-audit.stderr"
audit_rc=$?
printf '%s\n' "$audit_rc" > "$base/checks/sweep-audit.exit"
set -e
test "$audit_rc" = 0

stage() {
  local label=$1 revision=$2 driver=$3 worker=$4 plan=$5 limit=$6 protocol=$7
  local out="$base/runs/$label" started code
  test ! -e "$out"
  started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  set +e
  (ulimit -c 0; ulimit -v 4194304; timeout --signal=TERM --kill-after=5s "${limit}s" "$driver" run "$plan" "$worker" "$out") > "$out.transport.stdout" 2> "$out.transport.stderr"
  code=$?
  set -e
  printf '%s\n' "$code" > "$out.exit"
  node --input-type=module - "$out" "$revision" "$driver" "$worker" "$plan" "$started" "$code" "$protocol" <<'JS'
import fs from 'node:fs';import crypto from 'node:crypto';
const [out,revision,driver,worker,plan,started,code,protocol]=process.argv.slice(2);
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
fs.writeFileSync(out+'.transport.json',JSON.stringify({schema:'twist.run-receipt/v4',worker_and_driver_source_commit:revision,started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(code),argv:[driver,'run',plan,worker,out],driver_sha256:sha(driver),worker_sha256:sha(worker),plan_sha256:sha(plan),protocol_path:protocol,protocol_sha256:sha(protocol),stdout_sha256:sha(out+'.transport.stdout'),stderr_sha256:sha(out+'.transport.stderr'),transport_script_sha256:sha('research/twist_routes_1_3_20261021/execution-v3/run-cost-sequence.sh'),threads:1,address_space_kib:4194304,isolation:'L0',wall_time_admitted:false,complete_IC_cost:false},null,2)+'\n');
JS
  printf '%s exit=%s\n' "$label" "$code"
  test "$code" = 0
}

source_revision=31fec37f7029a3674759c53cf4865dc3d24855c7
source_root=/workspace/shared/twist-native-continue-20261026/$source_revision
stage decomposition-20261026-v1 "$source_revision" "$source_root/decomposition-cost" "$source_root/runner" "$base/decomposition-20261026/PLAN.json" 1900 "$base/DECOMPOSITION_PROTOCOL_V1.md"

source_revision=bb398a044246b5b6cb5a135a3c286b86d5d817ed
source_root=/workspace/shared/twist-native-continue-20261026/$source_revision
stage transfer-cost-20261026-v1 "$source_revision" "$source_root/transfer-cost" "$source_root/runner" "$base/transfer-20261026/PLAN.json" 2500 "$base/TRANSFER_COST_PROTOCOL_V1.md"
