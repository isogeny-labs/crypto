#!/usr/bin/env bash
set -euo pipefail
# Reversible local correctness runs only; no lab/funding/scheduling operation.
study_dir="$(cd -- "$(dirname -- "$0")/.." && pwd)"
repo_dir="$(cd -- "$study_dir/../.." && pwd)"
cd -- "$repo_dir"
panel="${1:?filters or torsion}"
output="${2:?fresh output directory}"
source_name="${3:-}"
case "$panel" in filters) cap=1400 ;; torsion|higher-torsion-input|higher-torsion-verify) cap=1800 ;; geometry-input|geometry-verify|adjudicate-cas|coupled-census|coupled-relations|coupled-energy|projection-replay|partial-replay|jacobian-replay|three-transfer|homogeneous-pilot|homogeneous-recheck) cap=3600 ;; *) exit 2 ;; esac
test ! -e "$output"
revision="$(git rev-parse HEAD)"
pinned_dir="/tmp/twist-v2-pinned/$revision"
mkdir -p "$pinned_dir" "$study_dir/execution-v2/receipts"
if test ! -e "$pinned_dir/runner"; then
  cp "$study_dir/runner/target/release/twist-route-experiments" "$pinned_dir/runner"
fi
receipt="$study_dir/execution-v2/receipts/$(basename -- "$output")"
test ! -e "$receipt.json"
argv=("$pinned_dir/runner" followup "$panel" "$study_dir/execution-v1/fixtures.json" "$output")
if test -n "$source_name"; then argv+=("$source_name"); fi
binary_sha="$(sha256sum "$pinned_dir/runner" | cut -d ' ' -f 1)"
started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
set +e
(
  ulimit -v 4194304
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
  timeout --signal=TERM --kill-after=5s "${cap}s" "${argv[@]}"
) > "$receipt.stdout" 2> "$receipt.stderr"
status=$?
set -e
node - "$receipt" "$revision" "$binary_sha" "$started" "$status" "$cap" "${argv[@]}" <<'NODE'
const fs=require('fs');const crypto=require('crypto');
const [receipt,revision,binary_sha,started,status,cap,...argv]=process.argv.slice(2);
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const study='research/twist_routes_1_3_20261021';
const homogeneous=argv[2].startsWith('homogeneous-');
const workers=argv[2]==='homogeneous-pilot'?3:1;
const protocol=study+'/execution-v2/'+(homogeneous?'HOMOGENEOUS_PROTOCOL_V1.md':argv[2].startsWith('higher-torsion-')?'HIGHER_TORSION_PROTOCOL_V1.md':argv[2].startsWith('coupled-')?'COUPLED_PROTOCOL_V1.md':argv[2].startsWith('projection-')||argv[2].startsWith('partial-')?'PROJECTION_PARTIAL_PROTOCOL_V1.md':['jacobian-replay','three-transfer'].includes(argv[2])?'DIVISOR_PROTOCOL_V1.md':'GLUING_PROTOCOL_V2.md');
fs.writeFileSync(receipt+'.json',JSON.stringify({schema:'twist.run-receipt/v2',source_commit:revision,
 binary_sha256:binary_sha,argv,started_utc:started,finished_utc:new Date().toISOString(),exit_code:Number(status),
 timeout_s:Number(cap),address_space_kib:4194304,threads:workers,per_engine_threads:1,concurrent_cells:workers,isolation:'L0',wall_time_admitted:false,
 protocol_path:protocol,protocol_sha256:hash(protocol),
 base_protocol_path:study+'/execution-v2/GLUING_PROTOCOL_V1.md',base_protocol_sha256:hash(study+'/execution-v2/GLUING_PROTOCOL_V1.md'),
 fixture_sha256:hash(study+'/execution-v1/fixtures.json'),registry_sha256:hash('docs/curves/registry.json'),
 stdout_sha256:hash(receipt+'.stdout'),stderr_sha256:hash(receipt+'.stderr'),
 performance_claim:false,geometric_quotient_claim:false},null,2)+'\n');
NODE
exit "$status"
