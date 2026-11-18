from pathlib import Path
import json,hashlib
C=Path(__file__).resolve().parent
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
A=Path('research/residue_lift_sieve_20261116_v84');B=Path('research/residue_cover_certificate_20261116_v84b');a=json.loads((A/'raw.json').read_text());b=json.loads((B/'raw.json').read_text());assert a['error']is None and b['error']is None;o=a['out'];v=b['out'];assert(o['positive_controls'],o['local_oracles'])==(64,1408);assert[o['leading_surviving_H_count'],o['leading_surviving_lift_pairs']]==[3*(1<<70),1<<73];assert all(r['final_lifts']for r in o['records']if r['supported_truth']);assert(v['toy_cases'],v['exact_first7_cover_size'])==(71,18552757630176)
from research.residue_lift_sieve_20261116_v84.kernel import code,divrem,mul
from research.residue_cover_certificate_20261116_v84b.worker import apply
D=json.loads((B/'DATA.json').read_text());rows=[[int(x,16)for x in s.split()]for s in Path(D['matrix']).read_text().splitlines()[1:]];base=rows[0];hc=rows[1:];assert base[4]>>17==1 and all(c[4]==0 for c in hc);cols=[0]*76;offset=0
for s,m in D['residue_fields']:
 for j,c in enumerate(hc):cols[j]^=(code(c,s,m)&((1<<(4*s))-1))<<offset
 for j in range(3):cols[73+j]^=divrem(mul(1<<j,D['modulus']),m)[1]<<offset
 offset+=4*s
assert all(apply(cols,x)==1<<i for i,x in enumerate(v['right_inverse']));assert all(apply(cols,x)==0 for x in v['kernel_basis']);print('C84 hashes,exact cover certificate,local outcomes and retained limitations verified; no runtime ratio')
