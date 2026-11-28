import pathlib,json,hashlib
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v60'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
failed=json.loads((R/'research/general_binary_cubic_20261115_v60/raw.json').read_text());assert failed['error'].startswith('StopIteration')
repair=json.loads((R/'research/general_binary_cubic_repair_20261115_v60/raw.json').read_text());assert repair['error']is None;assert repair['out']['tiny_controls']['complete_field_parameter_states']==640
from rootfinder.binaryfield import BinaryField
from research.original_chain_lift_20261115_v55.worker import eq
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
raw=json.loads((R/'research/full_ordinary_cubic_20261115_v60/raw.json').read_text());assert raw['error']is None;o=raw['out'];assert o['complete_original_three_source'];stats=o['native']['stats'];assert stats['outer_visited']==stats['outer_cardinality_exact']==262144;assert stats['candidates_visited']==stats['candidates_planned']==261457
v=json.loads((R/'research/general_binary_cubic_repair_20261115_v60/full_ordinary_instance.json').read_text());f=BinaryField(83,v['modulus'])
for row in o['verified_norms']:assert tuple(row['C'])==coefficients(f,v['A'],v['B'],v['z'],v['source_ordinate'],row['h'],row['r'])
for a,b,c,u in o['original_tuples']:assert not eq(a,b,u,v['B'],f.modulus)and not eq(u,c,v['z'],v['B'],f.modulus)
assert len(o['original_tuples'])==o['original_tuple_count_exact']==6;assert o['original_variable_count']==137
print('Hashes, preserved failed harness, repaired controls and complete original three-source root replay verified. Five-source/genuineGB gate remains open.')
