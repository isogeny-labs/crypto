import pathlib,json,hashlib
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v58'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.original_chain_lift_20261115_v55.worker import eq
cubic=json.loads((R/'research/torsion_shifted_cubic_20261115_v58/raw.json').read_text());assert cubic['error']is None
for row in cubic['out']:
 assert row['chart']['complete']and row['chart']['original_tuples']
 for values in row['chart']['original_tuples']:
  a,b,c,d,e,u,v,w=values;z=row['target'][0];assert all(eq(x,y,t,1,131 if row['n']==7 else 2053)==0 for x,y,t in[(a,b,u),(u,c,v),(v,d,w),(w,e,z)])
union=json.loads((R/'research/complete_chart_union_20261115_v58/raw.json').read_text());assert union['error']is None;out=union['out'];count=0
for zs,roots in out['all_original_roots'].items():
 z=int(zs);assert len(roots)==len({tuple(v)for v in roots})
 for a,b,c,d,e,u,v,w in roots:assert all(eq(x,y,t,1,131)==0 for x,y,t in[(a,b,u),(u,c,v),(v,d,w),(w,e,z)]);count+=1
assert count==out['complete_tiny_original_tuples']==out['main_original_tuples']+out['restored_original_tuples']==886
assert out['restored_original_tuples']==26 and out['target_count']==66
print('Hashes, restored counterexamples, all886 original tuples and complete-chart count arithmetic verified. No global hard solve or genuineGB speed claim.')
