#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto-audit
study=research/rho_constant_audit_20261112
binary=/workspace/scratch/ecbench-rho-frozen-61030c23
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py validate "$study/protocol.json"
python3 - <<'PY'
from pathlib import Path
import json,hashlib
r=Path('research/rho_constant_audit_20261112');p=json.loads((r/'protocol.json').read_text());x=p['extensions']
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert p['freeze']['status']=='frozen' and p['freeze']['filed_before_affected_data']
assert sha('/workspace/scratch/ecbench-rho-frozen-61030c23')==p['arms'][0]['instrument']['binary_sha256']
for name,value in [('spec.json',x['spec_sha256']),('analyze-main.sh',p['analysis_plan']['script']['sha256']),('analyze.sh',x['analysis_parent_sha256']),('evidence/build-provenance.json',x['build_provenance_sha256']),('export-preflight-complete.json',x['preflight_export_sha256']),('analysis/preflight/audit.json',x['preflight_audit_sha256'])]:assert sha(r/name)==value,name
n=x['analysis_native']
for name,key in [(n['source_path'],'source_sha256'),(n['queries_path'],'queries_sha256'),(n['negative_control_index_path'],'negative_control_index_sha256')]:assert sha(r/name)==n[key],name
assert sha(n['binary_path'])==n['binary_sha256']
assert sha(n['sqlite_runtime_path'])==n['sqlite_runtime_sha256']
a=json.loads((r/'analysis/preflight/audit.json').read_text());assert a['ok'] and a['verified_records']==12 and len(a['replays'])==12 and all(z['reproduced'] for z in a['replays'])
assert (r/'evidence/preflight-export-validation.exit').read_text().strip()=='0'
for q in json.loads((r/'evidence/build-provenance.json').read_text())['input_files']:assert sha(q['path'])==q['sha256'],q['path']
assert not (r/'sessions/main').exists() and not (r/'evidence/main-run.exit').exists()
PY
/workspace/scratch/rho-sql-analysis "$study/native/negative-control-index.sql" "$study/native/queries.sql" > "$study/evidence/analysis-negative-control.jsonl" 2> "$study/evidence/analysis-negative-control.stderr"
rg -q '"kind":"aa","warmup":0,"pairs":4,"mismatches":1,"pass":0' "$study/evidence/analysis-negative-control.jsonl"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
set +e
timeout --signal=INT --kill-after=5s 180s "$binary" run --wait --cpus none --spec "$study/spec.json" --out "$study/sessions/main" > "$study/evidence/main-run.stdout" 2> "$study/evidence/main-run.stderr"
rc=$?
printf '%s\n' "$rc" > "$study/evidence/main-run.exit"
exit "$rc"
