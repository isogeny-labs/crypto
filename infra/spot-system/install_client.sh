#!/bin/bash
# Run on the laptop; needs Python 3, AWS CLI and existing AWS authentication.
set -euo pipefail
command -v python3 >/dev/null
command -v aws >/dev/null
installer_root=$(cd -- "$(dirname -- "$0")" && pwd)
client_arguments=(--function arn:aws:lambda:us-east-2:104186811093:function:spot-experiments-Lifecycle-GXU4NpgoK61h --region us-east-2)
if [ "$#" -gt 0 ]; then
  client_arguments+=(--profile "$1")
fi
# Verify access before changing client files. No prompt/transcript is uploaded.
client_probe=$(mktemp -d)
trap 'rm -rf -- "$client_probe"' EXIT
python3 - "$installer_root" "$client_probe" "${1:-}" <<'PY'
import json,sys
from pathlib import Path
sys.path.insert(0,sys.argv[1])
from wake import invoke
config={'function':'arn:aws:lambda:us-east-2:104186811093:function:spot-experiments-Lifecycle-GXU4NpgoK61h','region':'us-east-2','profile':sys.argv[3] or None}
result=invoke(config,'status')
print('AWS lifecycle access verified; deployment status: '+result['state'])
PY
python3 "$installer_root/install_hooks.py" "${client_arguments[@]}"
printf '%s\n' 'Wake hooks and fixed-IP SSH proxy installed. Restart the coding clients.' 'In Codex, open /hooks and review/trust the new Spot definitions.'
