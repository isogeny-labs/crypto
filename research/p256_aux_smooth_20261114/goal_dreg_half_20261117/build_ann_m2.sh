#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_ann_m2.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
# Hash checks and format conversion only; no Python finite-field arithmetic.
python3 - "$study_dir" "$build_dir/cache.txt" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);repo=p.parents[2]
manifest=json.loads((p/'PACKAGE_HASHES.json').read_text())['files']
for name,digest in manifest.items():
    assert hashlib.sha256((repo/name).read_bytes()).hexdigest()==digest,name
b=p/'evidence/retained-depth8';a=p/'evidence/complete-law-depth8';c=p/'evidence/bound-assembly'
cert=json.loads((b/'certificate.json').read_text());summary=json.loads((b/'manifest.json').read_text())
assert summary['factor_base']=='FB1h16882188194e'
assert hashlib.sha256((b/'certificate.json').read_bytes()).hexdigest()==summary['certificate_sha256']
refs=json.loads((c/'complete-m2-reference.json').read_text())
ref=next(r for r in refs if r['kind']=='planted')
assert ref['tuples_checked']==82944 and ref['relations']==[[193,262],[262,193]]
assert ref['false_positives']==ref['false_negatives']==0
cell=next(r for r in json.loads((a/'summary.json').read_text())['cells'] if r['cell']=='m2-planted')
line=a/'m2-planted-line.json';circuit=a/'m2-planted-circuit.json'
assert hashlib.sha256(circuit.read_bytes()).hexdigest()==cell['circuit_sha256']
v=cert['variables'];out=['VARS '+str(len(v))+' '+' '.join(v),'POINTS '+str(len(cert['points']))]
out.extend(' '.join(point[x] for x in v) for point in cert['points'])
def mono(powers):
    return str(len(powers))+' '+' '.join(k+' '+str(e) for k,e in powers.items())
out.append('CHAIN_STANDARD '+str(len(cert['standard_monomials'])))
for monomial in cert['standard_monomials']:
    assert len(monomial)==1 and monomial[0]['coefficient']=='1'
    out.append(mono(monomial[0]['powers']))
out.append('CHAIN_GENERATORS '+str(len(cert['affine_generators'])))
for g in cert['affine_generators']:
    out.append(str(len(g)))
    out.extend(t['coefficient']+' '+mono(t['powers']) for t in g)
out.extend(['CHAIN_SHA '+summary['certificate_sha256'],'RELATIONS 2','193 262','262 193'])
target=cell['target'];out.append('TARGET '+str(int(target['infinity']))+' '+target['x']+' '+target['y'])
out.extend(['GROUP_DEGREE '+str(cell['relation_polynomial_degree_upper']),'DENSE_R 97',
    'CIRCUIT_SHA '+cell['circuit_sha256'],'LINE_SHA '+hashlib.sha256(line.read_bytes()).hexdigest(),'END'])
Path(sys.argv[2]).write_text('\n'.join(out)+'\n')
files=[p/'PACKAGE_HASHES.json',b/'certificate.json',b/'manifest.json',c/'complete-m2-reference.json',line,circuit]
Path(sys.argv[2]+'.bindings.json').write_text(json.dumps({str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in files},sort_keys=True,indent=2)+'\n')
PY
for name in retained_basis retained_depth8; do
  [[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/$name.cpp") == 1 ]]
done
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_basis.cpp" > "$build_dir/retained_prefix.inc"
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_depth8.cpp" > "$build_dir/retained_depth8_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/ann_m2.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/ann_m2.cpp" "$study_dir/retained_basis.cpp" "$study_dir/retained_depth8.cpp" "$build_dir/cache.txt" > "$build_dir/input-hashes.txt"
