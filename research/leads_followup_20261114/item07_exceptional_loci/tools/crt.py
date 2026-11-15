import sys,json
sys.path.insert(0,'.')
from fitcubic import fit
from math import gcd
def ratrec(r,M):
    # rational reconstruction
    r0,r1=M,r%M; s0,s1=0,1
    bound=int((M//2)**0.5)
    while r1>bound:
        q=r0//r1; r0,r1=r1,r0-q*r1; s0,s1=s1,s0-q*s1
    if s1==0 or abs(s1)>bound: return None
    return (r1,s1) if s1>0 else (-r1,-s1)
primes=[int(x) for x in sys.argv[1:]]
co={}
for p in primes:
    m,N,pts,R=fit(p)
    assert len(N)==1,(p,len(N))
    v=N[0]; v=v*pow(int(v[m.index((2,1))]),p-2,p)%p
    co[p]={m[i]:int(v[i]) for i in range(len(m))}
mons=sorted(co[primes[0]])
M=1
for p in primes: M*=p
for e in mons:
    x=0
    for p in primes:
        Mp=M//p; x+=co[p][e]*Mp*pow(Mp,-1,p)
    x%=M
    print(e,x,ratrec(x,M))
