#!/usr/bin/env python3
"""Grouped leave-one-base-out ridge analysis of full-domain top-form intervals."""
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path

study=Path(__file__).resolve().parents[3]
attempt=Path(__file__).resolve().parents[1]/'attempt-05'
out=Path(__file__).resolve().parent/'model'
if out.exists():raise SystemExit(f'preserving existing model output: {out}')
top_summary=json.loads((attempt/'top/top-summary.json').read_text())
assert top_summary['all_verified'] and top_summary['completed']==336
bases=list(range(27,34)); families=['koblitz','binary','prime']; targets=['natural','planted']
feature_names=['m','binary_terms','n_vars','input_equations','point_count','mean_logx','sd_logx',
 'mean_logy','sd_logy','mean_selector','sd_selector','upper_selector_fraction',
 'is_koblitz','is_prime','is_planted']
rows=[]
for base in bases:
 inp=json.loads((attempt/f'inputs/spike{base}/inputs.json').read_text())
 labels=json.loads((attempt/f'top/spike{base}/certificates/summary.json').read_text())
 replay=json.loads((attempt/f'top/spike{base}/replay-summary.json').read_text())
 assert replay['all_verified'] and len(labels['rows'])==48
 by_id={r['id']:r for r in labels['rows']}
 for x in inp:
  label=by_id[x['id']]
  assert label['source_sha256']==x['source_sha256']
  pts=x['factor_base_membership']
  xs=[int(p['x']) for p in pts]; ys=[int(p['y']) for p in pts]
  sels=[int(p['selector']) for p in pts]
  lx=[math.log2(v+1)/83 for v in xs]; ly=[math.log2(v+1)/83 for v in ys]
  def sd(a):return statistics.pstdev(a) if len(a)>1 else 0.0
  lower=label['dreg_lower_bound']; upper=label['dreg_upper_bound']
  y=(lower+upper)/2 if label['status']=='finite_interval' else None
  r={'group':base,'id':x['id'],'family':x['family'],'m':x['m'],'target_kind':x['target_kind'],
     'binary_terms':x['binary_terms'] or 0,'n_vars':x['n_vars'],'input_equations':x['input_equations'],
     'point_count':len(pts),'mean_logx':statistics.mean(lx),'sd_logx':sd(lx),
     'mean_logy':statistics.mean(ly),'sd_logy':sd(ly),
     'mean_selector':statistics.mean(sels)/(1<<27),'sd_selector':sd([v/(1<<27) for v in sels]),
     'upper_selector_fraction':sum(v>>26 for v in sels)/len(sels),
     'status':label['status'],'lower':lower,'upper':upper,'y':y,
     'input_sha256':x['source_sha256'],'certificate_sha256':__import__('hashlib').sha256((attempt/f'top/spike{base}/certificates/{x["id"]}.json').read_bytes()).hexdigest()}
  rows.append(r)

def features(r):
 return [float(r[k]) for k in feature_names[:12]]+[float(r['family']=='koblitz'),float(r['family']=='prime'),float(r['target_kind']=='planted')]
def solve(a,b):
 n=len(b);m=[list(map(float,a[i]))+[float(b[i])] for i in range(n)]
 for c in range(n):
  p=max(range(c,n),key=lambda r:abs(m[r][c]))
  if abs(m[p][c])<1e-12:continue
  m[c],m[p]=m[p],m[c];z=m[c][c];m[c]=[v/z for v in m[c]]
  for r in range(n):
   if r!=c:
    z=m[r][c]
    if z:m[r]=[m[r][j]-z*m[c][j] for j in range(n+1)]
 return [m[i][-1] for i in range(n)]
def ridge_predict(train,test):
 raw=[features(r) for r in train+test];ntr=len(train);p=len(raw[0])
 means=[sum(x[j] for x in raw[:ntr])/ntr for j in range(p)]
 scales=[math.sqrt(sum((x[j]-means[j])**2 for x in raw[:ntr])/ntr) or 1.0 for j in range(p)]
 matrix=[[1.0]+[(x[j]-means[j])/scales[j] for j in range(p)] for x in raw]
 tr,te=matrix[:ntr],matrix[ntr:];q=p+1;ys=[r['y'] for r in train]
 a=[[sum(x[i]*x[j] for x in tr)+(0 if i==0 else (10 if i==j else 0)) for j in range(q)] for i in range(q)]
 b=[sum(x[i]*y for x,y in zip(tr,ys)) for i in range(q)];w=solve(a,b)
 return [sum(x*z for x,z in zip(row,w)) for row in te],w

finite=[r for r in rows if r['y'] is not None]
outcomes=defaultdict(set)
for r in rows:outcomes[(r['family'],r['m'],r['target_kind'])].add((r['status'],r['lower'],r['upper']))
folds=[];prediction_rows=[];wins=0
for b in bases:
 train=[r for r in finite if r['group']!=b];test=[r for r in finite if r['group']==b]
 pred,w=ridge_predict(train,test)
 baseline={}
 for t in test:
  k=(t['family'],t['m'],t['target_kind'])
  vals=[r['y'] for r in train if (r['family'],r['m'],r['target_kind'])==k]
  assert vals;baseline[k]=statistics.mean(vals)
 mae=statistics.mean(abs(p-r['y']) for p,r in zip(pred,test))
 bmae=statistics.mean(abs(baseline[(r['family'],r['m'],r['target_kind'])]-r['y']) for r in test)
 won=mae<bmae;wins+=won
 folds.append({'heldout_base':b,'n':len(test),'ridge_mae':mae,'matched_configuration_baseline_mae':bmae,'improvement':bmae-mae,'won':won,'weights':dict(zip(['intercept']+feature_names,w))})
 for p,r in zip(pred,test):
  k=(r['family'],r['m'],r['target_kind'])
  prediction_rows.append({'heldout_base':b,'id':r['id'],'family':r['family'],'m':r['m'],'target_kind':r['target_kind'],'observed_midpoint_proxy':r['y'],'ridge_prediction':p,'baseline_prediction':baseline[k],'ridge_abs_error':abs(p-r['y']),'baseline_abs_error':abs(baseline[k]-r['y'])})
base_stats=[]
for b in bases:
 group=[r for r in rows if r['group']==b];ys=[r['y'] for r in group if r['y'] is not None]
 base_stats.append({'base':b,'rows':len(group),'finite_interval_rows':len(ys),'positive_dimensional_rows':sum(r['y'] is None for r in group),'mean_finite_interval_midpoint_proxy':statistics.mean(ys) if ys else None})
errors=[r['ridge_abs_error'] for r in prediction_rows];base_errors=[r['baseline_abs_error'] for r in prediction_rows]
result={'schema':'n83.full-domain-ridge-result/v1','protocol_sha256':'280a78bd11e6fc20ae274c47525070a2cb6e455401593774b1688dbdd9537e94','systems':len(rows),'finite_interval_midpoint_rows':len(finite),'positive_dimensional_rows':len(rows)-len(finite),'matched_configuration_outcomes_identical':sum(len(v)==1 for v in outcomes.values()),'matched_configuration_count':len(outcomes),'folds':folds,'aggregate_grouped_oof_mae':statistics.mean(errors) if errors else None,'aggregate_matched_configuration_baseline_mae':statistics.mean(base_errors) if base_errors else None,'heldout_base_wins':wins,'minimum_wins_for_exploratory_lead':5,'admissible_exploratory_lead':bool(errors and wins>=5 and statistics.mean(errors)<statistics.mean(base_errors)),'per_base_interval_midpoint_descriptives':base_stats,'label_scope':'Finite top-form interval midpoint proxy only. Positive-dimensional top ideals have no finite label here. No exact degree-of-regularity, FFD, LFD, affine solving degree, or general elite-base claim.'}
out.mkdir()
(out/'rows.json').write_text(json.dumps({'rows':rows},indent=2)+'\n')
(out/'predictions.json').write_text(json.dumps({'rows':prediction_rows},indent=2)+'\n')
(out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
