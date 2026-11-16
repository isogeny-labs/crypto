#!/usr/bin/env sage -python
"""Independent finite PGL3(F3) coverage check for degree-five node orbits."""
from sage.all import GF, matrix
from itertools import combinations, product
from pathlib import Path
import hashlib
import json

K=GF(3**5,name="zeta");z=K.gen();F=GF(3)

def encode(a):
    return sum(int(v)*3**i for i,v in enumerate(a.polynomial()))

decode={encode(a):a for a in K}
assert len(decode)==243

def normalize(p):
    k=next(i for i,v in enumerate(p) if v)
    return tuple(encode(v/p[k]) for v in p)

def orbit_key(p):
    return min(normalize(tuple(v**(3**j) for v in p)) for j in range(5))

plane=[(K(1),a,b) for a in K for b in K]
plane.extend((K(0),K(1),a) for a in K)
plane.append((K(0),K(0),K(1)))
assert len(plane)==3**10+3**5+1
five_orbits={orbit_key(p) for p in plane if any(v**3!=v for v in p)}
assert len(five_orbits)==11856

admissible=set()
for key in five_orbits:
    p=tuple(decode[v] for v in key)
    points=[tuple(v**(3**j) for v in p) for j in range(5)]
    if all(matrix(K,[points[i] for i in subset]).rank()==3
           for subset in combinations(range(5),4)):
        admissible.add(key)

projective_matrices=set()
for coeff in product(range(3),repeat=9):
    if matrix(F,3,3,coeff).det()==0:continue
    a=next(v for v in coeff if v)
    projective_matrices.add(tuple(int(F(v)/F(a)) for v in coeff))
assert len(projective_matrices)==5616

def position_orbit(a,b):
    p=(K(1),z**a,z**b)
    keys=set()
    for coeff in projective_matrices:
        q=tuple(sum((K(coeff[3*i+j])*p[j] for j in range(3)),K(0))
                for i in range(3))
        keys.add(orbit_key(q))
    return keys

first=position_orbit(127,143)
second=position_orbit(218,72)
for a,b in ((127,143),(218,72)):
    p=(K(1),z**a,z**b)
    nodes=[tuple(v**(3**j) for v in p) for j in range(5)]
    assert all(matrix(K,[nodes[i] for i in subset]).rank()==3
               for subset in combinations(range(5),3))
assert len(first)==len(second)==5616
assert not (first & second)
assert first|second==admissible
print(json.dumps({"schema":"ghs.trace-minus14.quintic-node-position-orbits/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "projective_plane_points":len(plane),"degree_five_frobenius_orbits":len(five_orbits),
                  "admissible_no_four_collinear":len(admissible),
                  "PGL3_F3_order":len(projective_matrices),
                  "representatives_no_three_collinear":True,
                  "representative_orbit_sizes":[len(first),len(second)],
                  "intersection":len(first&second),"uncovered":len(admissible-(first|second))},
                 sort_keys=True))
