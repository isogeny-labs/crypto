import sys,json,random,math,time,statistics as st
sys.path.insert(0,sys.argv[4])
import dstar_vs_V as d
n=int(sys.argv[1]); l=int(sys.argv[2]); B=int(sys.argv[3])
F=d.GF(n); a6=1
def xcoords(seed,k):
    r=random.Random(seed); out=[]
    while len(out)<k:
        z=r.randrange(1,1<<n)
        if d.is_xcoord(F,z,a6): out.append(z)
    return out
TRAIN=xcoords(11,6); FRESH=xcoords(999,60)
def meanD(V,zs,cap=None):
    Ds=[]
    for z in zs:
        if d.sat(F,V,z,a6): continue
        e,_=d.build(F,V,z,a6); Ds.append(d.dstar(e,2*l))
        if cap and len(Ds)>=cap: break
    return (sum(Ds)/len(Ds) if Ds else None), Ds
def randV(r):
    while True:
        V=[r.randrange(1,1<<n) for _ in range(l)]
        if d.f2rank(V)==l: return V
def move(V,r):
    while True:
        W=list(V); W[r.randrange(l)]=r.randrange(1,1<<n)
        if d.f2rank(W)==l: return W
def run_arm(kind,seed):
    r=random.Random(seed); pool=[]
    def score(V):
        m,_=meanD(V,TRAIN)
        if m is None: m=9
        if kind=='C': m=r.random()
        return m
    cur=None;cs=None
    for i in range(B):
        if kind=='A' or i<B//2 or cur is None: V=randV(r)
        else: V=move(cur,r)
        s=score(V); pool.append((s,V))
        if kind!='A':
            if i<B//2:
                if cs is None or s<cs: cur,cs=V,s
            else:
                T=0.3*(0.02/0.3)**((i-B//2)/(B/2))
                if s<=cs or r.random()<math.exp(-(s-cs)/T): cur,cs=V,s
    pool.sort(key=lambda x:x[0]); return pool[:5]
t0=time.time(); res={'n':n,'l':l,'B':B,'arms':{}}
for kind,seed in [('A',101),('B',202),('C',303)]:
    top=run_arm(kind,seed)
    ev=[]
    for s,V in top:
        m,Ds=meanD(V,FRESH,20); ev.append({'train':s,'fresh_mean':m,'fresh_D':Ds,'V':[hex(v) for v in V],'dimVV':d.VV_dim(F,V)})
    res['arms'][kind]=ev; print(kind,[e['train'] for e in ev],[e['fresh_mean'] for e in ev],flush=True)
r=random.Random(404); ev=[]
for _ in range(5):
    t=r.randrange(2,1<<n); V=[F.pw(t,j) for j in range(l)]
    if d.f2rank(V)<l: continue
    m,Ds=meanD(V,FRESH,20); ev.append({'fresh_mean':m,'fresh_D':Ds,'dimVV':d.VV_dim(F,V)})
res['arms']['D_poly']=ev; print('D',[e['fresh_mean'] for e in ev],flush=True)
r=random.Random(505); null=[]
for _ in range(40):
    V=randV(r); m,_=meanD(V,FRESH,20); null.append(m)
null.sort(); res['null_fresh_means']=null
res['null_p5']=null[1]; res['null_p95']=null[37]; res['null_median']=st.median(null)
fb=[e['fresh_mean'] for e in res['arms']['B']]; fa=[e['fresh_mean'] for e in res['arms']['A']]
obs=st.mean(fa)-st.mean(fb); pool=fa+fb; pr=random.Random(7); cnt=0
for _ in range(10000):
    pr.shuffle(pool)
    if st.mean(pool[:5])-st.mean(pool[5:])>=obs-1e-12: cnt+=1
res['perm_test_B_lt_A']={'obs_diff':obs,'p':(cnt+1)/10001}
res['secs']=time.time()-t0
print(json.dumps({k:res[k] for k in ['null_p5','null_median','null_p95','perm_test_B_lt_A','secs']}))
json.dump(res,open(sys.argv[5],'w'),indent=1)
