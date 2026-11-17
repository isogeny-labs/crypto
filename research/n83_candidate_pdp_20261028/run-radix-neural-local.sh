#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
cases=$1
manifest=$2
out=$3
protocol_sha=$4
dest=$5
binary="$study/target/release/radix_neural_probe"
expected_manifest=5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78
expected_binary=c363e53c2db69630714a13169253fabdff76272f9391c00094074dbb4f8dda83
expected_source=a94eba7295c755a1ebf2cc1452704992de8965e1e8c95cec75ae14852d9b1b5c
mkdir -p "$out"
python3 - "$cases" "$manifest" "$out" "$expected_manifest" <<'PYCHK'
import hashlib,json,pathlib,sys
cases=pathlib.Path(sys.argv[1]); manifest=pathlib.Path(sys.argv[2]); out=pathlib.Path(sys.argv[3]); expected=sys.argv[4]
b=manifest.read_bytes(); actual=hashlib.sha256(b).hexdigest(); assert actual==expected,(actual,expected)
items=json.loads(b); rows=[x for x in items if x['path'].startswith('cases/') and x['path'].endswith('.json')]
assert len(rows)==3010,len(rows)
for x in rows:
 p=cases/pathlib.Path(x['path']).name; data=p.read_bytes()
 assert len(data)==x['bytes'],p
 assert hashlib.sha256(data).hexdigest()==x['sha256'],p
assert len(list(cases.glob('*.json')))==3010
(out/'input-validation.json').write_text(json.dumps({'manifest_sha256':actual,'case_count':3010,'hashes_verified':3010},indent=2)+'\n')
PYCHK
printf '%s  %s\n' "$expected_binary" "$binary" | sha256sum -c -
sha256sum "$binary" > "$out/binary.sha256"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
printf 'command=%s %s %s\n' "$binary" "$cases" "$out" > "$out/command.txt"
started=$(date -u +%FT%TZ)
"$binary" "$cases" "$out"
python3 - "$out" "$protocol_sha" "$started" "$expected_binary" "$expected_source" <<'PYCHK'
import hashlib,json,pathlib,sys,datetime
out=pathlib.Path(sys.argv[1]); sha=sys.argv[2]; start=sys.argv[3]
files={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.iterdir()) if p.is_file() and p.name!='run-manifest.json'}
record={'schema':'n83.radix-neural-run/v1','run_id':'n83-radix-neural-20261031-local-r3','backend':'local-existing-host','protocol_sha256':sha,'started_at':start,'finished_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'input_manifest_sha256':'5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78','case_count':3010,'case_sha256_verified':3010,'binary_sha256':sys.argv[4],'source_sha256':sys.argv[5],'files':files}
(out/'run-manifest.json').write_text(json.dumps(record,indent=2)+'\n')
PYCHK
aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors
