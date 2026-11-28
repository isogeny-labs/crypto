import json, sys, itertools, numpy as np
sys.path.insert(0, '.')
import cnum as C
def setup(seed,Sb):
    d=json.load(open(f'dev/tau_{seed}.json')); T=np.array(d['T_re'])+1j*np.array(d['T_im'])
    S=np.zeros((3,3)); k=0
    for i in range(3):
        for j in range(i,3):
            S[i,j]=S[j,i]=Sb[k]; k+=1
    return C.theta_all((T+S)/2)
val,grad=setup(1,[0,0,1,0,0,0])
vecs=[c for c in C.CHARS if c!=((0,0,0),(0,0,0))]
def key(c): return c
subs=set()
for v1,v2 in itertools.combinations(vecs,2):
    v3=C.cadd(v1,v2)
    subs.add(frozenset([v1,v2,v3]))
print(len(subs))
mx=max(abs(val[c]) for c in C.EVEN)
rels=[]
Z=((0,0,0),(0,0,0))
for V in subs:
    Vl=[Z]+list(V)
    cos={}
    for m in C.EVEN:
        cs=frozenset(C.cadd(m,v) for v in Vl)
        if all(C.q(c)==0 for c in cs): cos[cs]=np.prod([val[c] for c in cs])/mx**4
    cl=list(cos.items())
    for (k1,a),(k2,b),(k3,c) in itertools.combinations(cl,3):
        for s2 in (1,-1,1j,-1j):
            for s3 in (1,-1,1j,-1j):
                if abs(a+s2*b+s3*c)<1e-11*max(abs(a),abs(b),abs(c)):
                    rels.append((V,(k1,k2,k3),(s2,s3)))
print('relations',len(rels))
import collections
print(collections.Counter(tuple(sorted((len(set(c[0] for c in k))) for k in r[1])) for r in rels))
print(collections.Counter(r[2] for r in rels))
json.dump([[[sorted(map(list,k)) for k in r[1]], [str(r[2][0]),str(r[2][1])]] for r in rels], open('dev/rels_seed1.json','w'))
