#!/usr/bin/env bash
set -euo pipefail
export AWS_RETRY_MODE=standard AWS_MAX_ATTEMPTS=1
HERE=$(cd "$(dirname "$0")" && pwd)
DEST=s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/attempt-07/results/
START=$(date -u +%FT%TZ)
CPU=$(taskset -pc $$ | awk -F': ' '{print $2}' | cut -d, -f1 | cut -d- -f1)
set +e
taskset -c "$CPU" timeout --preserve-status --signal=TERM --kill-after=45s 12850s bash "$HERE/run-spike38-topform.sh"
RC=$?
set -e
END=$(date -u +%FT%TZ)
python3 - "$HERE" "$START" "$END" "$RC" <<'PY'
import json,sys
from pathlib import Path
p=Path(sys.argv[1]);(p/'controller-status.json').write_text(json.dumps({'schema':'n83.topform-controller-status/v1','candidate':'spike38','attempt':'07','start_utc':sys.argv[2],'end_utc':sys.argv[3],'launcher_exit_code':int(sys.argv[4]),'wall_cap_seconds':12850,'termination_grace_seconds':45,'wall_clock_valid':False},indent=2)+'\n')
if not (p/'terminal.json').exists():(p/'terminal.json').write_text(json.dumps({'schema':'n83.candidate-top-run-terminal/v1','candidate_id':'spike38','attempt':'07','started_at':sys.argv[2],'finished_at':sys.argv[3],'exit_code':int(sys.argv[4]),'wall_valid':False},indent=2)+'\n')
PY
python3 - "$HERE" "$DEST" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);dest=sys.argv[2];files=[]
for f in sorted(p.rglob('*')):
 if not f.is_file() or f.name in ['S3_PERSISTENCE_INDEX.json','manifest.sha256']:continue
 b=f.read_bytes();rel=f.relative_to(p).as_posix();files.append({'path':rel,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'s3_uri':dest+rel})
(p/'manifest.sha256').write_text(''.join(f"{x['sha256']}  {x['path']}\n" for x in files))
files.append({'path':'manifest.sha256','bytes':(p/'manifest.sha256').stat().st_size,'sha256':hashlib.sha256((p/'manifest.sha256').read_bytes()).hexdigest(),'s3_uri':dest+'manifest.sha256'})
(p/'S3_PERSISTENCE_INDEX.json').write_text(json.dumps({'schema':'n83.candidate-top-output-index/v1','candidate':'spike38','attempt':'07','files':files},indent=2)+'\n')
PY
aws s3 sync "$HERE" "$DEST" --region us-east-2 --only-show-errors
exit "$RC"
