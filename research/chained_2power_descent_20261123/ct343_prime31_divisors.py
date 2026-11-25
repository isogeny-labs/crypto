#!/usr/bin/env python3
"""Bounded public Prime31 divisor-character qualification; no timing or DLP solver."""
import hashlib,json,pathlib,random,sys
import ct339 as c

def need(b,msg):
    if not b: raise ValueError(msg)
need(hashlib.sha256(pathlib.Path(c.__file__).read_bytes()).hexdigest()=='c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695','CT339 pin')
p=c.P=2147483647;c.DELTA=3;c.F=c.odd_f();lc=c.F[-1];roots=[pow(i,-1,p) for i in range(1,6)]
c.CS=(c.ecoeff('+'),c.ecoeff('-'));c.TS=(c.ecpoint(c.CS[0],c.K(25)/4,c.K(0)),c.ecpoint(c.CS[1],c.K(4)/25,c.K(0)))
need(pow(c.DELTA,(p-1)//2,p)==p-1,'nonsquare delta')

def bit(q):
    q%=p;v=pow(q,(p-1)//2,p);need(v in (1,p-1),'zero squareclass');return int(v==p-1)
def ev(u,a):
    r=0
    for x in reversed(u):r=(r*a+x)%p
    return r

def coeff(D,a):
    u=list(D[0]);n=len(u)-1;v=ev(u,a)
    if v:return bit(pow(-lc,n,p)*v)
    w=c.pquot(u,[(-a)%p,1]);v=ev(w,a);need(v!=0,'repeated branch')
    q=pow(lc,3,p)*ev([i*c.F[i] for i in range(1,len(c.F))],a)*pow(-lc,n-1,p)*v
    return bit(-q if '--bad-branch' in sys.argv else q)

def factored(D,a):
    u,v=D;n=len(u)-1;rr=[] if n==0 else [c.K(-u[0])] if n==1 else c.root2(u)
    out=c.K(1)
    for z in rr:
        if z==c.K(a):
            t=c.K(1)
            for b in roots:
                if b!=a:t=t*(lc*(a-b))
            out=out*t
        else:out=out*(lc*(z-a))
    need(out.b==0,'norm did not descend');return bit(out.a)

def sig(D):return tuple(coeff(D,a) for a in roots)
def xor(a,b):return tuple(x^y for x,y in zip(a,b))
def category(D):
    u=D[0];n=len(u)-1
    if n==0:return 'identity'
    if n==1:return 'infinity' if u[0]==0 else 'ramified' if -u[0]%p in roots else 'affine'
    rr=c.root2(u)
    if rr[0].b:return 'nonsplit'
    if rr[0]==rr[1]:return 'repeated'
    return 'split'

def ext_sqrt(t):
    if t.b==0:
        r=c.sqrt_base(t.a)
        if r is not None:return c.K(r)
        r=c.sqrt_base(t.a*c.inv(c.DELTA))
        return None if r is None else c.K(0,r)
    n=c.sqrt_base(t.a*t.a-c.DELTA*t.b*t.b)
    if n is None:return None
    for s in (n,-n):
        x=c.sqrt_base((t.a+s)*c.inv(2))
        if x:
            z=c.K(x,t.b*c.inv(2*x));need(z*z==t,'extension sqrt');return z
    return None

rng=random.Random(34331);points=[]
for _ in range(4096):
    z=rng.randrange(1,p);y=c.sqrt_base(ev(c.F,z))
    if y:points.append(c.norm([-z,1],[y]))
    if len(points)==64:break
need(len(points)==64,'affine cohort')
nonsplit=[]
for _ in range(4096):
    z=c.K(rng.randrange(p),rng.randrange(1,p));y=ext_sqrt(c.peval(c.F,z))
    if y is None:continue
    b=y.b*c.inv(z.b)%p;a=(y.a-b*z.a)%p
    D=c.norm([z.a*z.a-c.DELTA*z.b*z.b,-2*z.a,1],[a,b]);need(category(D)=='nonsplit','nonsplit construction');nonsplit.append(D)
    if len(nonsplit)==32:break
need(len(nonsplit)==32,'nonsplit cohort')
W=[c.O]+[c.norm([-a,1],[]) for a in roots];K1=W[5];K2=c.jadd(W[1],W[4]);K3=c.jadd(W[2],W[3]);ker=[c.O,K1,K2,K3]
need(c.jadd(K1,K2)==K3,'kernel sum')
ks=[sig(K) for K in ker]
need(ks==[(0,0,0,0,0),(1,0,0,1,0),(0,1,1,0,0),(1,1,1,1,0)],'kernel signatures')
indices=(0,0) if '--bad-pair' in sys.argv else (0,1)
need(len({tuple(s[i] for i in indices) for s in ks})==4,'pair rank')
for K in ker:need(c.jmul(K,2)==c.O and c.image(K)[0]==(None,None),'kernel map')
doubles=[c.jadd(D,D) for D in points];pairs=[(rng.choice(points+nonsplit+W),rng.choice(points+nonsplit+W)) for _ in range(64)];sums=[c.jadd(A,B) for A,B in pairs]
inf=[c.norm([0,1],[1]),c.norm([0,1],[-1])]
cohort=points+doubles+nonsplit+W+inf+sums;cats={};seen=set();shifts=0
for D in cohort:
    need(D==c.norm(*D),'Mumford membership');s=sig(D);need(s==tuple(factored(D,a) for a in roots),'factored character disagreement')
    cat=category(D);cats[cat]=cats.get(cat,0)+1;seen.add(D);im=c.image(D)[0]
    for K,k in zip(ker,ks):
        B=c.jadd(D,K);need(sig(B)==xor(s,k),'kernel character shift');need(c.image(B)[0]==im,'kernel coset image');shifts+=1
for D in doubles:need(sig(D)==(0,)*5,'doubling character')
for (A,B),D in zip(pairs,sums):need(sig(D)==xor(sig(A),sig(B)),'addition character')
need(set(cats)=={'identity','infinity','ramified','affine','nonsplit','repeated','split'},'category coverage')
print(json.dumps({'schema':'ct343-prime31-divisors/v1','p':p,'cohort':len(cohort),'unique':len(seen),'categories':cats,'coefficient_vs_factored':5*len(cohort),'kernel_coset_checks':shifts,'double_checks':len(doubles),'addition_checks':len(pairs),'kernel_signatures':ks},sort_keys=True,separators=(',',':')))
