import sys, os, json, time
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'))
import lib, subprocess
from concurrent.futures import ThreadPoolExecutor
def mac(eqs,N,d):
    s="%d %d %d\n"%(N,d,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
    r=subprocess.run([os.path.join(HERE,'macclose')],input=s,capture_output=True,text=True)
    if r.returncode: raise RuntimeError(r.stderr)
    return json.loads(r.stdout)
n,k=27,14; recs={}
for l in open(os.path.join(HERE,'..','..','N1-gf2-d4-profile','raw','n27_k14_d4.jsonl')):
    r=json.loads(l)
    if r['arm'] in ('A','C1','D1') and r['nsol']==0 and 'one' in r: recs.setdefault(r['arm'],[]).append(r)
jobs=[(a,r) for a in recs for r in sorted(recs[a],key=lambda r:r['s'])[:8]]
def go(x):
    a,r=x; t=time.time(); m=mac(lib.system(n,k,r['s'])[a],2*k,4)
    return dict(n=n,arm=a,s=r['s'],mac={q:m[q] for q in ('dim','low','one')},n1=dict(dim=r['dim4'],low=r['low4'],one=r['one']),wall=round(time.time()-t,1))
out=open(os.path.join(HERE,'..','raw','n27_macclose.jsonl'),'w')
with ThreadPoolExecutor(2) as ex:
    for o in ex.map(go,jobs): out.write(json.dumps(o)+'\n'); out.flush()
