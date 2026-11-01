#!/usr/bin/env bash
set -Eeuo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(git -C "$HERE" rev-parse --show-toplevel)
SRC=/tmp/n83-r38-source-52665
BIN=/tmp/n83-r38-target/release
BASE_RECOVERY="$REPO/research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r39-attempt-01/terminal-recovery-20261101T2020Z"
BASE_LOCAL=/home/ec2-user/n83-candidate-bases/r39
INPUTS="$HERE/candidate-inputs"
CERTS="$HERE/top-certificates"
DEST=s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike39/
MATCHED="$REPO/research/n83_candidate_pdp_20261028/evidence/pdp/baseline/inputs.json"
EXPECTED_PROTOCOL=1b0a27b19e5f32b78a23bd53a51e2a05cdc0c7a146c3d7a2269f93aa1679e3e5
EXPECTED_SOURCE=52665c7a6e8308f86201e10c9305afa346fd134b
EXPECTED_PDP=b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca
EXPECTED_TOP=4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a
STARTED=$(date -u +%FT%TZ)

upload() { aws s3 sync "$HERE" "$DEST" --region us-east-2 --only-show-errors; }
record_failure() {
  local code=$1 command=$2
  jq -n --arg command "$command" --argjson code "$code" \
    '{schema:"n83.candidate-top-run-failure/v1",candidate_id:"spike39",exit_code:$code,failing_command:$command}' > "$HERE/failure.json"
}
on_error() {
  local code=$? command=$BASH_COMMAND
  trap - ERR
  record_failure "$code" "$command"
  exit "$code"
}
finish() {
  local code=$?
  trap - EXIT ERR
  if [[ -n ${UPLOADER:-} ]]; then kill "$UPLOADER" 2>/dev/null || true; wait "$UPLOADER" 2>/dev/null || true; fi
  jq -n --arg start "$STARTED" --arg end "$(date -u +%FT%TZ)" --argjson code "$code" \
    '{schema:"n83.candidate-top-run-terminal/v1",candidate_id:"spike39",started_at:$start,finished_at:$end,exit_code:$code,wall_valid:false}' > "$HERE/terminal.json"
  (cd "$HERE" && find . -type f ! -name manifest.sha256 ! -name S3_PERSISTENCE_INDEX.json -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
  python3 - "$HERE" "$DEST" <<'PY'
import hashlib,json,sys
from pathlib import Path
root=Path(sys.argv[1]); dest=sys.argv[2]; files=[]
for p in sorted(root.rglob('*')):
    if not p.is_file() or p.name=='S3_PERSISTENCE_INDEX.json': continue
    b=p.read_bytes(); rel=p.relative_to(root).as_posix()
    files.append({'path':rel,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'s3_uri':dest+rel})
(root/'S3_PERSISTENCE_INDEX.json').write_text(json.dumps({'schema':'n83.candidate-top-output-index/v1','candidate':'spike39','files':files},indent=2)+'\n')
PY
  upload || exit 91
  exit "$code"
}
trap on_error ERR
trap finish EXIT
UPLOADER=

[[ ! -e "$INPUTS" && ! -e "$CERTS" ]] || { record_failure 2 'candidate output path already exists'; exit 2; }
[[ ! -e "$BASE_LOCAL" ]] || { record_failure 2 'persistent staging path already exists'; exit 2; }
[[ "$(git -C "$SRC" rev-parse HEAD)" == "$EXPECTED_SOURCE" ]]
[[ "$(sha256sum "$REPO/research/n83_candidate_pdp_20261028/protocol-top-factorbase-v2.json" | cut -d' ' -f1)" == "$EXPECTED_PROTOCOL" ]]
[[ "$(sha256sum "$BIN/pdp" | cut -d' ' -f1)" == "$EXPECTED_PDP" ]]
[[ "$(sha256sum "$BIN/top" | cut -d' ' -f1)" == "$EXPECTED_TOP" ]]
printf '%s  %s\n' f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82 "$MATCHED" | sha256sum -c -
printf '%s  %s\n' 0baee0b147a21fb2921d3fb897598c5596ce3defdd61e6321984ad677f052b30 "$BASE_RECOVERY/s3-file-index.json" | sha256sum -c -
printf '%s  %s\n' 3efdc9f650310f97ec12897aaa28e0146fd09cc15769093f0b3db680c1c50232 "$BASE_RECOVERY/restored/data/index.json" | sha256sum -c -
aws s3 ls "$DEST" --recursive --region us-east-2 > "$HERE/pre-run-s3-list.txt" 2> "$HERE/pre-run-s3-stderr.txt"
[[ ! -s "$HERE/pre-run-s3-list.txt" ]] || { record_failure 3 'candidate S3 output prefix is not empty'; exit 3; }
ps -eo pid=,args= | awk '($2 ~ /pdp$/ && $3 == "generate") || ($2 ~ /top$/ && ($3 == "single" || $3 == "single-replay"))' > "$HERE/pre-run-processes.txt"
[[ ! -s "$HERE/pre-run-processes.txt" ]] || { record_failure 4 'candidate PDP/top process already active'; exit 4; }
mkdir -p "$BASE_LOCAL/data"
cp "$BASE_RECOVERY/restored/data/index.json" "$BASE_LOCAL/data/index.json"
cp -a "$BASE_RECOVERY/restored/data/koblitz" "$BASE_LOCAL/data/"
cp -a "$BASE_RECOVERY/restored/data/binary" "$BASE_LOCAL/data/"
cp -a "$BASE_RECOVERY/restored/data/prime" "$BASE_LOCAL/data/"
python3 "$HERE/stage_r39_bases_from_s3.py" "$BASE_RECOVERY/s3-file-index.json" "$BASE_LOCAL/data"
mkdir -p "$HERE/instruments" "$INPUTS" "$CERTS"
cp "$BIN/pdp" "$BIN/top" "$HERE/instruments/"
cp "$REPO/research/n83_candidate_pdp_20261028/protocol-top-factorbase-v2.json" "$HERE/"
uname -a > "$HERE/host.txt"; lscpu >> "$HERE/host.txt"
upload
(while sleep 30; do upload; done) &
UPLOADER=$!
ulimit -v 8388608
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
timeout --signal=TERM --kill-after=30 86400 "$BIN/pdp" generate "$INPUTS" "$BASE_LOCAL/data" "$MATCHED" spike39 > "$HERE/generate.log" 2>&1
timeout --signal=TERM --kill-after=30 86400 "$BIN/top" single "$INPUTS" "$CERTS" > "$HERE/top-single.log" 2>&1
timeout --signal=TERM --kill-after=30 86400 "$BIN/top" single-replay "$INPUTS" "$CERTS" > "$HERE/top-replay.log" 2>&1
