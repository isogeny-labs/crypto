"""Paid exact preset, primality, group-order and public-target construction."""
from pathlib import Path
import json,hashlib
from lab.research.nist_fields import build_nist_target
root=Path(__file__).resolve().parents[2];rows=[];lines=[]
dep=json.load(open(Path(__file__).with_name('EXTERNAL_DEPENDENCIES.json')));library=root/dep['library_relative_path'];assert hashlib.sha256(library.read_bytes()).hexdigest()==dep['expected_sha256'],'pre-registered exact-proof library changed'
for name,d in [('P-192',40),('P-224',47),('P-256',53),('P-384',79),('P-521',106)]:
 item=build_nist_target(name,19,flint_library=library);record=item['preset'];source=root/record['pinned_source'];assert hashlib.sha256(source.read_bytes()).hexdigest()==record['pinned_source_sha256'],'preset source pin changed'
 p=record['p'];x,y=item['target'];neg_y=(-y)%p
 lines.append(' '.join(map(str,[name,p.bit_length(),d,p,record['a'],record['b'],x,neg_y])))
 rows.append(dict(name=name,dimension=d,preset_source=record['pinned_source'],preset_source_sha256=record['pinned_source_sha256'],target_positive_scalar=19,norm_target_sign=-1,target_x_hex=hex(x),target_negative_y_hex=hex(neg_y),proof=item['proof']))
print(json.dumps(dict(rows=rows,native_input='\n'.join(lines)+'\n'),separators=(',',':')))
