#!/usr/bin/env bash
# Materialize exact current-base unit-test inputs, refusing changed files.
# No network/cloud access. Object IDs and digests are in the adjacent manifest.
set -euo pipefail
fixture_root=$(cd -- "$(dirname -- "$0")/../../.." && pwd)
cd -- "$fixture_root"
python3 - <<'PY'
from pathlib import Path
import hashlib,json,subprocess
manifest=Path('research/relation_fusion_20261117/ROOT_UNIT_FIXTURE_SOURCES_150.json')
for row in json.loads(manifest.read_text()):
    data=subprocess.check_output(['git','cat-file','blob',row['git_blob']])
    if hashlib.sha256(data).hexdigest()!=row['sha256']:
        raise SystemExit('Git fixture digest mismatch: '+row['path'])
    path=Path(row['path'])
    if path.exists():
        if path.read_bytes()!=data:raise SystemExit('Existing fixture differs: '+str(path))
    else:
        path.parent.mkdir(parents=True,exist_ok=True)
        with path.open('xb') as f:f.write(data)
print('All unit fixture files match exact source Git blobs.')
PY
