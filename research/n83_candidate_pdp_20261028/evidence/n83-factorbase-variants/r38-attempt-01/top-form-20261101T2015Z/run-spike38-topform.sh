#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(git -C "$HERE" rev-parse --show-toplevel)
SRC=/tmp/n83-r38-source-52665
BIN=/tmp/n83-r38-target/release
BASE_RECOVERY="$REPO/research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r38-attempt-01/terminal-recovery-20261101T2000Z"
BASE_LOCAL=/tmp/n83-candidate-bases/r38
INPUTS="$HERE/candidate-inputs"
CERTS="$HERE/top-certificates"
DEST=s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/
MATCHED="$REPO/research/n83_candidate_pdp_20261028/evidence/pdp/baseline/inputs.json"
EXPECTED_PROTOCOL=1b0a27b19e5f32b78a23bd53a51e2a05cdc0c7a146c3d7a2269f93aa1679e3e5
EXPECTED_SOURCE=52665c7a6e8308f86201e10c9305afa346fd134b
EXPECTED_PDP=b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca
EXPECTED_TOP=4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a
UPLOADER=
STARTED=$(date -u +%FT%TZ)

upload() { aws s3 sync "$HERE" "$DEST" --region us-east-2 --only-show-errors; }
finish() {
  code=$?
  trap - EXIT
  if [[ -n ${UPLOADER:-} ]]; then kill "$UPLOADER" 2>/dev/null || true; wait "$UPLOADER" 2>/dev/null || true; fi
  jq -n --arg start "$STARTED" --arg end "$(date -u +%FT%TZ)" --argjson code "$code" \
    '{schema:"n83.candidate-top-run-terminal/v1",candidate_id:"spike38",started_at:$start,finished_at:$end,exit_code:$code,wall_valid:false}' > "$HERE/terminal.json"
  (cd "$HERE" && find . -type f ! -name manifest.sha256 ! -name S3_PERSISTENCE_INDEX.json -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
  python3 - "$HERE" "$DEST" <<'PY'
import hashlib,json,sys
from pathlib import Path
root=Path(sys.argv[1]); dest=sys.argv[2]
files=[]
for p in sorted(root.rglob('*')):
    if not p.is_file() or p.name=='S3_PERSISTENCE_INDEX.json': continue
    b=p.read_bytes(); rel=p.relative_to(root).as_posix()
    files.append({'path':rel,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'s3_uri':dest+rel})
(root/'S3_PERSISTENCE_INDEX.json').write_text(json.dumps({'schema':'n83.candidate-top-output-index/v1','candidate':'spike38','files':files},indent=2)+'\n')
PY
  upload || exit 91
  exit "$code"
}
trap finish EXIT

[[ ! -e "$INPUTS" && ! -e "$CERTS" ]] || { echo 'fresh r38 output paths required' >&2; exit 2; }
[[ "$(git -C "$SRC" rev-parse HEAD)" == "$EXPECTED_SOURCE" ]]
[[ "$(sha256sum "$REPO/research/n83_candidate_pdp_20261028/protocol-top-factorbase-v2.json" | cut -d' ' -f1)" == "$EXPECTED_PROTOCOL" ]]
[[ "$(sha256sum "$BIN/pdp" | cut -d' ' -f1)" == "$EXPECTED_PDP" ]]
[[ "$(sha256sum "$BIN/top" | cut -d' ' -f1)" == "$EXPECTED_TOP" ]]
printf '%s  %s\n' f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82 "$MATCHED" | sha256sum -c -
aws s3 ls "$DEST" --recursive > "$HERE/pre-run-s3-list.txt"
[[ ! -s "$HERE/pre-run-s3-list.txt" ]] || { echo 'candidate output prefix is not empty; refusing duplicate run' >&2; exit 3; }
ps -eo pid=,args= | awk '($2 ~ /pdp$/ && $3 == "generate") || ($2 ~ /top$/ && ($3 == "single" || $3 == "single-replay"))' > "$HERE/pre-run-processes.txt"
[[ ! -s "$HERE/pre-run-processes.txt" ]] || { echo 'candidate PDP/top process already active; refusing duplicate run' >&2; exit 4; }

mkdir -p "$BASE_LOCAL/data"
cp "$BASE_RECOVERY/restored/data/index.json" "$BASE_LOCAL/data/index.json"
cp -a "$BASE_RECOVERY/restored/data/koblitz" "$BASE_LOCAL/data/"
cp -a "$BASE_RECOVERY/restored/data/binary" "$BASE_LOCAL/data/"
cp -a "$BASE_RECOVERY/restored/data/prime" "$BASE_LOCAL/data/"
python3 - "$BASE_RECOVERY/s3-file-index.json" "$BASE_LOCAL/data" <<'PY'
import concurrent.futures,hashlib,json,subprocess,sys
from pathlib import Path
index=json.load(open(sys.argv[1])); root=Path(sys.argv[2])
files=[f for f in index['files'] if f['path'].endswith('.zst')]
assert len(files)==384
def fetch(f):
    out=root/f['path'].removeprefix('data/'); out.parent.mkdir(parents=True,exist_ok=True)
    subprocess.run(['aws','s3','cp',f['snapshot_key'],str(out),'--region','us-east-2','--only-show-errors'],check=True)
    h=hashlib.sha256(out.read_bytes()).hexdigest()
    if out.stat().st_size!=f['bytes'] or h!=f['sha256']: raise RuntimeError(f"hash/size mismatch: {f['path']}")
with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:
    list(pool.map(fetch,files))
print(json.dumps({'verified_shards':len(files),'bytes':sum(f['bytes'] for f in files)}))
PY

mkdir -p "$HERE/instruments" "$INPUTS" "$CERTS"
cp "$BIN/pdp" "$BIN/top" "$HERE/instruments/"
cp "$REPO/research/n83_candidate_pdp_20261028/protocol-top-factorbase-v2.json" "$HERE/"
uname -a > "$HERE/host.txt"; lscpu >> "$HERE/host.txt"
upload
(while sleep 30; do upload; done) &
UPLOADER=$!
ulimit -v 8388608
timeout --signal=TERM --kill-after=30 86400 "$BIN/pdp" generate "$INPUTS" "$BASE_LOCAL/data" "$MATCHED" spike38 > "$HERE/generate.log" 2>&1
timeout --signal=TERM --kill-after=30 86400 "$BIN/top" single "$INPUTS" "$CERTS" > "$HERE/top-single.log" 2>&1
timeout --signal=TERM --kill-after=30 86400 "$BIN/top" single-replay "$INPUTS" "$CERTS" > "$HERE/top-replay.log" 2>&1

