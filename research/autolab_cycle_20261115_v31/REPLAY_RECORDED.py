"""Verify matching roots, cold timing arithmetic and the refuted prediction."""
from pathlib import Path
import json,hashlib,collections,statistics,math
R=Path(__file__).resolve().parents[2]
def load(path,sha):
 data=(R/path).read_bytes();assert hashlib.sha256(data).hexdigest()==sha;v=json.loads(data);assert v['error']is None;return v['out']
old=load('research/subspace_factor_oracles_20261115_v30/raw.json','f5775d673e90e6462021ad859f5368739091ebdf67002e8014713d71d6966593')['actual']
run=load('research/subspace_factor_bench_20261115_v30/raw.json','5da6d82994a058d4ae0a92b03af7cc24a4cc95e378eeffecb26bf596a180ee91');expected=[]
def mul(a,b):
 v=0
 for i in range(83):
  if b>>i&1:v^=a<<i
 while v.bit_length()>83:v^=old['modulus']<<(v.bit_length()-84)
 return v
for case in old['records']:
 H=[1]
 for x in case['planted_roots']:
  P=[0]*(len(H)+1)
  for i,c in enumerate(H):P[i]^=mul(c,x);P[i+1]^=c
  H=P
 assert H==case['H'];good=all(x<1<<18 for x in case['planted_roots']);roots=[[x,m]for x,m in sorted(collections.Counter(case['planted_roots']).items())]if good else None;expected.append(dict(supported=good,roots=roots))
digest=hashlib.sha256(json.dumps(expected,sort_keys=True,separators=(',',':')).encode()).hexdigest();methods=['trace','support_trace','subspace'];assert len(run['records'])==9
for value in run['records']:
 assert value['method']in methods and value['rows']==expected and value['roots_sha256']==digest
 assert value['cold_wall_seconds']>=value['pre_stdout_seconds']>=value['solve_seconds']+value['verify_seconds']
for m in methods:
 rows=[v for v in run['records']if v['method']==m];assert sorted(v['repetition']for v in rows)==[0,1,2]
 med=statistics.median(v['cold_wall_seconds']for v in rows);spread=max(v['cold_wall_seconds']for v in rows)/min(v['cold_wall_seconds']for v in rows)-1
 assert math.isclose(med,run['median_cold_wall'][m],rel_tol=1e-12)and math.isclose(spread,run['AA_spread'][m],rel_tol=1e-12)
ratio=min(run['median_cold_wall'][m]for m in['trace','support_trace'])/run['median_cold_wall']['subspace'];assert math.isclose(ratio,run['component_ratio'],rel_tol=1e-12)and ratio<1
assert all(v<=.10 for v in run['AA_spread'].values())
print('VERIFIED: all432cold-query results, rootdigest, all9timings andcomponentratio; newuncachedarm slowerthanstrongestmatchingbaseline.')
