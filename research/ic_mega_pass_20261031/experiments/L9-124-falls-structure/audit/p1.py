import sys; sys.dont_write_bytecode=True
from aud_check import load, rank_basis
from collections import Counter
pref=sys.argv[1]; k=int(sys.argv[2])
monos,R2=load(pref,2); _,R3=load(pref,3)
B2=rank_basis(R2); B3=rank_basis(R3)
new=sorted(set(B3)-set(B2)); print('dims',len(B2),len(B3),'new pivots',len(new),'pivots lost',len(set(B2)-set(B3)))
mk=(1<<k)-1
bd=Counter(); deg=Counter()
for lb in new:
    m=monos[lb.bit_length()-1]; deg[bin(m).count('1')]+=1; bd[(bin(m&mk).count('1'),bin(m>>k).count('1'))]+=1
print('degree',dict(deg),'bidegree',dict(bd))
# swap stability: map each monomial a<->b; membership via echelon reduce
idx={m:i for i,m in enumerate(monos)}
sw=[idx[((m&mk)<<k)|(m>>k)] for m in monos]
def swap(r):
    o=0
    while r:
        lb=r&-r; o|=1<<sw[lb.bit_length()-1]; r^=lb
    return o
def inspan(B,r):
    while r:
        lb=r&-r; q=B.get(lb)
        if q is None: return False
        r^=q
    return True
print('swap-stable W2',all(inspan(B2,swap(r)) for r in R2),'W3',all(inspan(B3,swap(r)) for r in R3))
# reversed-order split: pivot = highest bit instead; recompute
def hb(rows):
    p={}
    for r in rows:
        while r:
            h=r.bit_length()-1; q=p.get(h)
            if q is None: p[h]=r; break
            r^=q
    return p
H2=hb(R2);H3=hb(R3); nh=sorted(set(H3)-set(H2)); bd2=Counter()
for h in nh:
    m=monos[h]; bd2[(bin(m&mk).count('1'),bin(m>>k).count('1'))]+=1
print('highest-bit pivot convention: new',len(nh),dict(bd2))
