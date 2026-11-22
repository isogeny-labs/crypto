#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto-audit
study=research/rho_constant_audit_20261112
binary=/workspace/scratch/ecbench-rho-frozen-61030c23
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py validate "$study/protocol-preflight.json"
python3 - <<'PY'
from pathlib import Path
import json,hashlib
r=Path('research/rho_constant_audit_20261112');p=json.loads((r/'protocol-preflight.json').read_text())
assert p['freeze']['status']=='frozen' and p['freeze']['filed_before_affected_data']
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha('/workspace/scratch/ecbench-rho-frozen-61030c23')==p['arms'][0]['instrument']['binary_sha256']
assert sha(r/'spec-preflight.json')==p['extensions']['spec_sha256']
assert sha(r/'analyze.sh')==p['analysis_plan']['script']['sha256']
assert sha(r/'evidence/build-provenance.json')==p['extensions']['build_provenance_sha256']
provenance=json.loads((r/'evidence/build-provenance.json').read_text())
for row in provenance['input_files']:assert sha(row['path'])==row['sha256'],row['path']
assert not (r/'sessions/preflight').exists()
assert not (r/'evidence/preflight-run.exit').exists()
PY
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
set +e
timeout --signal=INT --kill-after=5s 60s "$binary" run --wait --cpus none --spec "$study/spec-preflight.json" --out "$study/sessions/preflight" > "$study/evidence/preflight-run.stdout" 2> "$study/evidence/preflight-run.stderr"
rc=$?
printf '%s\n' "$rc" > "$study/evidence/preflight-run.exit"
exit "$rc"
