from pathlib import Path
import json,importlib.util
from circuit import extract
root=Path(__file__).resolve().parent;s=importlib.util.spec_from_file_location('ec',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec)
rows=[]
for x in [0,1,2,17,20261107]:
 for exponent in [5,10,15]:
  for a,b in [(2,3),(-x,1),(-1-x,1)]:
   z=extract(ec.r,x,a,b,exponent);z['public_secret']=x
   assert z['recovered'] in [None,x]
   if z['recovered'] is not None:assert ec.mul(z['recovered'],ec.G)==ec.mul(x,ec.G)
   rows.append(z)
toy=[extract(11,3,a,b) for a in range(11) for b in range(11)]
assert all(z['recovered']==3 if z['Wa'] else z['recovered'] is None for z in toy)
negative=extract(ec.r,17,2,3,mode='nondescending');assert negative['recovered']!=17;assert ec.mul(negative['recovered'],ec.G)!=ec.mul(17,ec.G)
frobenius=extract(ec.r,17,2,3,5*ec.r);assert frobenius['Wa']==frobenius['Wb']==0
out={'all_checks_pass':True,'r':ec.r,'fixtures':rows,'toy':toy,'negative':negative,'frobenius':frobenius,'counts_scope':'jet circuit and extractor only; input construction and candidate scalar verification excluded'}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n')
print('FIXTURES',len(rows),'TOY',len(toy),'TOY_EXTRACTED',sum(z['recovered'] is not None for z in toy),'TOY_POLES_EXTRACTED',sum(z['D'][0]==0 and z['recovered'] is not None for z in toy))
