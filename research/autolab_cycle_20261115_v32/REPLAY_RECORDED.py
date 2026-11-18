"""Verify compiled correctness equivalence and both matching timing datasets."""
from pathlib import Path
import json,hashlib,collections,statistics,math,runpy
R=Path(__file__).resolve().parents[2]
def load(path,sha,wrapped=True):
 data=(R/path).read_bytes();assert hashlib.sha256(data).hexdigest()==sha;v=json.loads(data)
 if wrapped:assert v['error']is None;return v['out']
 return v
old=load('research/subspace_factor_oracles_20261115_v30/raw.json','f5775d673e90e6462021ad859f5368739091ebdf67002e8014713d71d6966593');new=load('research/compiled_subspace_oracles_20261115_v31/raw.json','7b1ea647267298b6cfa332c4283eca27b0c1e4a06522774b445509ad23ad2fce')
runpy.run_path(str(R/'research/autolab_cycle_20261115_v30/REPLAY_RECORDED.py'))
assert old['cases']==new['cases']==3848
for a,b in zip(old['controls'],new['controls']):assert a['transcript_sha256']==b['transcript_sha256']and a['cases']==b['cases']and a['accepted']==b['accepted']
for a,b in zip(old['actual']['records'],new['actual']['records']):
 assert a['H']==b['H']and a['planted_roots']==b['planted_roots']and a['result']['supported']==b['result']['supported']and a['result']['roots']==b['result']['roots']
assert new['actual']['maximum_nodes']==14 and new['actual']['gcd_calls']==119
fresh=load('research/factor_density_bench_20261115_v32/DATA.json','3e839bc2ec00017efef89e7d438dad387bf804a96f089734bf4f89ae7758915c',False)
def mul(a,b):
 v=0
 for i in range(83):
  if b>>i&1:v^=a<<i
 while v.bit_length()>83:v^=0x800000000200000000007<<(v.bit_length()-84)
 return v
def expected(case,profile=None):
 H=[1]
 for x in case['planted_roots']:
  Q=[0]*(len(H)+1)
  for i,c in enumerate(H):Q[i]^=mul(c,x);Q[i+1]^=c
  H=Q
 assert H==case['H'];good=all(x<1<<18 for x in case['planted_roots']);out=dict(supported=good,roots=[[x,m]for x,m in sorted(collections.Counter(case['planted_roots']).items())]if good else None)
 if profile is not None:out['profile']=profile
 return out
simple=[expected(c)for c in old['actual']['records']];dense=[expected(c,g['name'])for g in fresh['groups']for c in g['cases']];assert len({tuple(c['H'])for g in fresh['groups']for c in g['cases']})==192
for path,sha,want in [('research/compiled_subspace_bench_20261115_v31/raw.json','92ebde43c8859ddb85d94dfc152918c90bc96df30648c9ff18229ea6981a55c2',simple),('research/factor_density_bench_20261115_v32/raw.json','478bbc42804a5eb1727ad898c0d889c5570708ad2c4717d99c45dde129bdbe34',dense)]:
 out=load(path,sha);assert len(out['records'])==9;digest=hashlib.sha256(json.dumps(want,sort_keys=True,separators=(',',':')).encode()).hexdigest()
 for v in out['records']:assert v['rows']==want and v['roots_sha256']==digest and v['cold_wall_seconds']>=v['pre_stdout_seconds']
 for m in ['trace','support_trace','subspace']:
  vs=[v['cold_wall_seconds']for v in out['records']if v['method']==m];assert len(vs)==3
  assert math.isclose(statistics.median(vs),out['median_cold_wall'][m],rel_tol=1e-12)and math.isclose(max(vs)/min(vs)-1,out['AA_spread'][m],rel_tol=1e-12)
 ratio=min(out['median_cold_wall'][m]for m in ['trace','support_trace'])/out['median_cold_wall']['subspace'];assert math.isclose(ratio,out['component_ratio'],rel_tol=1e-12)and out['AA_spread']['subspace']>.10
 if 'profile_median_loop'in out:
  for p,v in out['profile_median_loop'].items():
   for m,x in v.items():assert math.isclose(x,statistics.median(r['profile_seconds'][p]for r in out['records']if r['method']==m),rel_tol=1e-12)
   assert math.isclose(min(v[m]for m in ['trace','support_trace'])/v['subspace'],out['profile_component_ratios'][p],rel_tol=1e-12)
print('VERIFIED: compiled3848case equivalence;2160matching coldquery results; ratios/AA/profilearithmetic; bothcompiledAAspreads exceed10percent.')
