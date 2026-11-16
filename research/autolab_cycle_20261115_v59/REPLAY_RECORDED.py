import pathlib,json,hashlib
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v59'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from rootfinder.binaryfield import BinaryField
from research.original_chain_lift_20261115_v55.worker import eq
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
raw=json.loads((R/'research/full_shifted_cubic_20261115_v59/raw.json').read_text());assert raw['error']is None;o=raw['out'];assert o['complete_cubic_chart'];stats=o['native']['stats'];assert stats['outer_visited']==stats['outer_cardinality_exact']==262144;assert stats['candidates_visited']==stats['candidates_planned']==261941
inst=json.loads((R/'research/native_shifted_cubic_20261115_v59/full_width_A1_instance.json').read_text());f=BinaryField(83,inst['modulus'])
for row in o['verified_norms']:assert tuple(row['C'])==coefficients(f,1,1,inst['z'],inst['source_ordinate'],row['h'],row['r'])
for a,b,c,d,e,u,v,w in o['original_tuples']:
 z=inst['original_target'][0];assert all(eq(x,y,t,1,f.modulus)==0 for x,y,t in[(a,b,u),(u,c,v),(v,d,w),(w,e,z)])
assert len(o['original_tuples'])==o['original_tuple_count_exact']==6;assert len(o['verified_norms'])==o['supported_norm_count_exact']==1
print('Hashes, complete outer/candidate counts, accepted coefficients and6 original tuples verified. Main/global GB gate remains open.')
