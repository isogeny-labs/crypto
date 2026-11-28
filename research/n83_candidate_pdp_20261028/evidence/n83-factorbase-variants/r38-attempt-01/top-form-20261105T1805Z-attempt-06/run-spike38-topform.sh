#!/usr/bin/env bash
set -Eeuo pipefail
export AWS_RETRY_MODE=standard AWS_MAX_ATTEMPTS=1
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=/home/ec2-user/crypto-n83-degree-sweep-20261027
REC="$REPO/research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r38-attempt-01/terminal-recovery-20261101T2000Z"
BASE_LOCAL=/tmp/n83-candidate-bases/r38-attempt-06
INPUTS="$HERE/candidate-inputs"; CERTS="$HERE/top-certificates"; BIN="$HERE/instruments"
DEST=s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/attempt-06/results/
MATCHED="$REPO/research/n83_candidate_pdp_20261028/evidence/pdp/baseline/inputs.json"
EXPECTED_PROTOCOL=1b0a27b19e5f32b78a23bd53a51e2a05cdc0c7a146c3d7a2269f93aa1679e3e5
EXPECTED_INDEX=3fb72aa61147882832bbb17fc65a3ccfb8f5f179c047a43e4ccf65cecfa34623
EXPECTED_SHARD_INDEX=f990221ecd91d7a7a8bc3eadf89529aec5f06c7ed8fa3cad40f31dd245337765
EXPECTED_RECOVERY_AUDIT=ef4a4d3f7d42f55f3b7595369ab86dcc95d686ce796a7c80531b8b4ad3890d5a
EXPECTED_LIVE_INVENTORY=f2a6d3fddae48c0ddf6c83a00dae23c99c548d077aea97913b8f831697f4b97f
EXPECTED_PDP=b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca
EXPECTED_TOP=4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a
UPLOADER=
ulimit -v 8388608
STARTED=$(date -u +%FT%TZ)
error_trap(){ code=$?; line=${BASH_LINENO[0]:-$LINENO}; cmd=$BASH_COMMAND; trap - ERR; python3 - "$HERE" "$code" "$line" "$cmd" <<'PY'
import json,sys,datetime
from pathlib import Path
p=Path(sys.argv[1]);(p/'runner-error.json').write_text(json.dumps({'schema':'n83.runner-error/v1','exit_code':int(sys.argv[2]),'line':int(sys.argv[3]),'command':sys.argv[4],'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2)+'\n')
PY
 exit "$code"; }
finish(){ code=$?; trap - EXIT ERR; if [[ -n ${UPLOADER:-} ]]; then kill "$UPLOADER" 2>/dev/null || true; wait "$UPLOADER" 2>/dev/null || true; fi; jq -n --arg start "$STARTED" --arg end "$(date -u +%FT%TZ)" --argjson code "$code" '{schema:"n83.candidate-top-run-terminal/v1",candidate_id:"spike38",attempt:"06",started_at:$start,finished_at:$end,exit_code:$code,wall_valid:false}' > "$HERE/terminal.json"; exit "$code"; }
trap error_trap ERR
trap finish EXIT
upload(){ aws s3 sync "$HERE" "$DEST" --region us-east-2 --only-show-errors; }
[[ ! -e "$INPUTS" && ! -e "$CERTS" && ! -e "$BASE_LOCAL" ]]
for spec in "$HERE/source-pdp.rs:8e23a7287dc82afc967ef3f50e20c486f3f01d1e4a5f1847b5891b24df3b7629" "$HERE/source-top.rs:2b22d880eb68ba304c8cca67fec0694c8987f1523436ef728bfa9ad39a621e7e" "$BIN/pdp:$EXPECTED_PDP" "$BIN/top:$EXPECTED_TOP" "$HERE/factor-base-metadata/data/index.json:$EXPECTED_INDEX"; do f=${spec%:*}; h=${spec##*:}; printf '%s  %s\n' "$h" "$f" | sha256sum -c -; done
printf '%s  %s\n' "$EXPECTED_SHARD_INDEX" "$HERE/factor-base-metadata/s3-file-index.json" | sha256sum -c -
printf '%s  %s\n' "$EXPECTED_RECOVERY_AUDIT" "$HERE/factor-base-metadata/r38-recovery-audit.json" | sha256sum -c -
printf '%s  %s\n' "$EXPECTED_LIVE_INVENTORY" "$HERE/factor-base-metadata/s3-live-inventory.json" | sha256sum -c -
ACTUAL_PROTOCOL=$(sha256sum "$REPO/research/n83_candidate_pdp_20261028/protocol-top-factorbase-v2.json" | cut -d' ' -f1)
python3 - "$HERE/protocol-hash-check.json" "$EXPECTED_PROTOCOL" "$ACTUAL_PROTOCOL" <<'PYHASH'
import json,sys
json.dump({'schema':'n83.protocol-hash-check/v1','expected_sha256':sys.argv[2],'actual_sha256':sys.argv[3],'match':sys.argv[2]==sys.argv[3]},open(sys.argv[1],'w'),indent=2);open(sys.argv[1],'a').write('\n')
PYHASH
[[ "$ACTUAL_PROTOCOL" == "$EXPECTED_PROTOCOL" ]]
printf '%s  %s\n' f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82 "$MATCHED" | sha256sum -c -
aws s3api list-objects-v2 --bucket isolab-results-104186811093-us-east-2 --prefix runs/n83-pdp-top-factorbase-20261031-v2/spike38/attempt-06/results/ --region us-east-2 --output json > "$HERE/pre-run-results-s3-list.json" 2> "$HERE/pre-run-results-s3-list.stderr"
python3 - "$HERE/pre-run-results-s3-list.json" <<'PYEMPTY'
import json,sys
x=json.load(open(sys.argv[1])); assert x.get('KeyCount',0)==0 and not x.get('Contents'), 'attempt06 results prefix is not empty'
PYEMPTY
ps -eo pid=,args= | awk '($2 ~ /pdp$/ && $3 == "generate") || ($2 ~ /top$/ && ($3 == "single" || $3 == "single-replay"))' > "$HERE/pre-run-processes.txt"
[[ ! -s "$HERE/pre-run-processes.txt" ]] || { echo 'matching PDP/top process already active' >&2; exit 4; }
python3 - "$HERE/factor-base-metadata/s3-file-index.json" <<'PY'
import json,sys
x=json.load(open(sys.argv[1]));files=[f for f in x['files'] if f['path'].endswith('.zst')];assert len(files)==384 and sum(f['bytes'] for f in files)==4576000923
PY
mkdir -p "$BASE_LOCAL/data/koblitz" "$BASE_LOCAL/data/binary" "$BASE_LOCAL/data/prime"
cp "$HERE/factor-base-metadata/data/index.json" "$BASE_LOCAL/data/index.json"
# The recovered JSON part files are selector metadata; the 384 compressed shards are fetched and hash-checked below.
cp -a "$HERE/factor-base-metadata/data/koblitz" "$HERE/factor-base-metadata/data/binary" "$HERE/factor-base-metadata/data/prime" "$BASE_LOCAL/data/"
python3 - "$HERE/factor-base-metadata/s3-file-index.json" "$BASE_LOCAL/data" "$HERE" <<'PY'
import concurrent.futures,hashlib,json,subprocess,sys
from pathlib import Path
index=json.load(open(sys.argv[1]));root=Path(sys.argv[2]);attempt=Path(sys.argv[3]);files=[f for f in index['files'] if f['path'].endswith('.zst')];assert len(files)==384

def fetch(f):
 out=root/f['path'].removeprefix('data/');out.parent.mkdir(parents=True,exist_ok=True)
 try: subprocess.run(['aws','s3','cp',f['snapshot_key'],str(out),'--region','us-east-2','--only-show-errors'],check=True)
 except Exception as exc:
  (attempt/'shard-download-error.json').write_text(json.dumps({'schema':'n83.shard-download-error/v1','path':f['path'],'snapshot_key':f['snapshot_key'],'exception':repr(exc)},indent=2)+'\n'); raise
 h=hashlib.sha256(out.read_bytes()).hexdigest()
 if out.stat().st_size!=f['bytes'] or h!=f['sha256']:raise RuntimeError('size/hash mismatch '+f['path'])
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:list(pool.map(fetch,files))
print(json.dumps({'verified_shards':len(files),'bytes':sum(f['bytes'] for f in files)}),flush=True)
PY
mkdir -p "$HERE/instruments" "$INPUTS" "$CERTS"
upload
(while sleep 30; do upload; done) & UPLOADER=$!
timeout --signal=TERM --kill-after=30 12400 "$BIN/pdp" generate "$INPUTS" "$BASE_LOCAL/data" "$MATCHED" spike38 > "$HERE/generate.log" 2>&1
timeout --signal=TERM --kill-after=30 12400 "$BIN/top" single "$INPUTS" "$CERTS" > "$HERE/top-single.log" 2>&1
timeout --signal=TERM --kill-after=30 12400 "$BIN/top" single-replay "$INPUTS" "$CERTS" > "$HERE/top-replay.log" 2>&1
