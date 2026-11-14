#!/usr/bin/env python3
"""Implementation control: fallclose (C/M4RI) vs lfd_small.closure (pure Python) on small
R-type and T-type instances, D = 2..4, full fixed point; compares one_in and the cumulative
dims of W_D cap B_{<=e} exactly.  Usage: python3 crosscheck.py > out/crosscheck.jsonl"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
L = os.path.join(HERE, '..', '..', '..', 'lfd_lower_bound_20261111')
sys.path.insert(0, L)
import lfd_small as ls

def line(typ, n, seed, k):
    sc = 'emit_instance.py' if typ == 'R' else 'emit_instance_T.py'
    return subprocess.run([sys.executable, os.path.join(L, sc), str(n), str(seed), str(k)],
                          capture_output=True, text=True, check=True).stdout.strip()

def py_closure(ln, D):
    f = list(map(int, ln.split()))
    n, k, poly, a6, z, V = f[0], f[1], f[2], f[3], f[4], f[5:]
    K = ls.GF(n); assert K.poly == poly
    inst = dict(n=n, k=k, K=K, a6=a6, z=z, V=V)
    polys, N, nz = ls.descend(inst, 'ab')
    gens = ls.reduced_basis(polys, N)
    one, dim, cum = ls.closure(gens, N, D)
    return one, cum, nz

bad = 0
for typ, n, k in [('R', 9, 5), ('R', 11, 6), ('R', 13, 7), ('R', 15, 5), ('R', 15, 6), ('R', 15, 7), ('T', 11, 5), ('T', 13, 6), ('T', 15, 5), ('T', 15, 7), ('R', 16, 8), ('T', 17, 8)]:
    for seed in (1, 2):
        ln = line(typ, n, seed, k)
        for D in (2, 3, 4):
            c = json.loads(subprocess.run([os.path.join(HERE, 'fallclose'), str(D), '8192', '1'], input=ln,
                                          capture_output=True, text=True, check=True).stdout)
            one, cum, nz = py_closure(ln, D)
            # lfd_small stops at 1 in W; compare one_in always, dims only when 1 not in W
            ok = (bool(one) == bool(c['one_in'])) and (one or cum == c['W_dims'])
            bad += not ok
            print(json.dumps(dict(type=typ, n=n, k=k, seed=seed, D=D, sols=nz, py_one=bool(one), py_dims=cum,
                                  c_one=bool(c['one_in']), c_dims=c['W_dims'], agree=ok)), flush=True)
print(json.dumps(dict(disagreements=bad)))
