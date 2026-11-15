"""Candidate invariants of (alpha,beta) at t=1 (the normal form), over F_p. Polynomials: lists low->high."""
import sys
sys.path.insert(0,'/home/user/crypto/research/leads_chase_20261112/tools'); sys.path.insert(0,'.')
from unit_test import s3_coeffs
from ec import mk
def trim(a):
    while a and a[-1]==0: a.pop()
    return a
def pmod(a,m,p):
    a=trim([x%p for x in a]); inv=pow(m[-1],p-2,p); dm=len(m)-1
    while len(a)-1>=dm and a:
        c=a[-1]*inv%p; d=len(a)-1
        for i,v in enumerate(m): a[d-dm+i]=(a[d-dm+i]-c*v)%p
        a=trim(a)
    return a
def pmul(a,b,p):
    r=[0]*(len(a)+len(b)-1) if a and b else []
    for i,x in enumerate(a):
        for j,y in enumerate(b): r[i+j]=(r[i+j]+x*y)%p
    return r
def ppow_mod(a,e,m,p):
    r=[1]; b=pmod(a,m,p)
    while e:
        if e&1: r=pmod(pmul(r,b,p),m,p)
        b=pmod(pmul(b,b,p),m,p); e>>=1
    return r
def pgcd(a,b,p):
    a=trim(a[:]); b=trim(b[:])
    while b: a,b=b,pmod(a,b,p)
    return a
def psub(a,b,p):
    n=max(len(a),len(b)); return trim([((a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0))%p for i in range(n)])
def ddf(h,p,maxd=4):
    """distinct-degree factorisation degrees pattern of squarefree-part of h: list of (deg,count)"""
    h=trim([x%p for x in h]); out=[]; xp=[0,1]; cur=h
    d=0
    while len(cur)-1>0 and d<maxd:
        d+=1; xp=ppow_mod(xp,p,cur,p)
        g=pgcd(cur,psub(xp,[0,1],p),p)
        if len(g)-1>0:
            out.append((d,(len(g)-1)//d))
            # divide cur by g
            q=[0]*(len(cur)-len(g)+1); r=cur[:]
            inv=pow(g[-1],p-2,p)
            while len(r)>=len(g):
                c=r[-1]*inv%p; q[len(r)-len(g)]=c
                for i,v in enumerate(g): r[len(r)-len(g)+i]=(r[len(r)-len(g)+i]-c*v)%p
                r=trim(r)
            cur=trim(q); xp=pmod(xp,cur,p) if cur else xp
    if len(cur)-1>0: out.append((len(cur)-1,1))
    return tuple(out)
def lcm(a,b):
    from math import gcd
    return a*b//gcd(a,b)
def divisors(n):
    fs={}; m=n; q=2
    while q*q<=m:
        while m%q==0: fs[q]=fs.get(q,0)+1; m//=q
        q+=1
    if m>1: fs[m]=fs.get(m,0)+1
    ds=[1]
    for q,e in fs.items(): ds=[d*q**i for d in ds for i in range(e+1)]
    return sorted(ds)
def order_in_ring(h,p,elem=None):
    """order of x (or elem) in F_p[x]/(h), h squarefree with irreducible factors of degree<=4, x invertible"""
    L=1
    for i in range(1,5): L=lcm(L,p**i-1)
    e=elem or [0,1]
    for d in divisors(L):
        if ppow_mod(e,d,h,p)==[1]: return d
    return None
def v2(n):
    k=0
    while n%2==0: n//=2; k+=1
    return k
def features(p,al,be,F=None,sqrt=None,add=None):
    a,b,t=al,be,1
    f=(1+a+be)%p; disc=(4*a**3+27*be*be)%p
    D={}
    D['alpha']=al; D['beta']=be; D['f']=f; D['disc']=disc
    D['j']=(1728*4*a**3*pow(disc,p-2,p))%p
    D['chi_f']=pow(f,(p-1)//2,p) if f else 0
    D['chi_disc']=pow(disc,(p-1)//2,p)
    D['chi_alpha']=pow(al,(p-1)//2,p) if al else 0
    D['chi_beta']=pow(be,(p-1)//2,p) if be else 0
    u=(-2*(1+a))%p; v=(-2*(a+2*be))%p; w=(a*a-4*be)%p
    D['u']=u;D['v']=v;D['w']=w
    D['chi_u']=pow(u,(p-1)//2,p) if u else 0; D['chi_v']=pow(v,(p-1)//2,p) if v else 0; D['chi_w']=pow(w,(p-1)//2,p) if w else 0
    D['tan']=(3+a)%p
    D['alpha2m']=(a*a-4*be)%p
    # cubic f(x)=x^3+a x+b: factor pattern, order of roots
    cub=[be%p,a%p,0,1]
    D['f_pattern']=ddf(cub,p)
    D['f_rootord']=order_in_ring(cub,p) if cub[0] else None
    D['f_root_v2']=v2(D['f_rootord']) if D['f_rootord'] else None
    D['f_root_odd']=D['f_rootord']//(2**D['f_root_v2']) if D['f_rootord'] else None
    # H(x)=S_3(x,x,1)
    co=s3_coeffs(p,a,be,1); H=[0]*5
    for (e1,e2),c in co.items(): H[e1+e2]=(H[e1+e2]+c)%p
    D['H_pattern']=ddf(H,p); 
    if H[0]%p: 
        D['H_rootord']=order_in_ring(H,p); D['H_root_v2']=v2(D['H_rootord']); 
    # multiplication orders of T
    if F:
        T=(F(1),sqrt(f)); P=None; n=0
        while n<2000:
            P=add(P,T,a); n+=1
            if P is None: break
        D['ordT']=n; D['ordT_v2']=v2(n); D['ordT_odd']=n//2**v2(n)
        P2=add(T,T,a); D['x2T']=(P2[0].u if P2 else None) if (P2 is None or P2[0].v==0) else ('ext',P2[0].u,P2[0].v)
    return D
