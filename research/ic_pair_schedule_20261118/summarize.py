#!/usr/bin/env python3
import json,statistics,random,math,zipfile,hashlib
from pathlib import Path
root=Path(__file__).resolve().parent
rng=random.Random(202611182047)
metrics={'protocol':'EXP-IC-PAIR-SCHEDULE-20261118-R3','cells':{},'prior_R2_replay':{},'frozen_protocol_sha256':hashlib.sha256((root/'PROTOCOL.md').read_bytes()).hexdigest()}
for b in (16,32):
    d=json.loads((root/'raw'/f'B{b}.json').read_text())
    recs=d['records'];f=0.25 if b==16 else 0.5
    arms=('random','aa_random','distance','full')
    cell={'base':b,'fraction':f,'n':len(recs),'arms':{}}
    for a in arms:
        items=[r['arms'][a] for r in recs]
        phase={name:{'group_add_mean':round(statistics.mean(x['phases'][name]['group_add'] for x in items),3)} if name in ('factor_base','pair_table','collection','descent') else {'wall_ns_median':statistics.median(x['phases'][name]['wall_ns'] for x in items)} for name in ('scheduling','factor_base','pair_table','collection','matrix','linear_algebra','descent')}
        for name in ('factor_base','pair_table','collection','descent'):
            phase[name]['wall_ns_median']=statistics.median(x['phases'][name]['wall_ns'] for x in items)
        phase['linear_algebra']['Fr_mod_ops_mean']=round(statistics.mean(x['phases']['linear_algebra']['mod_ops'] for x in items),3)
        cell['arms'][a]={'solved':sum(x['status']=='ok' for x in items),'censored':sum(x['status']!='ok' for x in items),'mean_group_add':statistics.mean(x['total_group_add'] for x in items),'median_group_add':statistics.median(x['total_group_add'] for x in items),'mean_attempts':statistics.mean(x['relation_attempts'] for x in items),'mean_distinct_pair_sums':statistics.mean(x['distinct_pair_sums'] for x in items),'phase':phase,'mean_total_proxy_w01':statistics.mean(x['total_proxy_w01'] for x in items),'median_total_proxy_w01':statistics.median(x['total_proxy_w01'] for x in items),'mean_total_proxy_w1':statistics.mean(x['total_proxy_w1'] for x in items),'median_total_proxy_w1':statistics.median(x['total_proxy_w1'] for x in items)}
    both=[r for r in recs if r['arms']['random']['status']=='ok' and r['arms']['distance']['status']=='ok']
    ratios=[r['arms']['distance']['total_group_add']/r['arms']['random']['total_group_add'] for r in both]
    allratios=[(r['arms']['distance']['total_group_add']/r['arms']['random']['total_group_add'] if r['arms']['random']['status']=='ok' else 0.0) for r in recs]
    # Censored baseline treated as +inf cost, improving side as 0 ratio; ratio is 0 by convention, not a solved cost comparison.
    p_boot=[]
    for _ in range(5000):
        draw=[allratios[rng.randrange(len(allratios))] for i in range(len(allratios))]
        p_boot.append(statistics.median(draw))
    p_boot.sort()
    rho=[r['rho']['group_add'] for r in recs]
    cell['rho']={'mean_group_add':statistics.mean(rho),'median_group_add':statistics.median(rho),'ideal_group_add':math.sqrt(math.pi*1069/2)}
    cell['pairwise']={'common_solved':len(both),'distance_less_than_random':sum(x<1 for x in ratios),'median_distance_over_random_common_solved':statistics.median(ratios),'median_distance_over_random_censor_as_inf':statistics.median(allratios),'bootstrap_95_censor_as_inf':[p_boot[124],p_boot[4874]],'median_distance_over_rho':statistics.median(r['arms']['distance']['total_group_add']/r['rho']['group_add'] for r in recs)}
    cell['pass_rule']={'cost_primary_B16':(statistics.median(allratios)<=0.9 if b==16 else None),'size_control_B32':(statistics.median(allratios)<=1.05 if b==32 else None),'solves_both_ge22':cell['arms']['random']['solved']>=22 and cell['arms']['distance']['solved']>=22}
    metrics['cells'][str(b)]=cell
with zipfile.ZipFile('/mnt/data/ic_partial_pairtable_20261118.zip') as z:
    for b,prev,prev_arm in ((16,'raw/sensitivity.json','0.25'),(32,'raw/primary.json','0.5')):
        raw=json.loads(z.read(prev));now=json.loads((root/'raw'/f'B{b}.json').read_text()); lookup={r['seed']:r for r in now['records']}
        checked=0;bad=[]
        for prior in raw['records']:
            arm_old=prior['arms'][prev_arm];arm_new=lookup[prior['seed']]['arms']['random']
            stable=('pair_entries','distinct_pair_sums','rank','rows_sha256','relation_attempts','relations','dependent_rows','descent_trials','descent_pair','recovered_log','total_group_add','total_proxy_w01','total_proxy_w1')
            if not all(arm_old.get(x)==arm_new.get(x) for x in stable):bad.append(prior['seed'])
            checked+=1
        metrics['prior_R2_replay'][str(b)]={'matched':checked-len(bad),'checked':checked,'mismatched_seeds':bad}
metrics['frozen_h1_pass']=metrics['cells']['16']['pass_rule']['cost_primary_B16'] and metrics['cells']['16']['pass_rule']['solves_both_ge22'] and metrics['cells']['32']['pass_rule']['size_control_B32'] and metrics['cells']['32']['pass_rule']['solves_both_ge22']
(root/'SUMMARY.json').write_text(json.dumps(metrics,indent=2,sort_keys=True)+'\n')
print(json.dumps({'h1_pass':metrics['frozen_h1_pass'],'cells':{b:{'solved':[c['arms']['random']['solved'],c['arms']['distance']['solved']],'median_ratio':c['pairwise']['median_distance_over_random_censor_as_inf'],'rho_ratio':c['pairwise']['median_distance_over_rho'],'pass':c['pass_rule']} for b,c in metrics['cells'].items()},'replay':metrics['prior_R2_replay']},indent=2))
