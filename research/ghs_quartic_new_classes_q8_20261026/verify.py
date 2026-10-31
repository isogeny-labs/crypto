#!/usr/bin/env sage -python
"""Independent point-count and subgroup-gate replay for selected quartics."""

import hashlib
import json
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing, is_prime

ROOT = Path(__file__).resolve().parent
RAW = ROOT / 'search_results.json'
SOURCE = ROOT.parent / 'ghs_binary_cubic_isogeny_20261026/results_q8.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def make_curve(field, small, embed, coeffs):
    ring = PolynomialRing(field, names=('X','Y','Z'))
    X,Y,Z = ring.gens()
    monomials = [X**i * Y**j * Z**(4-i-j)
                 for i in range(4,-1,-1) for j in range(4-i,-1,-1)]
    assert len(monomials)==15
    return Curve(sum((embed(small.from_integer(c))*m
                      for c,m in zip(coeffs,monomials)),ring.zero()))


def coordinates(point):
    return [int(point[0].to_integer()),int(point[1].to_integer())]


def main():
    raw=json.loads(RAW.read_text())
    source=json.loads(SOURCE.read_text())
    assert raw['source_sha256']==sha(SOURCE)
    k=GF(8,'a',modulus='conway')
    k2=GF(64,'u',modulus='conway')
    K=GF(512,'w',modulus='conway')
    embeddings=((k,lambda x:x),(k2,k.embeddings(k2)[0]),
                (K,k.embeddings(K)[0]))
    by_order={}
    for row in source['source_population']:
        by_order.setdefault(row['order'],[]).append(row)
    records=[]
    for witness in raw['witnesses']:
        order=witness['group_order']
        assert by_order[order]==witness['sources']
        counts=[]
        curves=[]
        for field,embed in embeddings:
            curve=make_curve(field,k,embed,witness['coefficients'])
            assert curve.is_smooth()
            curves.append(curve)
            counts.append(len(curve.rational_points()))
        assert counts==witness['N1_N2_N3']
        assert (513-counts[2])%3==0
        trace=(513-counts[2])//3
        assert witness['weil_polynomial_coefficients']==[1,0,0,-trace,0,0,512]
        assert order==513-trace
        source_rows=witness['sources']
        assert all(z['direct_class']=='C' for z in source_rows)
        for z in source_rows:
            b=K.from_integer(z['b'])
            a2=K.from_integer(z['a'])
            E=EllipticCurve(K,[K.one(),a2,K.zero(),K.zero(),b])
            assert len(E.points())==order
            assert b+b**8+b**64 != 0
            assert b**8 != b
        selected=source_rows[0]
        r,h=selected['largest_prime_subgroup'],selected['cofactor']
        assert is_prime(r)
        assert r*h==order
        assert int(__import__('math').gcd(r,6-3*trace))==1
        E=EllipticCurve(K,[K.one(),K.from_integer(selected['a']),K.zero(),
                             K.zero(),K.from_integer(selected['b'])])
        G=next((h*P for P in E.points() if not (h*P).is_zero()),None)
        assert G is not None and (r*G).is_zero()
        Q=7*G
        assert not Q.is_zero() and (r*Q).is_zero()
        C=curves[0]
        rational_points=C.rational_points()
        base=rational_points[0].place().divisor()
        J=C.jacobian(model='hess',base_div=3*base).group()
        assert J.order()==order
        classes=[J.point(P.place().divisor()-base)
                 for P in rational_points[1:]]
        reach=any(not (h*D).is_zero() for D in classes)
        records.append({'group_order':order,'class_label':None,
                        'quartic_coefficients':witness['coefficients'],
                        'point_counts':counts,'weil_trace':trace,
                        'jacobian_order':int(J.order()),
                        'source_model_count':len(source_rows),
                        'subgroup_prime':r,'cofactor':h,
                        'simple_root_derivative':6-3*trace,
                        'selected_source_a':selected['a'],
                        'selected_source_b':selected['b'],
                        'G':coordinates(G),'Q':coordinates(Q),
                        'rational_point_class_reaches_prime':reach})
        print('VERIFIED',order,'source models',len(source_rows),
              'prime reach',reach,flush=True)
    result={'schema':'ghs-q8-new-quartic-independent-replay/v1',
            'search_sha256':sha(RAW),'source_sha256':sha(SOURCE),
            'verifier_sha256':sha(Path(__file__)),
            'records':records}
    (ROOT/'verification.json').write_text(json.dumps(result,indent=2,
                                                    sort_keys=True)+'\n')


if __name__=='__main__':
    main()
