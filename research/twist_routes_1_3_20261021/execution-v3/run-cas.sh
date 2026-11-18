#!/usr/bin/env bash
# Thin process/hash transport only; mathematical verification executes in PARI.
set -euo pipefail
script=${1:?GP script}
prefix=${2:?fresh receipt prefix}
test ! -e "$prefix.json"
test ! -e "$prefix.stdout"
test ! -e "$prefix.stderr"
cas=/workspace/shared/twist-tools-20261023/cas/root/usr
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
set +e
(ulimit -c 0; ulimit -v 4194304; export LD_LIBRARY_PATH="$cas/lib/x86_64-linux-gnu:$cas/lib/x86_64-linux-gnu/singular"; timeout --signal=TERM --kill-after=5s 300s "$cas/bin/gp" -q -f "$script") > "$prefix.stdout" 2> "$prefix.stderr"
code=$?
set -e
node --input-type=module - "$script" "$prefix" "$started" "$code" <<'JS'
import fs from 'node:fs';import crypto from 'node:crypto';
const [script,prefix,started,code]=process.argv.slice(2),gp='/workspace/shared/twist-tools-20261023/cas/root/usr/bin/gp';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.cas-transport/v3',argv:[gp,'-q','-f',script],started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(code),binary_sha256:sha(gp),script_sha256:sha(script),stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),wrapper_sha256:sha('research/twist_routes_1_3_20261021/execution-v3/run-cas.sh'),address_space_kib:4194304,timeout_s:300,wall_time_admitted:false},null,2)+'\n');
JS
exit "$code"
