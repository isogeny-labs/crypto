#!/bin/sh
set -eu
ROOT=/opt/n83-radix-neural
OUT=/work/results
EXPECTED_MANIFEST=5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78
EXPECTED_BINARY=1957a04d748ad087c2ca2e1cd6c63b9c4bc57e5a557dd4140282f68f2fff6bd7
EXPECTED_SOURCE=937110e442b09ab3d4a1eeb5d469e9598992334fc0d2d1c56738c9ddb5e26366
mkdir -p "$OUT"
PROTOCOL_SHA=${1:-}
if [ "${1:-}" != "--validate-only" ]; then
  : "${PROTOCOL_SHA:?pass the frozen protocol sha256 as argv[1]}"
fi
python3 - "$ROOT" "$EXPECTED_MANIFEST" <<'PY'
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
(root.parent/'input-validation.json').write_text(json.dumps({'manifest_sha256':actual,'case_count':len(cases),'hashes_verified':len(cases)},indent=2)+'\n')
PY
printf '%s  %s\n' "$EXPECTED_BINARY" /usr/local/bin/radix_neural_probe | sha256sum -c -
if [ "${1:-}" = "--validate-only" ]; then
  cat "$ROOT/../input-validation.json"
  exit 0
fi
: "${ISOLAB_RESULTS_URI:?missing ISOLAB_RESULTS_URI}"
/usr/local/bin/radix_neural_probe "$ROOT/cases" "$OUT"
python3 - "$OUT" "$PROTOCOL_SHA" <<'PY'
import hashlib,json,pathlib,sys
out=pathlib.Path(sys.argv[1])
files={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.iterdir()) if p.is_file() and p.name!='run-manifest.json'}
record={'schema':'n83.radix-neural-run/v1','protocol_sha256':sys.argv[2],'input_manifest_sha256':'5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78','case_count':3010,'case_sha256_verified':3010,'binary_sha256':'1957a04d748ad087c2ca2e1cd6c63b9c4bc57e5a557dd4140282f68f2fff6bd7','source_sha256':'937110e442b09ab3d4a1eeb5d469e9598992334fc0d2d1c56738c9ddb5e26366','files':files}
(out/'run-manifest.json').write_text(json.dumps(record,indent=2)+'\n')
PY
aws s3 sync "$OUT" "$ISOLAB_RESULTS_URI" --region us-east-2 --only-show-errors
