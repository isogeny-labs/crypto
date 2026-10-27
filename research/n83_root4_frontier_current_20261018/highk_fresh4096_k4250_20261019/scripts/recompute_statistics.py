#!/usr/bin/env python3
"""Replay frozen K4250 paired intervals from committed target certificates."""
import argparse,gzip,hashlib,json
from pathlib import Path
import numpy as np

def main():
 p=argparse.ArgumentParser();p.add_argument('--evidence',type=Path,default=Path(__file__).resolve().parent.parent);args=p.parse_args()
 e=args.evidence.resolve();parent=e.parent/'highk_fresh4096_20261019'
 plan=json.loads((e/'k4250_plan.json').read_text())
 fixture=json.loads((parent/'fixture_manifest.json').read_text())
 assert plan['status']=='FROZEN_BEFORE_K4250_RUN' and plan['targets_sha256']==fixture['targets_sha256']
 arrays={}
 for k in (4250,4300,4405):
  root=e if k==4250 else parent
  manifest={x['k']:x for x in json.loads((root/'target_blobs_manifest.json').read_text())}[k]
  blob=(root/'runs'/f'k{k}_wide3'/'targets_result.jsonl.gz').read_bytes()
  raw=gzip.decompress(blob)
  assert hashlib.sha256(blob).hexdigest()==manifest['gzip_sha256']
  assert hashlib.sha256(raw).hexdigest()==manifest['raw_sha256']
  rows=[json.loads(line) for line in raw.splitlines()]
  assert len(rows)==4096 and [x['fixture_index'] for x in rows]==list(range(4096))
  arrays[k]=np.array([x['target_ms'] for x in rows],dtype=np.float64)
 rng=np.random.Generator(np.random.PCG64(20261021))
 analysis=json.loads((e/'k4250_analysis.json').read_text())
 for other in (4300,4405):
  d=arrays[4250]-arrays[other];n=len(d);block=64
  sums=np.array([sum(d[(i+j)%n] for j in range(block)) for i in range(n)])
  rep=np.empty(20000)
  for start in range(0,20000,500):
   idx=rng.integers(0,n,size=(500,n//block),endpoint=False)
   rep[start:start+500]=sums[idx].sum(axis=1)/n
  expected=analysis['comparisons'][f'k4250_minus_k{other}']
  assert abs(float(np.mean(d))-expected['mean_ms'])<1e-12
  assert np.allclose(np.quantile(rep,[.005,.995]),expected['bootstrap_99pct_ci_ms'],rtol=0,atol=1e-12)
 result={'schema':'n83.wide3-fresh4096-k4250-statistics-replay/v1','status':'PASS','comparisons_verified':2,'mean_target_ms':{str(k):float(np.mean(v)) for k,v in arrays.items()},'selected_online_candidate_id':None}
 print(json.dumps(result,sort_keys=True))
if __name__=='__main__':main()
