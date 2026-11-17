import json, time
from l2lib import *
n=33; k=17; N=34; cnt={'A':0,'C1':0,'D1':0}
f=open(os.path.join(HERE,'..','raw','n33_k17_d4.jsonl'),'a')
for s in range(60):
    if all(v>=8 for v in cnt.values()): break
    sy=system(n,k,s)
    for a in cnt:
        if cnt[a]>=8: continue
        so=N1.call('bsols',N,2,sy[a]); rec=dict(n=n,k=k,s=s,arm=a,nsol=so['nsol'],cpu_sols=so['cpu'])
        if so['nsol']==0:
            t=time.time(); x=call5('closure',N,4,sy[a],cap=2400)
            rec.update(dim4=x.get('dim'),low4=x.get('low'),one=x['one'],cpu=x['cpu'],capped=x.get('capped',0),wall=round(time.time()-t,1)); cnt[a]+=1
        f.write(json.dumps(rec)+'\n'); f.flush()
print(json.dumps(dict(done=cnt,last_s=s)))
