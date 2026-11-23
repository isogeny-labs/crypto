"""Extract the K-relations in W3 (x) K not in W2 (x) K (window), K-RREF with cubic monomials first; print supports."""
import sys,json
from kker import *
n,name,s,pref=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]),sys.argv[4]; win=[int(x) for x in sys.argv[5].split(',')]
k=(n+1)//2; mul,frob,trm=field(n)
M,_,R2,_=solve(n,name,s,pref,win,2); _,_,R3,_=solve(n,name,s,pref,win,3)
deg=lambda m: len(m[0])+len(m[1])
order=sorted(range(len(M)),key=lambda i:(-deg(M[i]),M[i]))   # cubic first
def rref(vecs):
    out=[];piv=[]
    for v in vecs:
        v=v[:]
        for pc,rv in zip(piv,out):
            if v[pc]: f=v[pc]; v=[a^mul(f,b) for a,b in zip(v,rv)]
        nz=[i for i in order if v[i]]
        if not nz: continue
        pc=nz[0]; iv=kinv(v[pc],mul,n); v=[mul(iv,x) for x in v]
        for j in range(len(out)):
            if out[j][pc]: f=out[j][pc]; out[j]=[a^mul(f,b) for a,b in zip(out[j],v)]
        out.append(v); piv.append(pc)
    return out,piv
B2,p2=rref(R2); B3,p3=rref(R2+R3)   # R2 first so that new pivots are the new relations
new=[v for v,p in zip(B3,p3) if p not in p2]
# reduce the new ones fully against all of B3 rows that are in R2 span: B3 is RREF so they are reduced.
res=[]
for v in new:
    res.append({fmt(*M[i]):hex(v[i]) for i in order if v[i]})
z=base(n,k,s)[2]
print(json.dumps(dict(n=n,arm=name,s=s,window=win,dimK_R2=len(B2),dimK_R3=len(B3),new=len(new),z=hex(z),new_relations=res,
     R2_cubic_pivots=[fmt(*M[p]) for p in p2 if deg(M[p])==3])))
