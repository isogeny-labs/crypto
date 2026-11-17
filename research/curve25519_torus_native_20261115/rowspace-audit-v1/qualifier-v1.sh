#!/usr/bin/env bash
set -euo pipefail
cd /workspace/isogeny-labs/crypto-leased-affine-20261117
python3 - <<'META'
from pathlib import Path
import hashlib,json,datetime,subprocess
study=Path('research/curve25519_torus_native_20261115/rowspace-audit-v1')
build=json.loads((study/'BUILD_MANIFEST.json').read_text())
for path,digest in build['sources'].items(): assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==digest,path
assert hashlib.sha256(Path(build['binary']['path']).read_bytes()).hexdigest()==build['binary']['sha256']
assert hashlib.sha256((study/'qualifier-executed-v1.sh').read_bytes()).hexdigest()==build['qualifier_sha256']
identity=json.loads(subprocess.check_output([build['binary']['path'],'--identity']))
for path,digest in build['compiled_source_bindings'].items(): assert identity[path]==digest,path
record={'schema':'rowspace.precall/v1','at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sources_match':len(build['sources']),'compiled_bindings_match':len(identity),'binary':build['binary'],'qualifier_sha256':build['qualifier_sha256'],'planned_cases':7,'solver_calls':0}
with (study/'PRECALL_QUALIFICATION.json').open('x') as out: json.dump(record,out,indent=2);out.write('\n')
META
/workspace/curve25519-native-binaries/rowspace-audit-control-v1 research/curve25519_torus_native_20261115/rowspace-audit-v1/native-v1 > research/curve25519_torus_native_20261115/rowspace-audit-v1/.native-console-v1.log 2>&1
