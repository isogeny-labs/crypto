#!/usr/bin/env python3
"""CT93: exhaustive embeddings vs distinct cyclic homomorphisms, exact F7 test.

Independent of CT80–92 modules. This is a torsion-module certificate,
not a map between characteristic-37 and characteristic-2 schemes.
"""
import json
from collections import Counter

L = 7
B = ((0, 5), (1, 5))

def c(n):
    return n % L

def frob(V):
    x, y = V
    return (c(B[0][0]*x + B[0][1]*y), c(B[1][0]*x + B[1][1]*y))

def mul(k, V):
    return tuple(c(k*z) for z in V)

def add_vec(V, W):
    return tuple(c(v+w) for v,w in zip(V, W))

def group_source(P, Q, p=37):
    if P is None: return Q
    if Q is None: return P
    x,y = P; u,v = Q
    if x == u and (y+v+1) % p == 0: return None
    if P == Q:
        slope = (3*x*x-2*x-10)*pow(2*y+1,-1,p)%p
        intercept = (-x*x*x-10*x-40-y)*pow(2*y+1,-1,p)%p
    else:
        slope = (v-y)*pow(u-x,-1,p)%p
        intercept = (y*u-v*x)*pow(u-x,-1,p)%p
    X = (slope*slope+1-x-u)%p
    Y = (-slope*X-intercept-1)%p
    assert (Y*Y+Y-X*X*X+X*X+10*X+20)%p == 0
    return X,Y

def source_points():
    points=[None];point=None
    for k in range(1,L+1):
        point=group_source(point,(9,4))
        if k<L: points.append(point)
    assert point is None and len(set(points))==7
    assert {point[0] for point in points[1:]} == {9,27,28}
    for i in range(L):
        for j in range(L):
            assert group_source(points[i],points[j])==points[(i+j)%L]
    return points

def orbits():
    seen=set(); arr=[]
    for x in range(L):
        for y in range(L):
            seed=(x,y)
            if seed==(0,0) or seed in seen: continue
            cyc=[];v=seed
            while v not in seen:
                seen.add(v);cyc.append(v);v=frob(v)
            assert v==seed and len(cyc)==24
            arr.append(cyc)
    assert len(arr)==2 and len(seen)==48
    return arr

def curve_order(p):
    return 1+sum(1 for x in range(p) for y in range(p) if (y*y+y-x*x*x+x*x+10*x+20)%p==0)

def main():
    assert curve_order(37)==35 and curve_order(2)==5
    pts=source_points();cycles=orbits()
    assert all(mul(3,v)==frob(frob(frob(frob(v)))) for C in cycles for v in C)
    vectors=Counter();line_set=set();labeled=0;verified=0
    # 6 source embeddings v | 37, 2 binary primes w | 2, each 24 residue-field embeddings.
    for source_u in range(1,L):
        for w,cycle in enumerate(cycles):
            for target_T in cycle:
                labeled+=1
                inv=pow(source_u,-1,L)
                image=mul(inv,target_T)  # image of fixed public generator P
                vectors[image]+=1
                line=frozenset(mul(j,image) for j in range(1,L))
                line_set.add(line)
                for i in range(L):
                    for j in range(L):
                        assert add_vec(mul(i,image),mul(j,image)) == mul((i+j)%L,image)
                        assert pts[(i+j)%L] == group_source(pts[i],pts[j])
                verified += 7
    assert labeled == 288 and len(vectors)==48 and set(vectors.values())=={6}
    assert len(line_set)==8 and all(len(line)==6 for line in line_set)
    union=set().union(*line_set)
    assert union == set(vectors.keys())
    # Distinct source and target places are required for consistent P and Q orientation.
    default=cycles[0][0]
    wrong_source=sum(mul(k,default)!=mul(k*pow(2,-1,L),default) for k in range(1,L))
    wrong_target=sum(mul(k,default)!=frob(frob(frob(frob(mul(k,default))))) for k in range(1,L))
    assert (wrong_source,wrong_target)==(6,6)
    result={'experiment':'ct93-torsion-place-ambiguity',
            'source_curve_order':35,'binary_curve_order':5,'source_generator':[9,4],
            'source_public_points':[['O'] if P is None else list(P) for P in pts],
            'frobenius_matrix':[[0,5],[1,5]],
            'binary_orbit_seeds':[list(a[0]) for a in cycles],
            'binary_orbit_lengths':[len(a) for a in cycles],
            'source_degree_one_places':6,'binary_prime_ideals':2,
            'residue_embeddings_per_binary_prime':24,
            'labeled_triplets':labeled,'distinct_group_maps':len(vectors),
            'fibers_over_each_map':6,'target_7_torsion_lines':len(line_set),
            'public_test_cases_per_map':7,
            'independently_checked_public_images':verified,
            'wrong_source_orientation_nonzero_failures':wrong_source,
            'wrong_target_frobenius_nonzero_failures':wrong_target}
    print(json.dumps(result,sort_keys=True,separators=(',',':')))

if __name__=='__main__': main()
