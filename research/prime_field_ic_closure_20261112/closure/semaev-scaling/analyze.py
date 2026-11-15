import json, math, random, statistics as st
import numpy as np
def fit(xs,ys):
    A=np.vstack([xs,np.ones(len(xs))]).T; return float(np.linalg.lstsq(A,ys,rcond=None)[0][0])
def boot(xs,ys,groups,B=2000,seed=1):
    rng=random.Random(seed); keys=sorted(set(groups)); sl=[]
    by={k:[i for i,g in enumerate(groups) if g==k] for k in keys}
    for _ in range(B):
        idx=[i for k in keys for i in [rng.choice(by[k]) for _ in by[k]]]
        sl.append(fit([xs[i] for i in idx],[ys[i] for i in idx]))
    sl.sort(); return sl[int(.025*B)],sl[int(.975*B)]
out={}
for m in (3,4):
    Y=json.load(open(f'yield_m{m}.json'))
    lp=[math.log2(r['p']) for r in Y]; g=[r['bits'] for r in Y]
    rows={}
    rows['P_succ_by_bits']={b:round(st.mean(r['P_succ'] for r in Y if r['bits']==b),4) for b in sorted(set(g))}
    C=[math.log2(r['total_ops']) for r in Y]
    CLA=[math.log2(r['total_ops']+m*r['V_abscissas']**2) for r in Y]
    rho=[math.log2(r['rho_neg']) for r in Y]
    for k,ys in (('O1_relation_ops',C),('O1_relation_plus_LA',CLA),('rho_neg',rho)):
        rows[k]=dict(slope=round(fit(lp,ys),3),ci95=[round(x,3) for x in boot(lp,ys,g)])
    rows['ratio_O1_over_rho_by_bits']={b:round(st.mean(r['total_ops']/r['rho_neg'] for r in Y if r['bits']==b),1) for b in sorted(set(g))}
    # O2: per-query Macaulay columns at D_solve (UNSAT queries), exponent in p
    D=[json.loads(l) for l in open(f'dsolve_m{m}.log') if l.startswith('{')]
    U=[r for r in D if 'D_solve' in r]
    lp2=[math.log2(r['p']) for r in U]; qc=[math.log2(r['cols_at_D']) for r in U]
    rows['O2_unsat_fraction']=f"{len(U)}/{len(D)}"
    rows['O2_deg_inv_equals_n']=f"{sum(r['deg_inv']==r['n'] for r in U)}/{len(U)}"
    rows['O2_query_cols']=dict(slope=round(fit(lp2,qc),3),ci95=[round(x,3) for x in boot(lp2,qc,[r['bits'] for r in U])],
                                min_log2cols_over_log2p=round(min(r['log2cols_over_log2p'] for r in U),3))
    # O2 total lower bound: |Vx|/P_succ (from O1 at same bits) * cols
    vb={b:st.mean(r['V_abscissas']/max(r['P_succ'],1e-9) for r in Y if r['bits']==b) for b in set(g)}
    T2=[math.log2(vb[r['bits']]*r['cols_at_D']) for r in U if r['bits'] in vb]
    lp3=[math.log2(r['p']) for r in U if r['bits'] in vb]
    rows['O2_total_lower_bound']=dict(slope=round(fit(lp3,T2),3),ci95=[round(x,3) for x in boot(lp3,T2,[r['bits'] for r in U if r['bits'] in vb])])
    out[f'm{m}']=rows
print(json.dumps(out,indent=1)); json.dump(out,open('analysis.json','w'),indent=1)
