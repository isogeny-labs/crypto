#!/usr/bin/env bash
set -euo pipefail
input="${1:?GP script}"
receipt="${2:?fresh log prefix}"
cap="${3:-1800}"
case "$cap" in 1400|1800|3600) ;; *) exit 2 ;; esac
test ! -e "$receipt.json"
gp_bin="${TWIST_GP_BIN:-/tmp/twist-cas/root/usr/bin/gp}"
revision="$(git rev-parse HEAD)"
started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
set +e
(
  ulimit -v 4194304
  export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
  export LD_LIBRARY_PATH="${TWIST_CAS_LIB:-/tmp/twist-cas/root/usr/lib/x86_64-linux-gnu}"
  timeout --signal=TERM --kill-after=5s "${cap}s" "$gp_bin" -f -q "$input"
) > "$receipt.stdout" 2> "$receipt.stderr"
status=$?
set -e
# Transport metadata only. Scientific acceptance is decided by native Rust.
node - "$receipt" "$revision" "$gp_bin" "$input" "$status" "$cap" "$started" <<'NODE'
const fs=require('fs'),c=require('crypto');
const[receipt,revision,binary,input,status,cap,started]=process.argv.slice(2);
const sha=p=>c.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
fs.writeFileSync(receipt+'.json',JSON.stringify({schema:'twist.CAS-transport/v2',source_commit:revision,
 tool:'PARI/GP 2.17.2',tool_binary_sha256:sha(binary),argv:[binary,'-f','-q',input],
 script_sha256:sha(input),started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(status),
 timeout_s:Number(cap),address_space_kib:4194304,threads:1,stdout_sha256:sha(receipt+'.stdout'),
 stderr_sha256:sha(receipt+'.stderr'),scientific_acceptance:'requires-native-adjudication'},null,2)+'\n');
NODE
exit "$status"
