# Re-derivation of the cost-exponent projections (model-relative: T_dec = cols(D)^omega, cols = dim A_{<=D}).
# E_pre  = protocol formula 2/(m+1-theta)  (implicitly lets B exceed p^(1/m), i.e. fractional trials per relation)
# E_1rel = one relation per trial (B <= p^(1/m)):  2/(m+1-theta) if theta<=1, (1+theta)/m if 1<theta<m-1, else >=1
import math
from math import comb, log2
def H(x): return 0 if x<=0 or x>=1 else -(x*log2(x)+(1-x)*log2(1-x))
def log2cols_bool(n,D):
    D=min(D,n); s=sum(comb(n,i) for i in range(D+1)); return math.log2(s)
def E_pre(m,th): return 2/(m+1-th) if th<m-1 else float('inf')
def E_1rel(m,th):
    if th<=1: return 2/(m+1-th)
    if th<m-1: return (1+th)/m
    return float('inf')
print("(a) LB model (Theorem S: D=ceil(k/2)+1), Boolean, k=160:  m theta  E_pre(w=1) E_1rel(w=1) E_pre(w=2) E_1rel(w=2)")
for m in range(2,17):
    k=160; D=math.ceil(k/2)+1; th=log2cols_bool(m*k,D)/k
    print(f"  m={m:2d} theta={th:.3f}  {E_pre(m,th):.3f} {E_1rel(m,th):.3f}   {E_pre(m,2*th):.3f} {E_1rel(m,2*th):.3f}")
print("(b) smallest m with E<0.5 (LB model, k=160): ")
for w in (1,2):
    for name,Ef in (("E_pre",E_pre),("E_1rel",E_1rel)):
        for m in range(2,40):
            k=160; D=math.ceil(k/2)+1; th=w*log2cols_bool(m*k,D)/k
            if Ef(m,th)<0.5: print(f"  omega={w} {name}: m>={m}"); break
print("(c) p = 2^256, Boolean digits, one relation per trial, minimise over k (B=2^k <= p^(1/m)) the max(relation, LA) cost;")
print("    D = proven bound: ffd = m e + floor((k-e)/2)+1 if k>=e (Theorem F), else ceil(k/2)+1 (Theorem S); omega=1")
L=256
for m in range(2,13):
    e=2**(m-1); best=None
    for k in range(1,L//m+1):
        D = m*e+(k-e)//2+1 if k>=e else math.ceil(k/2)+1
        n=m*k; lc=log2cols_bool(n,D)
        rel = k + max(0, L-m*k) + lc          # log2( B * trials * T_dec )
        la = 2*k
        c=max(rel,la)
        if best is None or c<best[0]: best=(c,k,D,lc/k)
    c,k,D,th=best
    print(f"  m={m:2d}: best k={k:3d} D={D:4d} theta={th:.2f} log2 cost={c:.1f}  E={c/L:.3f}  (bound used: {'ffd' if k>=e else 'Thm S'})")
print("(d) same at p=2^256 but with the protocol's fractional-trial accounting (B may exceed p^(1/m), trials = p/B^m):")
for m in range(2,13):
    e=2**(m-1); best=None
    for k in range(1,L+1):
        if m*k> 4*L: break
        D = m*e+(k-e)//2+1 if k>=e else math.ceil(k/2)+1
        n=m*k; lc=log2cols_bool(n,D)
        rel = k + (L-m*k) + lc; la=2*k
        c=max(rel,la)
        if best is None or c<best[0]: best=(c,k,D,lc/k)
    c,k,D,th=best
    print(f"  m={m:2d}: best k={k:3d} D={D:4d} theta={th:.2f} E={c/L:.3f}")
print("(e) Hypothesis-G coverage (assumes the closure note's 'G needed only at theta - Delta >= tau_m*m*k' with Delta = deg f = m e):")
for m,tau in ((3,0.0246),(4,0.0417),(6,0.0615),(8,0.0725)):
    e=2**(m-1)
    ks=[k for k in range(e,4000) if tau*m*k <= (k-e)//2]
    print(f"  m={m}: tau*m={tau*m:.3f}; Theorem F covers the needed degree (tau*m*k <= j*-1) for k >= {ks[0] if ks else 'never (tau*m >= 1/2)'}; 256-bit p allows k <= {256//m}")
