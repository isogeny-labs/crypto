#!/bin/sh
set -eu
ROOT=/opt/n83-radix-neural
OUT=/work/results
EXPECTED_MANIFEST=5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78
EXPECTED_BINARY=c363e53c2db69630714a13169253fabdff76272f9391c00094074dbb4f8dda83
EXPECTED_SOURCE=a94eba7295c755a1ebf2cc1452704992de8965e1e8c95cec75ae14852d9b1b5c
mkdir -p "$OUT"
PROTOCOL_SHA=${1:-}
if [ "${1:-}" != "--validate-only" ]; then
  : "${PROTOCOL_SHA:?pass the frozen protocol sha256 as argv[1]}"
fi
python3 - "$ROOT" "$EXPECTED_MANIFEST" "$OUT" <<'PY'
import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); expected=sys.argv[2]
manifest_bytes=(root/'manifest.json').read_bytes()
actual=hashlib.sha256(manifest_bytes).hexdigest()
assert actual==expected,(actual,expected)
items=json.loads(manifest_bytes)
cases=[x for x in items if x['path'].startswith('cases/') and x['path'].endswith('.json')]
assert len(cases)==3010,len(cases)
seen=set()
for x in cases:
 p=x['path']; assert p not in seen,p; seen.add(p)
 b=(root/p).read_bytes()
 assert len(b)==x['bytes'],(p,len(b),x['bytes'])
 assert hashlib.sha256(b).hexdigest()==x['sha256'],p
assert len(list((root/'cases').glob('*.json')))==3010
(pathlib.Path(sys.argv[3])/'input-validation.json').write_text(json.dumps({'manifest_sha256':actual,'case_count':len(cases),'hashes_verified':len(cases)},indent=2)+'\n')
PY
printf '%s  %s\n' "$EXPECTED_BINARY" /usr/local/bin/radix_neural_probe | sha256sum -c -
if [ "${1:-}" = "--validate-only" ]; then
  cat "$OUT/input-validation.json"
  exit 0
fi
: "${ISOLAB_RESULTS_URI:?missing ISOLAB_RESULTS_URI}"
/usr/local/bin/radix_neural_probe "$ROOT/cases" "$OUT"
python3 - "$OUT" "$PROTOCOL_SHA" <<'PY'
import hashlib,json,pathlib,sys
out=pathlib.Path(sys.argv[1])
files={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.iterdir()) if p.is_file() and p.name!='run-manifest.json'}
record={'schema':'n83.radix-neural-run/v1','protocol_sha256':sys.argv[2],'input_manifest_sha256':'5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78','case_count':3010,'case_sha256_verified':3010,'binary_sha256':'c363e53c2db69630714a13169253fabdff76272f9391c00094074dbb4f8dda83','source_sha256':'a94eba7295c755a1ebf2cc1452704992de8965e1e8c95cec75ae14852d9b1b5c','files':files}
(out/'run-manifest.json').write_text(json.dumps(record,indent=2)+'\n')
PY
aws s3 sync "$OUT" "$ISOLAB_RESULTS_URI" --region us-east-2 --only-show-errors
