import json, math, collections
import numpy as np
OUT="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/out/prime-crosscheck"
D=json.load(open(OUT+"/records_extracted.json"))
TCRIT={1:12.706,2:4.303,3:3.182,4:2.776,5:2.571,6:2.447,7:2.365,8:2.306,9:2.262,10:2.228,11:2.201,12:2.179,13:2.160,14:2.145,15:2.131,16:2.120,17:2.110,18:2.101,19:2.093,20:2.086,21:2.080,22:2.074,23:2.069,24:2.064,25:2.060,26:2.056,27:2.052,28:2.048,29:2.045,30:2.042}
def tcrit(df): return TCRIT.get(df, 2.0 if df>30 else 2.0)
def ols(x, y):
    x=np.asarray(x,float); y=np.asarray(y,float)
    n=len(x); X=np.column_stack([np.ones(n), x])
    b,res,rk,sv=np.linalg.lstsq(X,y,rcond=None)
    yhat=X@b; rss=float(((y-yhat)**2).sum())
    if n>2:
        s2=rss/(n-2); cov=s2*np.linalg.inv(X.T@X); se=math.sqrt(cov[1,1])
        t=tcrit(n-2); lo,hi=b[1]-t*se,b[1]+t*se
        r2=1-rss/((y-y.mean())**2).sum() if ((y-y.mean())**2).sum()>0 else float('nan')
    else: se=lo=hi=r2=float('nan')
    return dict(n=n, slope=float(b[1]), se=se, lo95=lo, hi95=hi, r2=r2, intercept=float(b[0]))
def fmt(f):
    return f"beta={f['slope']:.3f} (SE {f['se']:.3f}; 95% t [{f['lo95']:.2f}, {f['hi95']:.2f}]; n={f['n']}; R2={f['r2']:.3f})"

def group_mean(rows, key, field):
    g=collections.defaultdict(list)
    for r in rows:
        if r.get(field) is not None: g[r[key]].append(r[field])
    return {k:(float(np.mean(v)), len(v)) for k,v in g.items()}

prime=D['prime']; binary=D['binary']
# ---- prime: IC plain arm (ic-d4-w), chain 1 (main) and all clean (main+xmain)
ic_all=[r for r in prime if r['arm']=='ic-d4-w' and r['status']=='verified']
ic_c1=[r for r in ic_all if r['session'].startswith('main-')]
# rho: dedupe by (curve, target) across sessions
seen=set(); rho_all=[]
for r in prime:
    if r['arm']!='rho-neg' or r['status']!='verified': continue
    k=(r['curve'], tuple(r['target']))
    dup = k in seen; seen.add(k)
    r=dict(r); r['dup']=dup; rho_all.append(r)
rho_dedup=[r for r in rho_all if not r['dup']]
rho_c1=[r for r in rho_dedup if r['session'].startswith(('main-','rho-'))]

# print per-p table for IC
print("== PRIME: per-p IC plain arm (ic-d4-w; main+xmain, all clean runs) ==")
print("p | runs | chains | trials(mean) | rel(mean) | cols(mean) | S(mean) | GAE_rel(mean) | GAE_la(mean) | GAE_tot(mean) | rho S(mean, n dedup) | rho GAE(mean, n dedup)")
ps=sorted(set(r['p'] for r in ic_all))
rows_tab=[]
for p in ps:
    ic=[r for r in ic_all if r['p']==p]
    rh=[r for r in rho_dedup if r['p']==p]
    chains=sorted(set(r['session'].split('-')[0] + ('' if not r['session'].startswith('x') else '') for r in ic))
    cs=sorted(set(r['curve'] for r in ic))
    m=lambda f,L: float(np.mean([x[f] for x in L]))
    line=dict(p=p, runs=len(ic), curves=len(cs), trials=m('trials',ic), rel=m('rel',ic), cols=m('cols',ic), S=m('S',ic), rel_gae=m('GAE_rel',ic), la_gae=m('GAE_la',ic), tot=m('GAE',ic), rhoS=m('S',rh), rhoGAE=m('GAE',rh), rho_n=len(rh))
    rows_tab.append(line)
    print(f"{p} | {len(ic)} | {len(cs)} curves | {line['trials']:.0f} | {line['rel']:.1f} | {line['cols']:.1f} | {line['S']:.4g} | {line['rel_gae']:.4g} | {line['la_gae']:.4g} | {line['tot']:.4g} | {line['rhoS']:.3f} (n={len(rh)}) | {line['rhoGAE']:.4g}")

print()
print("== PRIME fits (x = ln p) ==")
def fits(label, icrows, rhrows):
    res={}
    print("--", label)
    for name,field,L in [("IC relations GAE",'GAE_rel',icrows),("IC linear-algebra GAE",'GAE_la',icrows),("IC total GAE",'GAE',icrows),("IC S",'S',icrows)]:
        f=ols([math.log(r['q']) for r in L],[math.log(r[field]) for r in L]); res[name+" per-run"]=f
        print(f"  per-run  {name}: {fmt(f)}")
    for name,field,L in [("rho GAE",'GAE',rhrows),("rho S",'S',rhrows)]:
        f=ols([math.log(r['q']) for r in L],[math.log(r[field]) for r in L]); res[name+" per-run"]=f
        print(f"  per-run  {name}: {fmt(f)}")
    # rung means
    for name,field,L in [("IC relations GAE",'GAE_rel',icrows),("IC linear-algebra GAE",'GAE_la',icrows),("IC total GAE",'GAE',icrows),("IC S",'S',icrows)]:
        g=collections.defaultdict(list)
        for r in L: g[r['q']].append(r[field])
        xs=[math.log(q) for q in sorted(g)]; ys=[math.log(float(np.mean(g[q]))) for q in sorted(g)]
        f=ols(xs,ys); res[name+" rung"]=f
        print(f"  rung-mean {name}: {fmt(f)}")
    for name,field,L in [("rho GAE",'GAE',rhrows),("rho S",'S',rhrows)]:
        g=collections.defaultdict(list)
        for r in L: g[r['q']].append(r[field])
        xs=[math.log(q) for q in sorted(g)]; ys=[math.log(float(np.mean(g[q]))) for q in sorted(g)]
        f=ols(xs,ys); res[name+" rung"]=f
        print(f"  rung-mean {name}: {fmt(f)}")
    return res
P_all=fits("prime, plain IC arm all clean (main+xmain), rho dedup all clean", ic_all, rho_dedup)
P_c1=fits("prime, chain 1 only (main), rho dedup chain-1 sessions", ic_c1, rho_c1)
print()
print("rho duplicates removed:", sum(1 for r in rho_all if r['dup']), "of", len(rho_all), "; rho distinct:", len(rho_dedup))
print("IC runs all:", len(ic_all), " chain1:", len(ic_c1))
# ---- binary
print()
print("== BINARY: per-m, ic-wied (plain Wiedemann; the arm comparable to prime ic-d4-w) ==")
bic=[r for r in binary if r['arm']=='ic-wied' and r['status']=='verified']
brho=[r for r in binary if r['arm']=='rho-neg' and r['status']=='verified']
bdense=[r for r in binary if r['arm']=='ic-dense' and r['status']=='verified']
seen=set(); brho_d=[]
for r in brho:
    k=(r['curve'], tuple(r['target']))
    if k in seen: continue
    seen.add(k); brho_d.append(r)
for m in sorted(set(r['m'] for r in bic)):
    L=[r for r in bic if r['m']==m]; R=[r for r in brho_d if r['m']==m]
    mm=lambda f,L: float(np.mean([x[f] for x in L]))
    print(f"m={m} | IC runs {len(L)} | trials {mm('trials',L):.0f} | rel {mm('rel',L):.0f} | cols {mm('cols',L):.0f} | S {mm('S',L):.4g} | GAE_rel {mm('GAE_rel',L):.4g} | GAE_la {mm('GAE_la',L):.4g} | GAE {mm('GAE',L):.4g} || rho runs {len(R)} rho GAE {mm('GAE',R) if R else float('nan'):.4g} rho S {mm('S',R) if R else float('nan'):.4g}")
for m in sorted(set(r['m'] for r in bdense)):
    L=[r for r in bdense if r['m']==m]
    print(f"  ic-dense (reference only) m={m} runs {len(L)} GAE_rel {np.mean([x['GAE_rel'] for x in L]):.4g} GAE_la {np.mean([x['GAE_la'] for x in L]):.4g} GAE {np.mean([x['GAE'] for x in L]):.4g}")
print()
def bfits(label, icrows, rhrows):
    print("--", label)
    res={}
    for name,field in [("IC relations GAE",'GAE_rel'),("IC linear-algebra GAE",'GAE_la'),("IC total GAE",'GAE'),("IC S",'S')]:
        f=ols([math.log(r['q']) for r in icrows],[math.log(r[field]) for r in icrows]); res[name+" per-run"]=f
        print(f"  per-run  {name}: {fmt(f)}")
    for name,field in [("rho GAE",'GAE'),("rho S",'S')]:
        f=ols([math.log(r['q']) for r in rhrows],[math.log(r[field]) for r in rhrows]); res[name+" per-run"]=f
        print(f"  per-run  {name}: {fmt(f)}")
    for name,field in [("IC relations GAE",'GAE_rel'),("IC linear-algebra GAE",'GAE_la'),("IC total GAE",'GAE'),("IC S",'S')]:
        g=collections.defaultdict(list)
        for r in icrows: g[r['q']].append(r[field])
        qs=sorted(g); f=ols([math.log(q) for q in qs],[math.log(np.mean(g[q])) for q in qs]); res[name+" rung"]=f
        print(f"  rung-mean {name}: {fmt(f)}")
    for name,field in [("rho GAE",'GAE'),("rho S",'S')]:
        g=collections.defaultdict(list)
        for r in rhrows: g[r['q']].append(r[field])
        qs=sorted(g); f=ols([math.log(q) for q in qs],[math.log(np.mean(g[q])) for q in qs]); res[name+" rung"]=f
        print(f"  rung-mean {name}: {fmt(f)}")
    return res
B_all=bfits("binary, ic-wied (all clean; q=2^m, ln q = m ln2), rho dedup, m=8..17", bic, brho_d)
bic_no_pilot=bic
# binary IC only m>=10 subset as sensitivity
B_hi=bfits("binary, ic-wied restricted to m>=10", [r for r in bic if r['m']>=10], [r for r in brho_d if r['m']>=10])
json.dump({'prime_all':{k:v for k,v in P_all.items()}, 'prime_chain1':{k:v for k,v in P_c1.items()}, 'binary_all':B_all, 'binary_m_ge10':B_hi, 'prime_table':rows_tab}, open(OUT+"/fits.json","w"), indent=1, default=float)
