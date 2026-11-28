from pathlib import Path
import json,hashlib
from research.general_binary_points_repair_20261115_v52.worker import run
P=Path(__file__).resolve().parent;R=P.parents[1];raws={}
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;raws[name]=json.loads(data)
x=raws['research/torsion_triple_20261115_v52/raw.json'];assert x['error']is None;a=x['out']['actual'];mod=a['modulus'];cols=[]
for i in range(70):
 j=i if i<52 else i-52;v=1<<(2*j)
 while v.bit_length()>=mod.bit_length():v^=mod<<(v.bit_length()-mod.bit_length())
 cols.append(v^(1<<j)if i<52 else v)
piv={}
for v in cols:
 while v:
  p=v.bit_length()-1
  if p in piv:v^=piv[p]
  else:piv[p]=v;break
assert len(piv)==61 and a['kernel_dimension']==9
for v in a['expected_low_square_basis']:
 image=0
 for j,c in enumerate(cols):
  if v>>j&1:image^=c
 assert image==0
bad=raws['research/general_binary_points_20261115_v52/raw.json'];assert bad['error']is not None;good=raws['research/general_binary_points_repair_20261115_v52/raw.json'];assert good['error']is None and json.loads(json.dumps(run()))==good['out']
print('Verified rank61/kernel9 and repeated complete general-coordinate zero/pole oracle; failed harness preserved.')
