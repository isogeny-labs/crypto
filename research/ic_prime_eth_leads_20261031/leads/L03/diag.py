import sys, os, math, numpy as np
sys.argv=['x','14','0','t/diag.jsonl']; os.environ['CURVES']='1'
exec(open('exp.py').read().split("rows = open(outp")[0])
ci,b,p,a,bb,r,gx,gy = curves[0]
tab=np.fromfile(f'tab_{b}_{ci}.bin',dtype=np.uint32).reshape(-1,2).astype(np.int64); c=2*tab[:,0]+(tab[:,1]&1); nb=b+1
m=3; K=round(2*math.log2(r)); ks=[K//3+(1 if i<K%3 else 0) for i in range(3)]
out=[]
for sd in range(400):
    rng=np.random.default_rng(sd)
    rows=[rows_regular(nb,nrows(r,r/2**ks[i]),3,rng) for i in range(3)]
    ms_=[]
    for rw in rows:
        mk=m_parity(c,rw,rng); mk[0]=False; ms_.append(mk)
    sizes=[int(x.sum()) for x in ms_]
    X=np.rint(conv(ms_,r)); st=stats(X)
    nm=[m_random(r,s,rng) for s in sizes]; sn=stats(conv(nm,r))
    bit0=[any(rw_&1 for rw_ in rw) for rw in rows]
    out.append((st['P1']/sn['P1'], st['E']/sn['E'], X[0]/X.sum(), sum(bit0), min(sizes), max(sizes)))
o=np.array(out)
for nb0 in range(4):
    s=o[o[:,3]==nb0]
    if len(s): print('rows-touching-bit0 slots =',nb0,'n',len(s),'P1 ratio',s[:,0].mean().round(4),'E ratio',s[:,1].mean().round(4),'X0 frac',s[:,2].mean().round(4))
print('overall',o[:,0].mean(),o[:,1].mean())
