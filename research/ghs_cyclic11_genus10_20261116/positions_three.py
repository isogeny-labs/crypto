#!/usr/bin/env sage -python
"""PGL2(F3) orbits on four-point Frobenius-stable branch configurations."""
from sage.all import GF, PolynomialRing
from itertools import combinations, product
from pathlib import Path
import hashlib
import json

matrices = set()
for a,b,c,d in product(range(3), repeat=4):
    if (a*d-b*c)%3 == 0: continue
    v = (a,b,c,d)
    lead = next(x for x in v if x)
    matrices.add(tuple((x*lead)%3 if lead==2 else x for x in v))
assert len(matrices) == 24

def act(x, m, K):
    a,b,c,d = (K(v) for v in m)
    if x is None: return None if c==0 else a/c
    den = c*x+d
    return None if den==0 else (a*x+b)/den

def valkey(x): return 'infty' if x is None else str(x)

def closedkey(x,n):
    return tuple(sorted(str(x**(3**j)) for j in range(n)))

records=[]
for n in (2,3):
    K=GF(3**n,name='u')
    reps={closedkey(x,n):x for x in K if x**3!=x}
    rationals=[None,K(0),K(1),K(2)]
    configurations=[]
    if n==2:
        for pair in combinations(rationals,2):
            for key in reps:configurations.append((pair,key))
    else:
        for r in rationals:
            for key in reps:configurations.append((r,key))
    def key_of(conf):
        rational,closed=conf
        rkey=tuple(sorted(map(valkey,rational))) if n==2 else valkey(rational)
        return rkey,closed
    def orbit(conf):
        rational,closed=conf
        root=reps[closed]
        out=set()
        for m in matrices:
            moved=tuple(act(v,m,K) for v in rational) if n==2 else act(rational,m,K)
            moved_closed=closedkey(act(root,m,K),n)
            out.add(key_of((moved,moved_closed)))
        return out
    remaining={key_of(conf):conf for conf in configurations}
    sizes=[]
    while remaining:
        conf=next(iter(remaining.values()))
        image=orbit(conf)
        sizes.append(len(image))
        for key in image: assert remaining.pop(key,None) is not None
    assert sorted(sizes)==([6,12] if n==2 else [8,24])
    R=PolynomialRing(K,name='x');x=R.gen()
    f=(x*x+1) if n==2 else (x**3-x-1)
    roots=[z for z,_ in f.roots()]
    assert len(roots)==n
    closed=closedkey(roots[0],n)
    chosen=[((K(0),None),closed),((K(0),K(1)),closed)] if n==2 else [(None,closed),(K(0),closed)]
    selected=[orbit(conf) for conf in chosen]
    assert selected[0].isdisjoint(selected[1])
    assert sorted(len(o) for o in selected)==sorted(sizes)
    records.append({'degree':n,'closed_points':len(reps),
                    'configurations':len(configurations),'orbit_sizes':sorted(sizes),
                    'selected_orbit_sizes':[len(o) for o in selected]})

K=GF(9,name='u');R=PolynomialRing(K,name='x');x=R.gen()
q1=closedkey((x*x+1).roots()[0][0],2)
q2=closedkey((x*x+x+2).roots()[0][0],2)
assert q1!=q2
assert len({q1,q2})==2
quadratic_reps={closedkey(z,2):z for z in K if z**3!=z}
three_branch_orbit=set()
for m in matrices:
    three_branch_orbit.add((valkey(act(None,m,K)),
                            closedkey(act(quadratic_reps[q1],m,K),2)))
assert len(three_branch_orbit)==4*len(quadratic_reps)==12
assert (valkey(None),q1) in three_branch_orbit
rational_points=[None,K(0),K(1),K(2)]
three_rational_orbit={frozenset(valkey(act(v,m,K)) for v in rational_points[:3])
                      for m in matrices}
assert len(three_rational_orbit)==4
pair_orbit=set()
for m in matrices:
    pair_orbit.add(frozenset((closedkey(act(quadratic_reps[q1],m,K),2),
                              closedkey(act(quadratic_reps[q2],m,K),2))))
assert len(pair_orbit)==3
print(json.dumps({'schema':'ghs.cyclic11-branch-orbits/v2',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'PGL2_F3_order':len(matrices),'orbits':records,
                  'three_branch_rational_plus_quadratic_orbit_size':len(three_branch_orbit),
                  'three_branch_three_rational_orbit_size':len(three_rational_orbit),
                  'quadratic_pair_orbit_size':len(pair_orbit)},sort_keys=True))
