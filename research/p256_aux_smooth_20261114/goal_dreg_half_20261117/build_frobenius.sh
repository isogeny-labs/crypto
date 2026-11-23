#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_frobenius.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
python3 - "$study_dir" "$build_dir/cache.txt" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);q=p/'evidence/planted-annihilator';a=p/'evidence/complete-law-depth8'
for manifest_name in ('PACKAGE_HASHES.json','PLANTED_PACKAGE_HASHES.json'):
    data=json.loads((p/manifest_name).read_text())['files']
    for name,digest in data.items():
        f=p/name if manifest_name.startswith('PLANTED') else p.parents[2]/name
        assert hashlib.sha256(f.read_bytes()).hexdigest()==digest,name
cert=json.loads((q/'dense-membership.json').read_text());ann=json.loads((q/'manifest.json').read_text())
assert hashlib.sha256((q/'dense-membership.json').read_bytes()).hexdigest()==ann['dense_membership_certificate_sha256']
chain=json.loads((p/'evidence/retained-depth8/certificate.json').read_text())
v=cert['variables'];out=['VARS 2 '+' '.join(v),'POINTS 288']
out.extend(' '.join(point[x] for x in v) for point in cert['points'])
def mono(powers): return str(len(powers))+' '+' '.join(k+' '+str(e) for k,e in powers.items())
def poly(g):
    out.append(str(len(g)));out.extend(t['coefficient']+' '+mono(t['powers']) for t in g)
out.append('STANDARD '+str(len(cert['standard_monomials'])))
for m in cert['standard_monomials']:
    assert len(m)==1 and m[0]['coefficient']=='1';out.append(mono(m[0]['powers']))
out.append('GENERATORS '+str(len(cert['affine_generators'])))
for g in cert['affine_generators']:poly(g)
out.append('CERT_SHA '+ann['dense_membership_certificate_sha256'])
h=chain['hilbert_numerator_coefficients'];out.append('CHAIN_H '+str(len(h))+' '+' '.join(map(str,h)))
no2=json.loads((a/'no-two-torsion.json').read_text());out.append('NO_TWO 4')
for key in ('cubic','x_to_p_mod_cubic','bezout_cubic_multiplier','bezout_residue_minus_x_multiplier'):poly(no2[key])
cells=json.loads((a/'summary.json').read_text())['cells'];out.append('CELLS 12')
bindings={}
for c in cells:
    lf=a/(c['cell']+'-line.json');cf=a/(c['cell']+'-circuit.json')
    assert hashlib.sha256(cf.read_bytes()).hexdigest()==c['circuit_sha256']
    l=json.loads(lf.read_text());graph=json.loads(cf.read_text());t=c['target'];assert not t['infinity']
    lh=hashlib.sha256(lf.read_bytes()).hexdigest()
    out.append(' '.join([str(c['m']),c['kind'],str(c['relation_polynomial_degree_upper']),t['x'],t['y'],*l['coefficients'],l['parameter'],c['circuit_sha256'],lh]))
    out.append('DAG '+str(len(graph['nodes']))+' '+' '.join(map(str,[*graph['sum_outputs'],graph['relation_output']])))
    for n in graph['nodes']:
        out.append(' '.join([n['op'],n['name'] or '-',n['value'],str(n['a']),str(n['b']),str(n['degree_upper'])]))
    for f in (lf,cf):bindings[str(f.relative_to(p))]=hashlib.sha256(f.read_bytes()).hexdigest()
out.append('END');Path(sys.argv[2]).write_text('\n'.join(out)+'\n')
for f in (q/'dense-membership.json',q/'manifest.json',a/'no-two-torsion.json',p/'PACKAGE_HASHES.json',p/'PLANTED_PACKAGE_HASHES.json'):
    bindings[str(f.relative_to(p))]=hashlib.sha256(f.read_bytes()).hexdigest()
Path(sys.argv[2]+'.bindings.json').write_text(json.dumps(bindings,sort_keys=True,indent=2)+'\n')
PY
for name in retained_basis retained_depth8 ann_m2; do
  [[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/$name.cpp") == 1 ]]
done
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_basis.cpp" > "$build_dir/retained_prefix.inc"
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_depth8.cpp" > "$build_dir/retained_depth8_prefix.inc"
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/ann_m2.cpp" > "$build_dir/ann_m2_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/frobenius.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/frobenius.cpp" "$study_dir/ann_m2.cpp" "$build_dir/cache.txt" > "$build_dir/input-hashes.txt"
