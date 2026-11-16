import json
from math import comb as C
def R4(n,N): return n*(3*C(N-3,2)+N-3)-9*C(n,2)
rows=[]
for l in open('raw/main.jsonl'):
    d=json.loads(l); n,N=d['n'],d['N']; r4=R4(n,N); stall=r4<C(N,4)
    rec=dict(job=d['kind'],n=n,N=N,arm=d.get('arm','synthetic'),s=d.get('s',d.get('seed')),engine=d['engine'],
             low=d['low'],dim=d['dim'],one=d['one'],R4_model=r4,CN4=C(N,4),deficit=C(N,4)-r4,pred_stall=stall,cpu=d['cpu'])
    if stall: rec['pred_low']=n*(3*N-5); rec['pred_dim']=n*(3*N-5)+r4; rec['ok']=(d['low'],d['dim'],d['one'])==(rec['pred_low'],rec['pred_dim'],0)
    else: rec['ok']=d['low']>n*(3*N-5)   # escapes plateau
    rows.append(rec)
prior=[dict(n=31,N=32,arm='C1',src='L1 s0,s1',low=2821,dim=37293,pred_dim=2821+R4(31,32)),
       dict(n=33,N=34,arm='C1/C2',src='L8 8 draws',low=3201,dim=45507,pred_dim=3201+R4(33,34))]
p1=[json.loads(l) for l in open('raw/model_p1.jsonl')]
p4={'n27_C1':dict(round3_new_low=1341,surplus=-(C(28,4)-R4(27,28)),ok=True),'n27_syn':dict(round3_new_low=1341,surplus=1341,ok=True),
    'n29_C1_L1':dict(round3_new_low=261,surplus=261,ok=True),'n29_syn':dict(round3_new_low=261,surplus=261,ok=True),
    'n25_C1':dict(round3_new_low=1123,surplus=1900,ok=False,note='low space fills (2948 of 2952, 4 solutions); falls capped')}
json.dump(dict(formula=dict(low='n(3N-5)',R4='n*(3*C(N-3,2)+N-3) - 9*C(n,2)',deficit_N_eq_n_plus_1='n(n^3-38n^2+263n-274)/24',
    real_root=29.36,deficit_n31='1488 = 31*48 (48 = 1152/24)'),
    P1_pure_model=dict(pairs=len(p1),exact=sum(x['ok'] for x in p1),rows=[{k:x[k] for k in('n','N','rank3','pred3','rank4','pred4','ok')} for x in p1]),
    P2_P3_closure=rows,all_closure_ok=all(r['ok'] for r in rows),prior_consistency=prior,P4=p4),open('RESULTS.json','w'),indent=1)
print(all(r['ok'] for r in rows),len(rows))
