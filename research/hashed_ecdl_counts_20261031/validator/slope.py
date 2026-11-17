import sys,math,random,json
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import recompute as R
import numpy as np
C1014=R.load('/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl')
C16=R.load('/dev/shm/restore/v2/sessions/C-16/records.jsonl')
recs=C1014+C16
curves=['fp10','fp12','fp14','fp16']
def r_of(c): 
    return [x['workload']['curve']['r'] for x in recs if R.curve_key(x)==c][0]
lg={c:math.log2(r_of(c)) for c in curves}
print('log2 r',{c:round(v,4) for c,v in lg.items()}, {c:r_of(c) for c in curves})
def arm_s(c,arm,w): 
    ps=R.pairs(recs,'rho-neg',arm,c)
    return [R.priced_s(x if arm_==0 else y,w) for x,y in ps for arm_ in (0,1)][:0]
def pair_arrays(c,w,A='rho-neg',B='tensor-m2'):
    ps=R.pairs(recs,A,B,c); return np.array([R.priced_s(x,w) for x,_ in ps]),np.array([R.priced_s(y,w) for _,y in ps])
def ols(x,y):
    x=np.array(x);y=np.array(y);n=len(x)
    xb,yb=x.mean(),y.mean(); sxx=((x-xb)**2).sum(); b=((x-xb)*(y-yb)).sum()/sxx; a=yb-b*xb
    res=y-(a+b*x); s2=(res**2).sum()/(n-2); se=math.sqrt(s2/sxx)
    return b,a,se
t2=4.302652729911275 # t_{0.975,2}
rng=np.random.default_rng(20261101)
out={}
for w in [0.0,0.1,1.0]:
    ratios=[]
    for c in curves:
        a,b=pair_arrays(c,w); ratios.append(b.sum()/a.sum())
    xs=[lg[c] for c in curves]; ys=[math.log2(r) for r in ratios]
    b,a0,se=ols(xs,ys)
    succ=[ratios[i+1]/ratios[i] for i in range(3)]
    # bootstrap over targets
    bs=[]
    for _ in range(4000):
        yb=[]
        for c in curves:
            A,B=pair_arrays(c,w); idx=rng.integers(0,8,8); yb.append(math.log2(B[idx].sum()/A[idx].sum()))
        bs.append(ols(xs,yb)[0])
    lo,hi=np.percentile(bs,[2.5,97.5])
    print(f"w={w}: ratios={['%.4g'%r for r in ratios]} succ(x per 2 bits)={['%.2f'%s for s in succ]} OLS slope(log2R vs log2r)={b:.3f} se={se:.3f} 95%CI_t2=({b-t2*se:.2f},{b+t2*se:.2f}) boot95=({lo:.3f},{hi:.3f}) mean growth per 2 bits={4**b:.2f}x (per-2-bit factors from ends: {(ratios[3]/ratios[0])**(1/3):.2f})")
# P2: tensor median S slope, P3: rho median total GAE slope  (w=0 priced S == recorded S for tensor? use cost.s)
def med(c,arm,key):
    v=[x['cost'][key] for x in recs if R.curve_key(x)==c and x['arm']==arm and R.counts(x)]
    return float(np.median(v)),len(v)
for arm,key,label in [('tensor-m2','s','P2 tensor median S'),('tensor-m2','total_gae','tensor median GAE'),('rho-neg','total_gae','P3 rho median total_gae'),('rho-neg','s','rho median S'),('ic-mitm','s','ic-mitm median S')]:
    ys=[];xs=[]
    for c in curves:
        m,n=med(c,arm,key); ys.append(math.log2(m)); xs.append(lg[c])
    b,a0,se=ols(xs,ys)
    # bootstrap targets (resample the 8 per curve independently)
    bs=[]
    for _ in range(4000):
        yb=[]
        for c in curves:
            v=np.array([x['cost'][key] for x in recs if R.curve_key(x)==c and x['arm']==arm and R.counts(x)]); yb.append(math.log2(np.median(rng.choice(v,8))))
        bs.append(ols(xs,yb)[0])
    lo,hi=np.percentile(bs,[2.5,97.5])
    print(f"{label}: medians={['%.4g'%2**y for y in ys]} slope={b:.3f} se={se:.3f} CI_t2=({b-t2*se:.2f},{b+t2*se:.2f}) boot=({lo:.3f},{hi:.3f})")
