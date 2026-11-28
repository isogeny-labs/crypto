#!/usr/bin/env python3
"""Independent Euler-character and left-to-right group-law replay of holdout."""

import argparse
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent


def digest(data):
    return hashlib.sha256(data).hexdigest()


def prime(n):
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    return all(n % d for d in range(3, math.isqrt(n) + 1, 2))


def character(z, p):
    value = pow(z % p, (p - 1) // 2, p)
    return 0 if value == 0 else -1 if value == p - 1 else 1


def order(p, a, b):
    return p + 1 + sum(character((x*x*x+a*x+b) % p, p) for x in range(p))


def inverse(n, p):
    r0, r1 = p, n % p
    t0, t1 = 0, 1
    while r1:
        q = r0 // r1
        r0, r1 = r1, r0 - q*r1
        t0, t1 = t1, t0 - q*t1
    assert r0 == 1
    return t0 % p


def combine(p, a, u, v):
    if u is None:
        return v
    if v is None:
        return u
    x1, y1 = u
    x2, y2 = v
    if x1 == x2 and (y1+y2) % p == 0:
        return None
    if x1 == x2:
        numerator, denominator = (3*x1*x1+a) % p, (2*y1) % p
    else:
        numerator, denominator = (y2-y1) % p, (x2-x1) % p
    lam = numerator*inverse(denominator,p) % p
    x3 = (lam*lam-x1-x2) % p
    return (x3, (lam*(x1-x3)-y1) % p)


def scalar_mul(p, a, point, k):
    acc = None
    for bit in bin(k)[2:]:
        acc = combine(p, a, acc, acc)
        if bit == '1':
            acc = combine(p, a, acc, point)
    return acc


def first_point(p, a, b):
    for x in range(p):
        rhs = (x*x*x+a*x+b) % p
        if character(rhs,p) < 0:
            continue
        for y in range(p):
            if y*y % p == rhs:
                return x,y
    raise ValueError('no affine point')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--dir',type=Path,default=HERE/'z2xz8_holdout')
    args = parser.parse_args()
    root = args.dir
    manifest_bytes = (root/'manifest.json').read_bytes()
    manifest = json.loads(manifest_bytes)
    roster_bytes = (root/'roster.json').read_bytes()
    roster = json.loads(roster_bytes)
    protocol_sha = digest((HERE/'PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md').read_bytes())
    producer_sha = digest((HERE/'freeze_z2xz8_holdout.py').read_bytes())
    assert manifest['schema']=='pkm-z2xz8-holdout-manifest/v1'
    assert roster['schema']=='pkm-z2xz8-holdout-roster/v1'
    assert manifest['protocol_sha256']==roster['protocol_sha256']==protocol_sha
    assert manifest['producer_sha256']==roster['producer_sha256']==producer_sha
    assert manifest['roster_sha256']==digest(roster_bytes)
    assert len(manifest['entries'])==len(roster['curves'])==6
    tested=0
    for bits in (11,14):
        selected=[c for c in roster['curves'] if c['bits']==bits]
        assert len(selected)==3
        ps=[]
        candidate=1+32*((1<<bits)//32)
        while len(ps)<3:
            if prime(candidate):ps.append(candidate)
            candidate+=32
        assert [c['p'] for c in selected]==ps
        for c in selected:
            p,a,b,r=c['p'],c['a'],c['b'],c['r']
            assert (4*a*a*a+27*b*b)%p !=0 and prime(r) and r!=p
            assert order(p,a,b)==r
            rejected=0
            for aa in range(1,a+1):
                for bb in range(1,(b if aa==a else p-1)+1):
                    if aa==a and bb==b:break
                    if (4*aa*aa*aa+27*bb*bb)%p and (rr:=order(p,aa,bb))!=p and prime(rr):
                        raise AssertionError(f'earlier eligible curve {(p,aa,bb)}')
                    rejected+=1
            assert rejected==c['rejected_pairs']
            g=tuple(c['generator'])
            assert first_point(p,a,b)==g
            assert scalar_mul(p,a,g,r) is None
            assert len(c['targets'])==16
            seen=set()
            for j,t in enumerate(c['targets']):
                assert t['target_index']==j
                for retry in range(t['retry']+1):
                    d=digest(f'pkm-z2xz8-holdout-v1|{bits}|{c["curve_index"]}|{j}|{retry}'.encode())
                    q=1+int(d,16)%(r-1)
                    if retry<t['retry']:assert q in seen
                assert d==t['digest'] and q==t['scalar'] and q not in seen
                seen.add(q)
                point=scalar_mul(p,a,g,q)
                assert point is not None and t['point']=={'x':hex(point[0]),'y':hex(point[1])}
                seed=digest(f'pkm-z2xz8-relations-v1|{bits}|{c["curve_index"]}|{j}'.encode())
                assert seed==t['relation_seed_sha256']
                tested+=1
            e=next(e for e in manifest['entries'] if (e['bits'],e['curve_index'])==(bits,c['curve_index']))
            assert (e['p'],e['r'],e['target_scalars'])==(p,r,[t['scalar'] for t in c['targets']])
            raw=(root/e['path']).read_bytes()
            assert digest(raw)==e['sha256']
            w=json.loads(raw)
            assert w['schema']=='ic.prime-workload/v1'
            assert (int(w['curve']['a'],16),int(w['curve']['b'],16),int(w['curve']['field']['modulus'],16))==(a,b,p)
            assert (int(w['curve']['group_order'],16),int(w['curve']['subgroup_order'],16),int(w['curve']['cofactor'],16))==(r,r,1)
            assert w['curve']['generator']=={'x':hex(g[0]),'y':hex(g[1])}
            assert w['targets']==[{'scalar':t['scalar'],'point':t['point']} for t in c['targets']]
    print(json.dumps({'curves':6,'targets':tested,'manifest_sha256':digest(manifest_bytes),'roster_sha256':digest(roster_bytes)},sort_keys=True))


if __name__=='__main__':
    main()
