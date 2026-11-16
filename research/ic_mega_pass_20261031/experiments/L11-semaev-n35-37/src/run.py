import sys, os, json, time
from concurrent.futures import ThreadPoolExecutor
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'))
import lib
n=int(sys.argv[1]); k=(n+1)//2; N=2*k; U=8; ARMS=['A','C1']
out=open(os.path.join(HERE,'..','raw','n%d_d4.jsonl'%n),'a')
def one(s,a,eqs):
    t=time.time(); so=lib.call('bsols',N,2,eqs)
    r=dict(n=n,k=k,s=s,arm=a,nsol=so['nsol'],cpu_sols=so['cpu'])
    if so['nsol']==0:
        c=lib.call('closure',N,4,eqs); r.update(dim4=c['dim'],low4=c['low'],one=c['one'],cpu=c['cpu'])
    r['wall']=round(time.time()-t,1); return r
cnt={a:0 for a in ARMS}; s=0
with ThreadPoolExecutor(3) as ex:
    while s<120 and any(v<U for v in cnt.values()):
        jobs=[]
        for ss in range(s,s+3):
            sy=lib.system(n,k,ss); jobs+=[(ss,a,sy[a]) for a in ARMS if cnt[a]<U]
        s+=3
        for r in ex.map(lambda x: one(*x), jobs):
            if r['nsol']==0: cnt[r['arm']]+=1
            out.write(json.dumps(r)+'\n'); out.flush()
print(json.dumps(dict(done=cnt,last_s=s-1)),flush=True)
