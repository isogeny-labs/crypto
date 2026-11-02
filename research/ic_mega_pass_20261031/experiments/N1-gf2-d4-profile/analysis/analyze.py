"""Build RESULTS.json from raw/*.jsonl and raw/run_main.log (repair item 1).
Usage: python3 analysis/analyze.py  (from experiment dir or anywhere)."""
import json, glob, os, re
from math import comb
from scipy.stats import beta
HERE=os.path.dirname(os.path.abspath(__file__)); EXP=os.path.dirname(HERE); RAW=os.path.join(EXP,'raw')
ARMS=['A','C1','C2','D1','B','RM']; ALEV=0.05
PLANNED=[19,21,23,25,27,29,31]
def cp(x,n,side):  # exact Clopper-Pearson one-sided 95% bounds on a proportion
    if n==0: return None
    if side=='upper': return 1.0 if x==n else float(beta.ppf(1-ALEV,x+1,n-x))
    return 0.0 if x==0 else float(beta.ppf(ALEV,x,n-x+1))
log={}
for line in open(os.path.join(RAW,'run_main.log')):
    line=line.strip()
    if line.startswith('{'): d=json.loads(line); log[d['n']]=d
res=dict(source='raw/*.jsonl + raw/run_main.log', planned_n=PLANNED, completed_n=sorted(log), per_n={})
for fn in sorted(glob.glob(os.path.join(RAW,'n*_k*_d4.jsonl')), key=lambda f:int(re.findall(r'n(\d+)_',f)[0])):
    recs=[json.loads(l) for l in open(fn) if l.strip()]
    n=recs[0]['n']; k=recs[0]['k']; N=2*k; tot=sum(comb(N,i) for i in range(4))
    P={'n':n,'k':k,'N':N,'complete':n in log,'records':len(recs),'C(N,<=3)':tot,'n(N+1)':n*(N+1),'arms':{}}
    if n in log: P.update(seeds_used=log[n]['seeds_used'],child_cpu_s=log[n]['cpu'],wall_s=log[n]['wall'])
    viol={'P1_excess':[], 'P1_one':[], 'P3_low':[], 'P3_one':[], 'P4':[]}
    for a in ARMS:
        R=[r for r in recs if r['arm']==a]
        rnd=[r for r in R if r['sample']=='random' and 'low4' in r]
        uns=[r for r in R if r['nsol']==0 and 'one' in r]
        clos=[r for r in R if 'low4' in r]
        ex=[r['excess'] for r in rnd if r.get('excess') is not None]
        nfail=sum(1 for r in uns if r['one']==0) if a in ('A','C1','C2','D1') else sum(1 for r in uns if r['one']==1)
        Ae=dict(random_draws=len(rnd), random_excess0=sum(1 for e in ex if e==0), U=len(uns),
                one_in_W4=sum(r['one'] for r in uns),
                low4_values=sorted(set(r['low4'] for r in clos)),
                nsol_parity_all_even=all(r['nsol']%2==0 for r in R),
                rs_eq_nsol_all=all(r['r_s']==r['nsol'] for r in clos if r.get('r_s') is not None),
                dNS_values=sorted(set(r['dNS'] for r in clos)))
        if a in ('A','C1','C2','D1'):
            Ae['rate_1_in_W4']=Ae['one_in_W4']/len(uns) if uns else None
            Ae['CP95_upper_rate_1_notin_W4']=cp(len(uns)-Ae['one_in_W4'],len(uns),'upper')
            if ex and sum(1 for e in ex if e==0)<0.9*len(ex): viol['P1_excess'].append(a)
            if nfail: viol['P1_one'].append(a)
        else:
            Ae['rate_1_notin_W4']=(len(uns)-Ae['one_in_W4'])/len(uns) if uns else None
            Ae['CP95_upper_rate_1_in_W4']=cp(Ae['one_in_W4'],len(uns),'upper')
            if any(r['low4']!=n*(N+1) for r in clos): viol['P3_low'].append(a)
            if nfail: viol['P3_one'].append(a)
        if any(r['dNS']!=-1 for r in clos): viol['P4'].append(a)
        P['arms'][a]=Ae
    P['violations']=viol
    res['per_n'][str(n)]=P
# per-arm totals across completed n only (not pooled across arms: A,C1,C2,D1 share a mechanism)
tot={}
for a in ARMS:
    U=sum(res['per_n'][str(n)]['arms'][a]['U'] for n in res['completed_n'])
    one=sum(res['per_n'][str(n)]['arms'][a]['one_in_W4'] for n in res['completed_n'])
    x=(U-one) if a in ('A','C1','C2','D1') else one
    tot[a]=dict(U=U,one_in_W4=one,failures=x,CP95_upper_failure_rate=cp(x,U,'upper'))
res['per_arm_over_completed_n']=tot
res['last_completed_n']=max(res['completed_n']) if res['completed_n'] else None
res['partial_n']=[int(n) for n,P in res['per_n'].items() if not P['complete']]
res['cum_child_cpu_s']=round(sum(log[n]['cpu'] for n in log),1)
res['cost_rule']=('not implemented in code (manual); projection x4 per +2n from last completed n: '
   + str(round(log[max(log)]['cpu']*sum(4**i for i in range(1,1+len([m for m in PLANNED if m>max(log)])))/3600+res['cum_child_cpu_s']/3600,2))+' CPU-h vs 2.5 h threshold')
res['notes']=['r_s == nsol on every closure record, so excess = C(N,<=3)-nsol-low4 is not an independent observable',
 'U overshoot: batches of 3 seeds are processed before counts update, so U can reach 9-10 instead of 8',
 'A has even nsol always (a<->b swap), so UNSAT conditioning differs from B/RM',
 'P3 is a counting identity (null calibration); P1/P4 written after a 9-instance pilot',
 'cpu = child-process clock() only; excludes Python system generation, random_matched, rank_s']
json.dump(res,open(os.path.join(EXP,'RESULTS.json'),'w'),indent=1)
for n,P in res['per_n'].items():
    print(n,'complete' if P['complete'] else 'PARTIAL', {a:(v['U'],v['one_in_W4'],v['low4_values'][:2]) for a,v in P['arms'].items()}, P['violations'])
print(json.dumps(tot)); print(res['cost_rule'])
