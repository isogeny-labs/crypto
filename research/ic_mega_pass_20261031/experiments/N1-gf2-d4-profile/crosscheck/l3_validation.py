"""L3 audit item 5: saved validation artifacts. (a) festest rerun, (b) brute-force nsol at n=19 vs raw,
(c) independent recount of n=27..31 table from raw. 1 thread."""
import sys, os, json, subprocess, numpy as np
H=os.path.dirname(os.path.abspath(__file__)); E=os.path.dirname(H); sys.path.insert(0, os.path.join(E,'src'))
from lib import system
out={}
fe=subprocess.run([sys.executable, os.path.join(E,'src','festest.py'), os.path.join(E,'src','gf2close')],capture_output=True,text=True).stdout.split('\n')
out['festest']=dict(lines=len([l for l in fe if l.strip()]), ok=sum(1 for l in fe if '"ok":1' in l.replace(' ','')))
raw={}
for l in open(os.path.join(E,'raw','n19_k10_d4.jsonl')):
    r=json.loads(l); raw[(r['s'],r['arm'])]=r['nsol']
N=20; X=np.arange(1<<N,dtype=np.int64); mism=[]; checked=0
for s in range(6):
    sy=system(19,10,s)
    for a,eqs in sy.items():
        if (s,a) not in raw: continue
        ok=np.ones(1<<N,dtype=bool)
        for e in eqs:
            v=np.zeros(1<<N,dtype=np.int8)
            for m in e: v^=((X&m)==m).astype(np.int8)
            ok&=(v==0)
        n=int(ok.sum()); checked+=1
        if n!=raw[(s,a)]: mism.append([s,a,n,raw[(s,a)]])
out['bruteforce_n19']=dict(seeds='0..5',checked=checked,mismatches=mism)
tab={}
for n,k in ((27,14),(29,15),(31,16)):
    R=[json.loads(l) for l in open(os.path.join(E,'raw',f'n{n}_k{k}_d4.jsonl'))]
    for a in ['A','C1','C2','D1','B','RM']:
        u=[r for r in R if r['arm']==a and r['nsol']==0 and 'one' in r]
        rnd=[r for r in R if r['arm']==a and r['sample']=='random' and r.get('excess') is not None]
        tab[f'{n}_{a}']=dict(one_in_W4=sum(r['one'] for r in u),unsat=len(u),
            rand_excess0=sum(1 for r in rnd if r['excess']==0),rand=len(rnd),low4_set=sorted({r['low4'] for r in R if r['arm']==a and 'low4' in r})[:6])
out['recount_n27_31']=tab
json.dump(out,open(os.path.join(H,'l3_validation.json'),'w'),indent=1)
print(json.dumps(out)[:3000])
