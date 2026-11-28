#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_retained_depth8.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
# Format conversion and hash validation only; no Python field arithmetic.
python3 - "$study_dir/evidence/complete-law-depth8" "$build_dir/cache.txt" <<'PY'
import hashlib,json,sys
from pathlib import Path
p=Path(sys.argv[1]); expected=json.loads((p/'artifact-hashes.json').read_text())['files']
for name in ('factor-base.json','structured-span.json','construction.json'):
    assert hashlib.sha256((p/name).read_bytes()).hexdigest()==expected[name]
b=json.loads((p/'factor-base.json').read_text()); chain=json.loads((p/'construction.json').read_text())
assert b['factor_base']['fb_id']=='FB1h16882188194e'
rows=json.loads((p/'structured-span.json').read_text())['full_fibre_points']
by_x={r['X0']:r for r in rows}
out=['VARS 9 x y f1 f2 f3 f4 f5 f6 f7','POINTS 288']
for point in b['points']:
    x=str(int(point['x'],16));y=str(int(point['y'],16));r=by_x[x]
    out.append(' '.join([x,y]+[r['X'+str(i)] for i in range(1,8)]))
out.append('STEPS 8')
out.extend(s['r']+' '+s['delta'] for s in chain['chain'])
out.extend(['TERMINAL '+chain['terminal']['x'],'END'])
Path(sys.argv[2]).write_text('\n'.join(out)+'\n')
PY
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/retained_basis.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_basis.cpp" > "$build_dir/retained_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/retained_depth8.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/retained_depth8.cpp" "$study_dir/retained_basis.cpp" "$build_dir/cache.txt" > "$build_dir/input-hashes.txt"
