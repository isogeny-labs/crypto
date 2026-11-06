#!/usr/bin/env python3
import hashlib,json
from pathlib import Path
import numpy as np
HERE=Path(__file__).resolve().parent
RUN=Path('/Volumes/SSD990-2/tmp/n83_wide3_fresh4096_20261019')
plan=json.loads((HERE/'extension_plan.json').read_text())
assert plan['status']=='FROZEN_BEFORE_K4096_RUN'
assert plan['targets_sha256']==hashlib.sha256((HERE/'targets.jsonl').read_bytes()).hexdigest()
def arm(k):
 name=f'k{k}_wide3'
 p=RUN/f'{name}_run1'
 a=json.loads((p/'analysis.json').read_text())
 src=json.loads((p/'source.json').read_text())
 protocol=json.loads((HERE/f'{name}_fresh4096_protocol.json').read_text())
 assert a['status']=='PASS_INDEPENDENT_REPLAY' and len(a['online_ms_diagnostic'])==4096
 assert src['binary_sha256']==protocol['binary_sha256'] and src['target_sha256']==protocol['targets_sha256']
 assert src['runner_sha256']==protocol['pilot_runner_sha256']
 return a,np.array(a['online_ms_diagnostic'],dtype=np.float64)
info={k:arm(k) for k in (4096,4300,4405)}
res={'schema':'n83.wide3-fresh4096-extension-analysis/v1','status':'PASS_LOCAL_DIAGNOSTIC','hardware_tier':'local_correctness_diagnostic','selected_online_candidate_id':None,'point_count':4096,'target_sha256':plan['targets_sha256'],'arms':{str(k):{'mean_target_ms':float(np.mean(info[k][1])),'median_target_ms':float(np.median(info[k][1])),'cold_wall_seconds_diagnostic':info[k][0]['cold_wall_seconds_diagnostic'],'peak_sampled_rss_bytes':info[k][0]['peak_sampled_rss_bytes']} for k in info},'comparisons':{},'bootstrap_method':'circular moving blocks of 64, 20000 resamples per comparison, PCG64 seed 20261020'}
rng=np.random.Generator(np.random.PCG64(20261020))
for other in (4300,4405):
 d=info[4096][1]-info[other][1]
 n=len(d);block=64
 sums=np.array([sum(d[(i+j)%n] for j in range(block)) for i in range(n)])
 values=np.empty(20000)
 for start in range(0,20000,500):
  ids=rng.integers(0,n,size=(500,n//block),endpoint=False)
  values[start:start+500]=sums[ids].sum(axis=1)/n
 res['comparisons'][f'k4096_minus_k{other}']={'mean_ms':float(np.mean(d)),'median_ms':float(np.median(d)),'positive_fraction':float(np.mean(d>0)),'bootstrap_99pct_ci_ms':np.quantile(values,[0.005,0.995]).tolist(),'bootstrap_one_sided_p_nonpositive':float(np.mean(values<=0))}
(HERE/'extension_analysis.json').write_text(json.dumps(res,sort_keys=True,indent=2)+'\n')
print(json.dumps(res,sort_keys=True))
