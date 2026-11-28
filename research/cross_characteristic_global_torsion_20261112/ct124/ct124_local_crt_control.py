#!/usr/bin/env python3
"""CT124: exact CRT false-positive and Hensel-lift controls (no dependencies)."""
import json

P=59
ROOTS=(19,20,22,43,46)
CURVES={'plus':(59,-1,60,-10,39),'minus':(59,58,60,-10,39)}

def trim(a):
    while len(a)>1 and a[-1]==0: a.pop()
    return a

def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])

def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): c[i+j]+=x*y
    return trim(c)

def neg(a): return [-x for x in a]
def sub(a,b): return add(a,neg(b))
def powpoly(a,n):
    b=[1]
    while n:
        if n&1:b=mul(b,a)
        a=mul(a,a);n//=2
    return b

def psi11(v):
    a1,a2,a3,a4,a6=v
    b2=a1*a1+4*a2;b4=2*a4+a1*a3;b6=a3*a3+4*a6
    b8=a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    c=[b6,2*b4,b2,4]; u=[b8,3*b6,3*b4,b2,3]
    w=[b4*b8-b6*b6,b2*b8-b4*b6,10*b8,10*b6,5*b4,b2,2]
    a5=sub(mul(w,powpoly(c,2)),powpoly(u,3))
    a6p=mul(u,sub(a5,powpoly(w,2)))
    a7=sub(mul(a5,powpoly(u,3)),mul(powpoly(w,3),powpoly(c,2)))
    return sub(mul(a7,powpoly(a5,3)),mul(mul(w,powpoly(a6p,3)),powpoly(c,2)))

def ev(f,x,mod=None):
    y=0
    for a in reversed(f): y=(y*x+a) if mod is None else (y*x+a)%mod
    return y

def derivative(f):return [i*f[i] for i in range(1,len(f))]

def hensel(f,r,k=4):
    x=r
    d=ev(derivative(f),r,59)
    assert d!=0
    powers=[]
    for i in range(1,k+1):
        assert ev(f,x,59**i)==0
        powers.append(x)
        if i<k:
            quo=(ev(f,x,59**(i+1))//(59**i))%59
            x+=59**i*((-quo*pow(d,-1,59))%59)
    return powers

# General Weierstrass group law modulo 59, source E: y^2 + y = x^3 - x^2 - 10x - 20.
def curve_y(x):
    return [y for y in range(59) if (y*y+y-(x**3-x*x-10*x-20))%59==0]

def plus(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v+1)%59==0:return None
    if x==u:
        den=(2*y+1)%59
        assert den!=0
        lam=((3*x*x-2*x-10)*pow(den,-1,59))%59
    else:lam=((v-y)*pow((u-x)%59,-1,59))%59
    rx=(lam*lam+1-x-u)%59
    ry=(-lam*(rx-x)-y-1)%59
    assert (ry*ry+ry-(rx**3-rx*rx-10*rx-20))%59==0
    return rx,ry

def order_eleven_generator():
    for x in ROOTS:
        for y in curve_y(x):
            G=(x,y);R=None; orbit=[]
            for k in range(1,12):
                R=plus(R,G)
                if k<11:
                    assert R is not None
                    orbit.append((k,R))
            if R is None and sorted(set(t[1][0] for t in orbit))==list(ROOTS):return G,orbit
    raise AssertionError('no order-11 generator')

def demo():
    G,orbit=order_eleven_generator()
    expected_x={k:pt[0] for k,pt in orbit}
    answer={'ct':'CT124','source_generator':list(G),'source_x_by_scalar':{str(k):expected_x[k] for k in sorted(expected_x)},'models':{}}
    for name,aa in CURVES.items():
        f=psi11(aa)
        assert len(f)==61 and f[-1]==11
        lifts={str(r):hensel(f,r) for r in ROOTS}
        assert all(all(ev(f,v,59**(i+1))==0 for i,v in enumerate(vs)) for vs in lifts.values())
        assert all(ev(derivative(f),r,59)!=0 for r in ROOTS)
        cases=[]
        for k,point in orbit:
            r=point[0]
            # H_r(X)=60r+59X. Source: H_r(r) == r (mod59).
            # Binary: H_r(X) == X (mod2), independent of the public input.
            assert (60*r+59*r-r)%59==0
            assert (60*r)%2==0 and 59%2==1
            assert ev(f,r,59)==0
            # This affine substitution is NOT a global root of f:
            # f(60r+59X) cannot be divisible by squarefree f(X) in Q[X],
            # since a slope-59 affine map cannot permute a finite root set.
            assert 58*f[-2] != 60*11*(60*r)  # explicit X^59 coefficient witness
            recovered_candidates=(1,10) # target x(T) represents +/-1
            verified=any(plus(None,orbit[t-1][1])==point for t in recovered_candidates)
            assert verified==(k in recovered_candidates)
            cases.append({'m':k,'source_x':r,'source_local_root':True,'binary_local_root':True,'scalar_recovered_up_to_sign':1,'source_verification':verified})
        assert sum(c['source_verification'] for c in cases)==2
        answer['models'][name]={'hensel_root_mod_59_powers':lifts,'pass_local':len(cases),'pass_e2e':sum(c['source_verification'] for c in cases),'cases':cases}
    return answer

if __name__=='__main__':print(json.dumps(demo(),sort_keys=True,separators=(',',':')))
