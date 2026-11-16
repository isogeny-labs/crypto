import sys,json,collections
sys.path.insert(0,'.')
from nullsp import *
p=43
R=[json.loads(l) for w in (0,1) for l in open(f'../data/plane43_t1_w{w}.jsonl')]
print('cells',len(R))
V=[r for r in R if 'dim6' in r]; print('valid',len(V))
pred=json.load(open('../data/pred43.json'))
out={}
# P1
tan0=[r for r in V if r['alpha']==40]
c1=collections.Counter(r['dim6'] for r in tan0); out['P1_tan0_dims']=dict(c1)
out['P1_pass']=set(c1)<= {600,603} and 1<=c1.get(600,0)<=2
# P2
unit=[r for r in V if r['unit7']]
c2=collections.Counter(r['dim6'] for r in unit); out['P2_unit_dims']=dict(c2)
out['P2_pass']=set(c2)<={599,600,602,603,605,607,794}
# P3'
S={599,600,602,603,607}
obs={f"{r['alpha']},{r['beta']}":r['dim6'] for r in V if r['dim6'] in S}
miss=[k for k in pred if obs.get(k)!=pred[k]]; extra=[k for k in obs if k not in pred]
out['P3p_pred_n']=len(pred); out['P3p_obs_n']=len(obs); out['P3p_mismatch_pred_side']=miss; out['P3p_extra_observed']=extra
out['P3p_pass']=not miss and not extra
# P4
C=[r for r in V if r['dim6']>=786 and r['alpha']!=40]; out['P4_classC_n']=len(C)
uv=[r for r in unit]; out['P4_unit_n']=len(uv); fr=sum(1 for r in uv if r['dim6']!=605)/max(1,len(uv)); out['P4_unit_depart_frac']=fr
out['P4_pass']=28<=len(C)<=46 and 0.12<=fr<=0.30
# P5
pts=[(r['alpha'],r['beta']) for r in C]
nul={}
for d in range(1,8):
    m=monos(2,d)
    if len(m)>=len(pts): break
    nul[d]=len(nullspace(evalmat(pts,m,p),p))
out['P5_nullity_by_degree']=nul; out['P5_pass']=all(nul[d]==0 for d in nul if d<=5)
# extra info: dims of non-unit variants
out['all_dims']=dict(collections.Counter(r['dim6'] for r in V))
json.dump(out,open('../data/eval43.json','w'),indent=1)
print(json.dumps(out,indent=1))
