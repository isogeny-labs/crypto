#!/usr/bin/env python3
"""stair_u1.py <lm.out>: staircase of an msolve -g 1 leading-ideal file of the u-ring system and,
per s-variable, its chain count, border and how many border monomials are leading monomials (A3)."""
import re,sys,itertools
txt=open(sys.argv[1]).read()
body=txt[txt.index('[')+1:txt.rindex(']')]
V=['s1','s2','s3','s4','s5','w']
L=[]
for t in body.replace('\n','').split(','):
    e=[0]*6
    for f in t.split('*'):
        v,_,k=f.partition('^'); e[V.index(v)]=int(k) if k else 1
    L.append(tuple(e))
Ls=[l[:5] for l in L if l[5]==0]
print('LMs',len(L),'pure-s',len(Ls),'with w:',[l for l in L if l[5]])
LS=set(Ls)
def div(m):
    return any(all(a<=b for a,b in zip(l,m)) for l in Ls)
# enumerate staircase by BFS
from collections import deque
S=set();q=deque([(0,)*5]);
while q:
    m=q.popleft()
    if m in S or div(m): continue
    S.add(m)
    for i in range(5):
        n=list(m);n[i]+=1;q.append(tuple(n))
print('|S|',len(S))
for var in range(5):
    border=[m for m in S if (lambda n:(n not in S))(tuple(x+(i==var) for i,x in enumerate(m)))]
    stable=sum(1 for m in border if tuple(x+(i==var) for i,x in enumerate(m)) in LS)
    chains=sum(1 for m in S if m[var]==0)
    print(f's{var+1}: chains {chains} border {len(border)} border-in-LM {stable} maxchain {max(m[var] for m in S)+1}')
