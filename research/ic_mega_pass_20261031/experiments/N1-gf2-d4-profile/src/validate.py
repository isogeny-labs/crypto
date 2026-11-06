# validate C closure vs MP-comm_alg-1 raw/*.json : every cell, every arm, every draw
import json, glob, os, sys, time
from lib import *
from concurrent.futures import ThreadPoolExecutor
RAW=os.path.join(HERE,'..','..','MP-comm_alg-1','raw')
def cell(f):
    J=json.load(open(f)); n,k,d,dr=J['n'],J['k'],J['d'],J['draws']
    mism=[]; tot=0; t=time.time()
    for s in range(dr):
        sy=system(n,k,s)
        for a in ARMS:
            r=call('closure',2*k,d,sy[a])
            mine=[r['dim'],r['low'],r['one']]; tot+=1
            if mine!=J['res'][a][s]: mism.append((a,s,mine,J['res'][a][s]))
    return dict(cell=os.path.basename(f),n=n,k=k,d=d,checked=tot,mismatch=mism,wall=round(time.time()-t,1))
if __name__=='__main__':
    fs=sorted(glob.glob(RAW+'/*.json'))
    with ThreadPoolExecutor(3) as ex:
        out=list(ex.map(cell,fs))
    for o in out: print(json.dumps(o))
    print(json.dumps(dict(total=sum(o['checked'] for o in out),mismatches=sum(len(o['mismatch']) for o in out))))
