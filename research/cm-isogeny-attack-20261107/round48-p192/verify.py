from pathlib import Path
import json
from math import comb
c=json.loads(Path('certificate.json').read_text());r=c['r'];checked=0
for row in c['positive_cases']:
 H={(a,b):v for a,b,v in row['polynomial']};d=max(a+b for a,b in H);assert 0<d<r
 x=H.get((d-1,1),0)*pow(d*H[d,0]%r,-1,r)%r
 assert x==row['public_secret']==row['recovered']
 # Independently substitute a=U-xV and b=V, collect every coefficient.
 out={}
 for (a,b),v in H.items():
  for j in range(a+1):
   key=(a-j,b+j);out[key]=(out.get(key,0)+v*comb(a,j)*pow(-x,j,r))%r
 assert all(v==0 for (a,b),v in out.items() if b>0)
 assert all(a%5==0 for (a,b),v in out.items() if v);checked+=1
assert c['coordinate_polynomial_bound']['minimum_degree']==-(-(r-3)//3)
assert r<c['p'] and c['p']>3
assert c['label_collision_counterexample']['values'][0]!=c['label_collision_counterexample']['values'][1]
x=17;L=c['label_collision_counterexample']['labels'];assert (L[0][0]+x*L[0][1]-L[1][0]-x*L[1][1])%r==0
out={'all_checks_pass':True,'independent_top_degree_extractions':checked,'independent_full_substitution_checks':checked,'counterexample':True,'degree_floor_integer_check':True,'scope':'finite certificates; universal proof remains the written algebraic derivation'}
Path('verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
