#!/usr/bin/env python3
"""Validate vclose + mydescent on the 72 reference rows of ../../out/crosscheck.jsonl (fallclose == lfd_small)."""
import json, os, subprocess, sys
H = os.path.dirname(os.path.abspath(__file__)); E = os.path.join(H, '../../../../../lfd_lower_bound_20261111')
B = sys.argv[1]; tmp = sys.argv[2]; bad = n = 0
for l in open(os.path.join(H, '../../out/crosscheck.jsonl')):
    r = json.loads(l)
    if 'D' not in r: continue
    sc = 'emit_instance.py' if r['type'] == 'R' else 'emit_instance_T.py'
    ln = subprocess.run([sys.executable, sc, str(r['n']), str(r['seed']), str(r['k'])], cwd=E, capture_output=True, text=True, check=True).stdout
    open(tmp + '.line', 'w').write(ln)
    d = json.loads(subprocess.run([sys.executable, os.path.join(H, 'mydescent.py'), tmp + '.line', '/dev/null', tmp + '.sys'], capture_output=True, text=True, check=True).stdout)
    assert d['sols'] == r['sols'], (r, d['sols'])
    for seed in (3, 11):
        c = json.loads(subprocess.run([B, str(r['D']), tmp + '.sys', str(seed), '64', '1'], capture_output=True, text=True, check=True).stdout)
        ok = bool(c['one_in']) == r['c_one'] == r['py_one'] and (r['c_one'] or c['W_dims'] == r['c_dims'] == r['py_dims'])
        bad += not ok; n += 1
        print(json.dumps(dict(type=r['type'], n=r['n'], k=r['k'], seed=r['seed'], D=r['D'], col_seed=seed, sols=d['sols'],
              ref_one=r['c_one'], ref_dims=r['c_dims'], v_one=bool(c['one_in']), v_dims=c['W_dims'], agree=ok)), flush=True)
print(json.dumps(dict(runs=n, disagreements=bad)))
