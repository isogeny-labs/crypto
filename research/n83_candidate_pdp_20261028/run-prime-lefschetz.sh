#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
out=$2
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out/inputs" "$out/source"
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-prime-lefschetz-20261030-r1/local/
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 python3 - "$out" "$code" "$started" <<'PY'
from pathlib import Path
import json,sys,datetime
r=Path(sys.argv[1]);phases={}
for name in ['source-producer','certificate-producer','source-replay','certificate-replay']:
 p=r/(name+'-status.json');phases[name]=json.loads(p.read_text()) if p.exists() else {'started':False,'exit_code':125}
d={'exit_code':int(sys.argv[2]),'started_at':sys.argv[3],'finished_at':datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),'backend':'existing_host','wall_valid':False,'phases':phases}
(r/'terminal.json').write_text(json.dumps(d,indent=2)+'\n')
PY
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-prime-lefschetz.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol-prime-lefschetz.json" "$out/protocol-prime-lefschetz.json"
while IFS=$'\t' read -r name hash;do
 printf '%s  %s\n' "$hash" "$study/$name" | sha256sum -c -
 mkdir -p "$out/source/$(dirname "$name")"
 cp "$study/$name" "$out/source/$name"
done < <(jq -r '.extensions.source_files[]|[.path,.sha256]|@tsv' "$study/protocol-prime-lefschetz.json")
cp "$study/evidence/prime-terminal/summary.json" "$out/upper-summary.json"
while IFS=$'\t' read -r key source inputhash systemhash;do
 mkdir "$out/inputs/$key"
 printf '%s  %s\n' "$inputhash" "$source/input.json" "$systemhash" "$source/system.sing" | sha256sum -c -
 cp "$source/input.json" "$source/system.sing" "$out/inputs/$key/"
done < <(jq -r '.extensions.cases[]|[.key,.directory,.input_sha256,.source_sha256]|@tsv' "$study/protocol-prime-lefschetz.json")
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
export N83_LEFSCHETZ_FROZEN_PRODUCTION=1
phase(){
 name=$1;shift
 set +e
 (ulimit -v 2097152;timeout --signal=TERM --kill-after=5 120 "$@") > "$out/$name.log" 2>&1
 code=$?
 set -e
 printf '{"started":true,"exit_code":%s}\n' "$code" > "$out/$name-status.json"
 upload
 [[ "$code" == 0 ]]
}
phase source-producer "$out/instrument" run "$out" "$out/protocol-prime-lefschetz.json" "$out/source-audit"
phase certificate-producer python3 -B "$out/source/prime_lefschetz_certificate.py" run "$out" "$out/protocol-prime-lefschetz.json" "$out/results"
phase source-replay "$out/instrument" replay "$out" "$out/protocol-prime-lefschetz.json" "$out/source-audit"
phase certificate-replay python3 -B "$out/source/prime_lefschetz_certificate.py" replay "$out" "$out/protocol-prime-lefschetz.json" "$out/results"
