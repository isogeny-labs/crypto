#!/usr/bin/env bash
set -Eeuo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=/home/ec2-user/crypto-n83-degree-sweep-20261027
STAGE=/home/ec2-user/n83-candidate-bases/r39-ffd-attempt-01
RECOVERY="$REPO/research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r39-attempt-01/terminal-recovery-20261101T2020Z"
MATCHED="$REPO/research/n83_candidate_pdp_20261028/evidence/pdp/baseline/inputs.json"
PDP="$HERE/instruments/pdp"
INPUTS="$HERE/candidate-inputs"
S3ROOT=s3://isolab-results-104186811093-us-east-2/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike39/attempt-01
S3INPUTS="$S3ROOT/inputs-v1/"
RUN_LOG="$HERE/generation.log"
finished=0
finish() {
  code=$?
  if [[ $finished == 1 ]]; then return; fi
  finished=1
  printf '{"schema":"n83.spike39-input-generation-terminal/v1","exit_code":%s,"finished_utc":"%s","inputs_present":%s}\n' "$code" "$(date -u +%FT%TZ)" "$([[ -d "$INPUTS" ]] && echo true || echo false)" > "$HERE/GENERATION-TERMINAL.json"
  if [[ -d "$INPUTS" ]]; then aws s3 sync "$INPUTS/" "$S3INPUTS" --region us-east-2 --only-show-errors || true; fi
  aws s3 cp "$HERE/GENERATION-TERMINAL.json" "$S3ROOT/metadata/GENERATION-TERMINAL.json" --region us-east-2 --only-show-errors || true
  return "$code"
}
trap finish EXIT

[[ ! -e "$INPUTS" ]] || { echo 'candidate-inputs already exists; preserve it and stop' >&2; exit 2; }
[[ ! -e "$STAGE" ]] || { echo 'r39 staging path already exists; preserve it and use a fresh location' >&2; exit 2; }
[[ "$(sha256sum "$HERE/protocol-pdp-factorbase-v2.json" | cut -d' ' -f1)" == 8a9143e9588ce1ec5c204362b734f790696de90569ce1de3c086e4645e605885 ]]
[[ "$(sha256sum "$HERE/protocol-top-factorbase-v2.json" | cut -d' ' -f1)" == 1b0a27b19e5f32b78a23bd53a51e2a05cdc0c7a146c3d7a2269f93aa1679e3e5 ]]
[[ "$(sha256sum "$HERE/r39-base-s3-file-index.json" | cut -d' ' -f1)" == 0baee0b147a21fb2921d3fb897598c5596ce3defdd61e6321984ad677f052b30 ]]
[[ "$(sha256sum "$HERE/r39-base-index.json" | cut -d' ' -f1)" == 3efdc9f650310f97ec12897aaa28e0146fd09cc15769093f0b3db680c1c50232 ]]
[[ "$(sha256sum "$PDP" | cut -d' ' -f1)" == b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca ]]
[[ "$(sha256sum "$MATCHED" | cut -d' ' -f1)" == f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82 ]]
aws s3api list-objects-v2 --bucket isolab-results-104186811093-us-east-2 --prefix runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike39/attempt-01/inputs-v1/ --region us-east-2 --query KeyCount --output text | grep -qx None
aws s3api list-objects-v2 --bucket isolab-results-104186811093-us-east-2 --prefix runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike39/attempt-01/data-v1/ --region us-east-2 --query KeyCount --output text | grep -qx None
ps -eo args= | awk '($1 ~ /pdp$/ && $2 == "generate") || ($1 ~ /fall$/ && ($2 == "run" || $2 == "replay"))' | grep . && { echo 'matching generator/runner already active' >&2; exit 3; } || true

mkdir -p "$STAGE/data"
cp "$RECOVERY/restored/data/index.json" "$STAGE/data/index.json"
cp -a "$RECOVERY/restored/data/koblitz" "$RECOVERY/restored/data/binary" "$RECOVERY/restored/data/prime" "$STAGE/data/"
python3 "$HERE/stage-r39-bases.py" "$HERE/r39-base-s3-file-index.json" "$STAGE/data" > "$HERE/BASE-STAGING.json"
python3 - "$STAGE/data" "$HERE/r39-base-index.json" <<'PY'
import hashlib,json,sys
from pathlib import Path
root=Path(sys.argv[1]); expected=json.loads(Path(sys.argv[2]).read_text()); idx=json.loads((root/'index.json').read_text())
assert idx==expected and idx['candidate_id']=='spike39' and idx['complete'] is True and idx['candidate_x_count_per_curve']==134217728
assert len(list(root.rglob('*.zst')))==384
print(json.dumps({'candidate_id':idx['candidate_id'],'complete':idx['complete'],'candidate_x_count_per_curve':idx['candidate_x_count_per_curve'],'shards':384,'shard_bytes':sum(p.stat().st_size for p in root.rglob('*.zst')),'index_sha256':hashlib.sha256((root/'index.json').read_bytes()).hexdigest()}))
PY

export AWS_RETRY_MODE=standard AWS_MAX_ATTEMPTS=3 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
ulimit -v 8388608
mkdir -p "$HERE"
printf 'pdp generate %s %s %s spike39\n' "$INPUTS" "$STAGE/data" "$MATCHED" > "$HERE/GENERATION-COMMAND.txt"
taskset -c 0 timeout --signal=TERM --kill-after=30s 1200s "$PDP" generate "$INPUTS" "$STAGE/data" "$MATCHED" spike39 > "$RUN_LOG" 2>&1
aws s3 sync "$INPUTS/" "$S3INPUTS" --region us-east-2 --only-show-errors
finished=1
printf '{"schema":"n83.spike39-input-generation-terminal/v1","exit_code":0,"finished_utc":"%s","inputs_present":true}\n' "$(date -u +%FT%TZ)" > "$HERE/GENERATION-TERMINAL.json"
aws s3 cp "$HERE/GENERATION-TERMINAL.json" "$S3ROOT/metadata/GENERATION-TERMINAL.json" --region us-east-2 --only-show-errors
