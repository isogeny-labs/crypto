#!/usr/bin/env python3
"""Small exact S3 control for rank-deficiency != guaranteed lattice speedup.
Not a D4/S5 measurement. Python stdlib + sympy; no Sage or PARI.
"""
import argparse
import hashlib
import json
import random
import statistics
import time
from sympy import Matrix

P = 2147483647       # Mersenne prime p % 4 == 3
MONOS = [(i,j) for i in range(3) for j in range(3)]
PIVOT = MONOS.index((2,2))

def poly(A,B,c):
    co = {(2,2):1, (2,1):-2*c, (1,2):-2*c, (2,0):c*c, (0,2):c*c,
          (1,1):-4*A*c-2*B-2*c*c, (1,0):-2*B*c, (0,1):-2*B*c, (0,0):B*B}
    return [co[k]%P for k in MONOS]

def on_curve(x,A,B):
    z=x*(x*x+A*x+B)%P
    return z!=0 and pow(z,(P-1)//2,P)==1

def point(x,A,B,random_sign):
    z=x*(x*x+A*x+B)%P
    y=pow(z,(P+1)//4,P)
    assert y*y%P==z
    return (x, (y if random_sign==0 else -y)%P)

def add(U,V,A,B):
    if U is None:return V
    if V is None:return U
    x,y=U;u,v=V
    if x==u:
        if (y+v)%P==0 or y==0:return None
        slope=(3*x*x+2*A*x+B)*pow(2*y,-1,P)%P
    else:slope=(v-y)*pow(u-x,-1,P)%P
    xx=(slope*slope-A-x-u)%P
    return(xx,(slope*(x-xx)-y)%P)

def evalpoly(co,a,b,mod=None):
    z=sum(k*a**i*b**j for k,(i,j) in zip(co,MONOS))
    return z%mod if mod else z

def lattice(co,X):
    sc=[X**(i+j) for i,j in MONOS]
    # Pivot coefficient normalized to 1 in F_p, all other coordinates arbitrary residues.
    inv=pow(co[PIVOT],-1,P)
    coeff=[(v*inv)%P for v in co]
    rows=[[v*s for v,s in zip(coeff,sc)]]
    for i in range(len(sc)):
        if i == PIVOT: continue
        row=[0]*len(sc);row[i]=P*sc[i];rows.append(row)
    return Matrix(rows),sc

def first_norm(co,X):
    M,sc=lattice(co,X)
    v=list(M.lll(delta=__import__('sympy').Rational(3,4)).row(0))
    assert all(t%s==0 for t,s in zip(v,sc))
    unscaled=[int(t//s) for t,s in zip(v,sc)]
    # The LLL result must be a multiple of H in F_p, not arbitrary short data.
    assert all((unscaled[j]-unscaled[PIVOT]*co[j]*pow(co[PIVOT],-1,P))%P==0
               for j in range(len(MONOS)))
    return int(sum(t*t for t in v)),unscaled

def s3_symmetric_null(rng,a,b):
    v={k:rng.randrange(P) for k in [(0,0),(0,1),(0,2),(1,1),(1,2)]}
    D={(0,0):v[(0,0)], (0,1):v[(0,1)], (1,0):v[(0,1)],
       (0,2):v[(0,2)], (2,0):v[(0,2)],(1,1):v[(1,1)],
       (1,2):v[(1,2)],(2,1):v[(1,2)],(2,2):1}
    # Condition the null on the identical planted witness; otherwise the
    # LLL vanishing advantage is an invalid planted-versus-random comparison.
    D[(0,0)] = 0
    z=sum(D[k]*pow(a,k[0],P)*pow(b,k[1],P) for k in MONOS)%P
    D[(0,0)] = (-z)%P
    return [D[k] for k in MONOS]

def rank_modp(rows):
    # Exact rank over F_p; no floating-point matrix rank.
    a=[list(map(lambda v:v%P,r)) for r in rows]
    rank=0
    for c in range(len(MONOS)):
        q=next((j for j in range(rank,len(a)) if a[j][c]),None)
        if q is None:continue
        a[q],a[rank]=a[rank],a[q]
        inv=pow(a[rank][c],-1,P)
        a[rank]=[x*inv%P for x in a[rank]]
        for j in range(rank+1,len(a)):
            w=a[j][c]
            if w: a[j]=[(x-w*y)%P for x,y in zip(a[j],a[rank])]
        rank+=1
        if rank==len(a):break
    return rank

def run(cases,seed,X):
    rng=random.Random(seed)
    rows=[]; samples=[]; tries=0
    for i in range(cases):
        while True:
            tries+=1
            assert tries<100000
            A=rng.randrange(1,P); B=1
            xs=[x for x in range(2,X+1) if on_curve(x,A,B)]
            if len(xs)<2:continue
            a,b=rng.sample(xs,2)
            R=add(point(a,A,B,rng.randrange(2)),point(b,A,B,rng.randrange(2)),A,B)
            if R is None:continue
            c=R[0];break
        h=poly(A,B,c)
        assert evalpoly(h,a,b,P)==0, 'invalid S3 relation'
        corrupt=h.copy();corrupt[0]=(corrupt[0]+1)%P
        assert evalpoly(corrupt,a,b,P)!=0, 'negative relation mutation undetected'
        null=s3_symmetric_null(rng,a,b)
        assert evalpoly(null,a,b,P)==0
        start=time.process_time()
        norm_h,vh=first_norm(h,X)
        norm_null,vnull=first_norm(null,X)
        spent=time.process_time()-start
        # Norms are comparable for identical scaling and lattice shape.
        samples.append({'case':i,'A':A,'B':B,'a':a,'b':b,'xR':c,
                'actual_norm2':norm_h,'symmetric_null_norm2':norm_null,
                'norm_ratio':(norm_h/norm_null)**0.5,
                'first_vector_integer_vanish':evalpoly(vh,a,b)==0,
                'llltime_cpu_sec':spent,
                'actual_coefficient_sha256':hashlib.sha256(json.dumps(h).encode()).hexdigest()})
        rows.append(h)
    ratios=[z['norm_ratio'] for z in samples]
    return {'hypothesis':'coefficient-family rank-deficit alone yields >=2x LLL norm win vs permutation-symmetric null',
            'status':'S3 calibration only, NOT D4 S5 outcome',
            'p':P,'seed':seed,'cases':cases,'X':X,
            'coefficient_rank_modp':rank_modp(rows),'ambient_dimension':len(MONOS),
            'symmetric_null_construction':'a<->b symmetry; independent free coefficients, pivot 1; constant set for identical planted root (a,b)',
            'median_norm_ratio':statistics.median(ratios),
            'median_ratio_ge_half':statistics.median(ratios)>=0.5,
            'under_half_cases':sum(t<0.5 for t in ratios),
            'first_vector_integer_vanish_count':sum(s['first_vector_integer_vanish'] for s in samples),
            'min_norm_ratio':min(ratios),'max_norm_ratio':max(ratios),
            'total_cpu_sec':sum(s['llltime_cpu_sec'] for s in samples),
            'samples':samples}

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--cases',type=int,default=24);p.add_argument('--seed',type=int,default=20261118);p.add_argument('--X',type=int,default=16);p.add_argument('--output',default='RESULTS.json'); a=p.parse_args()
    o=run(a.cases,a.seed,a.X)
    with open(a.output,'w') as f:json.dump(o,f,indent=2,sort_keys=True)
    print(json.dumps({k:v for k,v in o.items() if k!='samples'},indent=2))
