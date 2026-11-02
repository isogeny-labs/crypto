#!/usr/bin/env python3
"""L09: generate prime-order short-Weierstrass curves (cofactor 1) and run the
patched Rust crate example (rust-dump/examples/btn_dump.rs) to dump the m=2 Bochum network."""
import random, subprocess, sys, os, math, json
def isprime(n):
    if n<2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n%q==0: return n==q
    d=n-1;s=0
    while d%2==0: d//=2;s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x=pow(a,d,n)
        if x in(1,n-1): continue
        for _ in range(s-1):
            x=x*x%n
            if x==n-1: break
        else: return False
    return True
def count(p,a,b):
    sq={}
    for y in range(p): sq[y*y%p]=sq.get(y*y%p,0)+1
    n=1
    for x in range(p): n+=sq.get((x**3+a*x+b)%p,0)
    return n
def add(P,Q,a,p):
    if P is None: return Q
    if Q is None: return P
    (x1,y1),(x2,y2)=P,Q
    if x1==x2 and (y1+y2)%p==0: return None
    l=(3*x1*x1+a)*pow(2*y1,-1,p)%p if P==Q else (y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)
def mul(k,P,a,p):
    R=None
    while k:
        if k&1: R=add(R,P,a,p)
        P=add(P,P,a,p); k>>=1
    return R
def find(bits,rng):
    while True:
        p=rng.randrange(1<<(bits-1),1<<bits)
        if not isprime(p) or p<7: continue
        for _ in range(200):
            a=rng.randrange(p);b=rng.randrange(p)
            if (4*a**3+27*b*b)%p==0: continue
            N=count(p,a,b)
            if isprime(N) and N>3:
                # point
                while True:
                    x=rng.randrange(p);r2=(x**3+a*x+b)%p
                    if pow(r2,(p-1)//2,p)==1 and r2!=0:
                        # sqrt (try brute for small p; Tonelli simple for p%4==3 else brute)
                        y=next(y for y in range(p) if y*y%p==r2) if p<(1<<17) else None
                        if y is None: continue
                        return p,a,b,N,(x,y)
def main():
    out=sys.argv[1]; bits=[int(x) for x in sys.argv[2].split(',')]
    exe=os.environ["TMPDIR"]+"/l09-crate-target/release/examples/btn_dump"
    rng=random.Random(20261031)
    cases=[(17,2,2,19,(5,1))]
    meta=[]
    for b in bits: cases.append(find(b,rng)[:3]+(None,)) if False else None
    cases=[("p17",)+cases[0]]
    for b in bits:
        p,a,bb,N,G=find(b,rng); cases.append((f"b{b}",p,a,bb,N,G))
    for name,p,a,b,r,G in cases:
        s=next(q for q in range(2*p,4*p+1) if isprime(q))
        tau=min(s,-(-s*max(math.isqrt(r)+1,20)//(r-1)))
        T=mul(rng.randrange(1,r),G,a,p)
        f=f"{out}/bochum_m2_{name}.json"
        args=[exe,f,p,a,b,r,G[0],G[1],T[0],T[1],s,tau,3,1]
        print(name,"p=",p,"r=",r,"s=",s,"tau=",tau,flush=True)
        subprocess.run([str(x) for x in args],check=True)
        json.dump(dict(p=p,a=a,b=b,r=r,G=G,T=T,hash_prime=s,tau=tau,alpha=3,beta=1,bits=p.bit_length(),m=2),open(f.replace('.json','.params.json'),'w'))
main()
