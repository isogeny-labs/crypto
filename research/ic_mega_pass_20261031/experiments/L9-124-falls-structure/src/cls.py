"""P0-P2: canonical new pivots of round r vs r-1, bidegree classification, swap stability."""
import sys
from an9 import *
pref=sys.argv[1]; k=int(sys.argv[2]); r=int(sys.argv[3]) if len(sys.argv)>3 else 3
monos,R2=load(pref,r-1); _,R3=load(pref,r)
B2=basis(R2); B3=basis(R3)
newp=sorted(set(B3.p)-set(B2.p)); lost=set(B2.p)-set(B3.p)
from collections import Counter
lm=[monos[p.bit_length()-1] for p in newp]
cls=Counter(bideg(m,k) for m in lm)
idx={m:i for i,m in enumerate(monos)}
sw=lambda row: row_of([swapmono(m,k) for m in poly_of(row,monos)],idx)
sw3=sum(1 for row in R3 if B3.red(sw(row)))   # rows whose swap leaves W3
sw2=sum(1 for row in R2 if B2.red(sw(row)))
# bidegree content of degree-3 leading forms of the new space: top part of U mod top parts of W2
deg=[bin(m).count('1') for m in monos]
top3mask=0
for i,m in enumerate(monos):
    if deg[i]==3: top3mask|=1<<i
# variables in leading monomials
va=Counter(); 
for m in lm:
    for v in range(2*k):
        if m>>v&1: va[v]+=1
print(json.dumps(dict(dim_prev=len(B2),dim=len(B3),new=len(newp),lost=len(lost),bideg_of_new_LM={str(a):b for a,b in sorted(cls.items())},
  swap_violations_W3=sw3,swap_violations_W2=sw2,var_use=[va[v] for v in range(2*k)],LM=[[i for i in range(2*k) if m>>i&1] for m in lm])))
