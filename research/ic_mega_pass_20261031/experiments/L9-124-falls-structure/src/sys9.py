"""L9 systems: N1 lib.system arms + coefficient/structure variants of A built from the SAME V,z,a6 of draw s."""
import sys, os, random
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'))
import lib
from tensor_closure import pmulmod, modulus, rand_subspace, to_eqs
def base(n,k,s):
    rng=random.Random(1000*n+100*k+s); m=modulus(n)
    V=rand_subspace(n,k,rng); z=rng.randrange(1,1<<n); a6=rng.randrange(1,1<<n); return m,V,z,a6
def variant(n,k,s,name):
    if name in ('A','C1','D1','B','C2'): return lib.system(n,k,s)[name]
    m,V,z,a6=base(n,k,s); mul=lambda x,y: pmulmod(x,y,m,n); sq=lambda x: mul(x,x)
    L=lambda x: sq(x)^mul(z,x); z2=sq(z)
    vr=random.Random(9_000_000+1000*n+100*k+s)
    V2=V
    if name=='A_V1V2': V2=rand_subspace(n,k,vr)
    if name=='A_Lrand':
        Mr=[vr.getrandbits(n) for _ in range(n)]
        def Lb(x):
            r=0
            for i in range(n):
                if x>>i&1: r^=Mr[i]
            return r
    else: Lb=L
    bilf={'A_ab':lambda x:x,'A_sq':sq}.get(name,Lb)
    bil=[[bilf(mul(V[j],V2[l])) for l in range(k)] for j in range(k)]
    affa=[mul(z2,sq(v)) for v in V]; affb=[mul(z2,sq(v)) for v in V2]
    if name=='A_noaff': affa=[0]*k; affb=[0]*k
    if name=='A_randaff': affa=[vr.getrandbits(n) for _ in range(k)]; affb=affa[:]
    if name=='A_lin': affa=[mul(z2,v) for v in V]; affb=affa[:]   # z^2(a+b) instead of z^2(a+b)^2
    return to_eqs(n,k,bil,affa,affb,a6)
