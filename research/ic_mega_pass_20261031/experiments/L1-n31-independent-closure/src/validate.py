import json, sys, time
from drv import *
ARMS=['A','Ap','B','C1','C2','D1','D2','D4','RM']
out=open(os.path.join(HERE,'..','raw','validation.jsonl'),'w'); mism=0; tot=0
# tiny python reference
for n in (5,7,9):
    k=(n+1)//2
    for s in range(3):
        sy=lib.system(n,k,s)
        for a in ARMS:
            for d in (3,4):
                m=mac(sy[a],2*k,d); p=pyref(sy[a],2*k,d); g=lib.call('closure',2*k,d,sy[a])
                t=[m['dim'],m['low'],m['one']]; ok=(t==[p['dim'],p['low'],p['one']]==[g['dim'],g['low'],g['one']])
                tot+=1; mism+=not ok
                out.write(json.dumps(dict(kind='pyref',n=n,s=s,arm=a,d=d,mac=t,py=[p['dim'],p['low'],p['one']],gf2=[g['dim'],g['low'],g['one']],ok=ok))+'\n')
for n in (13,15,17,19,21):
    k=(n+1)//2
    for s in range(3):
        sy=lib.system(n,k,s)
        for a in ARMS:
            m=mac(sy[a],2*k,4); g=lib.call('closure',2*k,4,sy[a])
            t=[m['dim'],m['low'],m['one']]; ok=t==[g['dim'],g['low'],g['one']]; tot+=1; mism+=not ok
            out.write(json.dumps(dict(kind='gf2close',n=n,s=s,arm=a,d=4,mac=t,gf2=[g['dim'],g['low'],g['one']],ok=ok,cpu_mac=m['cpu'],cpu_gf2=g['cpu']))+'\n'); out.flush()
    print(n,'done',tot,mism,flush=True)
print(json.dumps(dict(total=tot,mismatches=mism)))
