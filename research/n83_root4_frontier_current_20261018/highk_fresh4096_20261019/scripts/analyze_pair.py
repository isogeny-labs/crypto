#!/usr/bin/env python3
"""Frozen-plan paired analysis for the local N83 wide3 diagnostic."""
import hashlib
import json
from pathlib import Path
import numpy as np

HERE=Path(__file__).resolve().parent
RUN=Path('/Volumes/SSD990-2/tmp/n83_wide3_fresh4096_20261019')
PLAN=json.loads((HERE/'paired_plan.json').read_text())
assert PLAN['status']=='FROZEN_BEFORE_RUN'
assert PLAN['target_sha256']==hashlib.sha256((HERE/'targets.jsonl').read_bytes()).hexdigest()
ARMS=['k4300_wide3','k4405_wide3']
analysis=[]
rows=[]
phases=('target_query_ms','target_pdp_ms','target_relation_check_ms','target_descent_ms','target_recovery_check_ms')
for arm,k in zip(ARMS,(4300,4405)):
    out=RUN/f'{arm}_run1'
    a=json.loads((out/'analysis.json').read_text())
    u=json.loads((out/'usage.json').read_text())
    src=json.loads((out/'source.json').read_text())
    protocol=json.loads((HERE/f'k{k}_wide3_fresh4096_protocol.json').read_text())
    raw=[json.loads(line) for line in (out/'targets_result.jsonl').read_text().splitlines()]
    if (a['status']!='PASS_INDEPENDENT_REPLAY' or u['status']!='PASS_PRODUCER'
        or len(raw)!=4096 or len(a['online_ms_diagnostic'])!=4096
        or protocol['binary_sha256']!=src['binary_sha256']
        or protocol['source_sha256']!=src['source_sha256']
        or protocol['targets_sha256']!=src['target_sha256']
        or protocol['pilot_runner_sha256']!=src['runner_sha256']
        or [r['fixture_index'] for r in raw]!=list(range(4096))):
        raise ValueError(f'arm audit mismatch: {arm}')
    timings=np.array([r['target_ms'] for r in raw],dtype=np.float64)
    if not np.allclose(timings,a['online_ms_diagnostic'],rtol=0,atol=0):
        raise ValueError('replay timings differ')
    phase_summary={name:float(np.mean([r[name] for r in raw])) for name in phases}
    if not np.all(np.isfinite(timings)) or not np.all(timings>0):
        raise ValueError('invalid times')
    analysis.append(a);rows.append(timings)
    print(arm, float(np.mean(timings)),phase_summary)

d=rows[1]-rows[0]
block=64
n=4096
assert n%block==0
# Every possible circular 64-point block, then exactly 64 resampled blocks.
block_sums=np.array([sum(d[(i+j)%n] for j in range(block)) for i in range(n)])
rng=np.random.Generator(np.random.PCG64(20261019))
replicates=np.empty(20000,dtype=np.float64)
for start in range(0,len(replicates),500):
    index=rng.integers(0,n,size=(500,n//block),endpoint=False)
    replicates[start:start+500]=block_sums[index].sum(axis=1)/n
q=np.quantile(replicates,[0.005,0.995])
result={
 'schema':'n83.wide3-fresh4096-paired-analysis/v1',
 'status':'PASS_LOCAL_DIAGNOSTIC',
 'hardware_tier':'local_correctness_diagnostic',
 'selected_online_candidate_id':None,
 'point_count':n,
 'targets_sha256':PLAN['target_sha256'],
 'candidate_ids':[json.loads((HERE/f'k{k}_wide3_fresh4096_protocol.json').read_text())['candidate_id'] for k in (4300,4405)],
 'arm_order':ARMS,
 'verified_targets_per_arm':[4096,4096],
 'mean_target_ms':[float(np.mean(x)) for x in rows],
 'median_target_ms':[float(np.median(x)) for x in rows],
 'cold_wall_seconds_diagnostic':[a['cold_wall_seconds_diagnostic'] for a in analysis],
 'peak_sampled_rss_bytes':[a['peak_sampled_rss_bytes'] for a in analysis],
 'k4405_minus_k4300_mean_ms':float(np.mean(d)),
 'k4405_minus_k4300_median_ms':float(np.median(d)),
 'positive_fraction':float(np.mean(d>0)),
 'k4300_over_k4405_mean_ratio':float(np.mean(rows[0])/np.mean(rows[1])),
 'bootstrap_method':'circular moving blocks of 64, 20000 resamples, PCG64 seed 20261019',
 'bootstrap_99pct_ci_k4405_minus_k4300_ms':q.tolist(),
 'bootstrap_one_sided_p_nonpositive':float(np.mean(replicates<=0)),
 'mean_phase_ms':{arm:{name:float(np.mean([r[name] for r in [json.loads(line) for line in (RUN/f"{arm}_run1"/'targets_result.jsonl').read_text().splitlines()]])) for name in phases} for arm in ARMS},
}
(HERE/'paired_analysis.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ('status','mean_target_ms','k4405_minus_k4300_mean_ms','bootstrap_99pct_ci_k4405_minus_k4300_ms','bootstrap_one_sided_p_nonpositive','k4300_over_k4405_mean_ratio')},sort_keys=True))
