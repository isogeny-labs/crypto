#!/usr/bin/env python3
"""Scaled analogs of E-222 (Edwards, d small, #E=4*prime) and M-221 (Montgomery,
A small, #E=8*prime) plus a cofactor-1 Weierstrass control.  Each instance is
emitted in the `ic` prime-example.json schema (Weierstrass model, group
isomorphic to the Edwards/Montgomery curve), with a planted known log.
Deterministic from --seed.  Point counting: BSGS on random points inside the
Hasse interval (needs ord(P) > 4*sqrt(p), checked)."""
import argparse, json, random, sys, hashlib

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def isqrt(n):
    import math; return math.isqrt(n)

def legendre(a,p): return pow(a%p,(p-1)//2,p)

def sqrt_mod(a,p):
    a%=p
    if a==0: return 0
    if legendre(a,p)!=1: return None
    if p%4==3: return pow(a,(p+1)//4,p)
    q,s=p-1,0
    while q%2==0: q//=2; s+=1
    z=2
    while legendre(z,p)!=p-1: z+=1
    m,c,t,r=s,pow(z,q,p),pow(a,q,p),pow(a,(q+1)//2,p)
    while t!=1:
        i,t2=0,t
        while t2!=1: t2=t2*t2%p; i+=1
        b=pow(c,1<<(m-i-1),p); m=i; c=b*b%p; t=t*c%p; r=r*b%p
    return r

# Weierstrass y^2=x^3+ax+b affine arithmetic; None = infinity
def add(P,Q,a,p):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else: l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)
def neg(P,p): return None if P is None else (P[0],(-P[1])%p)
def mul(k,P,a,p):
    R=None
    if k<0: k=-k; P=neg(P,p)
    while k:
        if k&1: R=add(R,P,a,p)
        P=add(P,P,a,p); k>>=1
    return R

def rand_point(a,b,p,rng):
    while True:
        x=rng.randrange(p); y=sqrt_mod(x**3+a*x+b,p)
        if y is not None: return (x,y)

def count_order_candidates(P,a,p):
    """all N in [p+1-2sqrt p, p+1+2sqrt p] with N*P=O (BSGS)."""
    s=2*isqrt(p)+2; lo=p+1-s; hi=p+1+s
    m=isqrt(hi-lo)+1
    # lo + i*m + j ; baby steps j*P, giant (lo+i*m)P
    baby={}; R=None
    for j in range(m+1):
        baby.setdefault(R,[]).append(j); R=add(R,P,a,p)
    out=[]; G=mul(lo,P,a,p); step=mul(m,P,a,p)
    for i in range(0,(hi-lo)//m+2):
        # want G + j*P = O  => j*P = -G
        t=neg(G,p)
        for j in baby.get(t,[]):
            N=lo+i*m+j
            if lo<=N<=hi: out.append(N)
        G=add(G,step,a,p)
    return sorted(set(out))

def curve_order(a,b,p,rng,tries=6):
    cand=None
    for _ in range(tries):
        P=rand_point(a,b,p,rng)
        c=set(count_order_candidates(P,a,p))
        cand=c if cand is None else cand&c
        if len(cand)==1: return cand.pop()
        if not cand: raise RuntimeError("no order candidate")
    return None

def edwards_to_weier(d,p):
    # x^2+y^2=1+d x^2y^2  -> Montgomery B v^2=u^3+A u^2+u, A=2(1+d)/(1-d), B=4/(1-d)
    inv=pow(1-d,-1,p); A=2*(1+d)*inv%p; B=4*inv%p
    return mont_to_weier(A,B,p)

def mont_to_weier(A,B,p):
    # B y^2=x^3+Ax^2+x ; X=Bx,Y=B^2 y : Y^2=X^3+AB X^2+B^2 X ; X=u-a2/3
    a2=A*B%p; a4=B*B%p; i3=pow(3,-1,p)
    a=(a4-a2*a2*i3)%p
    b=(2*pow(a2,3,p)*pow(27,-1,p)-a2*a4*i3)%p
    return a,b

def prime_near(bits,rng_state_c=0):
    """p=2^bits-c, c smallest odd c>0 with p prime and p%4==3 preferred? any."""
    c=1
    while not is_prime((1<<bits)-c): c+=2
    return (1<<bits)-c, c

def factor_small(n,limit=1<<20):
    f={}; q=2
    while q<=limit and q*q<=n:
        while n%q==0: f[q]=f.get(q,0)+1; n//=q
        q+=1
    if n>1: f[n]=f.get(n,0)+1
    return f

def make(family,n,idx,seed):
    """family E/M/W; n target prime-order bits; idx index of curve."""
    rng=random.Random(f"{family}:{n}:{idx}:{seed}")
    h={"E":4,"M":8,"W":1}[family]
    pbits=n+{"E":2,"M":3,"W":0}[family]
    p,c=prime_near(pbits)
    # p must be larger-ish than random; search parameter
    for tr in range(4000):
        if family=="E":
            d=rng.randrange(2,1<<18)
            if legendre(d,p)==1 or d%p==0: continue
            a,b=edwards_to_weier(d,p); par={"d":d}
        elif family=="M":
            A=rng.randrange(3,1<<18)
            if (A*A-4)%p==0: continue
            a,b=mont_to_weier(A,1,p); par={"A":A,"B":1}
        else:
            a=rng.randrange(1,p); b=rng.randrange(1,p); par={}
        if (4*a**3+27*b*b)%p==0: continue
        N=curve_order(a,b,p,rng)
        if N is None or N%h: continue
        r=N//h
        if not is_prime(r) or r.bit_length()!=n and r.bit_length()!=n+1: continue
        if abs(p+1-N)==0: continue
        # find generator of prime-order subgroup
        while True:
            Q=rand_point(a,b,p,rng); G=mul(h,Q,a,p)
            if G is not None: break
        if mul(r,G,a,p) is not None: continue
        k=rng.randrange(2,r-1); T=mul(k,G,a,p)
        return {"schema_version":1,"name":f"{family}{n}-{idx}","family":family,
          "field":{"kind":"prime","modulus":str(p)},"a":str(a),"b":str(b),
          "subgroup_order":str(r),"cofactor":str(h),
          "generator":{"x":str(G[0]),"y":str(G[1])},"point":{"x":str(T[0]),"y":str(T[1])},
          "fixture":{"known_log":str(k),"seed":seed},
          "source_params":{"p_form":f"2^{pbits}-{c}",**par},"group_order":str(N),"r_bits":r.bit_length()}
    raise RuntimeError(f"no instance {family} {n} {idx}")

if __name__=="__main__":
    ap=argparse.ArgumentParser()
    ap.add_argument("--out",required=True); ap.add_argument("--families",default="E,M,W")
    ap.add_argument("--bits",default="24,28,32"); ap.add_argument("--per",type=int,default=8)
    ap.add_argument("--seed",default="e222m221-v1")
    a=ap.parse_args()
    import os; os.makedirs(a.out,exist_ok=True); man=[]
    for fam in a.families.split(","):
        for n in map(int,a.bits.split(",")):
            for i in range(a.per):
                inst=make(fam,n,i,a.seed)
                # verify: T = k*G, [r]G=O, group order = h*r, discriminant
                p=int(inst["field"]["modulus"]); aa=int(inst["a"]); bb=int(inst["b"]); r=int(inst["subgroup_order"])
                G=(int(inst["generator"]["x"]),int(inst["generator"]["y"])); T=(int(inst["point"]["x"]),int(inst["point"]["y"]))
                assert mul(r,G,aa,p) is None and mul(int(inst["fixture"]["known_log"]),G,aa,p)==T
                assert (G[1]**2-G[0]**3-aa*G[0]-bb)%p==0
                fn=f"{inst['name']}.json"; s=json.dumps(inst,indent=1)
                open(os.path.join(a.out,fn),"w").write(s)
                man.append({"file":fn,"sha256":hashlib.sha256(s.encode()).hexdigest(),"p":inst["field"]["modulus"],"r_bits":inst["r_bits"],"family":fam})
                print(fn,"p",p.bit_length(),"r",inst["r_bits"],file=sys.stderr)
    json.dump(man,open(os.path.join(a.out,"MANIFEST.json"),"w"),indent=1)
