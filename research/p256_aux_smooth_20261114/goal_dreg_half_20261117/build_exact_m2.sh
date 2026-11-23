#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_exact_m2.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
python3 - "$study_dir" "$build_dir/cache.txt" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);a=p/'evidence/complete-law-depth8';b=p/'evidence/retained-depth8';c=p/'evidence/bound-assembly'
manifest=json.loads((a/'artifact-hashes.json').read_text())['files']
for name in ('dense-CI.json','summary.json','factor-base.json'):
    assert hashlib.sha256((a/name).read_bytes()).hexdigest()==manifest[name]
retained=json.loads((b/'manifest.json').read_text())
assert hashlib.sha256((b/'certificate.json').read_bytes()).hexdigest()==retained['certificate_sha256']
for directory in (a,b,c):
    assert (directory/'primary.exit').read_text().strip()=='0'
    assert (directory/'replay.exit').read_text().strip()=='0'
    assert not (directory/'byte-replay.log').read_bytes()
ref=json.loads((c/'complete-m2-reference.json').read_text())
unplanted=next(r for r in ref if r['kind']=='unplanted')
assert unplanted['tuples_checked']==82944 and not unplanted['relations']
assert unplanted['false_positives']==unplanted['false_negatives']==0
d=json.loads((a/'dense-CI.json').read_text());cert=json.loads((b/'certificate.json').read_text())
cell=next(r for r in json.loads((a/'summary.json').read_text())['cells'] if r['cell']=='m2-unplanted')
assert d['applicable'] and d['CI_degrees']==[3,96] and cert['quotient_module_regularity']==5
h=cert['hilbert_numerator_coefficients']
Path(sys.argv[2]).write_text('POINTS 288\nDENSE_DEGREES 3 96\nCHAIN_H '+str(len(h))+' '+' '.join(map(str,h))+'\nGROUP_DEGREE '+str(cell['relation_polynomial_degree_upper'])+'\nREFERENCE 82944 0\nEND\n')
files=[a/'dense-CI.json',b/'certificate.json',c/'complete-m2-reference.json',a/'m2-unplanted-circuit.json',a/'m2-unplanted-line.json']
bindings={str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in files}
Path(sys.argv[2]+'.bindings.json').write_text(json.dumps(bindings,sort_keys=True,indent=2)+'\n')
PY
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/exact_m2.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
