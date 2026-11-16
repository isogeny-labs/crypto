import json, statistics as st, math
D='/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L6-frobenius-stable-nonsubspace-bases/'
Y=json.load(open(D+'raw/yields.json')); S=json.load(open(D+'raw/solve.json')); N1=json.load(open(D+'raw/n1.json'))
out={'experiment':'L6-frobenius-stable-nonsubspace-bases','conductor_task':'T-5736','curve':'y^2+xy=x^3+x^2+1','yields':{},'solve':{},'n1':N1}
for n,R in Y.items():
    tab={}
    for nm,y in R['bases'].items():
        m=y['B_points']; r={'11':991,'23':4196903}[n]
        N3nt=y['N3']-(3*m*m-3*m) if y['N3']>=0 else None
        tab[nm]=dict(size=y['size'],oncurve_x=y['oncurve_x'],subgroup_x=y['subgroup_x'],subgroup_points=m,
          columns=y['columns'],N2=y['N2'],N2_ratio_null=y['N2_ratio'],N2_per_column=y['N2_per_column'],
          N3_raw=y['N3'],N3_nontrivial=N3nt,N3_nontrivial_ratio_null=(N3nt/y['N3_null'] if N3nt is not None and m else None),
          N3_nontrivial_per_column=(N3nt/y['columns'] if N3nt is not None and y['columns'] else None),
          decomp_mean_16=y['decomp_per_target_mean'],decomp_null=y['decomp_null'],BD2_over_N=y['BD2_over_N'])
    out['yields'][n]=dict(meta={k:v for k,v in R.items() if k!='bases'},bases=tab)
for n,R in S.items():
    t={}
    for nm,b in R.items():
        rows=b['rows']
        def agg(key,idx):
            res={}
            for lab in ('SAT','UNSAT','CONSISTENT','TIMEOUT','TIMEOUT-SKIPPED'):
                ts=[r[key][1] for r in rows if r[key][0]==lab and r[key][1] is not None]
                c=sum(1 for r in rows if r[key][0]==lab)
                if c: res[lab]=dict(count=c,median_s=(st.median(ts) if ts else None))
            return res
        degs=[r['ms'][2] for r in rows if r['ms'][2] is not None]
        agree=all((r['sat'][0]=='SAT')==(r['gt_s3_ordered_pairs']>0) for r in rows if r['gt_s3_ordered_pairs'] is not None and r['sat'][0] in('SAT','UNSAT'))
        t[nm]=dict(vars_per_point=b['nvars_per_point'],targets=len(rows),msolve_npolys=rows[0]['msolve_npolys'],
          gt_targets_with_s3_solution=sum(1 for r in rows if (r['gt_s3_ordered_pairs'] or 0)>0),
          subgroup_decomposable_targets=sum(1 for r in rows if r['gt_subgroup_point_decomps']>0),
          sat=agg('sat',1),msolve=agg('ms',1),msolve_max_degree=(max(degs) if degs else None),solver_agrees_with_ground_truth=agree)
    out['solve'][n]=t
json.dump(out,open(D+'RESULTS.json','w'),indent=1)
print(json.dumps(out['solve'],indent=0)[:4000])
