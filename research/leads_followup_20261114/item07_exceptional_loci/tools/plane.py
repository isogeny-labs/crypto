#!/usr/bin/env python3
"""closure dim at (k=6,D=4) for every (alpha,beta) in F_p^2 at t=tt (default 1): (a,b,t)=(alpha tt^2, beta tt^3, tt).
usage: plane.py p tt worker nw out.jsonl"""
import sys, json
sys.path.insert(0,'.')
from census import *
p,tt,w,nw,outf=int(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),sys.argv[5]
out=open(outf,'a')
i=0
for al in range(p):
    for be in range(p):
        i+=1
        if i%nw!=w: continue
        a,b,t=al*tt*tt%p,be*pow(tt,3,p)%p,tt
        rec=dict(p=p,alpha=al,beta=be,t=t,a=a,b=b)
        rec['nonsing']=(4*a**3+27*b*b)%p!=0
        rec['nine']=len(s3_coeffs(p,a,b,t))==9
        rec['f0']=(t**3+a*t+b)%p==0
        rec['unit7']=bool(resultant_unit(p,a,b,t,7)['unit'])
        if rec['nonsing'] and rec['nine'] and not rec['f0']:
            d,one=closure_dim(p,a,b,t,6); rec['dim6']=d; rec['one6']=one
        out.write(json.dumps(rec)+'\n'); out.flush()
open(outf+f'.done{w}','w').write('done')
