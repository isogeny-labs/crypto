# Derived, from the committed ECC2K-130 descent cost model (same formulae), NOT a measurement.
# Typed variant: m disjoint per-summand coset bases of 2^l points each (columns m*2^l, no m! in the yield).
from math import comb, factorial, log2
n=131; RHO=60.81
def ladd(a,b): return max(a,b)+log2(1+2**(-abs(a-b)))
def untyped(m,l,D,w):
    N=(m-2)*n+m*l
    tg=l+max(0.0,log2(factorial(m))+n-m*l)
    orc=w*log2(sum(comb(N,i) for i in range(D+1)))
    la=log2(m)+2*l
    return ladd(tg+orc,la)
def typed(m,l,D,w):
    N=(m-2)*n+m*l
    cols=l+log2(m)                      # log2 of columns
    tg=cols+max(0.0,log2(m)+n-m*l-(0))  # relations needed = columns; queries per relation = 2^n/2^{ml}; careful below
    # queries per relation = 2^(n-m*l) (typed: no m!); relations needed = m*2^l
    tg=log2(m)+l+max(0.0,n-m*l)
    orc=w*log2(sum(comb(N,i) for i in range(D+1)))
    la=log2(m)+2*cols
    return ladd(tg+orc,la)
for m in (3,4):
  for D in (4,5):
    for w in (1,2):
        bu=min((untyped(m,l,D,w),l) for l in range(4,70))
        bt=min((typed(m,l,D,w),l) for l in range(4,70))
        print(f"m={m} D={D} w={w}: untyped 2^{bu[0]:.2f} (l={bu[1]})  typed 2^{bt[0]:.2f} (l={bt[1]})  typed-untyped {bt[0]-bu[0]:+.2f} bits")
