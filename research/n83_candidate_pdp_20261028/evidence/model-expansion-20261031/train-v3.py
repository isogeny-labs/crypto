import json, math, os, sys
from pathlib import Path
BASES=[27,28,29,30,31,32,33]
ROOT=Path('/tmp')
FAMS=['binary','koblitz','prime']; TARGETS=['natural','planted']
rows=[]
for b in BASES:
    inp=json.load(open(ROOT/f'n83-candidate-inputs/spike{b}/inputs.json'))
    summ=json.load(open(ROOT/f'n83-candidate-top/spike{b}-parent/certificates/summary.json'))
    labels={r['id']:r for r in summ['rows']}
    for d in inp:
        lab=labels[d['id']]
        if lab['status']=='finite_interval':
            y=(lab['dreg_lower_bound']+lab['dreg_upper_bound'])/2
        else:y=None
        pts=d['factor_base_membership']
        xs=[int(p['x']) for p in pts]; ys=[int(p['y']) for p in pts]
        row={'group':b,'id':d['id'],'family':d['family'],'target_kind':d['target_kind'],'m':d['m'],
             'binary_terms':d['binary_terms'] or 0,'n_vars':d['n_vars'],'input_equations':d['input_equations'],
             'point_count':len(pts),'mean_logx':sum(math.log2(x+1)/83 for x in xs)/len(xs) if xs else 0.0,
             'mean_logy':sum(math.log2(y+1)/83 for y in ys)/len(ys) if ys else 0.0,
             'status':lab['status'],'lower':lab['dreg_lower_bound'],'upper':lab['dreg_upper_bound'],'y':y}
        rows.append(row)
NUM=['m','binary_terms','n_vars','input_equations','point_count','mean_logx','mean_logy']
def features(rr):
    out=[]
    for r in rr:
        out += [float(r[k]) for k in NUM]
        out += [float(r['family']==v) for v in FAMS[1:]]
        out += [float(r['target_kind']==v) for v in TARGETS[1:]]
    return out

def solve(A,b):
    n=len(b); M=[list(map(float,A[i]))+[float(b[i])] for i in range(n)]
    for c in range(n):
        p=max(range(c,n),key=lambda r:abs(M[r][c]))
        if abs(M[p][c])<1e-12: continue
        M[c],M[p]=M[p],M[c]; z=M[c][c]; M[c]=[v/z for v in M[c]]
        for r in range(n):
            if r!=c:
                z=M[r][c]
                if z: M[r]=[M[r][j]-z*M[c][j] for j in range(n+1)]
    return [M[i][-1] for i in range(n)]

def pred(train,test):
    yy=[r['y'] for r in train]; mu=sum(yy)/len(yy)
    raw=features(train+test); p=len(raw)//(len(train)+len(test)); mat=[raw[i*p:(i+1)*p] for i in range(len(train)+len(test))]
    tr=mat[:len(train)]; te=mat[len(train):]
    means=[sum(x[j] for x in tr)/len(tr) for j in range(p)]
    sd=[math.sqrt(sum((x[j]-means[j])**2 for x in tr)/len(tr)) or 1 for j in range(p)]
    tr=[[ (x[j]-means[j])/sd[j] for j in range(p)] for x in tr]
    te=[[ (x[j]-means[j])/sd[j] for j in range(p)] for x in te]
    # intercept unpenalized
    tr=[[1.]+x for x in tr]; te=[[1.]+x for x in te]
    q=p+1; A=[[sum(x[i]*x[j] for x in tr)+(0 if i==0 else (10 if i==j else 0)) for j in range(q)] for i in range(q)]
    b=[sum(x[i]*y for x,y in zip(tr,yy)) for i in range(q)]
    w=solve(A,b)
    return [sum(a*z for a,z in zip(x,w)) for x in te],mu

finite=[r for r in rows if r['y'] is not None]
positive=[r for r in rows if r['y'] is None]
# Confirm every row label/status is identical across bases for each configuration.
config_outcomes={}
for r in rows: config_outcomes.setdefault((r['family'],r['m'],r['target_kind']),set()).add((r['status'],r['lower'],r['upper']))
assert all(len(v)==1 for v in config_outcomes.values())
folds=[]; all_err=[]; all_base=[]; wins=0
for g in BASES:
    tr=[r for r in finite if r['group']!=g]; te=[r for r in finite if r['group']==g]
    ps,mu=pred(tr,te); mae=sum(abs(a-r['y']) for a,r in zip(ps,te))/len(te)
    cats={(r['family'],r['m'],r['target_kind']):sum(q['y'] for q in tr if (q['family'],q['m'],q['target_kind'])==(r['family'],r['m'],r['target_kind']))/sum((q['family'],q['m'],q['target_kind'])==(r['family'],r['m'],r['target_kind']) for q in tr) for r in te}
    bm=sum(abs(cats[(r['family'],r['m'],r['target_kind'])]-r['y']) for r in te)/len(te)
    folds.append({'heldout_base':g,'n':len(te),'ridge_mae':mae,'matched_configuration_baseline_mae':bm,'improvement':bm-mae})
    all_err += [abs(a-r['y']) for a,r in zip(ps,te)]; all_base += [abs(cats[(r['family'],r['m'],r['target_kind'])]-r['y']) for r in te]
    wins += mae<bm
protocol_sha256=sys.argv[2]
result={'protocol_sha256':protocol_sha256,'rows':len(rows),'finite_interval_rows':len(finite),'positive_dimensional_rows':len(positive),'positive_dimensional_by_base':{str(g):sum(r['group']==g for r in positive) for g in BASES},'midpoint_proxy':'interval midpoint; not exact degree','identical_configuration_outcomes_across_bases':len(config_outcomes),'folds':folds,'aggregate_grouped_oof_mae':sum(all_err)/len(all_err),'aggregate_matched_configuration_baseline_mae':sum(all_base)/len(all_base),'heldout_groups_won':wins,'decision_threshold':{'aggregate_improvement':sum(all_base)/len(all_base)-sum(all_err)/len(all_err),'minimum_group_wins':4},'admissible_lead': wins>=4 and sum(all_err)<sum(all_base),'scope':'exploratory only; no elite-base ranking'}
out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False)
json.dump({'rows':rows},open(out/'rows.json','w'),indent=2)
json.dump(result,open(out/'result.json','w'),indent=2)
print(json.dumps(result,indent=2))
