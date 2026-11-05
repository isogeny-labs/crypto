import sys,json,random; sys.path.insert(0,'.'); sys.path.insert(0,'../../L9-124-falls-structure/src')
from evalsym import evalrels
from zmodel import Field, closure
from cmpl9 import newrels
from sys9 import base
S=json.load(open('../analysis/sym_rref.json'))
out={}
srt=lambda L: sorted(L,key=lambda r: next(k for k,v in r.items() if v==1 or v=='0x1'))
# (a) vs L9's numeric relations
for n,s in [(31,2),(31,3)]:
    m,V,z,a6=base(n,(n+1)//2,s); ev=evalrels(S['new_raw'],n,m,z,a6)
    L9=json.load(open('../../L9-124-falls-structure/analysis/newrel_n%d_A_s%d.json'%(n,s)))['new_relations']
    ok=sorted(json.dumps({k:hex(v) for k,v in r.items()},sort_keys=True) for r in ev)==sorted(json.dumps(r,sort_keys=True) for r in L9)
    out['L9_n%d_s%d'%(n,s)]=ok
# (b) vs numeric Z-model closure at other fields/instances (n=29..37, random draws incl. 2 random z,c each)
rng=random.Random(12)
for n in [29,31,33,35,37]:
    for s in range(3):
        if s==0: m,V,z,a6=base(n,(n+1)//2,0)
        else:
            from zmodel import modulus; m=modulus(n); z=rng.randrange(1,1<<n); a6=rng.randrange(1,1<<n)
        K=Field(n,m); st,R2,R3,_=closure(K,z,a6,0,4,range(4)); new,d2,d3=newrels(K,R2,R3)
        ev=evalrels(S['new_raw'],n,m,z,a6)
        ok=sorted(json.dumps({k:hex(v) for k,v in r.items()},sort_keys=True) for r in ev)==sorted(json.dumps(r,sort_keys=True) for r in new)
        out['Z_n%d_%d'%(n,s)]=[d2,d3,ok]
print(json.dumps(out))
