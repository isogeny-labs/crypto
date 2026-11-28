#!/usr/bin/env python3
"""Recompute summary from saved JSON; does not import the producer."""
import json,statistics as st, random,math,platform,sys,hashlib
from pathlib import Path
R=1069

def summarize(filename):
    d=json.loads(Path(filename).read_text());rs=d['records']
    assert len(rs)==d['seeds'] and all('arms' in r for r in rs)
    by={}
    for name in ['full','0.75','0.5','0.25']:
        arm=[r['arms'][name] for r in rs]
        div=[a['total_group_add']/r['arms']['full']['total_group_add'] for a,r in zip(arm,rs)]
        against_rho=[a['total_group_add']/r['rho']['group_add'] for a,r in zip(arm,rs)]
        by[name]={
            'complete':sum(a['status']=='ok' for a in arm),
            'median_group_add_ratio_vs_full':st.median(div),
            'mean_group_add_ratio_vs_full':st.mean(div),
            'count_group_add_below_full':sum(x<1 for x in div),
            'count_group_add_le_90pct_full':sum(x<=.9 for x in div),
            'mean_group_add':st.mean(a['total_group_add'] for a in arm),
            'median_group_add':st.median(a['total_group_add'] for a in arm),
            'median_ic_rho_ratio':st.median(against_rho),
            'mean_relation_attempts':st.mean(a['relation_attempts'] for a in arm),
            'mean_relation_rows':st.mean(a['relations'] for a in arm),
            'mean_descent_attempts':st.mean(a['descent_trials'] for a in arm),
            'mean_unique_sums':st.mean(a['distinct_pair_sums'] for a in arm),
            'mean_la_mod_ops':st.mean(a['phases']['linear_algebra']['mod_ops'] for a in arm),
            'mean_matrix_ns':st.mean(a['phases']['matrix']['wall_ns'] for a in arm),
            'mean_la_ns':st.mean(a['phases']['linear_algebra']['wall_ns'] for a in arm),
            'mean_phase_group_add':{key:st.mean(a['phases'][key]['group_add'] for a in arm)
                                    for key in ['factor_base','pair_table','collection','descent']},
            'median_proxy_ratio':{key:st.median(a[key]/r['arms']['full'][key] for a,r in zip(arm,rs))
                                  for key in ['total_group_add','total_proxy_w01','total_proxy_w1']},
        }
    half=[r['arms']['0.5']['total_group_add']/r['arms']['full']['total_group_add'] for r in rs]
    gen=random.Random(7);med=[]
    for k in range(5000):med.append(st.median(half[gen.randrange(len(half))] for _ in half))
    med.sort(); by['0.5']['median_95pct_seed_bootstrap_CI']=[med[124],med[4874]]
    return {'base':d['base'],'seeds':d['seeds'],'seed0':d['seed0'],
            'sha256':hashlib.sha256(Path(filename).read_bytes()).hexdigest(),'rho_mean_add':st.mean(r['rho']['group_add'] for r in rs),
            'rho_ideal_group_ops':math.sqrt(math.pi*R/2),'arms':by}

if __name__=='__main__':
    result={'protocol':'EXP-IC-PARTIAL-TABLE-20261118-R2','h1':{'criterion':'half median group add/full <=0.90 and >=22/24 full rank+descent'},
      'environment':{'python':platform.python_version(),'platform':platform.platform(),'processor':platform.processor()},
      'primary':summarize('raw/primary.json'),'sensitivity':summarize('raw/sensitivity.json')}
    v=result['primary']['arms']['0.5'];result['h1']['passed']=v['median_group_add_ratio_vs_full']<=.9 and v['complete']>=22
    Path('SUMMARY.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print('H1 PASS' if result['h1']['passed'] else 'H1 FAIL', 'B32 median=',v['median_group_add_ratio_vs_full'],'B16=',result['sensitivity']['arms']['0.5']['median_group_add_ratio_vs_full'])
