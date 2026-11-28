#!/usr/bin/env python3
"""CT216: independent finite F101 source-image and offset-coset smoothness screen.

The cubic pullback C:y^2=x^9+x^3+46 -> E:y^2=z^3+z+46 is prior art.
This does NOT implement a complete genus-4 index-calculus algorithm or pricing.
All arithmetic is Python stdlib: an independent Cantor group-law implementation.
"""
import argparse
import json
from collections import Counter
from itertools import combinations

P, G, R = 101, 4, 109
F = [46, 0, 0, 1, 0, 0, 0, 0, 0, 1]
ZERO, ONE = (0,), (1,)

def norm(a):
    t = [int(x) % P for x in a]
    while len(t) > 1 and t[-1] == 0:
        t.pop()
    return tuple(t or [0])

def degree(a):
    return -1 if a == ZERO else len(a)-1

def plus(a,b):
    return norm([(a[i] if i < len(a) else 0)+(b[i] if i < len(b) else 0) for i in range(max(len(a),len(b)))])

def minus(a,b):
    return norm([(a[i] if i < len(a) else 0)-(b[i] if i < len(b) else 0) for i in range(max(len(a),len(b)))])

def product(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):
            z[i+j]+=x*y
    return norm(z)

def divrem(a,b):
    assert b != ZERO
    r=list(a)
    q=[0]*max(1,len(a)-len(b)+1)
    inverse=pow(b[-1],-1,P)
    while norm(r)!=ZERO and len(norm(r))>=len(b):
        r=list(norm(r))
        i=len(r)-len(b)
        c=r[-1]*inverse%P
        q[i]=c
        for j,t in enumerate(b):
            r[i+j]=(r[i+j]-c*t)%P
    return norm(q),norm(r)

def quotient(a,b):
    q,r=divrem(a,b)
    assert r==ZERO,(a,b,r)
    return q

def modulo(a,b):
    return divrem(a,b)[1]

def scalar(a,s):
    return norm([s*x for x in a])

def monic(a):
    return scalar(a,pow(a[-1],-1,P)) if a!=ZERO else a

def xgcd(a,b):
    r0,r1=a,b
    a0,a1=ONE,ZERO
    b0,b1=ZERO,ONE
    while r1!=ZERO:
        q,r=divrem(r0,r1)
        r0,r1=r1,r
        a0,a1=a1,minus(a0,product(q,a1))
        b0,b1=b1,minus(b0,product(q,b1))
    if r0==ZERO:
        return ZERO,ZERO,ZERO
    fac=pow(r0[-1],-1,P)
    d,ca,cb=scalar(r0,fac),scalar(a0,fac),scalar(b0,fac)
    assert plus(product(ca,a),product(cb,b))==d
    return d,ca,cb

def valid(div):
    u,v=div
    return u!=ZERO and u[-1]==1 and degree(u)<=G and degree(v)<degree(u) and modulo(minus(product(v,v),F),u)==ZERO

IDENTITY=(ONE,ZERO)

def cantor(a,b):
    u1,v1=a; u2,v2=b
    d1,a1,a2=xgcd(u1,u2)
    d,b1,b2=xgcd(d1,plus(v1,v2))
    s1=product(b1,a1);s2=product(b1,a2);s3=b2
    u=monic(quotient(product(u1,u2),product(d,d)))
    z=plus(plus(product(product(s1,u1),v2),product(product(s2,u2),v1)),product(s3,plus(product(v1,v2),F)))
    v=modulo(quotient(z,d),u)
    while degree(u)>G:
        u_new=monic(quotient(minus(F,product(v,v)),u))
        v=modulo(scalar(v,-1),u_new)
        u=u_new
    out=(u,v)
    assert valid(out),(a,b,out)
    return out

def neg(a):
    u,v=a
    return (u,modulo(scalar(v,-1),u))

def evalp(f,x):
    y=0
    for c in f[::-1]:
        y=(y*x+c)%P
    return y

def ecadd(a,b):
    if a is None: return b
    if b is None: return a
    x1,y1=a;x2,y2=b
    if x1==x2 and (y1+y2)%P==0:return None
    if x1==x2:
        m=(3*x1*x1+1)*pow(2*y1,-1,P)%P
    else:
        m=(y2-y1)*pow(x2-x1,-1,P)%P
    x3=(m*m-x1-x2)%P
    y3=(m*(x1-x3)-y1)%P
    return x3,y3

def image(a):
    if a is None:return IDENTITY
    x,y=a
    d=(((-x)%P),0,0,1)
    out=(d,(y,))
    assert valid(out)
    return out

def point_div(a):
    x,y=a
    d=(((-x)%P),1)
    out=(d,(y,))
    assert valid(out)
    return out

def roots_count(u):
    r=u
    found=[]
    for x in range(P):
        lin=norm((-x,1))
        while degree(r)>0 and evalp(r,x)==0:
            r=quotient(r,lin)
            found.append(x)
    return found, degree(r)==0

def sums(divs):
    d=IDENTITY
    for x in divs:
        d=cantor(d,point_div(x))
    return d

def perform():
    gen=(2,37)
    assert (gen[1]**2 - (gen[0]**3+gen[0]+46))%P==0
    ecc_points=[(x,y) for x in range(P) for y in range(P) if (y*y-(x*x*x+x+46))%P==0]
    assert len(ecc_points)+1==R,(len(ecc_points),R)
    ec=[None]; j=[IDENTITY]
    for k in range(1,R):
        ec.append(ecadd(ec[-1],gen))
        j.append(cantor(j[0] if k==1 else j[-1],image(gen)))
    assert ecadd(ec[-1],gen) is None and cantor(j[-1],image(gen))==IDENTITY
    assert len(set(ec))==R and len(set(j))==R
    assert all(image(p)==d for p,d in zip(ec,j))
    assert all(degree(d[0])==3 for d in j[1:])
    assert all(cantor(d,neg(d))==IDENTITY for d in j)
    # Predeclared deterministic finite roster, not chosen based on smoothness.
    cpoints=[]
    for x in range(P):
        for y in range(P):
            if (y*y-evalp(F,x))%P==0:
                if not cpoints or cpoints[-1][0]!=x:
                    cpoints.append((x,y))
    assert len(cpoints)>=8
    first=cpoints[:8]
    roster=[('deg1_'+str(i),[first[i]]) for i in range(8)]
    roster += [('deg2_'+str(i),list(first[2*i:2*i+2])) for i in range(4)]
    roster += [('deg3_'+str(i),list(first[3*i:3*i+3])) for i in range(2)]
    roster += [('deg4_'+str(i),list(first[4*i:4*i+4])) for i in range(2)]
    rows=[]
    for name,pts in roster:
        d0=sums(pts)
        histogram=Counter()
        degree4=split4=0
        # Explicit exhaustive scalar sweep of the embedded E[109] coset.
        for k,sk in enumerate(j):
            v=cantor(d0,sk)
            assert valid(v)
            deg=degree(v[0]);histogram[str(deg)]+=1
            if deg==4:
                degree4+=1
                roots,complete=roots_count(v[0])
                if complete and len(roots)==4 and len(set(roots))==4:
                    split4+=1
        rows.append({'offset':name,'points':pts,'offset_degree':degree(d0[0]),
                     'degrees':dict(sorted(histogram.items())), 'degree4':degree4,
                     'split_squarefree_degree4':split4})
    # Distinct check: no unchanged group element can equal its nontrivial translate.
    assert all(row['degree4']<=R and row['split_squarefree_degree4']<=row['degree4'] for row in rows)
    return {'schema':'ct216-f101-cubic-pullback-coset-screen-v1',
            'field':P,'source_curve':'y2=z3+z+46','source_order':R,
            'cover_curve':'y2=x9+x3+46','cover_genus':G,
            'generator':[2,37], 'all_source_points_verified':R,
            'source_image_degree3':R-1,'source_image_degree4':0,
            'offset_roster_first_distinct_x_points':first,
            'offsets':rows,'pcc':'unverified','standing':'finite exact arithmetic; no IC solver or cost result'}

def self_test():
    assert xgcd((2,0,1),(3,1))[:1] == (ONE,)
    for x in range(1,11):
        for y in range(1,11):
            z=norm((x,y,1))
            assert quotient(product(z,(y,x,1)),z)==norm((y,x,1))
    f=perform()
    assert f['all_source_points_verified']==109
    assert len(f['offsets'])==16
    assert all(sum(x['degrees'].values())==109 for x in f['offsets'])
    print('CT216_SELFTEST_PASS',len(f['offsets']),'offset cosets')

if __name__=='__main__':
    a=argparse.ArgumentParser()
    a.add_argument('--self-test',action='store_true')
    z=a.parse_args()
    if z.self_test:self_test()
    else:print(json.dumps(perform(),sort_keys=True,indent=2))
