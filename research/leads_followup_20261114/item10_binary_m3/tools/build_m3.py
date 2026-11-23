#!/usr/bin/env python3
"""Build m=3 (S_4 descended, three copies of V) instances in the ic-bin-instance/1 format (variables = 3k),
with the repository builders csd.ic_dreg.binary_descent / binary_solutions (same as csd.ic_bounds_binary.m3_direct_system).
usage: build_m3.py SPEC.json OUTDIR   (SPEC = list of dicts: name,n,k,a,b,basis|seed,target)"""
import sys, json, hashlib, random, os, time
sys.path.insert(0, '/home/user/ml-cryptanalysis'); sys.path.insert(0, '/home/user/ml-cryptanalysis/scripts')
from csd import ic_dreg as I, ic_curve_sweep as S
from csd.ic_arbitrary_bases import canonical_basis
from csd.ic_research_campaign import field
from csd.ic_bounds_binary import system_hash as repo_hash
import ic_bounds_bin_big_run as R

def seed(*p): return int.from_bytes(hashlib.sha256(':'.join(map(str, p)).encode()).digest()[:8], 'big')

def build(s):
    n, k = s['n'], s['k']; F = field(n); r = 3 * k
    a, b = s['a'], s['b']; E = I.BinaryCurve(F, a, b)
    rng = random.Random(seed(s['seed'], s['name']))
    basis = canonical_basis(s['basis']) if 'basis' in s else canonical_basis(S.random_nonstable_subspace(F, k, rng, []))
    assert len(basis) == k
    t = s['target']
    if t == 'order-two': target = (0, F.sqrt(b))
    elif t == 'random-rational': target = S.random_rational_target(E, rng, 1 << n, exclude_x={0})
    else: target = tuple(t)
    assert E.on_curve(target)
    bases = [basis] * 3
    eqs = sorted(set(tuple(sorted(e)) for e in I.binary_descent(E, bases, target[0]) if e))
    t0 = time.time()
    sols = I.binary_solutions(E, bases, target[0])
    Z = sorted(c1 | (c2 << k) | (c3 << 2 * k) for c1, c2, c3 in sols)
    inst = dict(format='ic-bin-instance/1', name=s['name'], spec=s, n=n, modulus=F.modulus, k=k, variables=r, basis=basis,
                curve=dict(a=a, b=b), target=list(target), equations=[list(e) for e in eqs],
                system_hash=R.system_hash_bin(r, eqs), repo_system_hash=repo_hash('binary-m3-direct', r, eqs),
                points=Z, n_points=len(Z), points_sha256=hashlib.sha256(json.dumps(Z).encode()).hexdigest(),
                z_seconds=round(time.time() - t0, 1), n_equations=len(eqs),
                max_deg=max(m.bit_count() for e in eqs for m in e))
    return inst

if __name__ == '__main__':
    specs = json.load(open(sys.argv[1])); os.makedirs(sys.argv[2], exist_ok=True)
    for s in specs:
        inst = build(s)
        assert R.check_points(inst) if inst['points'] else True
        p = os.path.join(sys.argv[2], s['name'] + '.json')
        json.dump(inst, open(p, 'w'), separators=(',', ':'))
        print(s['name'], 'r=%d n=%d |Z|=%d eqs=%d deg=%d target=%s repo_hash=%s sha256=%s' % (inst['variables'], inst['n'], inst['n_points'],
              inst['n_equations'], inst['max_deg'], inst['target'], inst['repo_system_hash'][:16], hashlib.sha256(open(p,'rb').read()).hexdigest()[:16]), flush=True)
