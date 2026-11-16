import sys,os,json,time
H=os.path.dirname(os.path.abspath(__file__)); E=os.path.dirname(H); sys.path.insert(0,os.path.join(E,'src'))
from lib import system
sys.path.insert(0,os.path.join(E,'..','MP-comm_alg-1')); import tensor_closure as TC
raw={}
for l in open(os.path.join(E,'raw','n19_k10_d4.jsonl')):
    r=json.loads(l)
    if 'low4' in r: raw[(r['s'],r['arm'])]=[r['dim4'],r['low4'],r['one']]
res=[]
for s,a in ((0,'A'),(1,'B')):
    t=time.time(); r=TC.closure(list(system(19,10,s)[a]),20,4)
    res.append(dict(s=s,arm=a,python=str(r),c_raw=raw[(s,a)],sec=round(time.time()-t,1))); print(res[-1],flush=True)
json.dump(res,open(os.path.join(H,'l3_pyclosure_n19.json'),'w'),indent=1)
