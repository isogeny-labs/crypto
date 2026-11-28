from pathlib import Path
from collections import Counter
import hashlib,json
from rootfinder.binaryfield import BinaryField
from research.positive_binary_charts_20261115_v53 import math as m
P=Path(__file__).resolve().parent;R=P.parents[1]
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==digest
raw=json.loads((R/'research/positive_binary_charts_20261115_v53/raw.json').read_text());assert raw['error']is None;f=BinaryField(83,0x800000000200000000007)
for c in raw['out']['cases']:
 z,y=c['norm_target_Q'];assert m.on(f,c['A'],c['B'],(z,y));assert m.polynomial.norm(f,c['A'],c['B'],z,c['norm_coefficients'])==tuple(c['H']);assert c['factorization']['roots']==[list(v)for v in sorted(Counter(c['expected_source_x']).items())]
 assert m.residuals_literal(c['original_field_variables'],z,c['B'],f.modulus)==[0]*4;bits=int(c['original_root_bits_hex'],16);values=[(bits>>(18*i))&((1<<18)-1)for i in range(5)]+[(bits>>(90+83*i))&((1<<83)-1)for i in range(3)];assert values==c['original_field_variables'];bad=values[:];bad[5]^=1<<c['rejected_mutation_bit'];assert any(m.residuals_literal(bad,z,c['B'],f.modulus))
print('Verified4positive full-width norm witnesses, complete known factors,339-bit original memberships and rejected mutations; not global root completeness.')
