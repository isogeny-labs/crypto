from pathlib import Path
import json,hashlib,sys
C=Path(__file__).resolve().parent;sys.path.insert(0,str(C.parents[1]))
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
P=Path('research/twist_auxiliary_20261116_v86');a=json.loads((P/'twists_raw.json').read_text());b=json.loads((P/'auxiliary_raw.json').read_text());v=json.loads(Path('research/twist_three_isogeny_20261116_v86b/raw.json').read_text());assert all(r['error']is None for r in[a,b,v]);assert(a['out']['triples'],a['out']['coordinate_types'])==(35164,124);assert(b['out']['homomorphism_pairs'],b['out']['source_relation_tuples'],b['out']['S3_forward_checks'])==(2408,1881,608)
from research.twist_three_isogeny_20261116_v86b.curve import on,scale,phi,target,add
rows=v['out']['NIST'];assert len(rows)==5;assert[r['name']for r in rows if r['twist_rational3kernel']]==['P-224','P-256']
for r in rows:
 assert r['proved_p']['proven_prime']and r['proved_N']['proven_prime']and r['twist_order']==2*(r['p']+1)-r['original_order']
 if not r['twist_rational3kernel']:continue
 p=r['p'];A=r['twist_A'];B=r['twist_B'];T=tuple(r['kernel_point']);assert on(T,p,A,B)and scale(3,T,p,A)is None;assert target(T,p,A,B)==(r['auxiliary_A'],r['auxiliary_B'])
 for U,V in r['auxiliary_generator_images']:
  U=tuple(U);V=None if V is None else tuple(V);assert phi(U,T,p,A)==V and on(V,p,r['auxiliary_A'],r['auxiliary_B'])
print('C86 hashes,exact kernel points,auxiliary coordinates and recorded transfer outcomes verified; prime-proof replay needs the identified FLINT library')
