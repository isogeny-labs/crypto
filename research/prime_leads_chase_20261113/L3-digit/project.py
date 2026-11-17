# Derived projections: cost exponent from a degree model. cols(D) = #{e in [0,b-1]^n : sum e <= D}, n = m k.
import math
from functools import lru_cache
def cols(n,b,D):
    # number of exponent vectors with sum <= D, via DP on log scale (exact big ints)
    poly=[1]
    for _ in range(n):
        new=[0]*min(len(poly)+b-1, D+1)
        for i,c in enumerate(poly):
            if c==0: continue
            for e in range(b):
                if i+e<=D: new[i+e]+=c
        poly=new
    return sum(poly)
def E(m,theta):
    return 2/(m+1-theta) if theta < m-1 else float('inf')
def lb_model(b,k): return math.ceil(k*(b-1)/2)+1          # Theorem S / conditional b-ary
def ff_model(m,k):  return m*2**(m-1) + (k-2**(m-1))//2 + 1  # cited first-fall closed form (bool)
out=[]
for b in (2,3,4):
    for m in (2,3,4,5,6,8,9,12,16):
        for k in ([40,80,160] if b==2 else [30,60,120]):
            n=m*k
            D=lb_model(b,k)
            lc=math.log(cols(n,b,D))/ (k*math.log(b))
            out.append((b,m,k,'LB',D,round(lc,3),round(E(m,lc),3),round(E(m,2*lc),3)))
for m in (4,6,8,9,12):
    for k in (64, 256, 1024) if m<9 else (4096,16384):
        n=m*k; D=ff_model(m,k)
        if D>=n: out.append((2,m,k,'FF',D,'sat','-','-')); continue
        # entropy approx for big n
        x=D/n; H=-(x*math.log2(x)+(1-x)*math.log2(1-x))
        lc=n*H/k
        out.append((2,m,k,'FF',D,round(lc,3),round(E(m,lc),3),round(E(m,2*lc),3)))
print("b m k model D theta1=log_B cols  E(omega=1) E(omega=2)")
for r in out: print(*r)
