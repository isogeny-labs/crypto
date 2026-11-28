import json,sys
sys.path.insert(0,'.')
from consist import rank
from collections import Counter
C=Counter()
for line in open(sys.argv[1]):
    d=json.loads(line); r,cc,ct=d['r'],d['cc'],d['ct']; kc=cc-2
    L=[];T=[]
    for ents,rhs in d['rows']:
        v=[0]*cc
        for c,x in ents: v[c]=x%r
        (T if v[ct] else L).append(v)
    nt=[c for c in range(cc) if c!=ct]
    rL=rank([[v[c] for c in nt] for v in L],r); dL=len(nt)-rL
    inc=rank(L+T,r)-rL
    degfree=all(sum((4 if c==kc else 1)*v[c] for c in nt)%r==0 for v in L)
    z=sum(1 for c in nt if all(v[c]==0 for v in L))
    base=inc>=dL+1
    f1=inc>=dL-degfree+1
    f12=inc>=dL-degfree-z+1
    C[(base,f1,f12)]+=1
    print(d['q'],d['seed'],d['attempt'],'dL',dL,'inc',inc,'degfree',degfree,'z',z,'ok base/p0/p0+cover',base,f1,f12)
print(C)
