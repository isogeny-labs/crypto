from pathlib import Path
import json,hashlib
from model import *
root=Path(__file__).resolve().parent;inputs=json.loads((root/'input.json').read_text());p,r,a,b=inputs['p'],inputs['r'],inputs['a'],inputs['b'];G,Q=inputs['G'],inputs['Q'];k=r.bit_length();rows=[]
for n in [0,1,2,r-1,r,2**k-1]:
 states,values,branches,eqs=circuit(n,G,Q,p,a,b,k);assert states==[point(mul(n,G,p,a)),point(mul(n,Q,p,a))]
 rows.append({'digits_integer':n,'output':states,'witness_sha256':hashlib.sha256(json.dumps(values,separators=(',',':')).encode()).hexdigest(),'variables':len(values),'equations':eqs,'branch_counts':branches})
tp=17;ta=tb=2;tG=(5,1);tr=19;tQ=mul(7,tG,tp,ta);toy=[]
for n in range(32):
 states,values,branches,eqs=circuit(n,tG,tQ,tp,ta,tb,5);toy.append({'n':n,'output':states,'variables':len(values),'equations':eqs})
assert len({tuple(tuple(P) for P in row['output']) for row in toy})==19
# The equation degree is checked symbolically once; coordinates of the selected input are linear in a bit.
import sympy as S
names='x y o X Y O i j c d f s z e';x,y,o,X,Y,O,i,j,cc,d,f,ss,z,e=S.symbols(names);xc,yc=S.symbols('xc yc');u=e*xc;v=e*yc;q=1-e
import inspect
source=inspect.getsource(residuals).replace('def residuals(', 'def polynomial_template(').replace('return [x%p for x in out]','return out')
ns={};exec(source,ns);polys=ns['polynomial_template']((x,y,o),(u,v,q),(X,Y,O),[i,j,cc,d,f],ss,z,0,ta,tb)
variables=S.symbols(names);degrees=[S.Poly(S.expand(v),*variables).total_degree() for v in polys];assert len(degrees)==44 and max(degrees)==4
out={'all_checks_pass':True,'input':inputs,'k':k,'P192':rows,'toy':{'p':tp,'a':ta,'b':tb,'r':tr,'G':tG,'Q':tQ,'rows':toy,'projected_points':19},'template_equations':44,'template_degrees':degrees,'size_formula':{'variables':'21k','equations':'89k','maximum_degree':4},'scope':'compact polynomial graph, not elimination or quotient evaluation'}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('P192',k,21*k,'variables',89*k,'equations','TOY_PROJECTED',19)
