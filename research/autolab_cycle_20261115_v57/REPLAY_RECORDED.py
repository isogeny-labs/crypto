import pathlib,json,hashlib
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v57'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.original_chain_lift_20261115_v55.worker import eq
from rootfinder.binaryfield import BinaryField
from research.binary_norm_coverage_20261115_v57.worker import trace
raw=json.loads((R/'research/binary_norm_coverage_20261115_v57/raw.json').read_text());assert raw['error']is None
for row in raw['out']['actual']:assert all(row[k]for k in['O_triple_excluded','P0_triple_excluded','P0_pair_excluded','target_outside','translated_target_outside'])
ghost=json.loads((R/'research/binary_norm_ghost_20261115_v57/raw.json').read_text());assert ghost['error']is None
for row in ghost['out']['cases']:
 for v in row['finite_original_tuples']:
  a,b,c,d,e,u,vv,w=v;assert all(eq(x,y,z,row['B'],row['modulus'])==0 for x,y,z in[(a,b,u),(u,c,vv),(vv,d,w),(w,e,row['target'][0])])
 assert row['multiplicity_hermite_witnesses']and row['repeated_ghost_included']
negative=json.loads((R/'research/binary_norm_guard_counterexample_20261115_v57/raw.json').read_text());assert negative['error']is None
for row in negative['out']:
 f=BinaryField(row['n'],row['modulus']);a,b,c,d,e,u,v,w=row['original_values'];z=row['target'][0]
 assert all(eq(x,y,zv,row['B'],row['modulus'])==0 for x,y,zv in[(a,b,u),(u,c,v),(v,d,w),(w,e,z)])
 assert trace(f,row['H'][4]^z)==1
print('Evidence hashes, actual coverage guards, positive ghost original roots and exact trace counterexamples verified. Global h/k search and genuineGB release gate remain open.')
