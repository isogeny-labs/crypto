#!/usr/bin/env python3
"""Recompute the frozen paired block bootstrap from committed target certificates."""
import argparse,gzip,hashlib,json
from pathlib import Path
import numpy as np

def mean_interval(a,b,seed):
 d=a-b;n=len(d);block=64
 sums=np.array([sum(d[(i+j)%n] for j in range(block)) for i in range(n)])
 rng=np.random.Generator(np.random.PCG64(seed))
 out=np.empty(20000)
 for start in range(0,20000,500):
  ids=rng.integers(0,n,size=(500,n//block),endpoint=False)
  out[start:start+500]=sums[ids].sum(axis=1)/n
 return float(np.mean(d)),np.quantile(out,[0.005,0.995]).tolist(),float(np.mean(out<=0))

def main():
 p=argparse.ArgumentParser();p.add_argument('--evidence',type=Path,default=Path(__file__).resolve().parent.parent);args=p.parse_args()
 e=args.evidence.resolve();records={x['k']:x for x in json.loads((e/'target_blobs_manifest.json').read_text())}
 arrays={}
 for k in (4096,4300,4405):
  blob=(e/'runs'/f'k{k}_wide3'/'targets_result.jsonl.gz').read_bytes();record=records[k]
  if hashlib.sha256(blob).hexdigest()!=record['gzip_sha256']:raise ValueError(f'compressed digest: {k}')
  raw=gzip.decompress(blob)
  if hashlib.sha256(raw).hexdigest()!=record['raw_sha256']:raise ValueError(f'raw digest: {k}')
  rows=[json.loads(line) for line in raw.splitlines()]
  if len(rows)!=4096 or [x['fixture_index'] for x in rows]!=list(range(4096)):raise ValueError(f'ordered targets: {k}')
  arrays[k]=np.array([r['target_ms'] for r in rows],dtype=np.float64)
 paired=json.loads((e/'paired_analysis.json').read_text());extension=json.loads((e/'extension_analysis.json').read_text())
 comparisons=[(4405,4300,20261019,paired['k4405_minus_k4300_mean_ms'],paired['bootstrap_99pct_ci_k4405_minus_k4300_ms']),
              (4096,4300,20261020,extension['comparisons']['k4096_minus_k4300']['mean_ms'],extension['comparisons']['k4096_minus_k4300']['bootstrap_99pct_ci_ms'])]
 # The frozen extension uses one PCG64 stream for both comparisons in this order.
 for a,b,seed,expected,ci_expected in comparisons:
  mean,ci,_=mean_interval(arrays[a],arrays[b],seed)
  if abs(mean-expected)>1e-12 or not np.allclose(ci,ci_expected,rtol=0,atol=1e-12):raise ValueError(f'bootstrap differs: {a}-{b}')
 # Advance the extension RNG exactly as the original first comparison did.
 d=arrays[4096]-arrays[4405];n=len(d);block=64
 sums=np.array([sum(d[(i+j)%n] for j in range(block)) for i in range(n)])
 rng=np.random.Generator(np.random.PCG64(20261020));out=np.empty(20000)
 for start in range(0,20000,500):
  rng.integers(0,n,size=(500,n//block),endpoint=False)
 for start in range(0,20000,500):
  ids=rng.integers(0,n,size=(500,n//block),endpoint=False)
  out[start:start+500]=sums[ids].sum(axis=1)/n
 expected=extension['comparisons']['k4096_minus_k4405']
 if abs(float(np.mean(d))-expected['mean_ms'])>1e-12 or not np.allclose(np.quantile(out,[0.005,0.995]),expected['bootstrap_99pct_ci_ms'],rtol=0,atol=1e-12):raise ValueError('second extension comparison differs')
 result={'schema':'n83.wide3-fresh4096-statistics-replay/v1','status':'PASS','mean_target_ms':{str(k):float(np.mean(arrays[k])) for k in arrays},'comparisons_verified':3,'selected_online_candidate_id':None}
 print(json.dumps(result,sort_keys=True))
if __name__=='__main__':main()
