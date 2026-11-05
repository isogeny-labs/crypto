#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
DEST=s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/attempt-03/
START=$(date -u +%FT%TZ)
set +e
timeout --preserve-status --signal=TERM --kill-after=45s 13000s bash "$HERE/execute.sh"
RC=$?
set -e
END=$(date -u +%FT%TZ)
python3 - "$HERE" "$START" "$END" "$RC" <<'PY'
import json,sys
from pathlib import Path
r=Path(sys.argv[1]);(r/'controller-status.json').write_text(json.dumps({'schema':'n83.topform-controller-status/v1','candidate':'spike38','attempt':'03','start_utc':sys.argv[2],'end_utc':sys.argv[3],'launcher_exit_code':int(sys.argv[4]),'wall_cap_seconds':13000,'termination_grace_seconds':45,'wall_clock_valid':False},indent=2)+'\n')
PY
aws s3 sync "$HERE" "$DEST" --region us-east-2 --only-show-errors
exit "$RC"
