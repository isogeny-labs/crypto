import json,glob,os,sys
for b in (24,32,28):
    o=json.load(open(f'/dev/shm/dg3val/recompute-P{b}.json'))
    worst={'point':0.0,'boot':0.0,'curve':0.0,'curve_boot':0.0}; n=0; missing=[]
    for f in sorted(glob.glob(f'/dev/shm/dg3data/P{b}/analysis/compare-*.json')):
        nm=os.path.basename(f)[8:-5]; pair,w=nm.rsplit('-w',1); wk={'0':'0.0','0.1':'0.1','1':'1.0'}[w]
        if pair not in o['ratios']: missing.append(nm); continue
        c=json.load(open(f)); m=o['ratios'][pair][wk]; n+=1
        assert c['pricing_weight']==float(wk) and c['resample_seed']==20261002 and c['ops']['resamples']==4000
        rel=lambda x,y: abs(x-y)/abs(x) if x else abs(y)
        worst['point']=max(worst['point'],rel(c['ops']['ratio_b_over_a'],m['ratio_b_over_a']))
        for i in (0,1): worst['boot']=max(worst['boot'],rel(c['ops']['ci95_bootstrap'][i],m['ci95_ecbench_replica'][i]))
        for cv in c['curves']:
            mc=m['per_curve'][cv['slug']]
            worst['curve']=max(worst['curve'],rel(cv['ratio_b_over_a'],mc['ratio_b_over_a']))
            for i in (0,1): worst['curve_boot']=max(worst['curve_boot'],rel(cv['ci95_bootstrap'][i],mc['ci95_ecbench_replica'][i]))
        if c['ops']['status']!='ok' or c['ops']['censored_pairs']!=0 or c['ops']['unpaired_runs']!=0 or not c['ops']['same_seeds']: print('FLAG',nm,c['ops']['status'])
    print(f'P{b}: {n} compare files matched; worst rel diff', worst, 'not recomputed:', missing)
