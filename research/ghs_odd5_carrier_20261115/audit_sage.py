#!/usr/bin/env sage -python
"""Independent vectorized Sage replay of the complete F3 genus-five slice."""
import hashlib,json
from pathlib import Path
import numpy as np
from sage.all import GF,PolynomialRing,gcd
from sage.version import version as sage_version
P=Path(__file__).resolve().parent
screen=json.loads((P/'screen.json').read_text())
R=PolynomialRing(GF(3),'x');x=R.gen()
mod_masks=[0,1,7,5,7]
moduli=[]
for m,mask in enumerate(mod_masks,1):
    z=mask;f=x**m
    for j in range(m):f+=(z%3)*x**j;z//=3
    assert f.is_irreducible()
    moduli.append(f)
base=np.arange(3**11,dtype=np.int64)
coeff=np.empty((len(base),11),dtype=np.uint8)
for j in range(11):coeff[:,j]=(base//(3**j))%3
candidates=base.copy()
stage=[len(base)]
def code(v,m):
    if m==1:return int(v)
    digs=list(v.polynomial().list())
    return sum(int(digs[j])*3**j for j in range(len(digs)))
def element_at(K,i,m):
    if m==1:return K(i)
    return K(R([(i//(3**j))%3 for j in range(m)]))
for m in range(1,5):
    q=3**m
    K=GF(q,'a',modulus=moduli[m-1]) if m>1 else GF(3)
    elems=[element_at(K,i,m) for i in range(q)]
    assert sorted(code(v,m) for v in elems)==list(range(q))
    chi=np.full(q,-1,dtype=np.int8);chi[0]=0
    for v in elems[1:]:chi[code(v*v,m)]=1
    assert int(np.count_nonzero(chi==1))==(q-1)//2
    counts=np.zeros(len(candidates),dtype=np.int32)
    weights=np.array([3**j for j in range(m)],dtype=np.int32)
    for v in elems:
        powmat=np.zeros((11,m),dtype=np.uint8)
        w=K(1)
        for j in range(11):
            ci=code(w,m)
            for h in range(m):powmat[j,h]=(ci//(3**h))%3
            w*=v
        lead=code(w,m)
        digits=(coeff[candidates]@powmat)%3
        for h in range(m):digits[:,h]=(digits[:,h]+(lead//(3**h))%3)%3
        values=digits@weights
        counts+=chi[values]
    candidates=candidates[counts==0]
    stage.append(len(candidates))
valid=[]
for mask in candidates:
    f=x**11+sum(int(coeff[mask,j])*x**j for j in range(11))
    if gcd(f,f.derivative())==1:valid.append(int(mask))
stage.append(len(valid))
K=GF(3**5,'a',modulus=moduli[4]);elems=[element_at(K,i,5) for i in range(243)]
chi=np.full(243,-1,dtype=np.int8);chi[0]=0
for v in elems[1:]:chi[code(v*v,5)]=1
assert int(np.count_nonzero(chi==1))==121
weights=np.array([3**j for j in range(5)],dtype=np.int32)
counts=np.zeros(len(valid),dtype=np.int32)
for v in elems:
    powmat=np.zeros((11,5),dtype=np.uint8);w=K(1)
    for j in range(11):
        ci=code(w,5)
        for h in range(5):powmat[j,h]=(ci//(3**h))%3
        w*=v
    lead=code(w,5)
    digits=(coeff[valid]@powmat)%3
    for h in range(5):digits[:,h]=(digits[:,h]+(lead//(3**h))%3)%3
    counts+=chi[digits@weights]
rows=[];integral=0
for mask,character_sum in zip(valid,counts):
    n5=244+int(character_sum);diff=244-n5
    if diff%5:continue
    integral+=1;t=diff//5
    if abs(t)<=31 and t%3:rows.append([mask,4,10,28,82,n5,t])
stage += [integral,len(rows)]
assert stage==screen['stage_counts'],(stage,screen['stage_counts'])
assert rows==screen['rows'],(rows[:10],screen['rows'][:10])
out={'schema':'ghs-odd5-carrier-sage-audit/v1','sage_version':sage_version,'numpy_version':np.__version__,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'screen_sha256':hashlib.sha256((P/'screen.json').read_bytes()).hexdigest(),'moduli':[str(f) for f in moduli],'stage_counts':stage,'retained_count':len(rows),'unique_traces':sorted(set(row[6] for row in rows)),'all_rows_match':True}
target=P/'audit.json'
if target.exists():raise SystemExit('refusing to overwrite immutable audit receipt')
target.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
