#!/usr/bin/env python3
"""random (alpha,beta) samples at t=1 over F_p until NEED cubic-class points (dim in 599,600,602,607 and tan!=0) are found.
usage: sample_cubic.py p need maxsamples out.jsonl"""
import sys, json, random
sys.path.insert(0,'.')
from census import *
p,need,mx,outf=int(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3]),sys.argv[4]
rng=random.Random(p*1000003+7)
cells=[(a,b) for a in range(p) for b in range(p)]; rng.shuffle(cells)
out=open(outf,'w'); got=0; n=0
for al,be in cells[:mx]:
    a,b,t=al,be,1
    if (4*a**3+27*b*b)%p==0 or len(s3_coeffs(p,a,b,t))!=9 or (t**3+a*t+b)%p==0: continue
    d,one=closure_dim(p,a,b,t,6); n+=1
    out.write(json.dumps(dict(p=p,alpha=al,beta=be,dim6=d,one6=one,tan0=(3+al)%p==0))+'\n'); out.flush()
    if d in (599,600,602,607) and (3+al)%p: got+=1
    if got>=need: break
open(outf+'.done','w').write(f'{n} closures, {got} cubic points')
