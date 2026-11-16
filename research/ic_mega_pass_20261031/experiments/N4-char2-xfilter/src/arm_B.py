"""Arm B/C: climber j>=2. usage: arm_B.py n kind:a mode(P|E) j1,j2,..."""
import sys, json, time, numpy as np
sys.path.insert(0,'.'); from lib import *
n=int(sys.argv[1]); kind,a=sys.argv[2].split(':'); a=int(a); mode=sys.argv[3]; t0=time.time()
c=[x for x in load_curves() if x['n']==n and x['kind']==kind and x['a']==a][0]
import math
jq=lambda th:int(math.floor(th*math.log2(c['l'])))
js=sorted(set([2,jq(0.25),jq(1/3)])) if sys.argv[4]=='auto' else ([jq(0.25)] if sys.argv[4]=='q' else [int(v) if v.isdigit() else jq(0.25 if v=='q' else 1/3) for v in sys.argv[4].split(',')])
X,g=enumerate_x(c,mode,1); Xs=sym_perm(X,np.random.default_rng(5)); out=[]
for j in js:
    seed=20261101+1000*n+10*j
    br=climb(X,n,j,seed); bn=climb(Xs,n,j,seed)
    prf=[]
    for key in range(24):
        r=gain(prf_labels(X,j,key),j); prf.append(r['G']-1)
    rec={'curve':c,'mode':mode,'L':len(X),'j':j,'real':br,'null1sym':bn,'prf_max':max(prf),'prf_median':float(np.median(prf)),'secs':time.time()-t0}
    out.append(rec); print(json.dumps({k:v for k,v in rec.items() if k!='curve'}),flush=True)
json.dump(out,open(f'../raw/armB_n{n}_{kind}_a{a}_{mode}.json','w'),indent=1)
