#!/usr/bin/env python3
"""Exact 8-sign target support for a fixed guarded factor base (toy curve).

Only answers the probability that a *uniform point on E(F_p)* matches a sum
of one of the 256 fixed sign branches. Does not model target-adaptive bases.
"""
import itertools
import json

P=1019
A,B=46,158
POINTS=[(944,936),(243,525),(432,632),(225,527),(627,518),(387,878),(914,704),(764,816)]


def neg(pt): return None if pt is None else (pt[0],-pt[1]%P)


def add(u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; z,w=v
    if x==z and (y+w)%P==0: return None
    if x==z:
        assert y==w and y%P
        slope=(3*x*x+A)*pow(2*y,-1,P)%P
    else:
        slope=(w-y)*pow(z-x,-1,P)%P
    xx=(slope*slope-x-z)%P
    yy=(slope*(x-xx)-y)%P
    return xx,yy


def group_size():
    residues={y*y%P for y in range(P)}
    count=1
    for x in range(P):
        v=(x*x*x+A*x+B)%P
        if v==0: count+=1
        elif v in residues: count+=2
    return count


def run():
    assert all(y*y%P==(x*x*x+A*x+B)%P for x,y in POINTS)
    counts={}
    for signs in itertools.product((1,-1), repeat=8):
        pt=None
        for sign,base in zip(signs,POINTS):
            pt=add(pt,base if sign==1 else neg(base))
        counts[pt]=counts.get(pt,0)+1
    N=group_size()
    assert sum(counts.values())==256 and len(counts)<=N
    # Every point in the *uniform full elliptic group* occurs equally often.
    return {'p':P,'curve':{'a':A,'b':B},'group_order':N,
            'signed_branches':256,'distinct_signed_sums':len(counts),
            'collision_branches':256-len(counts),
            'uniform_target_hit_probability':f'{len(counts)}/{N}',
            'universal_upper_bound':f'256/{N}',
            'fixed_base_only':True}


if __name__=='__main__': print(json.dumps(run(),indent=2,sort_keys=True))
