#!/usr/bin/env python3
"""Validate iclose (independent closure) against the 72 reference rows of ../out/crosscheck.jsonl
(where fallclose and lfd_small.closure already agree), using gen_system.py for the descent.
Runs every case with two column-order seeds and both work-list orders.
Usage: python3 small_check.py ICLOSE_BIN > out/small_check.jsonl"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
L = os.path.join(HERE, '..', '..', '..', '..', 'lfd_lower_bound_20261111')
BIN = sys.argv[1]
bad = n = 0
for l in open(os.path.join(HERE, '..', 'out', 'crosscheck.jsonl')):
    r = json.loads(l)
    if 'D' not in r: continue
    sc = 'emit_instance.py' if r['type'] == 'R' else 'emit_instance_T.py'
    ln = subprocess.run([sys.executable, os.path.join(L, sc), str(r['n']), str(r['seed']), str(r['k'])],
                        capture_output=True, text=True, check=True).stdout
    sysf = '/tmp/claude-0/xc/sys.txt'
    open(sysf, 'w').write(subprocess.run([sys.executable, os.path.join(HERE, 'gen_system.py')], input=ln,
                                         capture_output=True, text=True, check=True).stdout)
    for seed, order in ((0, 1), (7, 0)):
        c = json.loads(subprocess.run([BIN, str(r['D']), sysf, '256', '1', str(seed), str(order)],
                                      capture_output=True, text=True, check=True).stdout)
        ref_one = r['c_one']
        ok = (bool(c['one_in']) == ref_one == r['py_one']) and (ref_one or c['W_dims'] == r['c_dims'] == r['py_dims'])
        bad += not ok; n += 1
        print(json.dumps(dict(type=r['type'], n=r['n'], k=r['k'], seed=r['seed'], D=r['D'], col_seed=seed,
                              order=['lifo', 'fifo'][order], ref_one=ref_one, ref_dims=r['c_dims'],
                              ic_one=bool(c['one_in']), ic_dims=c['W_dims'], agree=ok)), flush=True)
print(json.dumps(dict(runs=n, disagreements=bad)))
