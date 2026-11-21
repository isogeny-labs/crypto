from pathlib import Path
import json,sys,hashlib,base64,tarfile,io,itertools
C=Path(__file__).resolve().parent;R=C.parents[1];sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.table_propagation_20261116_v110.kernel import propagate,Budget
P='research/table_propagation_20261116_v110/';dom=json.loads(saved[P+'domains.json']);result=propagate(dom['domains'],12,dom['target'],Budget());assert result['kept']==[[tuple(x)for x in d]for d in dom['domains']]and result['exact_single_relation_tuple_count']==274316578275
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import apply,code,pmul,rem
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
raw=json.loads(saved['research/conditional_tables_20261116_v110b/profile_raw.json'])['out'];config=json.loads((R/'research/systematic_residue_20261116_v109/DATA.json').read_text());mapping=json.loads((R/'research/systematic_residue_20261116_v109/coordinates.json').read_text());rows=matrix(str(R/config['matrix']));E,eb,widths,first,F,fb,last,details=build(R/'research/residue_quotient_20261116_v99b');tables=first+[last]
for s,m in config['fields'][8:]:tables.append(allowed(s,rem(rows[0][4],m),multiplication(s,m)))
assert [hashlib.sha256(json.dumps(sorted(t)).encode()).hexdigest()for t in tables]==raw['field_table_hashes']
counts=[0]*8;controls=0
for cell in raw['cells']:
 base=sum(x<<o for x,o in zip(cell['fixed_codes'],[16,28,40,56]));accepted=[];eighth=0
 for a,b,c in itertools.product(*[sorted(t)for t in first[:3]]):
  y=base^a^(b<<4)^(c<<8);theta0=mapping['offset']^apply(mapping['R'],y)
  for z in range(16):
   controls+=1;theta=theta0^apply(mapping['K'],z)
   if apply(F,theta)^fb not in last:continue
   eighth+=1;counts[0]+=1;H=decode(rows,theta);H[0]^=pmul(theta>>73,config['modulus'])
   if not valid_guard(H,theta>>73):continue
   counts[1]+=1
   for j in range(8,14):
    if code(H,*config['fields'][j])not in tables[j]:break
    counts[j-6]+=1
   else:accepted.append(theta)
 assert eighth==cell['eighth_count']and accepted==cell['accepted_fourteen_and_guard']
assert counts==raw['stage_counts_eighth_guard_then_six']and controls==64512
print('C110 hashes, no-cut support fixed point and all64512conditional assignments independently replayed;32necessary cells empty;remaining outer cover and original all-roots goal unresolved.')
