#!/usr/bin/env python3
"""F_p census at (k=6,D=4). usage: census.py p worker nworkers out.jsonl [triples_file]"""
import sys, json, os
sys.path.insert(0,'/home/user/ml-cryptanalysis/scripts'); sys.path.insert(0,'/home/user/crypto/research/leads_chase_20261112/tools')
import ic_bounds_growth_lib as G
from unit_test import resultant_unit, s3_coeffs
def gen(p, a, b, t, k):
    N = 1 << k; g = {}
    for (e1, e2), v in s3_coeffs(p, a, b, t).items():
        key = (e1 % N, e2 % N); g[key] = (g.get(key, 0) + v) % p
    return {e: v for e, v in g.items() if v}
def closure_dim(p,a,b,t,k):
    L=G.Layout(k,4); B,piv=G.closure(L,gen(p,a,b,t,k),p)
    return len(piv), bool(L.idx[(0,0)] in set(int(c) for c in piv))
def admissible(p,a,b,t):
    if t%p==0 or (4*a**3+27*b**2)%p==0 or len(s3_coeffs(p,a,b,t))!=9: return False
    if (t**3+a*t+b)%p==0: return False
    return resultant_unit(p,a,b,t,7)['unit']
if __name__=='__main__':
    p,w,nw,outf=int(sys.argv[1]),int(sys.argv[2]),int(sys.argv[3]),sys.argv[4]
    if len(sys.argv)>5: trip=[tuple(x) for x in json.load(open(sys.argv[5]))]
    else: trip=[(a,b,t) for a in range(p) for b in range(p) for t in range(1,p)]
    done=set()
    if os.path.exists(outf):
        for l in open(outf): r=json.loads(l); done.add((r['a'],r['b'],r['t']))
    out=open(outf,'a'); n=0
    for i,(a,b,t) in enumerate(trip):
        if i%nw!=w or (a,b,t) in done: continue
        if not admissible(p,a,b,t): continue
        d,one=closure_dim(p,a,b,t,6)
        out.write(json.dumps(dict(p=p,a=a,b=b,t=t,dim6=d,one6=one))+'\n'); out.flush()
    open(outf+f'.done{w}','w').write('done')
