#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_bound_assembly.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
python3 - "$study_dir" "$build_dir/cache.txt" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]);q=p/'evidence/complete-law-depth8';r=p/'evidence/retained-depth8'
expected=json.loads((q/'artifact-hashes.json').read_text())['files']
for name in expected:
    assert hashlib.sha256((q/name).read_bytes()).hexdigest()==expected[name]
manifest=json.loads((r/'manifest.json').read_text())
assert hashlib.sha256((r/'certificate.json').read_bytes()).hexdigest()==manifest['certificate_sha256']
cert=json.loads((r/'certificate.json').read_text());base=json.loads((q/'factor-base.json').read_text())
assert base['factor_base']['fb_id']==manifest['factor_base']=='FB1h16882188194e'
v=cert['variables'];out=['VARS 9 '+' '.join(v),'POINTS 288']
for point,b in zip(cert['points'],base['points'],strict=True):
    assert point['x']==str(int(b['x'],16)) and point['y']==str(int(b['y'],16))
    out.append(' '.join(point[x] for x in v))
def polys(label,gs):
    out.append(label+' '+str(len(gs)))
    for g in gs:
        out.append(str(len(g)))
        for t in g:
            out.append(t['coefficient']+' '+str(len(t['powers']))+' '+' '.join(k+' '+str(e) for k,e in t['powers'].items()))
polys('CHAIN',cert['homogeneous_generators'])
d=json.loads((q/'dense-CI.json').read_text());polys('DENSE',[d['curve'],d['second_generator']])
cells=json.loads((q/'summary.json').read_text())['cells'];inds={c['m']:c['planted_replay_indices'] for c in cells if c['kind']=='planted'}
out.append('CELLS 12')
for c in cells:
    line=json.loads((q/(c['cell']+'-line.json')).read_text());target=c['target']
    values=[str(c['m']),c['kind'],str(int(target['infinity'])),target['x'],target['y'],*line['coefficients'],line['parameter'],c['circuit_sha256']]
    values.extend(str(x) for x in inds[c['m']])
    out.append(' '.join(values))
out.append('END');Path(sys.argv[2]).write_text('\n'.join(out)+'\n')
PY
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/complete_law.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/complete_law.cpp" > "$build_dir/complete_law_prefix.inc"
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/membership_certificate_v2.cpp" > "$build_dir/membership_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/bound_assembly.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/bound_assembly.cpp" "$study_dir/complete_law.cpp" "$build_dir/cache.txt" > "$build_dir/input-hashes.txt"
