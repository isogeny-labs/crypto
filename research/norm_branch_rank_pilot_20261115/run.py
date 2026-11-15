#!/usr/bin/env python3
"""Exact Fp norm-branch rank pilot; diagnostic only, not an ECDLP solver."""
import itertools as it
import json
import math
import random
import sys


def points(p, a, b, n, rng):
    assert p % 4 == 3 and (4*a**3 + 27*b*b) % p
    out = []
    seen = set()
    while len(out) < n:
        x = rng.randrange(p)
        t = (x*x*x+a*x+b) % p
        if x in seen or not t or pow(t, (p-1)//2, p) != 1:
            continue
        y = pow(t, (p+1)//4, p)
        y = y if rng.randrange(2) else -y % p
        seen.add(x)
        out.append((x, y))
    return out


def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    x,y = P; u,v = Q
    if x == u and (y+v) % p == 0: return None
    if x == u:
        s = ((3*x*x+a)*pow(2*y, -1, p)) % p
    else:
        s = ((v-y)*pow(u-x, -1, p)) % p
    z = (s*s-x-u) % p
    return z, (s*(x-z)-y) % p


def signed_sum(Ps, signs, p, a):
    z = None
    for (x,y), s in zip(Ps, signs):
        z = add(z, (x, y if s == 1 else -y % p), p, a)
    return z


def vand(xs, inds, p):
    z = 1
    for i,j in it.combinations(inds,2): z = z*(xs[j]-xs[i]) % p
    return z


def coeffs(Ps, p, target_sign=1):
    d = len(Ps); k = d-1; u = d//2+1; v = (d-1)//2
    xs = [x for x,y in Ps]; ys = [y for x,y in Ps]
    assert len(set(xs)) == d and all(ys) and u+v == d
    co = {}
    for S in it.combinations(range(d),v):
        Sc = sorted(set(range(d))-set(S))
        z = vand(xs,Sc,p)*vand(xs,S,p)*math.prod(ys[i] for i in S) % p
        z = z*(-1 if (sum(S)+v*u+v*(v-1)//2)%2 else 1) % p
        T = tuple(i for i in S if i < k)
        co[T] = z*(target_sign if k in S else 1) % p
        assert co[T] != 0
    return co



def check_squared_vandermonde(Ps, p, C):
    """Independently evaluate the closed-form coefficient identity."""
    d = len(Ps); v=(d-1)//2
    xs=[x for x,y in Ps]
    delta=vand(xs,range(d),p)
    scale=delta*(-1 if (v*(v-1)//2)%2 else 1)%p
    deriv=[math.prod((xs[i]-xs[j])%p for j in range(d) if i!=j)%p
           for i in range(d)]
    for T,c in C.items():
        S=T if len(T)==v else T+(d-1,)
        z=scale*pow(vand(xs,S,p),2,p)%p
        for i in S:
            z=z*Ps[i][1]*pow(deriv[i],-1,p)%p
        assert z==c


def matrix(Ps, signs, p, target_sign=1):
    d = len(Ps); u = d//2+1; v=(d-1)//2
    return [[pow(x,j,p) for j in range(u)] +
            [s*y*pow(x,j,p)%p for j in range(v)]
            for (x,y),s in zip(Ps, tuple(signs)+(target_sign,))]


def elimination(M,p):
    A = [[z%p for z in row] for row in M]
    piv = []
    for col in range(len(A[0])):
        i = len(piv)
        r = next((r for r in range(i,len(A)) if A[r][col]),None)
        if r is None: continue
        A[i],A[r] = A[r],A[i]
        q = pow(A[i][col],-1,p)
        A[i] = [t*q%p for t in A[i]]
        for r in range(len(A)):
            if r == i: continue
            q = A[r][col]
            if q: A[r] = [(x-q*y)%p for x,y in zip(A[r],A[i])]
        piv.append(col)
        if len(piv) == len(A): break
    return A,piv


def mul(F,G,p):
    H=[0]*(len(F)+len(G)-1)
    for i,f in enumerate(F):
        for j,g in enumerate(G): H[i+j]=(H[i+j]+f*g)%p
    return H


def norm_check(Ps, signs, p, a, b):
    M=matrix(Ps,signs,p)
    R,piv=elimination(M,p)
    free=next(j for j in range(len(M)) if j not in piv)
    sol=[0]*len(M); sol[free]=1
    for i,j in enumerate(piv): sol[j]=-R[i][free]%p
    assert all(sum(x*y for x,y in zip(row,sol))%p==0 for row in M)
    u=len(M)//2+1; A,B=sol[:u],sol[u:]
    F=mul(A,A,p); G=mul([b,a,0,1],mul(B,B,p),p)
    F += [0]*(max(len(G),len(F))-len(F))
    G += [0]*(len(F)-len(G))
    N=[(f-g)%p for f,g in zip(F,G)]
    prod=[1]
    for x,y in Ps: prod=mul(prod,[-x%p,1],p)
    N += [0]*(len(prod)-len(N))
    c=N[-1]
    assert c and N == [c*z%p for z in prod]


def semantic(seed=19):
    p,a,b,k=1019,2,3,8
    while True:
        rng=random.Random(seed)
        Ps=points(p,a,b,k,rng)
        planted=tuple(rng.choice((-1,1)) for _ in range(k))
        Q=signed_sum(Ps,planted,p,a)
        if Q is not None:
            target=(Q[0],-Q[1]%p)
            if target[1] and target[0] not in {x for x,y in Ps}: break
        seed+=1
    Ps.append(target)
    C=coeffs(Ps,p)
    check_squared_vandermonde(Ps,p,C)
    models=[]
    for s in it.product((-1,1),repeat=k):
        group=signed_sum(Ps,s+(1,),p,a) is None
        det=len(elimination(matrix(Ps,s,p),p)[1])<k+1
        poly=sum(z*math.prod(s[i] for i in T) for T,z in C.items())%p==0
        assert group==det==poly
        if group:
            models.append(s)
            norm_check(Ps,s,p,a,b)
    assert planted in models
    assert coeffs(Ps,p,-1)!=C
    for s in it.product((-1,1),repeat=k):
        flipped=sum(z*math.prod(s[i] for i in T) for T,z in coeffs(Ps,p,-1).items())%p==0
        assert flipped == (tuple(-z for z in s) in models)
    try: coeffs(Ps[:1]+[(Ps[0][0],Ps[1][1])]+Ps[2:],p)
    except AssertionError: pass
    else: raise AssertionError('duplicated x accepted')
    assert all((y*y-x*x*x-a*x-b)%p==0 for x,y in Ps)
    assert any((y*y-x*x*x-a*x-(b+1))%p for x,y in Ps)
    return dict(p=p,k=k,seed=seed,models=len(models),planted=list(planted))


def census(seed=19,max_k=18):
    p,a,b=10007,2,3
    rng=random.Random(seed)
    data=[]
    for k in range(6,max_k+1,2):
        Ps=points(p,a,b,k+1,rng)
        C=coeffs(Ps,p)
        check_squared_vandermonde(Ps,p,C)
        ms=[rng.randrange(1,p) for _ in Ps]
        D=coeffs([(x,y*m%p) for (x,y),m in zip(Ps,ms)],p)
        v=k//2
        for T,z in C.items():
            scale=math.prod(ms[i] for i in T)*(ms[-1] if len(T)==v-1 else 1)
            assert D[T]==z*scale%p
        L=list(range(k//2));R=list(range(k//2,k))
        for t in (v-1,v):
            for alpha in range(len(L)+1):
                beta=t-alpha
                if beta<0 or beta>len(R): continue
                ls=list(it.combinations(L,alpha));rs=list(it.combinations(R,beta))
                def block(co): return [[co[tuple(sorted(l+r))] for r in rs] for l in ls]
                N=block(C); Dm=block(D)
                n=random.Random((seed<<20)+(k<<10)+(t<<5)+alpha)
                null=[[n.randrange(1,p) for _ in row] for row in N]
                x=[n.randrange(1,p) for _ in ls];y=[n.randrange(1,p) for _ in rs]
                low=[[u*w%p for w in y] for u in x]
                nr=len(elimination(N,p)[1]); dr=len(elimination(Dm,p)[1]); rr=len(elimination(null,p)[1])
                assert nr==dr and len(elimination(low,p)[1])==1
                data.append(dict(k=k,t=t,alpha=alpha,beta=beta,rows=len(ls),cols=len(rs),natural=nr,scaled=dr,null=rr))
    return data


if __name__=='__main__':
    max_k=int(sys.argv[1]) if len(sys.argv)>1 else 18
    sem=semantic()
    data=census(max_k=max_k)
    assert census(max_k=max_k)==data
    print(json.dumps(dict(semantic=sem,blocks=len(data),deficits=[x for x in data if x['natural']<min(x['rows'],x['cols'])],natural_null_gaps=[x for x in data if x['natural']<x['null']],ranks=data),indent=2,sort_keys=True))
