import sys,json,time; sys.path.insert(0,'.'); sys.path.insert(0,'../../L9-124-falls-structure/src')
from zmodel import *; from sys9 import base
n=31; m,V,z,a6=base(n,16,2); K=Field(n,m); res=[]
for L in [3,4,5,6,7,8]:
    for p in ([0,1,2] if L<=6 else [0,1]):
        t=time.time(); st,_,_,_=closure(K,z,a6,-p,L+p,range(L)); st.update(L=L,p=p,new=st['dimW3win']-st['dimW2win'],pred=4*max(L-3,0),sec=round(time.time()-t,1)); res.append(st); print(json.dumps(st),flush=True)
json.dump(res,open('../analysis/p3_translation.json','w'),indent=0)
