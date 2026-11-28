# Checks: (1) the u128 guarded-subtraction test == u64 test == exponentwise divisibility for n<=8, exps<=127
# (2) 4-lane chunked first-match (movemask/tzcnt model) returns the same first index as the linear scan.
import random
def pack(e): return sum(x<<(8*i) for i,x in enumerate(e))
def G(n): return sum(0x80<<(8*i) for i in range(n))
M128=(1<<128)-1; M64=(1<<64)-1
def d128(a,b,g): return (((b|g)-a)&M128)&g==g
def d64(a,b,g): return (((b|g)-a)&M64)&g==g
random.seed(1); bad=0
for t in range(200000):
    n=random.randint(1,8); g=G(n); hi=random.choice([3,10,30,127])
    ea=[random.randint(0,hi) for _ in range(n)]; eb=[random.randint(0,hi) for _ in range(n)]
    if random.random()<0.3: ea=[random.randint(0,x) for x in eb]
    a,b=pack(ea),pack(eb); ref=all(x<=y for x,y in zip(ea,eb))
    bad+= (d128(a,b,g)!=ref) + (d64(a,b,g)!=ref)
print('pairwise mismatches',bad)
bad=0
for t in range(20000):
    n=6; g=G(n); L=[pack([random.randint(0,4) for _ in range(n)]) for _ in range(random.randint(0,45))]
    b=pack([random.randint(0,8) for _ in range(n)])
    lin=next((k for k,a in enumerate(L) if d128(a,b,g)),None)
    ch=None
    for c in range(0,len(L),4):
        mask=sum(1<<j for j,a in enumerate(L[c:c+4]) if d64(a,b,g))
        if mask: ch=c+((mask&-mask).bit_length()-1); break
    bad+= lin!=ch
    # counted probes identical: k+1 or len
print('first-index mismatches',bad)
