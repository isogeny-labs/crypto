#!/usr/bin/env bash
set -euo pipefail
study=research/twist_routes_1_3_20261021
base=$study/execution-v3
revision=${1:?pinned source revision}
pinned=/workspace/shared/twist-native-continue-20261026/$revision
out=$base/runs/linear-root-20261026-v1
plan=$base/linear-root-20261026/PLAN.json
protocol=$base/LINEAR_ROOT_PROTOCOL_V1.md
test ! -e "$out"
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
set +e
(ulimit -c 0; ulimit -v 4194304; timeout --signal=TERM --kill-after=5s 3900s "$pinned/decomposition-cost" run "$plan" "$pinned/runner" "$out") > "$out.transport.stdout" 2> "$out.transport.stderr"
code=$?
set -e
printf '%s\n' "$code" > "$out.exit"
node --input-type=module - "$out" "$revision" "$pinned" "$plan" "$protocol" "$started" "$code" <<'JS'
import fs from 'node:fs';import crypto from 'node:crypto';
const [out,revision,pinned,plan,protocol,started,code]=process.argv.slice(2);
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
fs.writeFileSync(out+'.transport.json',JSON.stringify({schema:'twist.run-receipt/v4',source_commit:revision,started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(code),argv:[pinned+'/decomposition-cost','run',plan,pinned+'/runner',out],driver_sha256:sha(pinned+'/decomposition-cost'),worker_sha256:sha(pinned+'/runner'),plan_sha256:sha(plan),protocol_sha256:sha(protocol),wrapper_sha256:sha('research/twist_routes_1_3_20261021/execution-v3/run-linear-root.sh'),stdout_sha256:sha(out+'.transport.stdout'),stderr_sha256:sha(out+'.transport.stderr'),isolation:'L0',wall_time_admitted:false},null,2)+'\n');
JS
exit "$code"
