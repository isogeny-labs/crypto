#!/usr/bin/env python3
"""Independently replay the s=7 modular solve and target descent receipt."""
from pathlib import Path
import hashlib
import json

here=Path(__file__).resolve().parent
source=here/'s7_attack_v2.sage'
transfer=here/'s7_transfer.json'
checkpoint=here/'s7_relations_v2.json'
result=here/'s7_attack_v2.json'
c=json.loads(checkpoint.read_text())
d=json.loads(result.read_text())
source_hash=hashlib.sha256(source.read_bytes()).hexdigest()
transfer_hash=hashlib.sha256(transfer.read_bytes()).hexdigest()
assert c['source_sha256']==d['source_sha256']==source_hash
assert c['transfer_receipt_sha256']==d['transfer_receipt_sha256']==transfer_hash
assert c['r']==d['r']==1070455751
assert c['factor_base_size']==d['factor_base_size']==1093
assert c['factor_base_coords']==d['factor_base_coords']
assert c['relation_rows']==d['relation_rows']
assert c['relation_rhs']==d['relation_rhs']
assert c['accepted_relations']==d['accepted_relations']==len(d['relation_rows'])
assert c['relation_rank']==d['relation_rank']==d['factor_base_size']
assert c['relation_attempts']==d['relation_attempts']
r=d['r'];h=c['h'];logs=d['factor_base_projected_logs']
assert len(logs)==d['factor_base_size']
assert all(sum(v*logs[j] for j,v in row)%r==rhs
           for row,rhs in zip(d['relation_rows'],d['relation_rhs']))
desc=sum(v*logs[j] for j,v in d['descent_row'])%r
recovered=(desc*pow(h,-1,r)-d['descent_offset'])%r
assert recovered==d['recovered']==d['secret']==c['secret']
timing=d['timing']
parts=('initial_setup_cpu_s','transfer_cpu_s','setup_factor_base_cpu_s',
       'relations_linear_algebra_verify_cpu_s','descent_verify_cpu_s')
assert all(timing[k]>=0 for k in parts)
assert abs(sum(timing[k] for k in parts)-timing['ghs_total_cpu_s'])<1e-6
out={'schema':'ghs.odd5.s7-attack-modular-audit/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'checkpoint_sha256':hashlib.sha256(checkpoint.read_bytes()).hexdigest(),
     'result_sha256':hashlib.sha256(result.read_bytes()).hexdigest(),
     'relations_checked':len(d['relation_rows']),
     'rank_reported':d['relation_rank'],'recovered_scalar':recovered,
     'all_modular_equations_checked':True,'descent_checked':True}
target=here/'s7_attack_audit.json'
assert not target.exists(), 'refusing to overwrite audit receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps(out,sort_keys=True))
