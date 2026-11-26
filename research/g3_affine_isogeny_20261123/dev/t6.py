# group-sign ambiguity: which group sign patterns satisfy all 315 relations, and do they change Aronhold a_ij?
import json, sys, itertools, numpy as np
sys.path.insert(0, '.')
import cnum as C
exec(open('dev/t5.py').read().split("vecs=")[0])
rels=json.load(open('dev/rels_seed1.json'))
def tc(x): return (tuple(x[0]),tuple(x[1]))
A3=C.BITS3
mx=max(abs(val[c]) for c in C.EVEN)
good=[]
for bits in itertools.product((1,-1),repeat=7):
    s=dict(zip(A3,(1,)+bits))
    th={c:val[c]*s[c[0]]/mx for c in C.EVEN}
    ok=True
    for ks,(s2,s3) in rels:
        m=[np.prod([th[tc(c)] for c in k]) for k in ks]
        if abs(m[0]+complex(s2)*m[1]+complex(s3)*m[2])>1e-9: ok=False;break
    if ok: good.append(s)
print('patterns satisfying all relations:',len(good))
for s in good: print([s[a] for a in A3])
