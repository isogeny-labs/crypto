import json, math, os, sys, statistics as st
sys.path.insert(0,os.environ['TMPDIR']+'/e2'); import analyse as A
E=os.environ['TMPDIR']+'/e2'; D=E+'/runs'
R={41:549756390943,51:17192986687,53:21044858204113,57:275295876199,59:10063074221,61:162888033982417}
LADDER=[1,2,3,4,6,8,12,16,24,32,48,64,96,128,192]
def sel(n):
    b={}
    for sym in (0,1):
        c=[]
        for K in LADDER:
            try: s,r=A.load(D,f'n{n}_K{K}_N16_T32_s{sym}_r1')
            except Exception: continue
            s0,d,_=A.costs(n,s,r); c.append((s0+d if sym==0 else s0*d,K,s0,d))
        b[sym]=min(c); b[(sym,'all')]=sorted(c,key=lambda x:x[1])
    return b
def point(n,K,N,sym,seed):
    s,r=A.load(D,f'n{n}_K{K}_N{N}_T64_s{sym}_r{seed}'); s0,d,_=A.costs(n,s,r); return s0,d
def row(n,base,rec,seeds=(2,3,4)):
    cuts=[];out=[]
    for sd in seeds:
        b=point(n,*base,sd); r=point(n,*rec,sd)
        cuts.append((b[0]*b[1])/(r[0]*r[1])); out.append((b,r))
    return out,cuts
def ols(x,y):
    n=len(x);mx=sum(x)/n;my=sum(y)/n
    sxx=sum((a-mx)**2 for a in x);b=sum((a-mx)*(c-my) for a,c in zip(x,y))/sxx
    res=sum((c-my-b*(a-mx))**2 for a,c in zip(x,y));se=math.sqrt(res/(n-2)/sxx)
    return b,se
T={1:12.706,2:4.303,3:3.182,4:2.776,5:2.571,6:2.447}
rows=[]
for n in sorted(R):
    try: S=sel(n); 
    except Exception as e: continue
    bk=(S[0][1],16,0); rk=(S[1][1],16,1)
    try: out,cuts=row(n,bk,rk)
    except Exception as e: print('skip',n,e); continue
    rows.append((n,bk,rk,out,cuts,S))
    print(f'n={n} r={R[n]:.3g} base K{bk[0]} plain  rec K{rk[0]} sym  edge? base_K_in_ladder_edge={bk[0] in (1,2)} rec_edge={rk[0]>=96}')
    for sd,(b,r),c in zip((2,3,4),out,cuts): print(f'   seed{sd} base S0={b[0]:.3g} d={b[1]:.3g} | rec S0={r[0]:.3g} d={r[1]:.3g} | cut={c:.2f}')
    lc=[math.log2(c) for c in cuts]
    print(f'   geomean cut={2**st.mean(lc):.2f} sd(log2)={st.stdev(lc):.2f}  range {min(cuts):.2f}-{max(cuts):.2f}')
    # decomposition at seed 1 (selection data, biased)
    pb=S[0]; ps=S[1]
    ps0d=min(x[2]*x[3] for x in S[(0,'all')])
    print(f'   seed1 (selection): base S0d={pb[2]*pb[3]:.3g}, best plain S0d={ps0d:.3g} (retune {pb[2]*pb[3]/ps0d:.2f}x), best sym S0d={ps[0]:.3g} (sym x{ps0d/ps[0]:.2f})')
def fit(label,xs,key):
    ys=[math.log2(2**st.mean([math.log2(c) for c in r[4]])) for r in rows]
    x=[key(r) for r in rows]; b,se=ols(x,ys); t=T[len(x)-2]
    print(f'{label}: slope of log2(cut) per unit = {b:.4f} +- {t*se:.4f} (95% CI [{b-t*se:.4f},{b+t*se:.4f}]), {len(x)} sizes')
if len(rows)>=3:
    fit('vs n',None,lambda r:r[0]); fit('vs log2 r',None,lambda r:math.log2(R[r[0]]))
    # per-seed pooled fit (seeds as replicates, treats points independent)
    for lab,key in (('n',lambda r:r[0]),('log2r',lambda r:math.log2(R[r[0]]))):
        x=[];y=[]
        for r in rows:
            for c in r[4]: x.append(key(r)); y.append(math.log2(c))
        b,se=ols(x,y); t=T.get(len(x)-2,2.1); print(f'per-seed pooled vs {lab}: {b:.4f} +- {t*se:.4f} (anticonservative)')
json.dump([(r[0],r[1],r[2],r[4]) for r in rows],open(E+'/final.json','w'))
