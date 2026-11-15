#!/usr/bin/env python3
"""Post hoc: stratify A6 part (a) rows (rows.jsonl, SHA-256 fa1111c1...a92ec6) by curve class.
Run: python3 -I stratify_curve_class.py ROWS.jsonl > stratify_curve_class.out.txt"""
import json, random, sys
rows=[json.loads(l) for l in open(sys.argv[1])]
rnd=random.Random(7)
def boot(xs,B=2000):
    n=len(xs); m=sum(xs)/n
    ms=sorted(sum(xs[rnd.randrange(n)] for _ in range(n))/n for _ in range(B))
    return m,ms[int(.025*B)],ms[int(.975*B)-1]
def cls(r):
    if r['b']==0 and r['a']==0: return 'a0b0'
    if r['b']==0: return 'b0'
    if r['a']==0: return 'a0'
    return 'generic'
for k in (2,3):
    for c in ('b0','a0','generic'):
        rs=[r for r in rows if r['k']==k and cls(r)==c and r['sub_trace'] and r['sub_rand']]
        if len(rs)<5: print(k,c,len(rs),'too few'); continue
        out=[]
        for i,name in ((1,'cov2'),(2,'cov3')):
            xs=[r['sub_trace'][0][i]-r['sub_rand'][0][i] for r in rs]
            m,lo,hi=boot(xs); out.append('%s %+.4f [%+.4f,%+.4f]'%(name,m,lo,hi))
        print('k=%d %-8s n=%4d  %s'%(k,c,len(rs),'  '.join(out)))
# by p mod 4 for b0 and by p mod 3 for a0, k=2
for k in (2,):
  for c,mod in (('b0',4),('a0',3)):
    for res in range(1,mod):
        rs=[r for r in rows if r['k']==k and cls(r)==c and r['p']%mod==res and r['sub_trace'] and r['sub_rand']]
        if len(rs)<5: continue
        xs=[r['sub_trace'][0][2]-r['sub_rand'][0][2] for r in rs]; m,lo,hi=boot(xs)
        print('k=2',c,'p mod',mod,'=',res,'n=%d'%len(rs),'cov3 %+.4f [%+.4f,%+.4f]'%(m,lo,hi))
