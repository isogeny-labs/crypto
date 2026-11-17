import sys, json, time, os, threading
from math import comb
from lib import *
from concurrent.futures import ThreadPoolExecutor
OUT=os.path.join(HERE,'..','raw')
ARM6=['A','C1','C2','D1','B','RM']
lock=threading.Lock()
def inst(n,k,s,a,sy,need_closure):
    N=2*k; t=time.time()
    so=call('sols' if a=='RM' else 'bsols',N,2,sy[a])
    rec=dict(n=n,k=k,s=s,arm=a,nsol=so['nsol'],cpu_sols=so['cpu'])
    if need_closure or so['nsol']==0:
        r=call('closure',N,4,sy[a]); st=call('static',N,4,sy[a])
        rs=rank_s(so['sols'],N,4) if so['nsol']<=4096 else None
        low=r['low']; tot=comb(N,0)+comb(N,1)+comb(N,2)+comb(N,3)
        exc=None if rs is None else tot-rs-low
        assert exc is None or exc>=0, rec
        if st['dNS']>0: assert r['one']==1, rec
        if so['nsol']==0 and exc is not None: assert (exc==0)==(r['one']==1), rec
        rec.update(low4=low,dim4=r['dim'],one=r['one'],r_s=rs,excess=exc,dNS=st['dNS'],cpu=r['cpu']+st['cpu']+so['cpu'],
                   nprod=r['nprod'])
    rec['wall']=round(time.time()-t,2)
    return rec
def run_n(n,R=10,U=8,SMAX=150):
    k=(n+1)//2; fn=os.path.join(OUT,'n%d_k%d_d4.jsonl'%(n,k)); f=open(fn,'w')
    cnt={a:0 for a in ARM6}; recs=[]
    with ThreadPoolExecutor(3) as ex:
        s=0
        while s<SMAX:
            if s>=R and all(cnt[a]>=U for a in ARM6): break
            batch=[]
            for ss in range(s,min(s+3,SMAX)):
                sy=system(n,k,ss)
                for a in ARM6:
                    if ss<R or cnt[a]<U: batch.append((ss,a,sy))
            s=min(s+3,SMAX)
            for rec in ex.map(lambda x: inst(n,k,x[0],x[1],x[2],x[0]<R), batch):
                rec['sample']='random' if rec['s']<R else 'unsat_scan'
                if rec['nsol']==0 and 'one' in rec: cnt[rec['arm']]+=1
                recs.append(rec); f.write(json.dumps(rec)+'\n'); f.flush()
    return recs,s
if __name__=='__main__':
    for n in map(int,sys.argv[1:]):
        t=time.time(); recs,smax=run_n(n)
        cpu=sum(r.get('cpu',r['cpu_sols']) for r in recs)
        print(json.dumps(dict(n=n,seeds_used=smax,cpu=round(cpu,1),wall=round(time.time()-t,1))),flush=True)
