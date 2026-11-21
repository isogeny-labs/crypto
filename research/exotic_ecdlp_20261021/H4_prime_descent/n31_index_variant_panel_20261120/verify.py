#!/usr/bin/env python3
"""Independent exact-target and paired-outcome audit of both n31 index variants."""
from sage.all import *
from sage.groups.generic import discrete_log_rho
import hashlib, inspect, itertools, json, random, statistics
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
P=json.loads((HERE/'protocol.json').read_text())
F=json.loads((HERE/'fixtures.json').read_text())
RESULT_PATH=HERE/'result.json'
O=json.loads(RESULT_PATH.read_text())
assert O['status']=='complete' and len(O['triples'])==10
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
for key in ('point_candidate','point_runner','point_result','packed_candidate','packed_runner','packed_result','prior_panel_fixtures'):
    assert sha(ROOT/P[key])==P[key+'_sha256']
assert sha(HERE/'make_fixtures.py')==P['fixture_generator_sha256']
assert sha(HERE/'fixtures.json')==P['fixtures_sha256']==O['fixture_sha256']
assert sha(HERE/'run.py')==P['panel_runner_sha256']
assert sha(HERE/'protocol.json')==O['protocol_sha256']
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest()==P['sage_rho_source_sha256']
CP=json.loads((ROOT/P['point_candidate']).read_text())
CK=json.loads((ROOT/P['packed_candidate']).read_text())
OP=json.loads((ROOT/P['point_result']).read_text())
OK=json.loads((ROOT/P['packed_result']).read_text())
assert CP['candidate_id']==OP['candidate_id']==O['candidate_ids']['point_ic']
assert CK['candidate_id']==OK['candidate_id']==O['candidate_ids']['packed_ic']
assert CP['candidate_id']!=CK['candidate_id']
assert CP['record']['factor_base']['encoded_points']==CK['record']['factor_base']['encoded_points']
assert OP['factor_logs']==OK['factor_logs']
field=CP['record']['field'];n,poly=field['n'],field['defining_polynomial_int'];assert n==31
F2=GF(2);PR=PolynomialRing(F2,'z');z=PR.gen()
mod=sum(F2((poly>>i)&1)*z**i for i in range(n+1))
K=GF(1<<n,'z',modulus=mod)
def elt(v):return K([(v>>i)&1 for i in range(n)])
def enc(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def rec(Q):return [enc(Q[0]),enc(Q[1])]
c=CP['record']['curve']['coefficients']
E=EllipticCurve(K,[1,elt(c['a2']),0,0,elt(c['a6'])])
def point(pair):return E(elt(pair[0]),elt(pair[1]))
G=point(CP['record']['curve']['G']);r=CP['record']['curve']['r']
assert r==P['subgroup_order']==34635389 and r*G==E(0)
assert rec(G)==F['generator']
base=[point(pair) for pair in CP['record']['factor_base']['encoded_points']]
assert len(base)==2090 and all(r*Q==E(0) for Q in base)
reps=sorted({min((Q,-Q),key=rec) for Q in base},key=rec)
assert len(reps)==len(OP['factor_logs'])==1045
logs=OP['factor_logs'];assert all(logs[i]*G==reps[i] for i in range(len(reps)))
col={Q:i for i,Q in enumerate(reps)}
def row_for(indices):
    row=[0]*len(reps)
    for i in indices:
        Q=base[i];rep=min((Q,-Q),key=rec);j=col[rep]
        row[j]=(row[j]+(1 if Q==rep else -1))%r
    return row
pair_sums={A+B for A in base for B in base}
assert len(pair_sums)==OP['distinct_pair_sums']==OK['distinct_pair_sums']==2116611
prior=json.loads((ROOT/P['prior_panel_fixtures']).read_text())
used={entry['known_scalar'] for entry in OP['relation_trace']}
assert used=={entry['known_scalar'] for entry in OK['relation_trace']}
assert F['excluded_prior_relation_scalars']==len(used)
used.add(OP['target_fixture_scalar'])
prior_scalars=[entry['fixture_scalar'] for entry in prior['targets']]
assert F['excluded_prior_panel_scalars']==prior_scalars
used.update(prior_scalars)
phase_names=('setup','isogeny','factor_base','precompute','queries','pdp','relation_check','matrix_build','relation_la','target_descent','recovery_check')
online_names=('target_query','target_pdp','target_relation_check','target_descent','target_recovery_check')
runs=[];ratios=[];rho_point=[];rho_packed=[]
for i,(fixture,triple) in enumerate(zip(F['targets'],O['triples'])):
    assert triple['index']==fixture['index']==i
    assert triple['order']==P['arm_order'][i].split(',')
    assert triple['workload_id']==fixture['workload_id']=='W'+digest(fixture['workload_record'])[:12]
    assert triple['target']==fixture['target']
    assert triple['target_point_sha256']==fixture['target_point_sha256']==digest(fixture['target'])
    counter=0
    while True:
        label=f'h4-n31-index-variant-panel-target-{i}-{counter}'
        scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),'big')%(r-1)+1
        if scalar not in used:break
        counter+=1
    assert fixture['target_label']==label and fixture['fixture_scalar']==scalar
    used.add(scalar)
    Q=point(fixture['target']);assert scalar*G==Q
    assert triple['scalar_certificate_pass']
    rho=triple['arms']['rho']
    assert rho['status']=='verified' and rho['scalar']==scalar and rho['scalar']*G==Q
    assert rho['wall_ns']>0
    a,b=triple['calibration_pre_ns'],triple['calibration_post_ns']
    assert a>0 and b>0 and abs(triple['calibration_ratio']-max(a,b)/min(a,b))<1e-12
    assert triple['calibration_pass']==(triple['calibration_ratio']<=1.25)
    for arm,C in (('point_ic',CP),('packed_ic',CK)):
        x=triple['arms'][arm]
        assert x['status']=='verified' and x['scalar']==scalar and x['scalar']*G==Q
        assert set(x['phase_wall_ns'])==set(online_names)
        assert sum(x['phase_wall_ns'].values())==x['wall_ns']<=x['outer_wall_ns']
        rng=random.Random(73031)
        for j,shift_row in enumerate(x['target_shifts']):
            shift=0 if j==0 else rng.randrange(1,r)
            assert shift_row['shift_number']==j and shift_row['shift_scalar']==shift
            shifted=Q+shift*G if shift else Q
            start=rng.randrange(len(base))
            first=next((k for k in range(len(base)) if shifted-base[(start+k)%len(base)] in pair_sums),None)
            if first is None:
                assert shift_row['status']=='exhausted' and shift_row['third_point_attempts']==len(base)
            else:
                assert shift_row['status']=='verified' and shift_row['third_point_attempts']==first+1
                indices=x['witness_indices']
                assert indices[2]==(start+first)%len(base)
                assert [rec(base[k]) for k in indices]==x['witness']
                assert sum((base[k] for k in indices),E(0))==shifted
                row=row_for(indices);assert digest(row)==x['row_sha256']
                recovered=(sum(v*logs[k] for k,v in enumerate(row))-shift)%r
                assert recovered==scalar and recovered*G==Q
                assert j==len(x['target_shifts'])-1
        run_id=C['candidate_id']+fixture['workload_id']+'R1'
        run={'schema_version':2,'kind':'full_dlp','status':'complete','proposal_id':None,
             'candidate_id':C['candidate_id'],'run_id':run_id,'workload_id':fixture['workload_id'],
             'pair_block_id':fixture['workload_id'],
             'source_curve_ref':'research/exotic_ecdlp_20261021/H4_prime_descent/n31_pair_20261120',
             'profile_id':'h4_n31_index_variant_'+arm+'_m3_shift16','isogeny_route_ref':'none',
             'subgroup_order':str(r),
             'provenance':{'workload_fixture_sha256':digest(fixture['workload_record']),
                           'source_sha256':P['point_runner_sha256'] if arm=='point_ic' else P['packed_runner_sha256'],
                           'host_id':O['host']['node'],
                           'resource_envelope_id':'one_local_sage_process_one_cpu_8GiB_observed_240s_triple_panel',
                           'calibration_id':'pre_post_512_additions_ratio_le_1.25',
                           'shared_precomputation_sha256':P['point_result_sha256'] if arm=='point_ic' else P['packed_result_sha256'],
                           'panel_runner_sha256':P['panel_runner_sha256'],
                           'rho_source_sha256':P['sage_rho_source_sha256']},
             'counts':{'ordinary_queries':0,'solved_queries':0,'verified_decompositions':0,
                       'verified_relations':0,'novel_rows':0,'effective_columns':len(reps),
                       'final_rank':len(reps),'pdp_attempts':0,'pdp_verified':0,
                       'pdp_proved_unsat':0,'pdp_timeout':0,'pdp_budget':0,
                       'pdp_error':0,'pdp_lift_rejected':0},
             'phase_operations':{name:None for name in phase_names},
             'phase_wall_ns':{name:None for name in phase_names},
             'online_phase_wall_ns':x['phase_wall_ns'],'online_wall_ns':x['wall_ns'],
             'rho_online_wall_ns':rho['wall_ns'],'rho_verified':True,'target_count':1,
             'target_point_sha256':fixture['target_point_sha256'],'precomputation_ready':True,
             'total_operations':None,'rho_operations':None,'verified_scalar':True,
             'scalar_certificate_ref':f'result.json#triples/{i}/{arm}',
             'peak_rss_bytes':triple['peak_rss_so_far_bytes'],
             'wall_ns':x['wall_ns']+rho['wall_ns']}
        runs.append(run)
    ptime=triple['arms']['point_ic']['wall_ns'];ktime=triple['arms']['packed_ic']['wall_ns']
    ratios.append(ptime/ktime);rho_point.append(rho['wall_ns']/ptime);rho_packed.append(rho['wall_ns']/ktime)
    assert triple['arms']['point_ic']['witness_indices']==triple['arms']['packed_ic']['witness_indices']
(HERE/'runs.jsonl').write_text(''.join(json.dumps(x,sort_keys=True)+'\n' for x in runs))
receipt={'schema':'h4-n31-index-variant-panel-validation-v1','status':'passed',
         'result_sha256':sha(RESULT_PATH),'fixtures_sha256':P['fixtures_sha256'],
         'verified_triples':len(O['triples']),'validated_one_target_rows':len(runs),
         'calibration_passes':sum(x['calibration_pass'] for x in O['triples']),
         'point_over_packed_ratios':ratios,'median_point_over_packed':statistics.median(ratios),
         'median_rho_over_point':statistics.median(rho_point),
         'median_rho_over_packed':statistics.median(rho_packed),
         'host_tier':'ordinary_exploratory'}
(HERE/'validation.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps(receipt,indent=2,sort_keys=True))
