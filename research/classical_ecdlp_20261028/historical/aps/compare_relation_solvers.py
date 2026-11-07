"""Independent APS m=3 solver comparison on identical sampled bases.

This is a bounded reference experiment, not the authors' Magma/F4 code and
not a cryptographic-size benchmark. Sampling is excluded from solver timings.
Full and hybrid Groebner calls have a per-base timeout. All completed solution
sets are compared with both direct polynomial evaluation and point addition.
Run: python compare_relation_solvers.py --samples 4 --timeout 3
"""
import argparse
import itertools
import json
import math
import multiprocessing as mp
from pathlib import Path
from random import Random
from statistics import median
from time import perf_counter

import sympy as sp


class Curve:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b

    def add(self, U, V):
        if U is None:
            return V
        if V is None:
            return U
        x, y = U
        z, v = V
        p = self.p
        if x == z and (y + v) % p == 0:
            return None
        k = ((v-y)*pow(z-x, -1, p) if x != z
             else (3*x*x+self.a)*pow(2*y, -1, p)) % p
        X = (k*k-x-z) % p
        return X, (k*(x-X)-y) % p

    def neg(self, U):
        return None if U is None else (U[0], -U[1] % self.p)

    def mul(self, n, U):
        R = None
        while n:
            if n & 1:
                R = self.add(R, U)
            U = self.add(U, U)
            n >>= 1
        return R


def prime_curve(p):
    roots = {}
    for y in range(p):
        roots.setdefault(y*y % p, []).append(y)
    for a in range(1, 12):
        for b in range(1, 30):
            if (4*a**3+27*b*b) % p == 0:
                continue
            r = 1 + sum(len(roots.get((x**3+a*x+b) % p, []))
                        for x in range(p))
            # Exclude anomalous curves (#E(F_p)=p), which admit a special attack.
            if r != p and sp.isprime(r):
                E = Curve(p, a, b)
                P = next((x, roots[(x**3+a*x+b) % p][0])
                         for x in range(p) if (x**3+a*x+b) % p in roots)
                assert E.mul(r, P) is None
                return E, r, P
    raise ValueError('No prime-order fixture found')


def sample(E, r, P, rng, s):
    secret = rng.randrange(1, r)
    Q = E.mul(secret, P)
    records, seen = [], set()
    while len(records) < s:
        u, v = rng.randrange(r), rng.randrange(r)
        R = E.add(E.mul(u, P), E.mul(v, Q))
        if R is not None and R[0] not in seen:
            seen.add(R[0])
            records.append({'R': R, 'u': u, 'v': v})
    return secret, Q, [records[i::3] for i in range(3)]


def s3(x, y, z, a, b):
    return ((x-y)**2*z*z - 2*((x+y)*(x*y+a)+2*b)*z
            + (x*y-a)**2 - 4*b*(x+y))


def direct_poly(E, groups):
    V = [[d['R'][0] for d in g] for g in groups]
    return sorted(t for t in itertools.product(*V)
                  if s3(*t, E.a, E.b) % E.p == 0)


def point_hash(E, groups):
    third = {d['R'][0] for d in groups[2]}
    out = set()
    for d1 in groups[0]:
        for d2 in groups[1]:
            for R2 in (d2['R'], E.neg(d2['R'])):
                R = E.add(d1['R'], R2)
                if R is not None and R[0] in third:
                    out.add((d1['R'][0], d2['R'][0], R[0]))
    return sorted(out)


def gcd_solve(E, groups):
    z = sp.Symbol('z')
    V = [[d['R'][0] for d in g] for g in groups]
    f = sp.Poly(sp.prod(z-c for c in V[2]), z, modulus=E.p)
    out = []
    for x in V[0]:
        for y in V[1]:
            h = sp.gcd(sp.Poly(s3(x, y, z, E.a, E.b), z, modulus=E.p), f)
            out.extend((x, y, c) for c in V[2] if h.eval(c) % E.p == 0)
    return sorted(out)


def gb_solve(E, groups, hybrid=False):
    x, y, z = sp.symbols('x y z')
    V = [[d['R'][0] for d in g] for g in groups]
    f = [sp.prod(T-c for c in S) for T, S in zip((x,y,z), V)]
    S = s3(x, y, z, E.a, E.b)
    out = []
    if hybrid:
        for c in V[0]:
            G = sp.groebner([S.subs(x,c), f[1], f[2]], y, z,
                           modulus=E.p, order='grevlex', method='f5b')
            funcs = [sp.lambdify((y,z), g.as_expr(), 'math') for g in G.polys]
            out.extend((c,u,v) for u,v in itertools.product(V[1],V[2])
                       if all(h(u,v) % E.p == 0 for h in funcs))
    else:
        G = sp.groebner([S,*f],x,y,z,modulus=E.p,order='grevlex',method='f5b')
        funcs = [sp.lambdify((x,y,z), g.as_expr(), 'math') for g in G.polys]
        out = [t for t in itertools.product(*V)
               if all(h(*t) % E.p == 0 for h in funcs)]
    return sorted(out)


def worker(q, E, groups, hybrid):
    try:
        start = perf_counter()
        result = gb_solve(E,groups,hybrid)
        q.put({'status':'ok','seconds':perf_counter()-start,'roots':result})
    except Exception as e:
        q.put({'status':'error','error':repr(e)})


def bounded_gb(E, groups, hybrid, timeout):
    q = mp.Queue()
    proc = mp.Process(target=worker,args=(q,E,groups,hybrid))
    proc.start()
    proc.join(timeout)
    if proc.is_alive():
        proc.terminate()
        proc.join()
        q.close()
        return {'status':'timeout','limit_seconds':timeout}
    try:
        return q.get(timeout=1)
    finally:
        q.close()


def extraction_check(E,r,P,Q,secret,groups,roots):
    lookup = {d['R'][0]:d for g in groups for d in g}
    usable = 0
    for xyz in roots:
        records = [lookup[c] for c in xyz]
        found = False
        for signs in itertools.product((1,-1),repeat=2):
            eps = (1,*signs)
            total = None
            for c,rec in zip(eps,records):
                total = E.add(total,rec['R'] if c==1 else E.neg(rec['R']))
            if total is None:
                found = True
                lam = sum(c*d['u'] for c,d in zip(eps,records)) % r
                mu = sum(c*d['v'] for c,d in zip(eps,records)) % r
                if mu:
                    w = -lam*pow(mu,-1,r) % r
                    assert w == secret and E.mul(w,P)==Q
                    usable += 1
        assert found
    return usable


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--samples',type=int,default=4)
    ap.add_argument('--timeout',type=float,default=3)
    ap.add_argument('--stratified',action='store_true',
                    help='Choose one natural successful and one failed base per prime; not a success-rate experiment')
    args = ap.parse_args()
    rng = Random(20261025)
    cases = []
    for p in [251,1021,4093,16381]:
        E,r,P = prime_curve(p)
        s = math.ceil(p**(1/3))
        fixtures = []
        if args.stratified:
            seen = set()
            for _ in range(1000):
                fixture = sample(E,r,P,rng,s)
                has_relation = bool(direct_poly(E,fixture[2]))
                if has_relation not in seen:
                    fixtures.append(fixture)
                    seen.add(has_relation)
                if len(seen)==2:break
            assert len(seen)==2
        else:
            fixtures = [sample(E,r,P,rng,s) for _ in range(args.samples)]
        for sample_id,(secret,Q,groups) in enumerate(fixtures):
            row = {'p':p,'r':r,'a':E.a,'b':E.b,'P':P,'Q':Q,
                   'sample':sample_id,'sizes':[len(g) for g in groups],
                   'groups':groups,'solvers':{}}
            expected = None
            for name,func in [('evaluation',direct_poly),('point_hash',point_hash),
                              ('gcd',gcd_solve)]:
                start = perf_counter()
                roots = func(E,groups)
                row['solvers'][name] = {'status':'ok','seconds':perf_counter()-start,
                                        'root_count':len(roots)}
                if expected is None: expected=roots
                else: assert expected == roots,(p,sample_id,name)
            row['root_count'] = len(expected)
            row['usable_relations'] = extraction_check(E,r,P,Q,secret,groups,expected)
            for name,hybrid in [('full_gb',False),('hybrid_gb',True)]:
                result = bounded_gb(E,groups,hybrid,args.timeout)
                if result['status']=='ok':
                    assert result.pop('roots')==expected,(p,sample_id,name)
                    result['root_count'] = len(expected)
                row['solvers'][name]=result
            cases.append(row)
            print(json.dumps({k:row[k] for k in ['p','sample','sizes','root_count','solvers']}),flush=True)
    result = {'seed':20261025,'sympy_version':sp.__version__,
              'samples_per_prime':2 if args.stratified else args.samples,
              'selection':'one successful and one failed base per prime' if args.stratified else 'unfiltered random bases',
              'timeout_seconds':args.timeout,
              'timing_scope':'per fixed base, including method-specific polynomial construction and root extraction; excludes base sampling',
              'cases':cases,'summary':[]}
    for p in [251,1021,4093,16381]:
        subset = [c for c in cases if c['p']==p]
        summary = {'p':p,'bases':len(subset),
                   'bases_with_relation':sum(c['root_count']>0 for c in subset),
                   'solvers':{}}
        for name in subset[0]['solvers']:
            values=[c['solvers'][name] for c in subset]
            good=[d['seconds'] for d in values if d['status']=='ok']
            summary['solvers'][name]={'completed':len(good),
                'timeouts':sum(d['status']=='timeout' for d in values),
                'median_seconds_completed':median(good) if good else None}
        result['summary'].append(summary)
    path=Path(__file__).with_name('solver-comparison-stratified.json' if args.stratified else 'solver-comparison.json')
    path.write_text(json.dumps(result,indent=2))
    print('SUMMARY',json.dumps(result['summary']),flush=True)


if __name__=='__main__':
    main()
